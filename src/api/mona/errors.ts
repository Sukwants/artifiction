export class MonaApiError extends Error {
    code: string
    details?: unknown

    constructor(code: string, message: string, details?: unknown) {
        super(message)
        this.name = "MonaApiError"
        this.code = code
        this.details = details
    }
}

export function requireValue<T>(value: T | null | undefined, code: string, message: string): T {
    if (value === null || value === undefined) {
        throw new MonaApiError(code, message)
    }
    return value
}
