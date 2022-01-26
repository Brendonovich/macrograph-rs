import type { Package } from "./Package";
import type { Output } from "./Output";
import type { Input } from "./Input";

export type Response = { type: "CreateNode", data: { id: number, name: string, inputs: Array<Input>, outputs: Array<Output>, } } | { type: "SetDefaultValue" } | { type: "ConnectIO" } | { type: "DisconnectIO" } | { type: "GetPackages", data: { packages: Array<Package>, } } | { type: "Reset" };