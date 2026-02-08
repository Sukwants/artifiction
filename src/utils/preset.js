import { deepCopy } from "@util/common"
import { upgradeCharacterConfig } from "@util/character"
import { upgradeWeaponConfig } from "@util/weapon"
import { upgradeTargetFunctionConfig } from "@util/targetFunction"
import { upgradeArtifactConfig } from "@util/artifacts"
import { convertArtifactName, convertArtifactStatName } from "@util/converter"

export function checkImportFormat(obj) {
    if (!Array.isArray(obj)) {
        return false
    }

    return true
}

export function upgradePresetItem(item) {
    let temp = deepCopy(item)

    const characterName = item.character.name
    temp.character.params = upgradeCharacterConfig(characterName, temp.character.params)

    const weaponName = item.weapon.name
    temp.weapon.params = upgradeWeaponConfig(weaponName, temp.weapon.params)

    const targetFunctionName = item.targetFunction.name
    temp.targetFunction.params = upgradeTargetFunctionConfig(targetFunctionName, temp.targetFunction.params)
    if (temp.artifactConfig) {
        temp.artifactConfig = upgradeArtifactConfig(temp.artifactConfig)
    }

    // console.log(temp)

    return temp
}

export function artifactSetNamesToConstraintSetMode(setNames) {
    if (!setNames || setNames.length === 0) {
        return "Any"
    }

    if (setNames.length === 1) {
        return {
            "Set4": convertArtifactName(setNames[0])
        }
    }

    return {
        "Set22": setNames.map(x => convertArtifactName(x))
    }
}

export function convertPresetToWasmInterface(item) {
    let wasm = {}

    wasm.character = item.character
    wasm.weapon = item.weapon
    wasm.target_function = item.targetFunction

    if (item.constraint) {
        wasm.constraint = {
            set_mode: artifactSetNamesToConstraintSetMode(item.constraint.setNames),
            recharge_min: item.constraint.minRecharge ?? 1,
            em_min: item.constraint.minElementalMastery ?? 0,
            crit_min: item.constraint.minCritical ?? 0,
            crit_damage_min: item.constraint.minCriticalDamage ?? 0
        }
    } else {
        wasm.constraint = null
    }

    if (item.artifactEffectMode === "custom") {
        wasm.artifact_config = item.artifactConfig
    }
    wasm.buffs = item.buffs ?? []

    if (item.algorithm) {
        wasm.algorithm = item.algorithm
    } else {
        wasm.algorithm = "AStar"
    }

    if (item.filter) {
        wasm.filter = {}
        if (item.filter.sandMainStats) {
            wasm.filter.sand_main_stat = item.filter.sandMainStats.map(x => convertArtifactStatName(x))
        }
        if (item.filter.gobletMainStats) {
            wasm.filter.goblet_main_stat = item.filter.gobletMainStats.map(x => convertArtifactStatName(x))
        }
        if (item.filter.headMainStats) {
            wasm.filter.head_main_stat = item.filter.headMainStats.map(x => convertArtifactStatName(x))
        }
    }

    return wasm
}

export function convertPresetListToWasmInterface(list) {
    let characters = []
    let i = 0

    for (let c of list) {

        let wasm = {}

        wasm.character = c.item.character
        wasm.weapon = c.item.weapon
        if (c.item.artifactEffectMode === "custom") {
            wasm.artifact_config = c.item.artifactConfig
        }
        wasm.buffs = c.item.buffs ?? []
        wasm.artifacts = []
        wasm.skill = null

        wasm.character_id = ++i
        wasm.team_id = 0
        if (i == 1) wasm.on_field = true
        else wasm.on_field = false
        
        characters.push(wasm)
    }

    let wasm_list = []
    i = 0

    for (let c of list) {

        let wasm = {}

        wasm.characters = characters
        wasm.active_character_id = ++i

        wasm.target_function = c.item.targetFunction

        if (c.item.constraint) {
            wasm.constraint = {
                set_mode: artifactSetNamesToConstraintSetMode(c.item.constraint.setNames),
                recharge_min: c.item.constraint.minRecharge ?? 1,
                em_min: c.item.constraint.minElementalMastery ?? 0,
                crit_min: c.item.constraint.minCritical ?? 0,
                crit_damage_min: c.item.constraint.minCriticalDamage ?? 0
            }
        } else {
            wasm.constraint = null
        }

        if (c.item.algorithm) {
            wasm.algorithm = c.item.algorithm
        } else {
            wasm.algorithm = "AStar"
        }

        if (c.item.filter) {
            wasm.filter = {}
            if (c.item.filter.sandMainStats) {
                wasm.filter.sand_main_stat = c.item.filter.sandMainStats.map(x => convertArtifactStatName(x))
            }
            if (c.item.filter.gobletMainStats) {
                wasm.filter.goblet_main_stat = c.item.filter.gobletMainStats.map(x => convertArtifactStatName(x))
            }
            if (c.item.filter.headMainStats) {
                wasm.filter.head_main_stat = c.item.filter.headMainStats.map(x => convertArtifactStatName(x))
            }
        }

        wasm_list.push(wasm)
    }

    return wasm_list
}
