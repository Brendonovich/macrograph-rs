import type { Value } from "./Value";
import type { Position } from "./Position";

export type Request = { type: "CreateNode", data: { package: string, schema: string, position: Position, } } | { type: "DeleteNode", data: { node: number, } } | { type: "SetDefaultValue", data: { node: number, input: string, value: Value, } } | { type: "ConnectIO", data: { output_node: number, output: string, input_node: number, input: string, } } | { type: "DisconnectIO", data: { node: number, io: string, is_input: boolean, } } | { type: "GetPackages" } | { type: "Reset" };