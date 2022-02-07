import type { Value } from "./Value";

export type Input = { variant: "Data", name: string, default_value: Value, } | { variant: "Exec", name: string, };