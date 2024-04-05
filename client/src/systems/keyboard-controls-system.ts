import { Entity, System, World } from 'super-ecs';
import { fromEvent } from 'rxjs';
import { DisposeBag } from '../utils/dispose-bag';
import { KeyboardControlsComponent } from '../components';

export class KeyboardControlsSystem extends System {
	private _disposeBag?: DisposeBag;

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

		this._disposeBag.completable$(world.entityAdded$([KeyboardControlsComponent.TYPE])).subscribe((entity: Entity) => {
			const keyboardControlsComponent = entity.getComponent<KeyboardControlsComponent>(KeyboardControlsComponent.TYPE);
			if (!keyboardControlsComponent) {
				return;
			}

			this._disposeBag.completable$(fromEvent(document, 'keydown')).subscribe((event: KeyboardEvent) => {
				if (event.key === 'ArrowRight') {
					keyboardControlsComponent.isRight = true;
					keyboardControlsComponent.isLeft = false;
				} else if (event.key === 'ArrowLeft') {
					keyboardControlsComponent.isRight = false;
					keyboardControlsComponent.isLeft = true;
				}

				if (event.key === 'ArrowUp') {
					keyboardControlsComponent.isUp = true;
					keyboardControlsComponent.isDown = false;
				} else if (event.key === 'ArrowDown') {
					keyboardControlsComponent.isUp = false;
					keyboardControlsComponent.isDown = true;
				}
			});

			this._disposeBag.completable$(fromEvent(document, 'keyup')).subscribe((event: KeyboardEvent) => {
				if (event.key === 'ArrowRight') {
					keyboardControlsComponent.isRight = false;
					// keyboardControlsComponent.isLeft = false;
				} else if (event.key === 'ArrowLeft') {
					// keyboardControlsComponent.isRight = false;
					keyboardControlsComponent.isLeft = false;
				}

				if (event.key === 'ArrowUp') {
					keyboardControlsComponent.isUp = false;
					// keyboardControlsComponent.isDown = false;
				} else if (event.key === 'ArrowDown') {
					// keyboardControlsComponent.isUp = false;
					keyboardControlsComponent.isDown = false;
				}
			});
		});
	}
}
