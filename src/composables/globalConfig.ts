// @ts-ignore
import { globalConfigData } from "@globalConfig"
import { RandomIDProvider } from "@/utils/idProvider"

export function useGlobalConfig() {
    const values = ref<any>({});

    function clearLinkedGlobalConfig() {
        for(let i in globalConfigConfig) {
            values.value[globalConfigConfig[i].name] = [
                {
                    "priority": 0,
                    "value": globalConfigConfig[i].default,
                    "update_value": () => {}
                }
            ]
        }
    }

    function linkGlobalConfig(configs: any, config: any, unlinked: any) {
        for (const i of configs) {
            if (i.type == "globalLink" && unlinked[i.name] !== true) {
                values.value[i.name].push({
                    "priority": i.priority,
                    "value": config[i.name],
                    "update_value": (value: any) => {config[i.name] = value}
                })
            }
        }
    }

    function updateGlobalConfig(name: string, value: any) {
        let max_priority = -1;
        for (let i of values.value[name]) {
            if (i.priority > max_priority) {
                max_priority = i.priority;
            }
        }
        for (let i of values.value[name]) {
            if (i.priority === max_priority) {
                i.update_value(value);
            }
        }
    }

    const globalConfig = computed((): any => {
        let res: any = {};
        for (let name in values.value) {
            let current_priority = -1;
            for (let id in values.value[name]) {
                if (values.value[name][id].priority > current_priority) {
                    current_priority = values.value[name][id].priority;
                    res[name] = values.value[name][id].value;
                }
            }
        }
        return res;
    })

    let globalConfigConfig: any = {};

    for(let i in globalConfigData) {
        globalConfigConfig[globalConfigData[i].name] = globalConfigData[i].config[0];
    }

    clearLinkedGlobalConfig()

    return {
        globalConfig,
        globalConfigConfig,
        linkGlobalConfig,
        updateGlobalConfig,
        clearLinkedGlobalConfig,
    };
}