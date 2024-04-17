import { Component, ComponentProps } from 'super-ecs';

export class PlayerControlsComponent implements Component {
	public static TYPE: symbol = Symbol('KeyboardControlsComponent');
	public name: symbol = PlayerControlsComponent.TYPE;

	public isLeft: boolean = false;
	public isRight: boolean = false;
	public isUp: boolean = false;
	public isDown: boolean = false;

	public speed: number = 6;

	constructor(props?: ComponentProps<PlayerControlsComponent>) {
		// empty
	}
}
