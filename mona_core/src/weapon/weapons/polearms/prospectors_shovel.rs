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

pub struct ProspectorsShovelEffect {
    pub moonsign: Moonsign,
}

impl<T: Attribute> WeaponEffect<T> for ProspectorsShovelEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::EnhanceElectroCharged, "Buff: 掘金之锹「当机立断」", refine * 0.12 + 0.36);

        let val = (refine * 0.03 + 0.09) * if self.moonsign.is_ascendant() { 2.0 } else { 1.0 };
        attribute.set_value_by(AttributeName::EnhanceLunarCharged, "Buff: 掘金之锹「当机立断」", val);
    }
}

impl ProspectorsShovelEffect {
    pub fn new(config: &WeaponConfig) -> ProspectorsShovelEffect {
        let moonsign = match *config {
            WeaponConfig::ProspectorsShovel { moonsign } => moonsign,
            _ => Moonsign::default()
        };

        ProspectorsShovelEffect {
            moonsign,
        }
    }
}

pub struct ProspectorsShovel;

impl WeaponTrait for ProspectorsShovel {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ProspectorsShovel,
        internal_name: "Polearm_ProspectorsShovel",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "感电反应造成的伤害提升 <span style=\"color: #409EFF;\">48%-60%-72%-84%-96%</span> ，月感电反应伤害提升 <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> 。月兆·满辉：月感电反应伤害额外提升 <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> 。",
            en: "Electro-Charged DMG is increased by <span style=\"color: #409EFF;\">48%-60%-72%-84%-96%</span> , and Lunar-Charged DMG is increased by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> . Moonsign: Ascendant Gleam: Lunar-Charged DMG is increased by an additional <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> ."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "掘金之锹",
            en: "Prospector's Shovel"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::None, ItemConfig::PRIORITY_WEAPON),
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(ProspectorsShovelEffect::new(config)))
    }
}
