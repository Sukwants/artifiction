// @ts-ignore
import { globalConfigData } from "@globalConfig"
import { RandomIDProvider } from "@/utils/idProvider"

export function useGlobalConfig() {
    const values = ref<any>({});

    function linkGlobalConfig(name: string, priority: number, get_value: Function, update_value: Function) {
        values.value[name].push({
            "priority": priority,
            "get_value": get_value,
            "update_value": update_value
        })
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

    function clearLinkedGlobalConfig() {
        for(let i in globalConfigConfig) {
            values.value[globalConfigConfig[i].name] = [
                {
                    "priority": 0,
                    "get_value": () => globalConfigConfig[i].default,
                    "update_value": () => {}
                }
            ]
        }
    }

    const globalConfig = computed((): Record<string, any> => {
        let res: Record<string, any> = {};
        for (let name in values.value) {
            let current_priority = -1;
            for (let id in values.value[name]) {
                if (values.value[name][id].priority > current_priority) {
                    current_priority = values.value[name][id].priority;
                    res[name] = values.value[name][id].get_value();
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