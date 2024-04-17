// automatically generated by the FlatBuffers compiler, do not modify

import * as flatbuffers from 'flatbuffers';

import { PlayerData } from '../gameplay-fbdata/player-data.js';


export class GameWorldUpdate {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):GameWorldUpdate {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsGameWorldUpdate(bb:flatbuffers.ByteBuffer, obj?:GameWorldUpdate):GameWorldUpdate {
  return (obj || new GameWorldUpdate()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsGameWorldUpdate(bb:flatbuffers.ByteBuffer, obj?:GameWorldUpdate):GameWorldUpdate {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new GameWorldUpdate()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

playerDataList(index: number, obj?:PlayerData):PlayerData|null {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? (obj || new PlayerData()).__init(this.bb!.__vector(this.bb_pos + offset) + index * 16, this.bb!) : null;
}

playerDataListLength():number {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.__vector_len(this.bb_pos + offset) : 0;
}

static startGameWorldUpdate(builder:flatbuffers.Builder) {
  builder.startObject(1);
}

static addPlayerDataList(builder:flatbuffers.Builder, playerDataListOffset:flatbuffers.Offset) {
  builder.addFieldOffset(0, playerDataListOffset, 0);
}

static startPlayerDataListVector(builder:flatbuffers.Builder, numElems:number) {
  builder.startVector(16, numElems, 8);
}

static endGameWorldUpdate(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  return offset;
}

static createGameWorldUpdate(builder:flatbuffers.Builder, playerDataListOffset:flatbuffers.Offset):flatbuffers.Offset {
  GameWorldUpdate.startGameWorldUpdate(builder);
  GameWorldUpdate.addPlayerDataList(builder, playerDataListOffset);
  return GameWorldUpdate.endGameWorldUpdate(builder);
}
}