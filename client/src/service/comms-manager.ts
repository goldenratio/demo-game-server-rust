import {Disposable, DisposeBag} from '../utils/dispose-bag';
import {fromEvent, Observable, ReplaySubject, Subject} from 'rxjs';
import {Builder, ByteBuffer} from "flatbuffers";
import {GameEvent, GameEventType, Gameplay, PlayerControl, PlayerPosition} from "../gen/gameplay-fbdata";

interface PeerPlayerUpdate {
  readonly playerId: string;
  readonly x: number;
  readonly y: number;
}

export class CommsManager implements Disposable {
	private readonly _connectedSubject$ = new ReplaySubject<void>(1);
  private readonly _opponentPlayerUpdateSubject$ = new Subject<PeerPlayerUpdate>();
  private readonly _removePeerPlayerSubject$ = new Subject<{ readonly playerId: string }>();
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
    })

    this._disposeBag.completable$(fromEvent(this._socket, 'message')).subscribe((event: MessageEvent) => {
      const bytes = new Uint8Array(event.data);
      // console.log('message from server, ', bytes);
      const buffer = new ByteBuffer(bytes);
      const gameEvent = GameEvent.getRootAsGameEvent(buffer);
      const eventType = gameEvent.eventType();
      
      if (eventType === GameEventType.PlayerPositionUpdate) {
        const pos = gameEvent.playerPosition();
        this._opponentPlayerUpdateSubject$.next({
          playerId: gameEvent.playerId(),
          x: pos.x(),
          y: pos.y()
        })
      } else if (eventType === GameEventType.PlayerLeft) {
        const playerId = gameEvent.playerId();
        this._removePeerPlayerSubject$.next({
          playerId: playerId
        });
      } else if (eventType === GameEventType.PlayerJoined) {
        // TODO
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

  get peerPlayerUpdate$(): Observable<PeerPlayerUpdate> {
    return this._opponentPlayerUpdateSubject$.asObservable();
  }

  get removePeerPlayer$(): Observable<{ readonly playerId: string }> {
    return this._removePeerPlayerSubject$.asObservable();
  }

  sendUpdates(): void {
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

    const playerIdOffset = builder.createString('2121');

    Gameplay.startGameplay(builder);

    Gameplay.addPlayerControls(builder, PlayerControl.createPlayerControl(builder, this._isUp, this._isDown, this._isLeft, this._isRight));
    Gameplay.addPlayerPosition(builder, PlayerPosition.createPlayerPosition(builder, this._posX, this._posY));
    Gameplay.addPlayerId(builder, playerIdOffset);

    const offset = Gameplay.endGameplay(builder);
    builder.finish(offset);

    const bytes = builder.asUint8Array();
    this._socket.send(bytes);
  }
}
