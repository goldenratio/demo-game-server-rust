import { Application, Container, Assets, Sprite, Texture } from 'pixi.js';
import { World, Entity } from 'super-ecs';
import { firstValueFrom } from 'rxjs';

import { KeyboardControlsComponent, PositionComponent, SpriteComponent } from './components';
import {
  KeyboardControlsSystem,
  KeyboardMovementSystem, PeerPlayerDisplaySystem,
  PositionSystem,
  RandomMovementSystem,
  SpriteSystem,
} from './systems';
import { CommsManager } from './service';

const app = new Application({
	width: 800,
	height: 600,
	backgroundColor: 0x1099bb,
	resolution: window.devicePixelRatio || 1,
	sharedTicker: true,
	hello: true,
});

// @ts-ignore
document.body.appendChild(app.view);
const container = new Container();
app.stage.addChild(container);

const ghostContainer = new Container();
app.stage.addChild(ghostContainer);

Assets.addBundle('assets', {
	p1: './assets/p1_front.png',
	p2: './assets/p2_front.png',
});

const commsManager = new CommsManager();

Assets.loadBundle('assets')
	.then(() => firstValueFrom(commsManager.connected$))
	.then(() => init());

// @ts-ignore
function wsInit() {
	// const builder = new Builder(0);
	// builder.clear();
	// const offset = User.createUser(builder, builder.createString("Arthur Dent"), BigInt(42));
	// builder.finish(offset);
	//
	// const bytes = builder.asUint8Array();
	// console.log(bytes);

	// const buffer = new ByteBuffer(bytes);
	// const user = User.getRootAsUser(buffer);
	// console.log(user.name());
	// console.log(user.id());

	const socket = new WebSocket('ws://localhost:8090/ws');
	socket.binaryType = 'arraybuffer';
	socket.addEventListener('open', event => {
		console.log('socket open!');
		// @ts-ignore
		window['debug_sendMessage'] = () => {
			// const builder = new Builder(0);
			// builder.clear();
      // const playerControl = PlayerControl.createPlayerControl(builder, true, false, false, false);
			// const offset = Gameplay.createGameplay(builder, playerControl, builder.createString('2121'));
			// builder.finish(offset);
      //
			// const bytes = builder.asUint8Array();
			// console.log('sending, ', bytes);
			// socket.send(bytes);
		};
	});

	socket.addEventListener('message', event => {
		const bytes = new Uint8Array(event.data);
		console.log('message from server, ', bytes);
		// const buffer = new ByteBuffer(bytes);
		// const user = User.getRootAsUser(buffer);
		// console.log(user.name());
		// console.log(Number(user.id()));
	});
}

// @ts-ignore
function init(): void {
	const world = new World();

	// systems
	world
		.addSystem(new SpriteSystem(container))
		.addSystem(new PositionSystem())
		.addSystem(new RandomMovementSystem())
    .addSystem(new PeerPlayerDisplaySystem(commsManager, ghostContainer))
		.addSystem(new KeyboardMovementSystem(commsManager))
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
		// .addComponent(new RandomMovementComponent({ direction }))
		.addComponent(new KeyboardControlsComponent())
		.addComponent(
			new SpriteComponent({
				sprite: new Sprite(Texture.from(textureName)),
			})
		);

	return hero;
}
