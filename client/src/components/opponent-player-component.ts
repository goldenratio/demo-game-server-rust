import { Component, ComponentProps } from 'super-ecs';

export class OpponentPlayerComponent implements Component {
	public static TYPE: symbol = Symbol('OpponentPlayerComponent');
	public name: symbol = OpponentPlayerComponent.TYPE;

	constructor(props?: ComponentProps<OpponentPlayerComponent>) {
		// empty
	}
}
