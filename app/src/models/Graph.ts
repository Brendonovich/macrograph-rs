import { makeAutoObservable, runInAction } from "mobx";
import { Position, Graph as RawGraph } from "@macrograph/core-types";

import {
  Core,
  DataInput,
  DataOutput,
  ExecInput,
  ExecOutput,
  Node,
  NodeSchema,
} from ".";
import { pinsCanConnect, send, action, debouncedAction } from "~/utils";

interface Args extends RawGraph {
  core: Core;
}

export class Graph {
  id: number;
  name: string;
  nodes: Record<number, Node> = {};

  constructor(data: Args) {
    makeAutoObservable(this);

    this.id = data.id;
    this.name = data.name;
    this.nodes = data.nodes.reduce(
      (acc, n) => ({
        ...acc,
        [n.id]: new Node({
          ...n,
          graph: this,
          schema: data.core.schema(n.schema.package, n.schema.name)!,
        }),
      }),
      {}
    );

    data.nodes.forEach((n) => {
      n.inputs.forEach((i) => {
        if (!i.connection) return;

        const output = this.nodes[i.connection.node].output(i.connection.io)!;
        const input = this.nodes[n.id].input(i.name);
        
        if (!output || !input) return;

        this.connectPins(output, input, false);
      });
    });
  }

  addNode(node: Node) {
    this.nodes[node.id] = node;
  }

  async connectPins(
    output: DataOutput | ExecOutput,
    input: DataInput | ExecInput,
    _send = true
  ) {
    if (_send) {
      if (!pinsCanConnect(output, input)) return;

      await send("ConnectIO", {
        graph: this.id,
        output_node: output.node.id,
        output: output.name,
        input_node: input.node.id,
        input: input.name,
      });
    }

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

  createNode(schema: NodeSchema, position: Position) {
    action({
      request: "CreateNode",
      data: {
        graph: this.id,
        package: schema.package.name,
        schema: schema.name,
        position,
      },
      run: (data) => {
        const node = new Node({
          ...data,
          graph: this,
          schema,
          position,
        });

        this.addNode(node);
      },
    });
  }

  async deleteNode(id: number) {
    await send("DeleteNode", {
      graph: this.id,
      node: id,
    });

    runInAction(() => {
      const node = this.nodes[id];

      node.inputs.forEach((i) => i.disconnect(false));
      node.outputs.forEach((o) => o.disconnect(false));

      delete this.nodes[id];
    });
  }

  async rename(name: string) {
    this.name = name;

    debouncedAction({
      request: "RenameGraph",
      data: {
        id: this.id,
        name,
      },
      timeout: 100,
      key: `${this.id}`,
    });
  }
}
