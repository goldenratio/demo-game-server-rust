import { Container } from 'pixi.js';
import { Entity, System, World } from 'super-ecs';

import { SpriteComponent } from '../components';
import { DisposeBag } from '../utils/dispose-bag';

export class SpriteSystem extends System {
	private readonly _container: Container;
	private _disposeBag?: DisposeBag;

	constructor(container: Container) {
		super();
		this._container = container;
	}

	removedFromWorld(world: World): void {
		super.removedFromWorld(world);
    if(this._disposeBag) {
      this._disposeBag.dispose();
      this._disposeBag = undefined;
    }
	}

	addedToWorld(world: World): void {
		super.addedToWorld(world);

		this._disposeBag = new DisposeBag();

		this._disposeBag.completable$(world.entityAdded$([SpriteComponent.TYPE])).subscribe((entity: Entity) => {
			const spriteComponent = entity.getComponent<SpriteComponent>(SpriteComponent.TYPE);
			if (!spriteComponent) {
				return;
			}

			const { sprite } = spriteComponent;
			if (sprite) {
				this._container.addChild(sprite);
			}
		});

		this._disposeBag.completable$(world.entityRemoved$([SpriteComponent.TYPE])).subscribe((entity: Entity) => {
			const spriteComponent = entity.getComponent<SpriteComponent>(SpriteComponent.TYPE);
			if (!spriteComponent) {
				return;
			}

			const { sprite } = spriteComponent;
			if (sprite) {
				this._container.removeChild(sprite);
			}
		});
	}
}
