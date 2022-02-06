import { makeAutoObservable } from "mobx";
import { Position, Node as RawNode } from "@macrograph/core-types";

import { NodeSchema } from "./NodeSchema";
import { DataInput, DataOutput, ExecInput, ExecOutput } from "./Pin";
import { Graph } from ".";

interface NodeArgs extends Omit<RawNode, "schema" | "graph"> {
  graph: Graph;
  schema: NodeSchema;
}

export class Node {
  id: number;
  graph: Graph;
  position: Position;
  schema: NodeSchema;
  inputs: (DataInput | ExecInput)[];
  outputs: (DataOutput | ExecOutput)[];

  selected = false;

  constructor(args: NodeArgs) {
    makeAutoObservable(this);

    this.id = args.id;
    this.graph = args.graph;
    this.position = args.position;
    this.schema = args.schema;
    this.inputs = args.inputs.map((i) =>
      i.variant === "Data"
        ? new DataInput(this, i.name, i.default_value)
        : new ExecInput(this, i.name)
    );
    this.outputs = args.outputs.map((o) =>
      o.variant === "Data"
        ? new DataOutput(this, o.name, o.type)
        : new ExecOutput(this, o.name)
    );
  }

  setSelected(selected: boolean) {
    this.selected = selected;
  }

  setPosition(position: XY) {
    this.position = position;
  }
}
