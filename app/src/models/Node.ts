import { makeAutoObservable } from "mobx";
import { Position, Node as RawNode } from "@macrograph/core-types";

import { NodeSchema } from "./NodeSchema";
import { DataInput, DataOutput, ExecInput, ExecOutput } from "./IO";
import { Graph } from ".";
import { debouncedAction } from "~/utils";

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
        ? new DataInput({ ...i, node: this })
        : new ExecInput({ ...i, node: this })
    );
    this.outputs = args.outputs.map((o) =>
      o.variant === "Data"
        ? new DataOutput({ ...o, node: this })
        : new ExecOutput({ ...o, node: this })
    );
  }

  setSelected(selected: boolean) {
    this.selected = selected;
  }

  setPosition(position: XY) {
    this.position = position;

    debouncedAction({
      request: "SetNodePosition",
      data: {
        graph: this.graph.id,
        node: this.id,
        position,
      },
      timeout: 100,
      key: `node-${this.id}`,
    });
  }

  input(name: string) {
    return this.inputs.find((i) => i.name === name);
  }

  output(name: string) {
    return this.outputs.find((o) => o.name === name);
  }
}
