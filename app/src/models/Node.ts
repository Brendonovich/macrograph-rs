import { makeAutoObservable } from "mobx";
import { Input, Output } from "@macrograph/core-types";

import { NodeSchema } from "./NodeSchema";
import { DataInput, DataOutput, ExecInput, ExecOutput } from "./Pin";

interface NodeArgs {
  id: number;
  name: string;
  schema: NodeSchema;
  position: XY;
  inputs: Input[];
  outputs: Output[];
}

export class Node {
  id = 0;
  name = "";
  position = {
    x: 0,
    y: 0,
  };
  schema: NodeSchema;
  inputs: (DataInput | ExecInput)[];
  outputs: (DataOutput | ExecOutput)[];

  selected = false;

  constructor(args: NodeArgs) {
    makeAutoObservable(this);

    this.id = args.id;
    this.name = args.name;
    this.position = args.position;
    this.schema = args.schema;
    this.inputs = args.inputs.map((i) =>
      i.variant === "Data"
        ? new DataInput(this, i.id, i.name, i.default_value)
        : new ExecInput(this, i.id, i.name)
    );
    this.outputs = args.outputs.map((o) =>
      o.variant === "Data"
        ? new DataOutput(this, o.id, o.name, o.type)
        : new ExecOutput(this, o.id, o.name)
    );
  }

  setSelected(selected: boolean) {
    this.selected = selected;
  }

  setPosition(position: XY) {
    this.position = position;
  }
}
