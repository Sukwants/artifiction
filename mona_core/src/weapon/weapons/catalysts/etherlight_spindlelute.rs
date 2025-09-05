use crate::attribute::{AttributeName, Attribute, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::{Moonsign, WeaponType};
use super::super::super::weapon_effect::WeaponEffect;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::WeaponName;
use super::super::super::weapon_config::WeaponConfig;

pub struct EtherlightSpindleluteEffect {
}

impl<T: Attribute> WeaponEffect<T> for EtherlightSpindleluteEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::ElementalMastery, "Buff: 天光的纺琴「最后的歌者」", refine * 25.0 + 75.0);
    }
}

impl EtherlightSpindleluteEffect {
    pub fn new(config: &WeaponConfig) -> EtherlightSpindleluteEffect {
        EtherlightSpindleluteEffect {}
    }
}

pub struct EtherlightSpindlelute;

impl WeaponTrait for EtherlightSpindlelute {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::EtherlightSpindlelute,
        internal_name: "Catalyst_EtherlightSpindlelute",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "施放元素战技后的20秒内，装备者的元素精通提升 <span style=\"color: #409EFF;\">100-125-150-175-200</span> 点。",
            en: "For 20s after using an Elemental Skill, the equipping character's Elemental Mastery is increased by <span style=\"color: #409EFF;\">100-125-150-175-200</span> ."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "天光的纺琴",
            en: "Etherlight Spindlelute"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(EtherlightSpindleluteEffect::new(config)))
    }
}
