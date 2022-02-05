import type { Output } from "./Output";
import type { Package } from "./Package";
import type { Input } from "./Input";

export type Response = { type: "CreateNode", data: { id: number, inputs: Array<Input>, outputs: Array<Output>, } } | { type: "DeleteNode" } | { type: "SetDefaultValue" } | { type: "ConnectIO" } | { type: "DisconnectIO" } | { type: "GetPackages", data: { packages: Array<Package>, } } | { type: "Reset" };