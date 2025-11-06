use std::collections::HashMap;
use askama::Template;
use mona::character::{CharacterName, CharacterStaticData};
use mona::character::traits::{CharacterSkillMap, CharacterSkillMapItem};
use mona::common::global_config::GlobalConfigName;
use mona::common::item_config_type::ItemConfig;
use lazy_static::lazy_static;
use crate::gen_meta::gen_locale::get_index_mapping;
use crate::utils::config_to_json;
use crate::utils::icon_hashmap::ICON_HASHMAP;

struct GlobalConfigMeta {
    name: String,
    config: String,
}

struct SkillMapItem {
    index: usize,
    locale_index: usize,
}

#[derive(Template)]
#[template(path = "gc_meta_template.js")]
struct GlobalConfigMetaTemplate {
    global_configs: Vec<GlobalConfigMeta>,
}

pub fn gen_gc_meta_as_js_file() -> String {
    let mut data: Vec<GlobalConfigMeta> = Vec::new();
    let index_mapping = get_index_mapping();
    let icon_hashmap = &ICON_HASHMAP;

    for i in 0_usize..GlobalConfigName::LEN {
        let name_enum: GlobalConfigName = num::FromPrimitive::from_usize(i).unwrap();

        let name: String = name_enum.get_global_config_data().name.to_string();
        let config_data: String = config_to_json(name_enum.get_global_config_data());

        data.push(GlobalConfigMeta {
            name: name,
            config: config_data,
        })
    }

    let t = GlobalConfigMetaTemplate {
        global_configs: data,
    };

    t.render().unwrap()
}
