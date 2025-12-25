use crate::attribute::*;
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

pub struct SerenitysCallEffect {
    pub reaction: bool,
    pub moonsign: Moonsign,
}

impl<T: Attribute> WeaponEffect<T> for SerenitysCallEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        if self.reaction {
            let val = (refine * 0.04 + 0.12) * if self.moonsign.is_ascendant() { 2.0 } else { 1.0 };

            attribute.add_hp_percentage("Buff: 谧音吹哨「沉声止语」", val);
        }
    }
}

impl SerenitysCallEffect {
    pub fn new(config: &WeaponConfig) -> SerenitysCallEffect {
        let (reaction, moonsign) = match *config {
            WeaponConfig::SerenitysCall { reaction, moonsign } => (reaction, moonsign),
            _ => (false, Moonsign::default())
        };

        SerenitysCallEffect {
            reaction,
            moonsign,
        }
    }
}

pub struct SerenitysCall;

impl WeaponTrait for SerenitysCall {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SerenitysCall,
        internal_name: "Sword_SerenitysCall",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge133),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "触发元素反应后的12秒内，生命值上限提高 <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> 。月兆 · 满辉：该效果中的生命值上限额外提高 <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> 。装备者处于队伍后台时，依然能触发上述效果。",
            en: "Upon causing an Elemental Reaction, increases Max HP by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> for 12s. Moonsign: Ascendant Gleam: Max HP from this effect is further increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> . This effect can be triggered even if the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "谧音吹哨",
            en: "Serenity's Call"
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
        Some(Box::new(SerenitysCallEffect::new(config)))
    }
}
