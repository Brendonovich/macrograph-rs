import type { Connection } from "./Connection";
import type { Primitive } from "./Primitive";
import type { ValueType } from "./ValueType";

export type Input = { variant: "Data", name: string, type: ValueType, default_value: Primitive, connection: Connection | null, } | { variant: "Exec", name: string, connection: Connection | null, };