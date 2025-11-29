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

pub struct MasterKeyEffect {
    pub reaction: bool,
    pub moonsign: Moonsign,
}

impl<T: Attribute> WeaponEffect<T> for MasterKeyEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        if self.reaction {
            let val = (refine * 15.0 + 45.0) * if self.moonsign.is_ascendant() { 2.0 } else { 1.0 };

            attribute.set_value_by(AttributeName::ElementalMastery, "Buff: 万能钥匙「迎刃而解」", val);
        }
    }
}

impl MasterKeyEffect {
    pub fn new(config: &WeaponConfig) -> MasterKeyEffect {
        let (reaction, moonsign) = match *config {
            WeaponConfig::MasterKey { reaction, moonsign } => (reaction, moonsign),
            _ => (false, Moonsign::default())
        };

        MasterKeyEffect {
            reaction,
            moonsign,
        }
    }
}

pub struct MasterKey;

impl WeaponTrait for MasterKey {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MasterKey,
        internal_name: "Claymore_MasterKey",
        weapon_type: WeaponType::Claymore,
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
            zh_cn: "万能钥匙",
            en: "Master Key"
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
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::None, ItemConfig::PRIORITY_WEAPON),
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MasterKeyEffect::new(config)))
    }
}
