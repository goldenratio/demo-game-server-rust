import { Entity, System, World } from 'super-ecs';

import { CommsManager } from '../service';
import { DisposeBag } from '../utils/dispose-bag';
import { OpponentPlayerComponent, PositionComponent, SpriteComponent } from '../components';
import { Sprite, Texture } from 'pixi.js';

export class OpponentPlayerSystem extends System {
	private readonly _service: CommsManager;
	private _disposeBag?: DisposeBag;

	constructor(commsManager: CommsManager) {
		super();
		this._service = commsManager;
	}

	removedFromWorld(world: World): void {
		super.removedFromWorld(world);
		if (this._disposeBag) {
			this._disposeBag.dispose();
			this._disposeBag = undefined;
		}
	}

	addedToWorld(world: World) {
		super.addedToWorld(world);
		this._disposeBag = new DisposeBag();

		this._disposeBag.completable$(this._service.peerPlayerJoined$).subscribe(data => {
			const peerEntity = createPeerEntity(data.playerId);
			world.addEntity(peerEntity);
		});

		this._disposeBag.completable$(this._service.peerPlayerUpdate$).subscribe(playerList => {
			playerList.forEach(data => {
				const entities = world.getEntities([OpponentPlayerComponent.TYPE, PositionComponent.TYPE]);
				const peerEntity: Entity | undefined = entities.find(
					et => et.getComponent<OpponentPlayerComponent>(OpponentPlayerComponent.TYPE).playerId === data.playerId
				);

				if (peerEntity) {
					// peer already exist, update only position
					const position = peerEntity.getComponent<PositionComponent>(PositionComponent.TYPE);
					position.x = data.x;
					position.y = data.y;
				} else {
					// create new one
					console.log('create new ', data.playerId);
					const peerEntity = createPeerEntity(data.playerId, data.x, data.y);
					world.addEntity(peerEntity);
				}
			});
		});

		this._disposeBag.completable$(this._service.peerPlayerLeft$).subscribe(data => {
			const peerEntities = world.getEntities([OpponentPlayerComponent.TYPE]).filter(entity => {
				const comp = entity.getComponent<OpponentPlayerComponent>(OpponentPlayerComponent.TYPE);
				return comp && comp.playerId === data.playerId;
			});
			peerEntities.forEach(entity => world.removeEntity(entity));
		});
	}
}

function createPeerEntity(playerId: string, x: number = 0, y: number = 0): Entity {
	const entity = new Entity();
	entity
		.addComponent(new PositionComponent({ x, y }))
		.addComponent(new OpponentPlayerComponent({ playerId: playerId }))
		.addComponent(
			new SpriteComponent({
				sprite: new Sprite(Texture.from('p2')),
			})
		);
	return entity;
}
