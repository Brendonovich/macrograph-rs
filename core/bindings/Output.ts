import type { Value } from "./Value";

export type Output = { variant: "Data", name: string, type: Value, } | { variant: "Exec", name: string, };