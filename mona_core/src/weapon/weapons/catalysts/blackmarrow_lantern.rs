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

pub struct BlackmarrowLanternEffect {
    pub moonsign: Moonsign,
}

impl<T: Attribute> WeaponEffect<T> for BlackmarrowLanternEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::EnhanceBloom, "Buff: 乌髓孑灯「结契的凭证」", refine * 0.12 + 0.36);

        let val = (refine * 0.03 + 0.09) * if self.moonsign.is_ascendant() { 2.0 } else { 1.0 };
        attribute.set_value_by(AttributeName::EnhanceLunarBloom, "Buff: 乌髓孑灯「结契的凭证」", val);
    }
}

impl BlackmarrowLanternEffect {
    pub fn new(config: &WeaponConfig) -> BlackmarrowLanternEffect {
        let moonsign = match *config {
            WeaponConfig::BlackmarrowLantern { moonsign } => moonsign,
            _ => Moonsign::default()
        };

        BlackmarrowLanternEffect {
            moonsign,
        }
    }
}

pub struct BlackmarrowLantern;

impl WeaponTrait for BlackmarrowLantern {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BlackmarrowLantern,
        internal_name: "Catalyst_BlackmarrowLantern",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM48),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "绽放反应造成的伤害提升 <span style=\"color: #409EFF;\">48%-60%-72%-84%-96%</span> ，月绽放反应伤害提升 <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> 。月兆·满辉：月绽放反应伤害额外提升 <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> 。",
            en: "Bloom DMG is increased by <span style=\"color: #409EFF;\">48%-60%-72%-84%-96%</span> , and Lunar-Bloom DMG is increased by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> . Moonsign: Ascendant Gleam: Lunar-Bloom DMG is increased by an additional <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> ."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "乌髓孑灯",
            en: "Blackmarrow Lantern"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::None, ItemConfig::PRIORITY_WEAPON),
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(BlackmarrowLanternEffect::new(config)))
    }
}
