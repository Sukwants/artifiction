import assert from "node:assert/strict"
import {defineProjectTestSuite} from "../../framework"

const algorithms = ["AStar", "Heuristic", "Naive", undefined] as const
const presetName = (algorithm: string | undefined, version = 4) => `optimizer-v${version}-${algorithm ?? "default"}`

function preset(algorithm: string | undefined, version: number) {
    const name = presetName(algorithm, version)
    const params = version === 3 ? "NoConfig" : {}
    return {name, version, item: {
        name, ...(algorithm === undefined ? {} : {algorithm}),
        character: {name: "Amber", level: 90, ascend: false, constellation: 0, skill1: 7, skill2: 7, skill3: 7, tags: [], params},
        weapon: {name: "FavoniusWarbow", level: 90, ascend: false, refine: 1, params},
        targetFunction: {name: "AmberDefault", params},
        constraint: {setNames: [], minRecharge: 1, minCritical: 0, minCriticalDamage: 0, minElementalMastery: 0},
        filter: {sandMainStats: [], gobletMainStats: [], headMainStats: []},
        artifactEffectMode: "auto", useDSL: false, dslSource: "", buffs: version === 3 ? [] : {},
    }}
}

export default defineProjectTestSuite({
    id: "single-optimizer",
    name: "单人配装与旧预设兼容",
    description: "验证算法选择、旧预设持久化和真实 Worker 返回的完整前 100。",
    cases: [
        {
            id: "preset-import-export",
            name: "版本 3/4 预设原名称导入导出",
            async run({api}) {
                const entries = [3, 4].flatMap(version => algorithms.map(algorithm => preset(algorithm, version)))
                const imported = await api.presets.import(entries)
                assert.equal(imported.imported, entries.length)
                const exported = await api.presets.export(entries.map(entry => entry.name))
                assert.equal(exported.schemaVersion, 1)
                for (const entry of exported.data) {
                    const original = entries.find(item => item.name === entry.name)!
                    assert.equal(entry.version, 4)
                    assert.equal(entry.item.algorithm, original.item.algorithm)
                    assert.deepEqual(entry.item.filter, original.item.filter)
                    assert.deepEqual(entry.item.constraint, original.item.constraint)
                    assert.equal(entry.item.dslSource, original.item.dslSource)
                }
                await api.presets.import(exported, {overwrite: true})
                assert.deepEqual(await api.presets.export(entries.map(entry => entry.name)), exported)
                await api.ready()
            }
        },
        {
            id: "preset-reload-and-ui",
            name: "重新加载后选择和保存不改写旧算法名称",
            async run({api, reload, navigate, page}) {
                await reload()
                await api.ready()
                await navigate("calculator")
                assert.equal(await api.calculator.isActive(), true)
                const id = (await api.calculator.characters.list())[0].id
                for (const algorithm of algorithms) {
                    const name = presetName(algorithm)
                    assert.equal((await api.presets.get(name)).item.algorithm, algorithm)
                    const state = await api.calculator.characters.applyPreset(id, name)
                    assert.equal(state.optimization.algorithm, algorithm ?? "AStar")
                }
                await api.calculator.characters.applyPreset(id, presetName("Heuristic"))
                await page.click('[data-testid="optimization-settings"]')
                await page.waitForSelector('[data-testid="optimization-algorithm"]', {visible: true})
                assert.equal(await page.$eval('[data-testid="optimization-algorithm"] input:checked', input => (input as HTMLInputElement).value), "AStar")
                assert.match(await page.$eval('[data-testid="optimization-algorithm-description"]', element => element.textContent ?? ""), /不保证|does not guarantee/)
                await page.keyboard.press("Escape")
                await page.waitForSelector('[data-testid="optimization-algorithm"]', {hidden: true})
                await page.click('[data-testid="save-optimization-preset"] .el-button')
                assert.equal((await api.presets.get(presetName("Heuristic"))).item.algorithm, "Heuristic")

                await page.click('[data-testid="optimization-settings"]')
                await page.waitForSelector('[data-testid="optimization-algorithm"]', {visible: true})
                await page.click('[data-testid="optimization-algorithm"] label:has(input[value="Naive"])')
                assert.equal((await api.calculator.characters.get(id)).optimization.algorithm, "Naive")
                await page.keyboard.press("Escape")
                await page.waitForSelector('[data-testid="optimization-algorithm"]', {hidden: true})
                await page.click('[data-testid="save-optimization-preset"] .el-button')
                assert.equal((await api.presets.get(presetName("Heuristic"))).item.algorithm, "Naive")
                await api.ready()
            }
        },
        {
            id: "state-round-trip",
            name: "完整计算器状态及账号切换兼容旧算法名称",
            async run({api, navigate, reload}) {
                await reload()
                await navigate("calculator")
                const id = (await api.calculator.characters.list())[0].id
                assert.equal((await api.presets.get(presetName("Heuristic"))).item.algorithm, "Naive")
                await api.calculator.characters.patch(id, {optimization: {algorithm: "Heuristic"}})
                const saved = await api.calculator.state.get()
                await api.calculator.state.set(saved)
                const restored = await api.calculator.characters.list()
                assert.equal((await api.calculator.characters.get(restored[0].id)).optimization.algorithm, "Heuristic")
                const account = await api.accounts.getCurrent()
                const other = await api.accounts.create({name: "optimizer-isolation"})
                await api.accounts.switch(other.id)
                assert.equal((await api.presets.list()).length, 0)
                await api.accounts.switch(account.id)
                assert.equal((await api.presets.get(presetName("Naive"))).item.algorithm, "Naive")
                assert.equal((await api.presets.get(presetName(undefined))).item.algorithm, undefined)
            }
        },
        {
            id: "worker-top-100",
            name: "三种旧标识均返回一致的前 100 套不同配装",
            category: "calculation",
            async run({api, navigate, page}) {
                await navigate("calculator")
                const id = (await api.calculator.characters.list())[0].id
                await api.calculator.characters.applyPreset(id, presetName("AStar"))
                const slots = ["flower", "feather", "sand", "cup", "head"]
                const stats = ["lifeStatic", "attackStatic", "attackPercentage", "fireBonus", "critical"]
                const values = [4780, 311, 0.466, 0.466, 0.311]
                const ids: number[][] = []
                // Deliberately identical profiles with different IDs: 4^5 builds.
                for (let slot = 0; slot < slots.length; slot++) {
                    ids.push([])
                    for (let i = 0; i < 4; i++) {
                        const art = await api.artifacts.create({position: slots[slot], setName: "gladiatorFinale", star: 5, level: 20,
                            mainTag: {name: stats[slot], value: values[slot]}, normalTags: [{name: "criticalDamage", value: 0.14}]})
                        ids[slot].push(art.id)
                    }
                    // Store IDs are random safe integers, not creation order.
                    ids[slot].sort((a, b) => a - b)
                }
                const expectedIds = ids[0].flatMap(a => ids[1].flatMap(b => ids[2].flatMap(c => ids[3].flatMap(d => ids[4].map(e => [a, b, c, d, e]))))).slice(0, 100)
                let reference: any[] | undefined
                for (const algorithm of ["Naive", "AStar", "Heuristic"]) {
                    await api.calculator.characters.patch(id, {optimization: {algorithm}})
                    const {results} = await api.calculator.optimize.run({characterId: id, applyResult: 100})
                    assert.equal(results.length, 100)
                    assert.deepEqual(results.map((r: any) => [r.flower, r.feather, r.sand, r.goblet, r.head]), expectedIds)
                    assert.ok(results.every((r: any) => Number.isFinite(r.value) && Number.isFinite(r.ratio)))
                    if (reference) assert.deepEqual(results, reference)
                    else reference = results
                    assert.match(await page.$eval('[data-testid="optimization-results"]', element => element.textContent ?? ""), /100/)
                }
                return {combinations: 1024, returned: reference!.length}
            }
        }
    ]
})
