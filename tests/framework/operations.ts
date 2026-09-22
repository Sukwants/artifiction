import {readFile} from "node:fs/promises"
import {dirname, resolve} from "node:path"
import {fileURLToPath} from "node:url"
import type {MonaApiPage, MonaApiTestSession} from "../mona-api/framework"
import {
    ProjectTestFailure,
    serializeProjectTestError,
    type ProjectTestCaseCategory,
    type ProjectTestCaseDefinition,
    type ProjectTestFailureType
} from "./registry"

const artifactPositions = ["flower", "feather", "sand", "cup", "head"] as const
type ArtifactPosition = typeof artifactPositions[number]
type DamageMetric = "expectation" | "critical" | "nonCritical"

interface ArtifactFixture {
    position: ArtifactPosition
    contentHash: string
    id?: number
    [key: string]: unknown
}

interface ArtifactLoadoutFixture {
    title: string
    dir?: boolean
    artifactIds?: number[]
    [key: string]: unknown
}

interface ArtifactDocument extends Record<ArtifactPosition, ArtifactFixture[]> {
    kumi?: ArtifactLoadoutFixture[]
}

interface ProjectTestState {
    artifacts?: ArtifactDocument
    artifactFolderId?: number
    characterIds: Record<string, number>
    currentCharacterId?: number
}

export interface ProjectTestContext {
    readonly session: MonaApiTestSession
    readonly fixtureDirectory: string
    readonly state: ProjectTestState
}

export type ProjectTestOperation = (context: ProjectTestContext) => Promise<unknown>

export interface ProjectTestCaseOptions {
    id: string
    name?: string
    category?: ProjectTestCaseCategory
    operations: readonly ProjectTestOperation[]
}

function failure(
    type: ProjectTestFailureType,
    message: string,
    details?: unknown
): ProjectTestFailure {
    return new ProjectTestFailure(type, message, details)
}

function asRecord(
    value: unknown,
    label: string,
    type: ProjectTestFailureType = "fixture-invalid"
): Record<string, unknown> {
    if (!value || typeof value !== "object" || Array.isArray(value)) {
        throw failure(type, `${label} 必须是对象`, {value})
    }
    return value as Record<string, unknown>
}

async function runOperation<T>(
    type: ProjectTestFailureType,
    label: string,
    action: () => Promise<T>
): Promise<T> {
    try {
        return await action()
    } catch (error) {
        if (error instanceof ProjectTestFailure) throw error
        throw failure(type, `${label}失败: ${error instanceof Error ? error.message : String(error)}`, {
            cause: serializeProjectTestError(error)
        })
    }
}

async function readJsonFixture(context: ProjectTestContext, fixture: string): Promise<unknown> {
    const path = resolve(context.fixtureDirectory, fixture)
    let source: string
    try {
        source = await readFile(path, "utf8")
    } catch (error) {
        throw failure("fixture-missing", `测试数据文件不存在: ${fixture}`, {
            path,
            cause: serializeProjectTestError(error)
        })
    }

    try {
        return JSON.parse(source) as unknown
    } catch (error) {
        throw failure("fixture-invalid", `测试数据文件不是合法 JSON: ${fixture}`, {
            path,
            cause: serializeProjectTestError(error)
        })
    }
}

function parseArtifacts(value: unknown): ArtifactDocument {
    const document = asRecord(value, "圣遗物数据") as Partial<Record<ArtifactPosition, unknown>> & {
        kumi?: unknown
    }
    const parsed = {} as ArtifactDocument
    for (const position of artifactPositions) {
        const artifacts = document[position]
        if (!Array.isArray(artifacts) || artifacts.length === 0) {
            throw failure("fixture-invalid", `圣遗物数据缺少 ${position} 部位`, {position, artifacts})
        }
        parsed[position] = artifacts.map((artifact, index) => {
            const candidate = asRecord(artifact, `${position}[${index}] 圣遗物`)
            if (candidate.position !== position || typeof candidate.contentHash !== "string") {
                throw failure("fixture-invalid", `${position}[${index}] 圣遗物字段不完整`, {artifact})
            }
            return candidate as unknown as ArtifactFixture
        })
    }
    if (document.kumi !== undefined) {
        if (!Array.isArray(document.kumi)) {
            throw failure("fixture-invalid", "圣遗物数据的 kumi 必须是数组", {kumi: document.kumi})
        }
        parsed.kumi = document.kumi.map((entry, index) => {
            const candidate = asRecord(entry, `圣遗物数据.kumi[${index}]`)
            if (typeof candidate.title !== "string") {
                throw failure("fixture-invalid", `圣遗物数据.kumi[${index}] 缺少 title`, {entry})
            }
            if (candidate.dir === false
                && (!Array.isArray(candidate.artifactIds)
                    || candidate.artifactIds.some(id => typeof id !== "number"))) {
                throw failure("fixture-invalid", `圣遗物数据.kumi[${index}] 缺少有效 artifactIds`, {entry})
            }
            return candidate as ArtifactLoadoutFixture
        })
    }
    return parsed
}

function parsePresets(value: unknown): Array<Record<string, unknown>> {
    if (!Array.isArray(value) || value.length === 0) {
        throw failure("fixture-invalid", "预设数据必须是非空数组", {value})
    }
    return value.map((entry, index) => {
        const candidate = asRecord(entry, `预设数据[${index}]`)
        if (typeof candidate.name !== "string" || !candidate.item || typeof candidate.item !== "object") {
            throw failure("fixture-invalid", `预设数据[${index}] 字段不完整`, {entry})
        }
        return candidate
    })
}

function importedArtifactIds(
    artifacts: unknown,
    source: ArtifactDocument,
    loadoutName: string
): number[] {
    if (!Array.isArray(artifacts)) {
        throw failure("loadout-error", "已导入圣遗物列表不是数组", {artifacts})
    }
    let fixtures: ArtifactFixture[]
    if (source.kumi) {
        const loadout = source.kumi.find(entry => entry.title === loadoutName && entry.dir === false)
        if (!loadout || !Array.isArray(loadout.artifactIds)) {
            throw failure("fixture-invalid", `圣遗物数据中找不到套装 ${loadoutName}`, {
                loadoutName,
                available: source.kumi.filter(entry => entry.dir === false).map(entry => entry.title)
            })
        }
        const byId = new Map<number, ArtifactFixture>()
        for (const position of artifactPositions) {
            for (const fixture of source[position]) {
                if (typeof fixture.id === "number") byId.set(fixture.id, fixture)
            }
        }
        fixtures = loadout.artifactIds.map(id => {
            const fixture = byId.get(id)
            if (!fixture) {
                throw failure("fixture-invalid", `套装 ${loadoutName} 引用了不存在的圣遗物 ${id}`, {
                    loadoutName,
                    artifactId: id
                })
            }
            return fixture
        })
        if (fixtures.length !== artifactPositions.length
            || new Set(fixtures.map(fixture => fixture.position)).size !== artifactPositions.length) {
            throw failure("fixture-invalid", `套装 ${loadoutName} 必须包含五个不同部位`, {
                loadoutName,
                positions: fixtures.map(fixture => fixture.position)
            })
        }
    } else {
        fixtures = artifactPositions.map(position => {
            if (source[position].length !== 1) {
                throw failure("fixture-invalid", `应用套装时 ${position} 必须只提供一件圣遗物`, {
                    position,
                    count: source[position].length
                })
            }
            return source[position][0]
        })
    }

    return fixtures.map(fixture => {
        const position = fixture.position
        const imported = artifacts.find(item => {
            const candidate = asRecord(item, "已导入圣遗物", "loadout-error")
            const sameId = typeof fixture.id === "number" && candidate.id === fixture.id
            return candidate.position === position
                && (sameId || candidate.contentHash === fixture.contentHash)
        })
        if (!imported) {
            throw failure("loadout-error", `${position} 圣遗物导入后无法定位`, {
                position,
                contentHash: fixture.contentHash
            })
        }
        const id = asRecord(imported, "已导入圣遗物", "loadout-error").id
        if (typeof id !== "number") {
            throw failure("loadout-error", `${position} 圣遗物缺少有效 id`, {imported})
        }
        return id
    })
}

function currentCharacterId(context: ProjectTestContext): number {
    if (typeof context.state.currentCharacterId !== "number") {
        throw failure("calculation-error", "尚未应用预设，无法执行计算器操作")
    }
    return context.state.currentCharacterId
}

function characterIdFor(context: ProjectTestContext, presetName?: string): number {
    if (presetName !== undefined) {
        const characterId = context.state.characterIds[presetName]
        if (typeof characterId !== "number") {
            throw failure("calculation-error", `尚未添加预设 ${presetName} 对应的角色`, {presetName})
        }
        return characterId
    }
    return currentCharacterId(context)
}

function calculateDamageError(actualDamage: number, expectedDamage: number) {
    const absoluteDifference = Math.abs(actualDamage - expectedDamage)
    const relativeError = expectedDamage === 0
        ? (actualDamage === 0 ? 0 : Number.POSITIVE_INFINITY)
        : absoluteDifference / Math.abs(expectedDamage)
    return {absoluteDifference, relativeError}
}

function findDamageVariant(value: unknown, variantKey: string, metric: DamageMetric) {
    const damage = asRecord(value, "计算结果", "calculation-error")
    if (!Array.isArray(damage.variants)) {
        throw failure("calculation-error", "计算结果缺少 variants", {damage})
    }
    const variant = damage.variants.find(item => {
        const candidate = asRecord(item, "伤害变体", "calculation-error")
        const result = asRecord(candidate.result, "伤害变体.result", "calculation-error")
        return candidate.key === variantKey && typeof result[metric] === "number"
    })
    if (!variant) {
        throw failure("calculation-error", `计算结果中找不到伤害变体 ${variantKey}`, {
            variantKeys: damage.variants.map(item => asRecord(item, "伤害变体", "calculation-error").key),
            metric
        })
    }
    const candidate = asRecord(variant, "伤害变体", "calculation-error")
    const result = asRecord(candidate.result, "伤害变体.result", "calculation-error")
    return {key: String(candidate.key), damage: result[metric] as number}
}

async function readPresetCharacter(
    context: ProjectTestContext,
    name: string
): Promise<{preset: Record<string, unknown>; characterName: string}> {
    const preset = await runOperation("calculation-error", `读取预设 ${name}`, () => (
        context.session.api.presets.get(name)
    ))
    const presetRecord = asRecord(preset, "预设", "calculation-error")
    const item = asRecord(presetRecord.item, "预设内容", "calculation-error")
    const character = asRecord(item.character, "预设角色", "calculation-error")
    if (typeof character.name !== "string") {
        throw failure("calculation-error", `预设 ${name} 未指定角色`, {preset})
    }
    return {preset: presetRecord, characterName: character.name}
}

export function projectTestCase(
    moduleUrl: string,
    options: ProjectTestCaseOptions
): ProjectTestCaseDefinition {
    return {
        id: options.id,
        name: options.name,
        category: options.category,
        async run(session) {
            const context: ProjectTestContext = {
                session,
                fixtureDirectory: dirname(fileURLToPath(moduleUrl)),
                state: {characterIds: {}}
            }
            let details: unknown
            for (const operation of options.operations) {
                details = await operation(context)
            }
            return details
        }
    }
}

export const projectSteps = {
    importArtifacts(fixture: string): ProjectTestOperation {
        return async context => {
            const artifacts = parseArtifacts(await readJsonFixture(context, fixture))
            const total = artifactPositions.reduce((count, position) => count + artifacts[position].length, 0)
            const importDocument = Object.fromEntries(
                artifactPositions.map(position => [position, artifacts[position]])
            ) as Record<ArtifactPosition, ArtifactFixture[]>
            const report = await runOperation("import-error", "导入圣遗物", () => context.session.api.artifacts.import(
                importDocument,
                {mode: "replace", duplicate: artifacts.kumi ? "allow" : "skip", confirm: true}
            )) as Record<string, unknown>
            if (report.total !== total || report.imported !== total) {
                throw failure("import-error", "圣遗物导入数量不正确", {expectedTotal: total, report})
            }
            context.state.artifacts = artifacts
            return report
        }
    },

    importPresets(fixture: string): ProjectTestOperation {
        return async context => {
            const presets = parsePresets(await readJsonFixture(context, fixture))
            const report = await runOperation("import-error", "导入预设", () => context.session.api.presets.import(
                presets,
                {overwrite: true}
            )) as Record<string, unknown>
            if (report.total !== presets.length || report.imported !== presets.length) {
                throw failure("import-error", "预设导入数量不正确", {expectedTotal: presets.length, report})
            }
            return report
        }
    },

    enter(page: MonaApiPage = "calculator"): ProjectTestOperation {
        return context => runOperation("calculation-error", `进入 ${page} 页面`, () => context.session.navigate(page))
    },

    addCharacter(presetName: string, onField = false): ProjectTestOperation {
        return async context => {
            const {characterName} = await readPresetCharacter(context, presetName)
            let calculatorCharacter: unknown
            if (Object.keys(context.state.characterIds).length === 0) {
                const characters = await runOperation("calculation-error", "读取初始计算器角色", () => (
                    context.session.api.calculator.characters.list()
                ))
                if (!Array.isArray(characters)) {
                    throw failure("calculation-error", "计算器角色列表不是数组", {characters})
                }
                const initialCharacter = characters.find(entry => (
                    typeof asRecord(entry, "计算器角色", "calculation-error").id === "number"
                ))
                if (initialCharacter) {
                    const initialId = asRecord(initialCharacter, "计算器角色", "calculation-error").id
                    if (typeof initialId !== "number") {
                        throw failure("calculation-error", "初始计算器角色缺少有效 id", {initialCharacter})
                    }
                    calculatorCharacter = await runOperation("calculation-error", `配置初始角色 ${characterName}`, () => (
                        context.session.api.calculator.characters.patch(initialId, {
                            teamId: 1,
                            onField,
                            character: {name: characterName, level: 90, ascend: false}
                        })
                    ))
                }
            }
            if (!calculatorCharacter) {
                calculatorCharacter = await runOperation("calculation-error", `创建角色 ${characterName}`, () => (
                    context.session.api.calculator.characters.add({
                        teamId: 1,
                        onField,
                        character: {name: characterName, level: 90, ascend: false}
                    })
                ))
            }
            const characterId = asRecord(calculatorCharacter, "计算器角色", "calculation-error").id
            if (typeof characterId !== "number") {
                throw failure("calculation-error", "计算器角色缺少有效 id", {calculatorCharacter})
            }
            context.state.characterIds[presetName] = characterId
            context.state.currentCharacterId = characterId
            return {characterId, preset: presetName, character: characterName, onField}
        }
    },

    applyPreset(name: string): ProjectTestOperation {
        return async context => {
            const {characterName} = await readPresetCharacter(context, name)
            let characterId = context.state.characterIds[name]
            if (typeof characterId !== "number") {
                const characters = await runOperation("calculation-error", "读取计算器角色", () => (
                    context.session.api.calculator.characters.list()
                ))
                if (!Array.isArray(characters)) {
                    throw failure("calculation-error", "计算器角色列表不是数组", {characters})
                }
                const existing = characters.find(entry => (
                    typeof asRecord(entry, "计算器角色", "calculation-error").id === "number"
                ))
                const calculatorCharacter = existing ?? await runOperation("calculation-error", `创建角色 ${characterName}`, () => (
                    context.session.api.calculator.characters.add({
                        teamId: 1,
                        onField: true,
                        character: {name: characterName, level: 90, ascend: false}
                    })
                ))
                const createdCharacterId = asRecord(calculatorCharacter, "计算器角色", "calculation-error").id
                if (typeof createdCharacterId !== "number") {
                    throw failure("calculation-error", "计算器角色缺少有效 id", {calculatorCharacter})
                }
                characterId = createdCharacterId
            }
            await runOperation("calculation-error", `应用预设 ${name}`, () => (
                context.session.api.calculator.characters.applyPreset(characterId, name)
            ))
            context.state.characterIds[name] = characterId
            context.state.currentCharacterId = characterId
            return {characterId, preset: name, character: characterName}
        }
    },

    selectCharacter(presetName: string): ProjectTestOperation {
        return async context => {
            const characterId = characterIdFor(context, presetName)
            await runOperation("calculation-error", `选择角色 ${presetName}`, () => (
                context.session.api.calculator.selection.select(characterId)
            ))
            context.state.currentCharacterId = characterId
            return {characterId, preset: presetName}
        }
    },

    setOnField(presetName: string, onField = true): ProjectTestOperation {
        return async context => {
            const characterId = characterIdFor(context, presetName)
            await runOperation("calculation-error", `设置角色 ${presetName} 的场上状态`, () => (
                context.session.api.calculator.characters.setOnField(characterId, onField)
            ))
            if (onField) context.state.currentCharacterId = characterId
            return {characterId, preset: presetName, onField}
        }
    },

    setConfig(
        presetName: string,
        moduleName: string,
        objectName: string,
        configName: string,
        value: unknown,
        mode: "local" | "effective" = "local"
    ): ProjectTestOperation {
        return async context => {
            const characterId = characterIdFor(context, presetName)
            const configs = await runOperation("calculation-error", `读取角色 ${presetName} 的配置`, () => (
                context.session.api.calculator.config.list({characterId})
            ))
            if (!Array.isArray(configs)) {
                throw failure("calculation-error", "计算器配置列表不是数组", {configs, characterId})
            }
            const config = configs.find(entry => {
                const candidate = asRecord(entry, "计算器配置", "calculation-error")
                const address = asRecord(candidate.address, "计算器配置地址", "calculation-error")
                return address.module_name === moduleName
                    && address.object_name === objectName
                    && address.config_name === configName
            })
            if (!config) {
                throw failure("calculation-error", `找不到配置 ${moduleName}.${objectName}.${configName}`, {
                    characterId,
                    available: configs.map(entry => asRecord(entry, "计算器配置", "calculation-error").address)
                })
            }
            const configId = asRecord(config, "计算器配置", "calculation-error").id
            if (typeof configId !== "string") {
                throw failure("calculation-error", "计算器配置缺少有效 id", {config})
            }
            await runOperation("calculation-error", `设置配置 ${moduleName}.${objectName}.${configName}`, () => (
                context.session.api.calculator.config.set(configId, value, {mode})
            ))
            return {characterId, preset: presetName, configId, value, mode}
        }
    },

    applyArtifactSet(name: string, presetName?: string): ProjectTestOperation {
        return async context => {
            const characterId = characterIdFor(context, presetName)
            const source = context.state.artifacts
            if (!source) throw failure("loadout-error", "尚未导入圣遗物，无法应用套装")
            const artifacts = await runOperation("loadout-error", "读取已导入圣遗物", () => context.session.api.artifacts.list())
            const artifactIds = importedArtifactIds(artifacts, source, name)
            let folderId = context.state.artifactFolderId
            if (typeof folderId !== "number") {
                const folder = await runOperation("loadout-error", "创建圣遗物套装目录", () => (
                    context.session.api.artifactLoadouts.folders.create({name: "project-fixtures"})
                ))
                const createdFolderId = asRecord(folder, "圣遗物套装目录", "loadout-error").id
                if (typeof createdFolderId !== "number") {
                    throw failure("loadout-error", "圣遗物套装目录缺少有效 id", {folder})
                }
                folderId = createdFolderId
                context.state.artifactFolderId = folderId
            }
            const loadout = await runOperation("loadout-error", `创建圣遗物套装 ${name}`, () => (
                context.session.api.artifactLoadouts.create({folderId, name, artifactIds})
            ))
            const loadoutRecord = asRecord(loadout, "圣遗物套装", "loadout-error")
            if (!Array.isArray(loadoutRecord.artifactIds)
                || loadoutRecord.artifactIds.length !== artifactIds.length
                || loadoutRecord.artifactIds.some((id, index) => id !== artifactIds[index])) {
                throw failure("loadout-error", "创建的圣遗物套装与导入数据不一致", {loadout})
            }
            await runOperation("loadout-error", `应用圣遗物套装 ${name}`, () => (
                context.session.api.calculator.characters.setArtifacts(characterId, loadoutRecord.artifactIds)
            ))
            const state = await runOperation("loadout-error", "校验计算器圣遗物装备状态", () => (
                context.session.api.calculator.characters.get(characterId)
            ))
            const equipped = asRecord(asRecord(state, "计算器角色状态", "loadout-error").artifacts, "计算器装备圣遗物", "loadout-error")
            for (const [index, position] of artifactPositions.entries()) {
                if (equipped[position] !== artifactIds[index]) {
                    throw failure("loadout-error", `${position} 圣遗物未正确应用`, {
                        expected: artifactIds[index],
                        actual: equipped[position]
                    })
                }
            }
            context.state.currentCharacterId = characterId
            return {characterId, loadout: name, artifactIds}
        }
    },

    setEnemyLevel(level: number): ProjectTestOperation {
        return async context => {
            if (!Number.isInteger(level) || level <= 0) {
                throw failure("fixture-invalid", "怪物等级必须是正整数", {level})
            }
            const characterId = currentCharacterId(context)
            await runOperation("calculation-error", `设置怪物等级 ${level}`, () => (
                context.session.api.calculator.characters.patch(characterId, {enemy: {level}})
            ))
            const state = await runOperation("calculation-error", "校验怪物等级", () => (
                context.session.api.calculator.characters.get(characterId)
            ))
            const enemy = asRecord(
                asRecord(state, "计算器角色状态", "calculation-error").enemy,
                "计算器敌人状态",
                "calculation-error"
            )
            if (enemy.level !== level) {
                throw failure("calculation-error", "怪物等级未正确设置", {
                    characterId,
                    expected: level,
                    actual: enemy.level
                })
            }
            return {characterId, enemyLevel: level}
        }
    },

    selectSkill(index: number): ProjectTestOperation {
        return async context => {
            if (!Number.isInteger(index) || index < 0) {
                throw failure("fixture-invalid", "技能索引必须是非负整数", {index})
            }
            const characterId = currentCharacterId(context)
            await runOperation("calculation-error", `选择技能 ${index}`, () => (
                context.session.api.calculator.characters.patch(characterId, {skill: {selectedIndex: index}})
            ))
            return {characterId, skillIndex: index}
        }
    },

    expectDamage(
        variantKey: string,
        metric: DamageMetric,
        expectedDamage: number,
        allowedError: number
    ): ProjectTestOperation {
        return async context => {
            if (!variantKey || !Number.isFinite(expectedDamage) || !Number.isFinite(allowedError)
                || allowedError < 0 || allowedError > 1) {
                throw failure("fixture-invalid", "伤害断言参数无效", {
                    variantKey,
                    metric,
                    expectedDamage,
                    allowedError
                })
            }
            const characterId = currentCharacterId(context)
            const damage = await runOperation("calculation-error", "计算伤害", () => (
                context.session.api.calculator.damage.getCurrent({characterId, includeRaw: true})
            ))
            const variant = findDamageVariant(damage, variantKey, metric)
            const {absoluteDifference, relativeError} = calculateDamageError(variant.damage, expectedDamage)
            if (relativeError > allowedError) {
                throw failure(
                    "damage-out-of-tolerance",
                    `伤害超出允许误差: actual=${variant.damage}, expected=${expectedDamage}, relativeError=${relativeError}, allowedErrorRatio=${allowedError}`,
                    {
                        characterId,
                        metric,
                        variantKey: variant.key,
                        actualDamage: variant.damage,
                        expectedDamage,
                        absoluteDifference,
                        relativeError,
                        allowedErrorRatio: allowedError
                    }
                )
            }
            return {
                characterId,
                metric,
                variantKey: variant.key,
                actualDamage: variant.damage,
                expectedDamage,
                absoluteDifference,
                relativeError,
                allowedErrorRatio: allowedError
            }
        }
    }
}
