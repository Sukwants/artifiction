import backend from "@/store/backend"
import {changeAccount, deleteAccount, useAccountStore, waitForAccountStoreReady} from "@/store/pinia/account"
import {useArtifactStore, watchContent as artifactContent} from "@/store/pinia/artifact"
import {useKumiStore} from "@/store/pinia/kumi"
import {usePresetStore} from "@/store/pinia/preset"
import type {IArtifactContentOnly} from "@/types/artifact"
import type {KumiItem} from "@/types/kumi"
import {hash as hashArtifact} from "@/utils/artifactHash"
import {deepCopy} from "@/utils/common"
import {MonaApiError, requireValue} from "./errors"
import {emitMonaApiEvent} from "./events"

function nonEmptyName(name: unknown) {
    if (typeof name !== "string" || name.trim() === "") {
        throw new MonaApiError("INVALID_ARGUMENT", "Name must be a non-empty string")
    }
    return name.trim()
}

export const accountsApi = {
    async list() {
        return deepCopy(useAccountStore().allAccounts)
    },
    async getCurrent() {
        const store = useAccountStore()
        return deepCopy(requireValue(
            store.allAccounts.find(item => item.id === store.currentAccountId.value),
            "NOT_FOUND",
            "Current account does not exist",
        ))
    },
    async create(input: {name: string}) {
        const store = useAccountStore()
        store.addAccount(nonEmptyName(input?.name))
        const account = store.allAccounts[store.allAccounts.length - 1]
        emitMonaApiEvent("account.created", account)
        return deepCopy(account)
    },
    async rename(id: number, name: string) {
        const store = useAccountStore()
        requireValue(store.allAccounts.find(item => item.id === id), "NOT_FOUND", `Account ${id} does not exist`)
        store.changeAccountName(id, nonEmptyName(name))
        const account = store.allAccounts.find(item => item.id === id)!
        emitMonaApiEvent("account.changed", account)
        return deepCopy(account)
    },
    async remove(id: number) {
        const store = useAccountStore()
        requireValue(store.allAccounts.find(item => item.id === id), "NOT_FOUND", `Account ${id} does not exist`)
        if (store.currentAccountId.value === id) {
            throw new MonaApiError("CURRENT_ACCOUNT_CANNOT_BE_REMOVED", "The current account cannot be removed")
        }
        await deleteAccount(id)
        emitMonaApiEvent("account.removed", {id})
    },
    async switch(id: number) {
        const store = useAccountStore()
        requireValue(store.allAccounts.find(item => item.id === id), "NOT_FOUND", `Account ${id} does not exist`)
        await changeAccount(id)
        const account = store.allAccounts.find(item => item.id === id)!
        emitMonaApiEvent("account.switched", account)
        return deepCopy(account)
    },
}

function getArtifact(id: number) {
    return requireValue(useArtifactStore().getArtifact(id), "NOT_FOUND", `Artifact ${id} does not exist`)
}

function validateArtifact(input: any): asserts input is IArtifactContentOnly {
    if (!input || typeof input !== "object" || typeof input.setName !== "string"
        || typeof input.position !== "string" || typeof input.star !== "number"
        || typeof input.level !== "number" || !input.mainTag || !Array.isArray(input.normalTags)) {
        throw new MonaApiError("INVALID_ARTIFACT", "Artifact has an invalid structure", input)
    }
}

const artifactPositions = ["flower", "feather", "sand", "cup", "head"] as const

function validateArtifactImport(data: any): asserts data is Record<typeof artifactPositions[number], IArtifactContentOnly[]> {
    if (!data || typeof data !== "object" || Array.isArray(data)
        || !artifactPositions.every(position => Array.isArray(data[position]))) {
        throw new MonaApiError("INVALID_IMPORT", "Artifact import data must contain an array for every artifact position", data)
    }
}

function toArtifactContent(input: any): IArtifactContentOnly {
    return deepCopy({
        setName: input.setName,
        position: input.position,
        star: input.star,
        level: input.level,
        mainTag: input.mainTag,
        normalTags: input.normalTags,
    }) as IArtifactContentOnly
}

export const artifactsApi = {
    async list(query: any = {}) {
        let result = Array.from(useArtifactStore().artifacts.value.values())
        if (query.ids) result = result.filter(item => query.ids.includes(item.id))
        if (query.position) result = result.filter(item => item.position === query.position)
        if (query.setName) result = result.filter(item => item.setName === query.setName)
        if (query.star !== undefined) result = result.filter(item => item.star === query.star)
        if (query.level !== undefined) result = result.filter(item => item.level === query.level)
        if (query.locked !== undefined) result = result.filter(item => item.omit === query.locked)
        return deepCopy(result)
    },
    async get(id: number) {
        return deepCopy(getArtifact(id))
    },
    async create(input: IArtifactContentOnly & {locked?: boolean}) {
        validateArtifact(input)
        const id = useArtifactStore().addArtifact(toArtifactContent(input), !!input.locked)
        const artifact = getArtifact(id)
        emitMonaApiEvent("artifact.created", artifact)
        return deepCopy(artifact)
    },
    async update(id: number, patch: Partial<IArtifactContentOnly>) {
        const current = getArtifact(id)
        const next = {...deepCopy(current), ...deepCopy(patch)}
        validateArtifact(next)
        useArtifactStore().updateArtifact(id, toArtifactContent(next))
        const artifact = getArtifact(id)
        emitMonaApiEvent("artifact.changed", artifact)
        return deepCopy(artifact)
    },
    async setLocked(id: number, locked: boolean) {
        getArtifact(id)
        if (locked) useArtifactStore().lockArtifact(id)
        else useArtifactStore().unlockArtifact(id)
        const artifact = getArtifact(id)
        emitMonaApiEvent("artifact.changed", artifact)
        return deepCopy(artifact)
    },
    async remove(id: number) {
        getArtifact(id)
        useArtifactStore().removeArtifact(id)
        emitMonaApiEvent("artifact.removed", {id})
    },
    async export() {
        return {schema: "mona.artifacts", schemaVersion: 1, data: deepCopy(artifactContent())}
    },
    async import(document: any, options: {mode?: "append" | "replace", duplicate?: "skip" | "allow" | "overwrite" | "error", dryRun?: boolean, confirm?: boolean} = {}) {
        const data = document?.schema === "mona.artifacts" ? document.data : document
        validateArtifactImport(data)
        const items = artifactPositions.flatMap(position => data[position])
        for (const item of items) validateArtifact(item)
        if (options.mode === "replace" && options.confirm !== true) {
            throw new MonaApiError("CONFIRMATION_REQUIRED", "Replacing artifacts requires confirm: true")
        }
        const store = useArtifactStore()
        let imported = 0
        let skipped = 0
        const duplicate = options.duplicate ?? "skip"
        const existingHashes = new Set(options.mode === "replace"
            ? []
            : Array.from(store.artifacts.value.values(), item => item.contentHash))
        const planned: any[] = []
        const plannedIndex = new Map<string, number>()
        for (const item of items) {
            const hash = hashArtifact(item)
            if (duplicate === "allow") {
                planned.push(item)
                imported++
                continue
            }
            if (duplicate === "overwrite") {
                const previous = plannedIndex.get(hash)
                if (previous === undefined) {
                    plannedIndex.set(hash, planned.length)
                    planned.push(item)
                    imported++
                } else {
                    planned[previous] = item
                    skipped++
                }
                continue
            }
            if (existingHashes.has(hash) || plannedIndex.has(hash)) {
                if (duplicate === "error") throw new MonaApiError("DUPLICATE_ARTIFACT", "Duplicate artifact found", item)
                skipped++
                continue
            }
            plannedIndex.set(hash, planned.length)
            planned.push(item)
            imported++
        }
        if (!options.dryRun) {
            if (options.mode === "replace") store.deleteAll()
            for (const item of planned) {
                const hash = hashArtifact(item)
                if (duplicate === "overwrite") {
                    for (const current of Array.from(store.artifacts.value.values())) {
                        if (current.contentHash === hash) store.removeArtifact(current.id)
                    }
                }
                store.addArtifact(toArtifactContent(item), !!item.omit)
            }
            emitMonaApiEvent("artifacts.imported", {imported, skipped, mode: options.mode ?? "append"})
        }
        return {imported, skipped, total: items.length, dryRun: !!options.dryRun}
    },
}

function getKumi(id: number) {
    return requireValue(useKumiStore().itemById(id), "NOT_FOUND", `Artifact loadout item ${id} does not exist`)
}

export const artifactLoadoutsApi = {
    folders: {
        async list() { return deepCopy(useKumiStore().dirs.value) },
        async create(input: {name: string}) {
            const store = useKumiStore()
            const before = new Set(store.dirs.value.map(item => item.id))
            store.createDir(nonEmptyName(input?.name))
            const item = store.dirs.value.find(entry => !before.has(entry.id))!
            emitMonaApiEvent("artifactLoadout.folder.created", item)
            return deepCopy(item)
        },
        async rename(id: number, name: string) {
            const item = getKumi(id)
            if (!item.dir) throw new MonaApiError("INVALID_ARGUMENT", `${id} is not a folder`)
            useKumiStore().rename(id, nonEmptyName(name))
            emitMonaApiEvent("artifactLoadout.folder.changed", item)
            return deepCopy(item)
        },
        async remove(id: number) {
            const item = getKumi(id)
            if (!item.dir) throw new MonaApiError("INVALID_ARGUMENT", `${id} is not a folder`)
            useKumiStore().deleteDir(id)
            emitMonaApiEvent("artifactLoadout.folder.removed", {id})
        },
    },
    async list(query: {folderId?: number} = {}) {
        const store = useKumiStore()
        if (query.folderId !== undefined) return deepCopy(store.kumisByDirId.value[query.folderId] ?? [])
        return deepCopy(store.kumi.value.filter(item => !item.dir))
    },
    async get(id: number) {
        const item = getKumi(id)
        if (item.dir) throw new MonaApiError("INVALID_ARGUMENT", `${id} is a folder`)
        return deepCopy(item)
    },
    async create(input: {folderId: number, name: string, artifactIds?: Array<number | null>}) {
        const store = useKumiStore()
        const folder = getKumi(input.folderId)
        if (!folder.dir) throw new MonaApiError("INVALID_ARGUMENT", `${input.folderId} is not a folder`)
        const ids = input.artifactIds ?? [null, null, null, null, null]
        if (ids.length !== 5) throw new MonaApiError("INVALID_ARGUMENT", "artifactIds must contain five slots")
        const id = store.addKumi(input.folderId, nonEmptyName(input.name), ids as number[])
        const item = getKumi(requireValue(id, "NOT_FOUND", "Folder does not exist"))
        emitMonaApiEvent("artifactLoadout.created", item)
        return deepCopy(item)
    },
    async rename(id: number, name: string) {
        const item = getKumi(id)
        if (item.dir) throw new MonaApiError("INVALID_ARGUMENT", `${id} is a folder`)
        useKumiStore().rename(id, nonEmptyName(name))
        emitMonaApiEvent("artifactLoadout.changed", item)
        return deepCopy(item)
    },
    async updateArtifacts(id: number, artifactIds: Array<number | null>) {
        const item = getKumi(id)
        if (item.dir) throw new MonaApiError("INVALID_ARGUMENT", `${id} is a folder`)
        if (artifactIds.length !== 5) throw new MonaApiError("INVALID_ARGUMENT", "artifactIds must contain five slots")
        item.artifactIds = deepCopy(artifactIds) as KumiItem["artifactIds"]
        emitMonaApiEvent("artifactLoadout.changed", item)
        return deepCopy(item)
    },
    async remove(id: number) {
        const item = getKumi(id)
        if (item.dir) throw new MonaApiError("INVALID_ARGUMENT", `${id} is a folder`)
        useKumiStore().deleteKumi(id)
        emitMonaApiEvent("artifactLoadout.removed", {id})
    },
}

export const presetsApi = {
    async list() { return deepCopy(usePresetStore().allFlat.value) },
    async get(name: string) {
        return deepCopy(requireValue(usePresetStore().getPreset(name), "NOT_FOUND", `Preset ${name} does not exist`))
    },
    async save(name: string, item: any, options: {overwrite?: boolean} = {}) {
        const store = usePresetStore()
        name = nonEmptyName(name)
        if (store.getPreset(name) && !options.overwrite) throw new MonaApiError("ALREADY_EXISTS", `Preset ${name} already exists`)
        item = deepCopy(item)
        item.name = name
        store.addOrOverwrite(name, item)
        const result = store.getPreset(name)!
        emitMonaApiEvent("preset.changed", result)
        return deepCopy(result)
    },
    async rename(name: string, newName: string) {
        const store = usePresetStore()
        const entry = requireValue(store.getPreset(name), "NOT_FOUND", `Preset ${name} does not exist`)
        newName = nonEmptyName(newName)
        if (store.getPreset(newName)) throw new MonaApiError("ALREADY_EXISTS", `Preset ${newName} already exists`)
        const item = deepCopy(entry.item)
        item.name = newName
        store.addOrOverwrite(newName, item, entry.version)
        store.deletePreset(name)
        emitMonaApiEvent("preset.renamed", {name, newName})
        return deepCopy(store.getPreset(newName))
    },
    async remove(name: string) {
        requireValue(usePresetStore().getPreset(name), "NOT_FOUND", `Preset ${name} does not exist`)
        usePresetStore().deletePreset(name)
        emitMonaApiEvent("preset.removed", {name})
    },
    async export(names?: string[]) {
        const entries = names ? names.map(name => requireValue(usePresetStore().getPreset(name), "NOT_FOUND", `Preset ${name} does not exist`)) : usePresetStore().allFlat.value
        return {schema: "mona.presets", schemaVersion: 1, data: deepCopy(entries)}
    },
    async import(document: any, options: {overwrite?: boolean, dryRun?: boolean} = {}) {
        const entries = document?.schema === "mona.presets" ? document.data : document
        if (!Array.isArray(entries)) throw new MonaApiError("INVALID_IMPORT", "Preset import data must be an array")
        let imported = 0
        let skipped = 0
        for (const entry of entries) {
            if (!entry?.name || !entry?.item) throw new MonaApiError("INVALID_IMPORT", "Invalid preset entry", entry)
            if (usePresetStore().getPreset(entry.name) && !options.overwrite) skipped++
            else imported++
        }
        if (!options.dryRun) {
            for (const entry of entries) {
                if (usePresetStore().getPreset(entry.name) && !options.overwrite) continue
                usePresetStore().addOrOverwrite(entry.name, entry.item, entry.version)
            }
            emitMonaApiEvent("presets.imported", {imported, skipped})
        }
        return {imported, skipped, total: entries.length, dryRun: !!options.dryRun}
    },
}

export async function waitForMonaData() {
    await waitForAccountStoreReady()
    await backend.allReady()
}
