import { targetFunctionData } from "@targetFunction"

export function upgradeTargetFunctionConfig(name, oldConfig) {
    if (!name) {
        return null
    }

    const data = targetFunctionData[name]
    if (!data) {
        return null
    }

    const configs = data.config ?? []
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
    // console.log(oldConfig, newConfig)

    return {
        [name]: newConfig
    }
}
