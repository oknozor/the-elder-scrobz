export type ValueType =
    | string
    | number
    | boolean
    | null
    | ValueType[]
    | Record<string, unknown>;
