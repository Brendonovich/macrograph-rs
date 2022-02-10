import type { ValueType } from "./ValueType";

export type Output = { variant: "Data", name: string, type: ValueType, } | { variant: "Exec", name: string, };