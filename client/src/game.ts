import { Application, Container, Assets, Sprite, Texture } from 'pixi.js';
import { World, Entity } from 'super-ecs';
import { firstValueFrom } from 'rxjs';

import { PlayerComponent, PlayerControlsComponent, PositionComponent, SpriteComponent } from './components';
import {
  KeyboardControlsSystem,
  PlayerMovementSystem, OpponentPlayerSystem,
  PositionSystem,
  SpriteSystem,
} from './systems';
import { CommsManager } from './service';

const app = new Application({
	width: 1024,
	height: 600,
	backgroundColor: 0x1099bb,
	resolution: 1,
	sharedTicker: true,
	hello: true,
});

// @ts-ignore
document.body.appendChild(app.view);
const container = new Container();
app.stage.addChild(container);

globalThis.__PIXI_APP__ = app;

// const ghostContainer = new Container();
// app.stage.addChild(ghostContainer);

Assets.addBundle('assets', {
	p1: './assets/p1_front.png',
	p2: './assets/p2_front.png',
});

const commsManager = new CommsManager();

Assets.loadBundle('assets')
	// .then(() => firstValueFrom(commsManager.connected$))
	.then(() => init());

function init(): void {
	const world = new World();

	// systems
	world
		.addSystem(new SpriteSystem(container))
		.addSystem(new PositionSystem())
    .addSystem(new OpponentPlayerSystem(commsManager))
		.addSystem(new PlayerMovementSystem(commsManager))
		.addSystem(new KeyboardControlsSystem());

	const entity = createHeroEntity();
	world.addEntity(entity);

	// game loop
	app.ticker.add(deltaTime =>
		world.update({
			deltaTime,
			// todo: find out below values
			elapsedMS: 0,
			lastTime: 0,
		})
	);
}

function createHeroEntity(): Entity {
	// const direction = Math.floor(Math.random() * 10) > 5 ? -1 : 1;
	const x = Math.floor(Math.random() * 600);
	const y = Math.floor(Math.random() * 400);
	const textureName = 'p1'; // Math.floor(Math.random() * 10) > 5 ? 'p1' : 'p2';

	const hero = new Entity();
	hero
		.addComponent(new PositionComponent({ x, y }))
    .addComponent(new PlayerComponent())
		.addComponent(new PlayerControlsComponent())
		.addComponent(
			new SpriteComponent({
				sprite: new Sprite(Texture.from(textureName)),
			})
		);

	return hero;
}
