// @ts-ignore
import { tagData } from "@tag"

export function useTag() {
    const tags = []
    for (let name in tagData) {
        tags.push(tagData[name])
    }

    return {
        tags,
    }
}