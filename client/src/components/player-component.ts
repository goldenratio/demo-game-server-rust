import { Component, ComponentProps } from 'super-ecs';

export class PlayerComponent implements Component {
	public static TYPE: symbol = Symbol('PlayerComponent');
	public name: symbol = PlayerComponent.TYPE;

	constructor(props?: ComponentProps<PlayerComponent>) {
		// empty
	}
}
