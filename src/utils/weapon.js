import { weaponData } from "@asset/weapon"

export function getWeaponDefaultConfigWasmInterface(name) {
    const data = weaponData[name]
    const hasConfig = data.configs && data.configs.length > 0

    if (hasConfig) {
        let defaultConfigs = {}
        for (let c of data.configs) {
            defaultConfigs[c.name] = c.default
        }

        return {
            [name]: defaultConfigs
        }
    } else {
        return null
    }
}

export function upgradeWeaponConfig(name, oldConfig) {
    if (!name) {
        return null
    }

    const data = weaponData[name]
    if (!data) {
        return null
    }

    const configs = data.configs ?? []
    if (configs.length === 0) {
        return null
    }

    if (Object.prototype.hasOwnProperty.call(oldConfig, name)) {
        oldConfig = oldConfig[name]
    } else {
        oldConfig = {}
    }
    let newConfig = {}
    for (let c of configs) {
        const configName = c.name
        if (Object.prototype.hasOwnProperty.call(oldConfig, configName)) {
            newConfig[configName] = oldConfig[configName]
        } else {
            newConfig[configName] = c.default
        }
    }

    return {
        [name]: newConfig
    }
}
