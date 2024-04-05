import { Disposable, DisposeBag } from '../utils/dispose-bag';
import {Observable, ReplaySubject, Subject} from 'rxjs';
import { webSocket, WebSocketSubject } from 'rxjs/webSocket';
import {Builder, ByteBuffer} from "flatbuffers";
import {Gameplay, PlayerControl, PlayerPosition} from "../gen/gameplay";

interface PlayerControlsData {
  readonly up: boolean;
  readonly down: boolean;
  readonly left: boolean;
  readonly right: boolean;
}

interface PlayerPositionData {
  readonly x: number;
  readonly y: number;
}

interface PeerPlayerUpdate {
  readonly playerId: string;
  readonly x: number;
  readonly y: number;
}

export class CommsManager implements Disposable {
	private readonly _connectedSubject$ = new ReplaySubject<void>(1);
  private readonly _opponentPlayerUpdateSubject$ = new Subject<PeerPlayerUpdate>();
  private readonly _removePeerPlayerSubject$ = new Subject<{ readonly playerId: string }>();
	private readonly _socketSubject$: WebSocketSubject<any>;
	private readonly _disposeBag = new DisposeBag();
  private readonly _socket: WebSocket;

  private _isSocketClosed: boolean = false;

	constructor() {
		this._socketSubject$ = webSocket({
			url: 'ws://localhost:8090/ws',
			binaryType: 'arraybuffer',
			openObserver: {
				next: () => {
					console.log('socket connection opened!');
					this._connectedSubject$.next();
					this._connectedSubject$.complete();
				},
				error: err => {
					console.log('socket error! ', err);
				},
			},
		});

		// this._disposeBag.completable$(this._socketSubject$).subscribe(event => {
		// 	console.log('socket stream! ', event);
		// });

    this._socket = new WebSocket('ws://localhost:8090/ws');
    this._socket.binaryType = 'arraybuffer';
    this._socket.addEventListener('open', () => {
      console.log('socket connection opened!');
      this._connectedSubject$.next();
      this._connectedSubject$.complete();
    });

    this._socket.addEventListener('message', event => {
      const bytes = new Uint8Array(event.data);
      // console.log('message from server, ', bytes);
      const buffer = new ByteBuffer(bytes);
      const gameplay = Gameplay.getRootAsGameplay(buffer);
      // console.log(gameplay.playerId(), { x: gameplay.playerPosition().x(), y: gameplay.playerPosition().y() });
      const pos = gameplay.playerPosition();
      this._opponentPlayerUpdateSubject$.next({
        playerId: gameplay.playerId(),
        x: pos.x(),
        y: pos.y()
      })
    });

    this._socket.addEventListener('close', event => {
      console.log('socket closed!');
      this._isSocketClosed = true;
    });
	}

	dispose(): void {
		this._socketSubject$.complete();
		this._disposeBag.dispose();
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

  sendUpdates(playerControls: PlayerControlsData, playerPosition: PlayerPositionData): void {
    // console.log('sendUpdates ', playerPosition);
    if (this._isSocketClosed) {
      return;
    }
    const builder = new Builder(0);
    builder.clear();

    const playerIdOffset = builder.createString('2121');

    Gameplay.startGameplay(builder);

    Gameplay.addPlayerControls(builder, PlayerControl.createPlayerControl(builder, playerControls.up, playerControls.down, playerControls.left, playerControls.right));
    Gameplay.addPlayerPosition(builder, PlayerPosition.createPlayerPosition(builder, playerPosition.x, playerPosition.y));
    Gameplay.addPlayerId(builder, playerIdOffset);

    const offset = Gameplay.endGameplay(builder);
    builder.finish(offset);

    const bytes = builder.asUint8Array();
    // console.log('sending, ', bytes);

    this._socket.send(bytes);
    // this._socketSubject$.next(bytes);
  }
}
