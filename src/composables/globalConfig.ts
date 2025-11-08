// @ts-ignore
import { globalConfigData } from "@globalConfig"

const globalConfigConfig: any = {};

for(let i in globalConfigData) {
    globalConfigConfig[globalConfigData[i].name] = globalConfigData[i].config[0];
}

export function getGlobalConfigConfig() {
    return globalConfigConfig;
}

export function getObjectConfig(config: any) {
    if (config === "NoConfig") {
        return "NoConfig";
    }
    const res: any = {};
    for (const i in config) {
        res[i] = {};
        for (const j in config[i]) {
            res[i][j] = config[i][j].config;
        }
    }
    return res;
}

export function getObjectConfigValue(config: any) {
    if (config === "NoConfig") {
        return "NoConfig";
    }
    const res: any = {};
    for (const i in config) {
        res[i] = {};
        for (const j in config[i]) {
            res[i][j] = config[i][j].configValue;
        }
    }
    return res;
}

export function getObjectConfigUnlinked(config: any) {
    if (config === "NoConfig") {
        return "NoConfig";
    }
    const res: any = {};
    for (const i in config) {
        res[i] = {};
        for (const j in config[i]) {
            res[i][j] = config[i][j].unlinked;
        }
    }
    return res;
}

export function restoreObjectConfig(config: any, configValue: any, unlinked: any) {
    if (config === "NoConfig" || configValue === "NoConfig" || unlinked === "NoConfig") {
        return "NoConfig";
    }
    const res: any = {};
    for (const i in config) {
        res[i] = {};
        for (const j in config[i]) {
            if (!res[i][j]) res[i][j] = {};
            res[i][j].config = config[i][j];
        }
    }
    for (const i in configValue) {
        for (const j in configValue[i]) {
            if (!res[i][j]) res[i][j] = {};
            res[i][j].configValue = configValue[i][j];
        }
    }
    for (const i in unlinked) {
        for (const j in unlinked[i]) {
            if (!res[i][j]) res[i][j] = {};
            res[i][j].unlinked = unlinked[i][j];
        }
    }
    return res;
}

export function useGlobalConfig() {
    const values: any = {};

    function setGlobalConfig(list: any) {
        for(let i in globalConfigConfig) {
            values[globalConfigConfig[i].name] = [
                {
                    "priority": 0,
                    "value": globalConfigConfig[i].default,
                    "update_value": (val: any) => {}
                }
            ]
        }

        for (const p of list) {
            for (const i of p.configConfig) {
                if (i.type == "globalLink" && p.config[i.name].unlinked !== true) {
                    values[i.name].push({
                        "priority": i.priority,
                        "value": p.config[i.name].config,
                        "update_value": (val: any) => {
                            p.config[i.name].config = val;
                        }
                    })
                }
            }
        }

        let res: any = {};
        for (const name in values) {
            let current_priority = -1;
            for (const i of values[name]) {
                if (i.priority > current_priority) {
                    current_priority = i.priority;
                    res[name] = i.value;
                }
            }
        }

        for (const p of list) {
            for (const i of p.configConfig) {
                if (i.type == "globalLink" && p.config[i.name].unlinked !== true) {
                    p.config[i.name].configValue = res[i.name];
                } else {
                    p.config[i.name].configValue = p.config[i.name].config;
                }
            }
        }
    }

    function updateGlobalConfig(name: string, value: any) {
        let max_priority = -1;
        for (let i of values[name]) {
            if (i.priority > max_priority) {
                max_priority = i.priority;
            }
        }
        for (let i of values[name]) {
            if (i.priority === max_priority) {
                i.update_value(value);
            }
        }
    }

    setGlobalConfig([])

    return {
        setGlobalConfig,
        updateGlobalConfig,
    };
}