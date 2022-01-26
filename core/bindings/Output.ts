import type { Value } from "./Value";

export type Output = { variant: "Data", id: string, name: string, type: Value, } | { variant: "Exec", id: string, name: string, };