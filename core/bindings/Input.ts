import type { Value } from "./Value";

export type Input = { variant: "Data", id: string, name: string, default_value: Value, } | { variant: "Exec", id: string, name: string, };