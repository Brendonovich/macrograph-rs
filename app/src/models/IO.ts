import { makeAutoObservable, runInAction } from "mobx";
import {
  Value,
  Input,
  Output,
  ValueType,
  Primitive,
} from "@macrograph/core-types";

import { Node } from "./Node";
import { debouncedAction, send } from "~/utils";

interface DataInputArgs extends Extract<Input, { variant: "Data" }> {
  node: Node;
}

export class DataInput {
  name: string;
  defaultValue: Primitive;
  type: ValueType;
  node: Node;
  connection: DataOutput | null = null;

  constructor(args: DataInputArgs) {
    makeAutoObservable(this);

    this.name = args.name;
    this.defaultValue = args.default_value;
    this.node = args.node;
    this.type = args.type;
  }

  async disconnect(_send = true) {
    if (_send)
      await send("DisconnectIO", {
        graph: this.node.graph.id,
        node: this.node.id,
        io: this.name,
        is_input: true,
      });

    runInAction(() => {
      this.connection?.connections.splice(
        this.connection.connections.indexOf(this),
        1
      );
      this.connection = null;
    });
  }

  async setDefaultValue(value: Primitive) {
    this.defaultValue = value;

    debouncedAction({
      request: "SetDefaultValue",
      data: {
        graph: this.node.graph.id,
        node: this.node.id,
        input: this.name,
        value: value,
      },
      timeout: 100,
      key: `default-${this.node.id}-${this.name}`,
    });
  }

  get connected() {
    return this.connection !== null;
  }
}

interface DataOutputArgs extends Extract<Output, { variant: "Data" }> {
  node: Node;
}

export class DataOutput {
  connections: DataInput[] = [];
  node: Node;
  name: string;
  type: ValueType;

  constructor(args: DataOutputArgs) {
    makeAutoObservable(this);

    this.node = args.node;
    this.name = args.name;
    this.type = args.type;
  }

  async disconnect(_send = true) {
    if (_send)
      await send("DisconnectIO", {
        graph: this.node.graph.id,
        node: this.node.id,
        io: this.name,
        is_input: false,
      });

    runInAction(() => {
      this.connections.forEach((c) => (c.connection = null));
      this.connections = [];
    });
  }

  get connected() {
    return this.connections.length > 0;
  }
}

interface ExecInputArgs extends Extract<Input, { variant: "Exec" }> {
  node: Node;
}

export class ExecInput {
  connection: ExecOutput | null = null;
  node: Node;
  name: string;

  constructor(args: ExecInputArgs) {
    makeAutoObservable(this);

    this.node = args.node;
    this.name = args.name;
  }

  async disconnect(_send = true) {
    if (_send)
      await send("DisconnectIO", {
        graph: this.node.graph.id,
        node: this.node.id,
        io: this.name,
        is_input: true,
      });

    runInAction(() => {
      if (this.connection) this.connection.connection = null;
      this.connection = null;
    });
  }

  get connected() {
    return this.connection !== null;
  }
}

interface ExecOutputArgs extends Extract<Output, { variant: "Exec" }> {
  node: Node;
}

export class ExecOutput {
  connection: ExecInput | null = null;
  public node: Node;
  public name: string;

  constructor(args: ExecOutputArgs) {
    makeAutoObservable(this);

    this.node = args.node;
    this.name = args.name;
  }

  async disconnect(_send = true) {
    if (_send)
      await send("DisconnectIO", {
        graph: this.node.graph.id,
        node: this.node.id,
        io: this.name,
        is_input: false,
      });

    runInAction(() => {
      if (this.connection) this.connection.connection = null;
      this.connection = null;
    });
  }

  get connected() {
    return this.connection !== null;
  }
}

export type ExecPin = ExecInput | ExecOutput;
export type DataPin = DataInput | DataOutput;
export type Pin = ExecPin | DataPin;
