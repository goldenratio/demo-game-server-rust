import { Component, ComponentProps } from 'super-ecs';

export class OpponentPlayerComponent implements Component {
	public static TYPE: symbol = Symbol('OpponentPlayerComponent');
	public name: symbol = OpponentPlayerComponent.TYPE;

  public readonly playerId: string;

	constructor(props: ComponentProps<OpponentPlayerComponent>) {
		this.playerId = props.playerId;
  }
}
