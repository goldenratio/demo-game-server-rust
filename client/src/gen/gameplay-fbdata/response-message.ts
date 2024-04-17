// automatically generated by the FlatBuffers compiler, do not modify

import { GameWorldUpdate } from '../gameplay-fbdata/game-world-update';
import { RemotePeerJoined } from '../gameplay-fbdata/remote-peer-joined';
import { RemotePeerLeft } from '../gameplay-fbdata/remote-peer-left';
import { RemotePeerPositionUpdate } from '../gameplay-fbdata/remote-peer-position-update';


export enum ResponseMessage {
  NONE = 0,
  RemotePeerJoined = 1,
  RemotePeerLeft = 2,
  RemotePeerPositionUpdate = 3,
  GameWorldUpdate = 4
}

export function unionToResponseMessage(
  type: ResponseMessage,
  accessor: (obj:GameWorldUpdate|RemotePeerJoined|RemotePeerLeft|RemotePeerPositionUpdate) => GameWorldUpdate|RemotePeerJoined|RemotePeerLeft|RemotePeerPositionUpdate|null
): GameWorldUpdate|RemotePeerJoined|RemotePeerLeft|RemotePeerPositionUpdate|null {
  switch(ResponseMessage[type]) {
    case 'NONE': return null;
    case 'RemotePeerJoined': return accessor(new RemotePeerJoined())! as RemotePeerJoined;
    case 'RemotePeerLeft': return accessor(new RemotePeerLeft())! as RemotePeerLeft;
    case 'RemotePeerPositionUpdate': return accessor(new RemotePeerPositionUpdate())! as RemotePeerPositionUpdate;
    case 'GameWorldUpdate': return accessor(new GameWorldUpdate())! as GameWorldUpdate;
    default: return null;
  }
}

export function unionListToResponseMessage(
  type: ResponseMessage,
  accessor: (index: number, obj:GameWorldUpdate|RemotePeerJoined|RemotePeerLeft|RemotePeerPositionUpdate) => GameWorldUpdate|RemotePeerJoined|RemotePeerLeft|RemotePeerPositionUpdate|null,
  index: number
): GameWorldUpdate|RemotePeerJoined|RemotePeerLeft|RemotePeerPositionUpdate|null {
  switch(ResponseMessage[type]) {
    case 'NONE': return null;
    case 'RemotePeerJoined': return accessor(index, new RemotePeerJoined())! as RemotePeerJoined;
    case 'RemotePeerLeft': return accessor(index, new RemotePeerLeft())! as RemotePeerLeft;
    case 'RemotePeerPositionUpdate': return accessor(index, new RemotePeerPositionUpdate())! as RemotePeerPositionUpdate;
    case 'GameWorldUpdate': return accessor(index, new GameWorldUpdate())! as GameWorldUpdate;
    default: return null;
  }
}
