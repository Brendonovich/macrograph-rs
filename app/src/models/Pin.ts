import { makeAutoObservable, runInAction } from "mobx";
import { Value } from "@macrograph/core-types";

import { Node } from "./Node";
import { send } from "~/utils";

export class DataInput {
  connection: DataOutput | null = null;

  constructor(
    public node: Node,
    public id: string,
    public name: string,
    public defaultValue: Value
  ) {
    makeAutoObservable(this);
  }

  async disconnect() {
    await send("DisconnectIO", {
      node: this.node.id,
      id: this.id,
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

  async setDefaultValue(value: Value["value"]) {
    this.defaultValue.value = value;

    if (this.defaultValueDebounce) {
      clearTimeout(this.defaultValueDebounce);
    }

    setTimeout(async () => {
      let valueObj: Value;
      switch (typeof value) {
        case "string":
          valueObj = {
            type: "string",
            value,
          };
          break;
        case "boolean":
          valueObj = {
            type: "bool",
            value,
          };
          break;
      }

      await send("SetDefaultValue", {
        node: this.node.id,
        input: this.id,
        value: valueObj,
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

  constructor(
    public node: Node,
    public id: string,
    public name: string,
    public type: Value
  ) {
    makeAutoObservable(this);
  }

  async disconnect() {
    await send("DisconnectIO", {
      node: this.node.id,
      id: this.id,
      is_input: true,
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

  constructor(public node: Node, public id: string, public name: string) {
    makeAutoObservable(this);
  }

  async disconnect() {
    await send("DisconnectIO", {
      node: this.node.id,
      id: this.id,
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

  constructor(public node: Node, public id: string, public name: string) {
    makeAutoObservable(this);
  }

  async disconnect() {
    await send("DisconnectIO", {
      node: this.node.id,
      id: this.id,
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

export type ExecPin = ExecInput | ExecOutput;
export type DataPin = DataInput | DataOutput;
export type Pin = ExecPin | DataPin;
