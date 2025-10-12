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

pub struct SnareHookEffect {
    pub reaction: bool,
    pub moonsign: Moonsign,
}

impl<T: Attribute> WeaponEffect<T> for SnareHookEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        if self.reaction {
            let val = (refine * 15.0 + 45.0) * if self.moonsign.is_ascendant() { 2.0 } else { 1.0 };

            attribute.set_value_by(AttributeName::ElementalMastery, "Buff: 罗网勾针「矫捷无影」", val);
        }
    }
}

impl SnareHookEffect {
    pub fn new(config: &WeaponConfig) -> SnareHookEffect {
        let (reaction, moonsign) = match *config {
            WeaponConfig::SnareHook { reaction, moonsign } => (reaction, moonsign),
            _ => (false, Moonsign::default())
        };

        SnareHookEffect {
            reaction,
            moonsign,
        }
    }
}

pub struct SnareHook;

impl WeaponTrait for SnareHook {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SnareHook,
        internal_name: "Bow_SnareHook",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge133),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "触发元素反应后的12秒内，元素精通提升 <span style=\"color: #409EFF;\">60-75-90-105-120</span> 。月兆·满辉：该效果中的元素精通额外提升 <span style=\"color: #409EFF;\">60-75-90-105-120</span> 。装备者处于队伍后台时，依然能触发上述效果。",
            en: "Upon causing an Elemental Reaction, increases Elemental Mastery by <span style=\"color: #409EFF;\">60-75-90-105-120</span> for 12s. Moonsign: Ascendant Gleam: Elemental Mastery from this effect is further increased by <span style=\"color: #409EFF;\">60-75-90-105-120</span> . This effect can be triggered even if the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "罗网勾针",
            en: "Snare Hook"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "reaction",
            title: locale!(
                zh_cn: "触发元素反应",
                en: "Trigger Elemental Reaction"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig::MOONSIGN3
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SnareHookEffect::new(config)))
    }
}
