use std::str::FromStr;

use mona_derive::{GlobalConfigData, EnumLen};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum::*;
use strum_macros::{Display, EnumString, EnumIter};

use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_static_data::CharacterStaticData;
use crate::character::CharacterConfig;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterTrait};
use crate::common::ChangeAttribute;
use crate::common::{Element, Moonsign};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::i18n::{I18nLocale, locale};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[derive(Display, FromPrimitive, EnumString, GlobalConfigData, EnumLen, EnumIter)]
pub enum GlobalConfigName {
    Moonsign,
}

impl GlobalConfigName {
    pub const GLOBAL_CONFIG_MOONSIGN: ItemConfig = ItemConfig {
        name: "moonsign",
        title: locale!(
            zh_cn: "月兆",
            en: "Moonsign",
        ),
        config: ItemConfigType::Moonsign3 { default: Moonsign::None },
    };

    pub const fn get_config(&self) -> &'static ItemConfig {
        match *self {
            GlobalConfigName::Moonsign => &Self::GLOBAL_CONFIG_MOONSIGN,
        }
    }
}
