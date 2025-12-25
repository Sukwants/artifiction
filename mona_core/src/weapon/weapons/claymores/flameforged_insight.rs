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

pub struct FlameforgedInsightEffect {
    pub reaction: bool,
}

impl<T: Attribute> WeaponEffect<T> for FlameforgedInsightEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        if self.reaction {
            attribute.set_value_by(AttributeName::ElementalMastery, "Buff: 拾慧铸熔「盛放的思绪」", refine * 15.0 + 45.0);
        }
    }
}

impl FlameforgedInsightEffect {
    pub fn new(config: &WeaponConfig) -> FlameforgedInsightEffect {
        let reaction = match *config {
            WeaponConfig::FlameforgedInsight { reaction } => reaction,
            _ => false
        };

        FlameforgedInsightEffect {
            reaction,
        }
    }
}

pub struct FlameforgedInsight;

impl WeaponTrait for FlameforgedInsight {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FlameforgedInsight,
        internal_name: "Claymore_FlameforgedInsight",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM36),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "触发感电、月感电、绽放或月绽放反应时，恢复 <span style=\"color: #409EFF;\">12-15-18-21-24</span> 点元素能量，并在接下来的15秒内使元素精通提升 <span style=\"color: #409EFF;\">60-75-90-105-120</span> 点。该效果每15秒至多触发一次。装备者处于队伍后台时，依然能触发上述效果。",
            en: "When Electro-Charged, Lunar-Charged, Bloom, or Lunar-Bloom is triggered, restore <span style=\"color: #409EFF;\">12-15-18-21-24</span> Elemental Energy and increase Elemental Mastery by <span style=\"color: #409EFF;\">60-75-90-105-120</span> for 15 seconds. This effect can be triggered once every 15s and can be triggered even when the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "拾慧铸熔",
            en: "Flame-Forged Insight"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "reaction",
            title: locale!(
                zh_cn: "触发感电、月感电、绽放或月绽放反应",
                en: "Trigger Electro-Charged, LunarCharged, Bloom, or Lunar Bloom Reaction"
            ),
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(FlameforgedInsightEffect::new(config)))
    }
}
