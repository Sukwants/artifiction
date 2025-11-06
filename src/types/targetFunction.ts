export interface ITargetFunction {
    dsl_source?: string,
    use_dsl: boolean,
    params: any,
    originalParams: any,
    configUnlinked: any,
    name: string
}

export type TargetFunctionName = string
