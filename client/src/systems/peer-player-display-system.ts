import {System, World} from "super-ecs";
import {CommsManager} from "../service";
import {Disposable, DisposeBag} from "../utils/dispose-bag";
import {Container, Sprite, Texture} from "pixi.js";

export class PeerPlayerDisplaySystem extends System {
  private readonly _commsManager: CommsManager;
  private readonly _container: Container;
  private _disposeBag?: DisposeBag;

  constructor(commsManager: CommsManager, container: Container) {
    super();
    this._commsManager = commsManager;
    this._container = container;
  }

  removedFromWorld(world: World): void {
    super.removedFromWorld(world);
    if (this._disposeBag) {
      this._disposeBag.dispose();
      this._disposeBag = undefined;
    }
  }

  addedToWorld(world: World): void {
    super.addedToWorld(world);
    this._disposeBag = new DisposeBag();

    const ghost = new GhostPlayerManager(this._container);
    this._disposeBag.add(ghost);

    this._disposeBag.completable$(this._commsManager.peerPlayerUpdate$).subscribe(data => {
      ghost.updatePeerPlayerPosition(data.playerId, data.x, data.y);
    });

    this._disposeBag.completable$(this._commsManager.removePeerPlayer$).subscribe(data => {
      ghost.removePeerPlayer(data.playerId);
    });
  }
}

class GhostPlayerManager implements Disposable {
  private readonly _container: Container;
  private readonly _peerPlayerMap = new Map<string, Sprite>();

  constructor(container: Container) {
    this._container = container;
  }

  dispose(): void {
    //
  }

  updatePeerPlayerPosition(playerId: string, x: number, y: number): void {
    let peerSprite: Sprite | undefined = this._peerPlayerMap.get(playerId);
    if (!peerSprite) {
      peerSprite = new Sprite(Texture.from('p2'));
      this._peerPlayerMap.set(playerId, peerSprite);
      this._container.addChild(peerSprite);
    }

    peerSprite.position.set(x, y);
  }

  removePeerPlayer(playerId: string): void {
    let peerSprite: Sprite | undefined = this._peerPlayerMap.get(playerId);
    if (peerSprite) {
      this._container.removeChild(peerSprite);
      this._peerPlayerMap.delete(playerId);
    }
  }
}
