import { Disposable, DisposeBag } from '../utils/dispose-bag';
import { fromEvent, Observable, ReplaySubject, Subject } from 'rxjs';
import { Builder, ByteBuffer } from 'flatbuffers';
import {
	GameReponseEvent,
	GameRequestEvent,
	GameWorldUpdate,
	PlayerControl,
	PlayerMoved,
	RemotePeerJoined,
	RemotePeerLeft,
	RemotePeerPositionUpdate,
	RequestMessages,
	ResponseMessage,
	Vec2,
	WeaponFired,
} from '../gen/gameplay-fbdata';

interface PeerPlayerUpdate {
	readonly playerId: string;
	readonly x: number;
	readonly y: number;
}

export class CommsManager implements Disposable {
	private readonly _connectedSubject$ = new ReplaySubject<void>(1);
	private readonly _peerPlayerUpdateSubject$ = new ReplaySubject<ReadonlyArray<PeerPlayerUpdate>>(1);
	private readonly _peerPlayerLeftSubject$ = new Subject<{ readonly playerId: string }>();
	private readonly _peerPlayerJoinedSubject$ = new Subject<{ readonly playerId: string }>();
	private readonly _disposeBag = new DisposeBag();
	private readonly _socket: WebSocket;

	private _isSocketClosed: boolean = false;
	private _posX: number = 0;
	private _posY: number = 0;
	private _isLeft: boolean = false;
	private _isRight: boolean = false;
	private _isUp: boolean = false;
	private _isDown: boolean = false;

	private _isDirty: boolean = false;

	constructor() {
		this._socket = new WebSocket('ws://localhost:8090/ws');
		this._socket.binaryType = 'arraybuffer';

		this._disposeBag.completable$(fromEvent(this._socket, 'open')).subscribe(() => {
			console.log('socket connection opened!');
			this._connectedSubject$.next();
			this._connectedSubject$.complete();
		});

		this._disposeBag.completable$(fromEvent(this._socket, 'message')).subscribe((event: MessageEvent) => {
			const bytes = new Uint8Array(event.data);
			// console.log('message from server, ', bytes);
			const buffer = new ByteBuffer(bytes);
			const gameResponseEvent = GameReponseEvent.getRootAsGameReponseEvent(buffer);
			const eventType = gameResponseEvent.msgType();

			// console.info('gameResponseEvent: ', eventType);

			if (eventType === ResponseMessage.RemotePeerJoined) {
				const joinData = RemotePeerJoined.getRootAsRemotePeerJoined(buffer);
				const msg: RemotePeerJoined = gameResponseEvent.msg(joinData);

				const playerData = msg.playerData();
				const playerPosition = msg.playerData().playerPosition();
				const playerId = BigInt(playerData.playerId()).toString();

				console.log('Remote player joined: ', playerId, { x: playerPosition.x(), y: playerPosition.y() });
				this._peerPlayerJoinedSubject$.next({
					playerId: playerId,
				});
			} else if (eventType === ResponseMessage.RemotePeerLeft) {
				const leaveData = RemotePeerLeft.getRootAsRemotePeerLeft(buffer);
				const msg: RemotePeerLeft = gameResponseEvent.msg(leaveData);

				const playerId = BigInt(msg.playerId()).toString();

				console.log('Remote player left: ', playerId);
				this._peerPlayerLeftSubject$.next({
					playerId: playerId,
				});
			} else if (eventType === ResponseMessage.RemotePeerPositionUpdate) {
				const updateDate = RemotePeerPositionUpdate.getRootAsRemotePeerPositionUpdate(buffer);
				const msg: RemotePeerPositionUpdate = gameResponseEvent.msg(updateDate);

				const playerData = msg.playerData();
				const playerPosition = msg.playerData().playerPosition();
				const playerId = BigInt(playerData.playerId()).toString();

				// console.log('RemotePeerPositionUpdate: ', playerId, { x: playerPosition.x(), y: playerPosition.y() });
				this._peerPlayerUpdateSubject$.next([
					<PeerPlayerUpdate>{
						playerId: playerId,
						x: playerPosition.x(),
						y: playerPosition.y(),
					},
				]);
			} else if (eventType === ResponseMessage.GameWorldUpdate) {
				const updateDate = GameWorldUpdate.getRootAsGameWorldUpdate(buffer);
				const msg: GameWorldUpdate = gameResponseEvent.msg(updateDate);

				const playerUpdateList = Array.from({ length: msg.playerDataListLength() }).map((_, index) => {
					const playerData = msg.playerDataList(index);
					const pos = playerData.playerPosition();
					const update: PeerPlayerUpdate = {
						playerId: playerData.playerId().toString(),
						x: pos.x(),
						y: pos.y(),
					};
					return update;
				});
				this._peerPlayerUpdateSubject$.next(playerUpdateList);
			}
		});

		this._disposeBag.completable$(fromEvent(this._socket, 'close')).subscribe(() => {
			console.log('socket closed!');
			this._isSocketClosed = true;
		});
	}

	dispose(): void {
		if (this._socket) {
			this._socket.close();
		}
		this._disposeBag.dispose();
	}

	setPlayerPosition(x: number, y: number): CommsManager {
		if (this._posX !== x || this._posY !== y) {
			this._posX = x;
			this._posY = y;
			this._isDirty = true;
		}
		return this;
	}

	setKeyPressed(isUp: boolean, isDown: boolean, isLeft: boolean, isRight: boolean): CommsManager {
		if (this._isUp !== isUp || this._isDown !== isDown || this._isLeft !== isLeft || this._isRight !== isRight) {
			this._isUp = isUp;
			this._isDown = isDown;
			this._isLeft = isLeft;
			this._isRight = isRight;
			this._isDirty = true;
		}
		return this;
	}

	get connected$(): Observable<void> {
		return this._connectedSubject$.asObservable();
	}

	get peerPlayerUpdate$(): Observable<ReadonlyArray<PeerPlayerUpdate>> {
		return this._peerPlayerUpdateSubject$.asObservable();
	}

	get peerPlayerJoined$(): Observable<{ readonly playerId: string }> {
		return this._peerPlayerJoinedSubject$.asObservable();
	}

	get peerPlayerLeft$(): Observable<{ readonly playerId: string }> {
		return this._peerPlayerLeftSubject$.asObservable();
	}

	sendPlayerMoved(): void {
		// console.log('sendUpdates ', playerPosition);
		if (this._isSocketClosed) {
			return;
		}

		if (!this._isDirty) {
			return;
		}

		this._isDirty = false;
		const builder = new Builder(0);
		builder.clear();

		PlayerMoved.startPlayerMoved(builder);
		PlayerMoved.addPlayerControls(
			builder,
			PlayerControl.createPlayerControl(builder, this._isUp, this._isDown, this._isLeft, this._isRight)
		);
		PlayerMoved.addPlayerPosition(builder, Vec2.createVec2(builder, this._posX, this._posY));
		const msgOffset = PlayerMoved.endPlayerMoved(builder);

		const offset = GameRequestEvent.createGameRequestEvent(builder, RequestMessages.PlayerMoved, msgOffset);
		builder.finish(offset);

		const bytes = builder.asUint8Array();
		this._socket.send(bytes);
	}

	sendWeaponFired(angle: number, power: number): void {
		const builder = new Builder(0);
		builder.clear();

		WeaponFired.startWeaponFired(builder);
		WeaponFired.addAngle(builder, angle);
		WeaponFired.addPower(builder, power);
		const msgOffset = WeaponFired.endWeaponFired(builder);

		const offset = GameRequestEvent.createGameRequestEvent(builder, RequestMessages.WeaponFired, msgOffset);
		builder.finish(offset);

		const bytes = builder.asUint8Array();
		this._socket.send(bytes);
	}
}
