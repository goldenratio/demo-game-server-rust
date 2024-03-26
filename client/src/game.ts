import { Application, Container, Assets, Sprite, Texture } from 'pixi.js';
import { World, Entity } from 'super-ecs';
import { Builder, ByteBuffer } from 'flatbuffers';

import { PositionComponent, RandomMovementComponent, SpriteComponent } from './components';
import { PositionSystem, RandomMovementSystem, SpriteSystem } from './systems';
import { User } from "./gen/users";

const app = new Application({
	width: 600,
	height: 400,
	backgroundColor: 0x1099bb,
	resolution: window.devicePixelRatio || 1,
	sharedTicker: true,
  hello: true
});

// @ts-ignore
document.body.appendChild(app.view);
const container = new Container();
app.stage.addChild(container);

Assets.addBundle('assets', {
  'p1': './assets/p1_front.png',
  'p2': './assets/p2_front.png'
});

Assets.loadBundle('assets')
  .then(() => init());

function init(): void {

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


  const socket = new WebSocket('ws://localhost:8080/ws/');
  socket.binaryType = "arraybuffer";
  socket.addEventListener('open', event => {
    console.log('socket open!');
    // @ts-ignore
    window['debug_sendMessage'] = () => {
      const builder = new Builder(0);
      builder.clear();
      const offset = User.createUser(builder, builder.createString("Arthur Dent"), BigInt(42));
      builder.finish(offset);

      const bytes = builder.asUint8Array();
      console.log('sending, ', bytes);
      socket.send(bytes);
    };
  });

  socket.addEventListener('message', event => {
    const bytes = new Uint8Array(event.data);
    console.log('message from server, ', bytes);
    const buffer = new ByteBuffer(bytes);
    const user = User.getRootAsUser(buffer);
    console.log(user.name());
    console.log(Number(user.id()));
  });

	const world = new World();

	// systems
	world.addSystem(new SpriteSystem(container)).addSystem(new PositionSystem()).addSystem(new RandomMovementSystem());

	// entities
	Array.from({ length: 50 }).forEach(() => {
		const entity = createHeroEntity();
		world.addEntity(entity);
	});

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
	const direction = Math.floor(Math.random() * 10) > 5 ? -1 : 1;
	const x = Math.floor(Math.random() * 600);
	const y = Math.floor(Math.random() * 400);
	const textureName = Math.floor(Math.random() * 10) > 5 ? 'p1' : 'p2';

	const hero = new Entity();
	hero
		.addComponent(new PositionComponent({ x, y }))
		.addComponent(new RandomMovementComponent({ direction }))
		.addComponent(
			new SpriteComponent({
				sprite: new Sprite(Texture.from(textureName)),
			})
		);

	return hero;
}
