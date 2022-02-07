import type { Output } from "./Output";
import type { Package } from "./Package";
import type { Input } from "./Input";
import type { Graph } from "./Graph";

export type Response = { type: "CreateNode", data: { id: number, inputs: Array<Input>, outputs: Array<Output>, } } | { type: "DeleteNode" } | { type: "SetDefaultValue" } | { type: "ConnectIO" } | { type: "DisconnectIO" } | { type: "GetPackages", data: { packages: Array<Package>, } } | { type: "GetProject", data: { graphs: Array<Graph>, } } | { type: "Reset" };