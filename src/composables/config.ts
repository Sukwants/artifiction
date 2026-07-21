import { ref } from "vue";

export type GlobalLinkMeta = {
    key: string,
    priority: number,
    unlinked: boolean,
    team_shared?: boolean,
}

export type ConfigMeta = {
    type: string,
    name: string,
    default: any,
    global_link?: GlobalLinkMeta,
    [key: string]: unknown,
}

export class ConfigAddress {
    public character_id: number;
    public module_name: string
    public object_name: string;
    public config_name: string;

    constructor(character_id: number, module_name: string, object_name: string, config_name: string) {
        this.character_id = character_id;
        this.module_name = module_name;
        this.object_name = object_name;
        this.config_name = config_name;
    }

    str(): string {
        return `${this.character_id}#${this.module_name}:${this.object_name}:${this.config_name}`;
    }

    from_str(s: string): ConfigAddress {
        const [character_id_str, rest] = s.split('#');
        const [module_name, object_name, config_name] = rest.split(':');
        return new ConfigAddress(parseInt(character_id_str), module_name, object_name, config_name);
    }

    str_without_character(): string {
        return `${this.module_name}:${this.object_name}:${this.config_name}`;
    }
};

export class ConfigManager {
    private version = ref(0); // 配置版本号，每次配置更新时自增，用于触发组件更新
    private values: Map<string, any> = new Map(); // 配置项值
    private global_link_metas: Map<string, GlobalLinkMeta> = new Map(); // 配置项的全局链接信息
    private global_key_lists: Map<string, Set<ConfigAddress>> = new Map(); // 全局链接键对应的配置项地址集合

    registerObject(character_id: number, module_name: string, object_name: string, config_metas: ConfigMeta[] | undefined): Record<string, ConfigAddress> {
        // 将对象注册到配置系统中，返回配置项地址的映射

        if (!config_metas) return {};

        ++this.version.value;
        config_metas = structuredClone(config_metas);

        let config_addresses: Record<string, ConfigAddress> = {};

        for (let config of config_metas) {
            config_addresses[config.name] = new ConfigAddress(character_id, module_name, object_name, config.name);

            this.values.set(config_addresses[config.name].str(), config.default);

            if (config.global_link) {
                this.global_link_metas.set(config_addresses[config.name].str(), config.global_link);
                if (!this.global_key_lists.has(config.global_link.key)) {
                    this.global_key_lists.set(config.global_link.key, new Set());
                }
                this.global_key_lists.get(config.global_link.key)?.add(config_addresses[config.name]);
            }
        }
        return config_addresses;
    }

    unregisterObject(config_addresses: Record<string, ConfigAddress>) {
        // 将对象从配置系统中注销

        ++this.version.value;

        for (let config_name in config_addresses) {
            this.values.delete(config_addresses[config_name].str());

            if (this.global_link_metas.has(config_addresses[config_name].str())) {
                this.global_key_lists.get(this.global_link_metas.get(config_addresses[config_name].str())!.key)?.delete(config_addresses[config_name]);
            }

            this.global_link_metas.delete(config_addresses[config_name].str());
        }
    }

    getActiveAddresses(current_address: ConfigAddress): ConfigAddress[] {
        // 对于给定的配置项地址，返回所有对该配置项覆写生效的配置项地址集合

        void this.version.value;

        const current_info = this.global_link_metas.get(current_address.str());
        if (!current_info || current_info.unlinked) return [];

        const addresses = this.global_key_lists.get(current_info.key);
        if (!addresses) return [];

        let max_priority = -Infinity;
        let active_addresses: ConfigAddress[] = [];
        for (const address of addresses) {
            const info = this.global_link_metas.get(address.str())!;
            if (!info || info.unlinked) continue;
            if ((address.character_id == current_address.character_id || info.team_shared) && info.priority >= max_priority) {
                if (info.priority > max_priority) {
                    max_priority = info.priority;
                    active_addresses = [];
                }
                active_addresses.push(address);
            }
        }
        return structuredClone(active_addresses);
    }

    getConfigValue(config_address: ConfigAddress): any {
        // 获取配置项的值，如果该配置项被全局链接覆写，则返回优先级最高的任意配置项的值

        void this.version.value;

        if (this.global_link_metas.has(config_address.str()) && !this.global_link_metas.get(config_address.str())!.unlinked) {
            const active_addresses = this.getActiveAddresses(config_address);
            if (active_addresses.length > 0) {
                return this.values.get(active_addresses[0].str());
            }
        }
        return structuredClone(this.values.get(config_address.str()));
    }

    updateConfigValue(config_address: ConfigAddress, value: any): void {
        // 更新配置项的值，如果该配置项被全局链接覆写，则更新优先级最高的所有配置项的值

        ++this.version.value;
        value = structuredClone(value);

        if (this.global_link_metas.has(config_address.str()) && !this.global_link_metas.get(config_address.str())!.unlinked) {
            const active_addresses = this.getActiveAddresses(config_address);
            for (const address of active_addresses) {
                this.values.set(address.str(), value);
            }
        } else {
            this.values.set(config_address.str(), value);
        }
    }
    
    getObjectValue(config_addresses: Record<string, ConfigAddress>): Record<string, any> {
        let res: Record<string, any> = {};
        for (let config_name in config_addresses) {
            res[config_name] = this.getConfigValue(config_addresses[config_name]);
        }
        return structuredClone(res);
    }

    updateObjectValue(config_addresses: Record<string, ConfigAddress>, values: Record<string, any>): void {
        for (let config_name in config_addresses) {
            if (config_name in values) {
                this.updateConfigValue(config_addresses[config_name], values[config_name]);
            }
        }
    }

    getModuleValue(config_addresses: Record<string, Record<string, ConfigAddress>>): Record<string, Record<string, any>> {
        let res: Record<string, Record<string, any>> = {};
        for (let object_name in config_addresses) {
            res[object_name] = this.getObjectValue(config_addresses[object_name]);
        }
        return structuredClone(res);
    }

    updateModuleValue(config_addresses: Record<string, Record<string, ConfigAddress>>, values: Record<string, Record<string, any>>): void {
        for (let object_name in config_addresses) {
            if (object_name in values) {
                this.updateObjectValue(config_addresses[object_name], values[object_name]);
            }
        }
    }

    getUnlinkedStatus(config_address: ConfigAddress): boolean {

        void this.version.value;
        
        return structuredClone(this.global_link_metas.has(config_address.str()) && this.global_link_metas.get(config_address.str())!.unlinked);
    }

    updateUnlinkedStatus(config_address: ConfigAddress, unlinked: boolean): void {

        ++this.version.value;
        unlinked = structuredClone(unlinked);

        if (this.global_link_metas.has(config_address.str())) {
            this.global_link_metas.get(config_address.str())!.unlinked = unlinked;
        }
    }

    getAllUnlinkedStatus(character_id: number): Record<string, boolean> {

        void this.version.value;
        
        let res: Record<string, boolean> = {};
        for (let [address, info] of this.global_link_metas) {
            if (ConfigAddress.prototype.from_str(address).character_id == character_id && info.unlinked) {
                res[ConfigAddress.prototype.from_str(address).str_without_character()] = info.unlinked;
            }
        }
        return structuredClone(res);
    }

    updateAllUnlinkedStatus(character_id: number, statuses: Record<string, boolean>): void {

        ++this.version.value;
        statuses = structuredClone(statuses);

        for (let [address, info] of this.global_link_metas) {
            if (ConfigAddress.prototype.from_str(address).character_id == character_id) {
                info.unlinked = statuses[ConfigAddress.prototype.from_str(address).str_without_character()] || false;
            }
        }
    }

    removeCharacter(character_id: number): void {

        ++this.version.value;

        for (let address of Array.from(this.values.keys())) {
            if (ConfigAddress.prototype.from_str(address).character_id == character_id) {
                this.values.delete(address);
                if (this.global_link_metas.has(address)) {
                    this.global_key_lists.get(this.global_link_metas.get(address)!.key)?.delete(ConfigAddress.prototype.from_str(address));
                }
                this.global_link_metas.delete(address);
            }
        }
    }
};