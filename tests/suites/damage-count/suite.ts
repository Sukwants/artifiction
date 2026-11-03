import assert from "node:assert/strict"
import {defineProjectTestSuite} from "../../framework"

export default defineProjectTestSuite({
    id: "damage-count",
    name: "伤害次数从计算核心到页面",
    cases: [{
        id: "mitya-expected-hit-count",
        category: "calculation",
        async run({api, navigate, page}) {
            await api.ready()
            await navigate("calculator")
            assert.equal(await api.calculator.isActive(), true)
            const characters = await api.calculator.characters.list()
            const characterId = characters[0].id
            await api.calculator.characters.patch(characterId, {
                character: {name: "Mitya", level: 90, ascend: false, constellation: 6, skill1: 0, skill2: 0, skill3: 0},
                weapon: {name: "ApprenticesNotes", level: 1, ascend: false, refine: 1},
                skill: {selectedIndex: 4, params: {Mitya: {
                    core_state: 2, beacon_count: 5,
                    talent2_crit_stack: 4, beacon_generated_during_charge: true,
                }}},
            })
            // MityaDamageEnum: 4 为单枚星超导，5 为聚爆星超导总伤害。
            const single = await api.calculator.damage.getCurrent({characterId, includeRaw: true})
            assert.equal(single.variants[0].damageCount, 1)

            for (const [constellation, generated, count] of [[0, false, 4], [0, true, 5], [6, false, 5 / 0.66], [6, true, 6 / 0.66]] as const) {
                await api.calculator.characters.patch(characterId, {
                    character: {constellation},
                    skill: {selectedIndex: 5, params: {Mitya: {beacon_generated_during_charge: generated}}},
                })
                const total = await api.calculator.damage.getCurrent({characterId, includeRaw: true})
                const variant = total.variants[0]
                assert.ok(Math.abs(variant.damageCount - Number(count)) < 1e-12)
                assert.equal(total.raw.ElevativeDamage.damage_count, variant.damageCount)
                assert.deepEqual(total.raw.ElevativeDamage.em_ratio, single.raw.ElevativeDamage.em_ratio)
                assert.deepEqual(total.raw.ElevativeDamage.reaction_extra, single.raw.ElevativeDamage.reaction_extra)
                await page.waitForFunction((text: string) => document.querySelector(".damage-analysis-div")?.textContent?.includes(text),
                    {}, String(Number(Number(count).toFixed(6))))
                if (constellation === 6 && generated) {
                    assert.ok(Math.abs(variant.result.expectation - single.variants[0].result.expectation * Number(count)) < 1e-6)
                }
            }

            await page.click('[data-testid="damage-analysis-button"]')
            await page.waitForSelector(".damage-count", {visible: true})
            const displayed = await page.$eval(".damage-summary", element => ({
                formula: element.querySelector(".damage-count")?.textContent ?? "",
                count: element.querySelector(".damage-count")?.getAttribute("title"),
                total: Number(element.querySelector(".damage-display")?.textContent),
            }))
            assert.ok(Math.abs(Number(displayed.count) - 6 / 0.66) < 1e-12)
            assert.ok(displayed.formula.includes("9.090909"))
            const singleDisplayed = Number(displayed.formula.match(/\d+/)?.[0])
            assert.ok(Number.isFinite(singleDisplayed) && singleDisplayed > 0)
            assert.ok(Math.abs(displayed.total - singleDisplayed * Number(displayed.count)) <= Number(displayed.count) / 2 + 1)
            return displayed
        },
    }],
})
