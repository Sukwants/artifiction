import assert from "node:assert/strict"
import {test} from "node:test"
import {normalizeDamageEvent} from "../../src/api/mona/damage"

test("damage API preserves fractional hit count, single-hit sources, and already-totalled damage", () => {
    const count = 6 / 0.66
    const raw = {ElevativeDamage: {
        damage_count: count,
        em_ratio: {skill: 0.9},
        reaction_extra: {buff: 1000},
        result: {critical: 2000 * count, non_critical: 1000 * count, expectation: 1500 * count},
    }}
    const normalized = normalizeDamageEvent(raw)
    const variant = normalized.variants[0]
    assert.equal(variant.damageCount, count)
    assert.equal(variant.result.expectation, raw.ElevativeDamage.result.expectation)
    assert.equal(variant.details.find(item => item.key === "em_ratio")?.children?.[0].value, 0.9)
    assert.equal(variant.details.find(item => item.key === "damage_count")?.value, count)
    assert.deepEqual(normalized.raw, raw)
})

test("damage API defaults legacy counts to one and retains an explicit zero", () => {
    const result = {critical: 0, non_critical: 0, expectation: 0}
    const normalized = normalizeDamageEvent({Damage: {
        normal: {result},
        melt: {damage_count: 0, result},
    }})
    assert.deepEqual(normalized.variants.map(item => item.damageCount), [1, 0])
})
