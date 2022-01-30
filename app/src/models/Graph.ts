import { makeAutoObservable, runInAction } from "mobx";
import { Position } from "@macrograph/core-types";

import {
  DataInput,
  DataOutput,
  ExecInput,
  ExecOutput,
  Node,
  NodeSchema,
} from ".";
import { pinsCanConnect, send } from "~/utils";

export class Graph {
  nodes: Node[] = [];

  constructor() {
    makeAutoObservable(this);
  }

  addNode(node: Node) {
    this.nodes.push(node);
  }

  async connectPins(
    output: DataOutput | ExecOutput,
    input: DataInput | ExecInput
  ) {
    if (!pinsCanConnect(output, input)) return;

    console.log({
      output_node: output.node.id,
      output: output.name,
      input_node: input.node.id,
      input: input.name,
    });

    await send("ConnectIO", {
      output_node: output.node.id,
      output: output.name,
      input_node: input.node.id,
      input: input.name,
    });

    runInAction(() => {
      if (output instanceof DataOutput) {
        const dataOutput = output as DataOutput;
        const dataInput = input as DataInput;

        dataOutput.connections.push(dataInput);
        dataInput.connection?.connections.splice(
          dataInput.connection.connections.indexOf(dataInput),
          1
        );
        dataInput.connection = dataOutput;
      } else {
        const execOutput = output as ExecOutput;
        const execInput = input as ExecInput;

        if (execOutput.connection) execOutput.connection.connection = null;
        if (execInput.connection) execInput.connection.connection = null;

        execOutput.connection = execInput;
        execInput.connection = execOutput;
      }
    });
  }

  async createNode(schema: NodeSchema, position: Position) {
    const res = await send("CreateNode", {
      package: schema.package.name,
      schema: schema.name,
      position,
    });

    runInAction(() => {
      const node = new Node({
        ...res,
        schema,
        position,
      });

      this.nodes.push(node);
    });
  }
}
