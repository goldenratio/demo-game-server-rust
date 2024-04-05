import { Component, ComponentProps } from 'super-ecs';

export class KeyboardControlsComponent implements Component {
	public static TYPE: symbol = Symbol('KeyboardControlsComponent');
	public name: symbol = KeyboardControlsComponent.TYPE;

	public isLeft: boolean = false;
	public isRight: boolean = false;
	public isUp: boolean = false;
	public isDown: boolean = false;

	public speed: number = 6;

	constructor(props?: ComponentProps<KeyboardControlsComponent>) {
		// empty
	}
}
