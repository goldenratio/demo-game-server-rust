import { System, TickerDataLike } from 'super-ecs';
import { PlayerControlsComponent, PositionComponent } from '../components';
import {CommsManager} from "../service";

export class PlayerMovementSystem extends System {
	private readonly _stageWidth: number;
	private readonly _stageHeight: number;
  private readonly _service: CommsManager;

  constructor(service: CommsManager, props = { width: 800, height: 600 }) {
		super();
    this._service = service;
		this._stageWidth = props.width;
		this._stageHeight = props.height;
	}

	update(tickerData: TickerDataLike): void {
		const entities = this.world.getEntities([PositionComponent.TYPE, PlayerControlsComponent.TYPE]);

		if (entities.length === 0) {
			return;
		}

		const { deltaTime } = tickerData;
    const entity = entities[0];
    if (!entity) {
      return;
    }

    const positionComponent = entity.getComponent<PositionComponent>(PositionComponent.TYPE);
    const keyboardControlsComponent = entity.getComponent<PlayerControlsComponent>(PlayerControlsComponent.TYPE);

    if (positionComponent && keyboardControlsComponent) {
      const speed = keyboardControlsComponent.speed;

      if (keyboardControlsComponent.isLeft) {
        const direction = -1;
        positionComponent.x += speed * direction * deltaTime;
      } else if (keyboardControlsComponent.isRight) {
        const direction = 1;
        positionComponent.x += speed * direction * deltaTime;
      }

      if (keyboardControlsComponent.isUp) {
        const direction = -1;
        positionComponent.y += speed * direction * deltaTime;
      } else if (keyboardControlsComponent.isDown) {
        const direction = 1;
        positionComponent.y += speed * direction * deltaTime;
      }

      const stageWidth = this._stageWidth;
      const stageHeight = this._stageHeight;

      const offset = 92;

      if (positionComponent.x < -offset) positionComponent.x = stageWidth + offset;

      if (positionComponent.y < -offset) positionComponent.y = stageHeight + offset;

      if (positionComponent.x > stageWidth + offset) positionComponent.x = -offset;

      if (positionComponent.y > stageHeight + offset) positionComponent.y = -offset;

      this._service
        .setPlayerPosition(positionComponent.x, positionComponent.y)
        .setKeyPressed(keyboardControlsComponent.isUp, keyboardControlsComponent.isDown, keyboardControlsComponent.isLeft, keyboardControlsComponent.isRight)
        .sendPlayerMoved();
    }
	}
}
