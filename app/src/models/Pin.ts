import { makeAutoObservable, runInAction } from "mobx";
import { Value } from "@macrograph/core-types";

import { Node } from "./Node";
import { send } from "~/utils";

export class DataInput {
  connection: DataOutput | null = null;

  constructor(
    public node: Node,
    public name: string,
    public defaultValue: Value
  ) {
    makeAutoObservable(this);
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

  private defaultValueDebounce: any = null;

  async setDefaultValue(value: Value) {
    this.defaultValue = value;

    if (this.defaultValueDebounce) {
      clearTimeout(this.defaultValueDebounce);
    }

    setTimeout(async () => {
      await send("SetDefaultValue", {
        graph: this.node.graph.id,
        node: this.node.id,
        input: this.name,
        value: value,
      });
    }, 100);
  }

  get connected() {
    return this.connection !== null;
  }

  get type() {
    return this.defaultValue;
  }
}

export class DataOutput {
  connections: DataInput[] = [];

  constructor(public node: Node, public name: string, public type: Value) {
    makeAutoObservable(this);
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

export class ExecInput {
  connection: ExecOutput | null = null;

  constructor(public node: Node, public name: string) {
    makeAutoObservable(this);
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

export class ExecOutput {
  connection: ExecInput | null = null;

  constructor(public node: Node, public name: string) {
    makeAutoObservable(this);
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
