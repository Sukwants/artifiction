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

pub struct DawningFrostEffect {
    pub rate1: f64,
    pub rate2: f64,
}

impl<T: Attribute> WeaponEffect<T> for DawningFrostEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::ElementalMastery, "霜辰「深宵的胎梦」", 
            (54.0 + 18.0 * refine) * self.rate1 + (36.0 + 12.0 * refine) * self.rate2);
    }
}

impl DawningFrostEffect {
    pub fn new(config: &WeaponConfig) -> DawningFrostEffect {
        match *config {
            WeaponConfig::DawningFrost { rate1, rate2 } => DawningFrostEffect {
                rate1,
                rate2
            },
            _ => DawningFrostEffect {
                rate1: 0.0,
                rate2: 0.0
            }
        }
    }
}

pub struct DawningFrost;

impl WeaponTrait for DawningFrost {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::DawningFrost,
        internal_name: "Catalyst_DawningFrost",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage120),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "重击命中敌人后的10秒内，元素精通提升 <span style=\"color: #409EFF;\">72-90-108-126-144</span> 点；元素战技命中敌人后的10秒内，元素精通提升 <span style=\"color: #409EFF;\">48-60-72-84-96</span> 点。",
            en: "For 10s after a Charged Attack hits an opponent, Elemental Mastery is increased by <span style=\"color: #409EFF;\">72-90-108-126-144</span>. For 10s after an Elemental Skill hits an opponent, Elemental Mastery is increased by <span style=\"color: #409EFF;\">48-60-72-84-96</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "霜辰",
            en: "Dawning Frost"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "效果1比例",
                en: "Effect1 Ratio",
            ),
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "效果2比例",
                en: "Effect2 Ratio"
            ),
            config: ItemConfig::RATE01_TYPE
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (rate1, rate2) = match *config {
            WeaponConfig::DawningFrost { rate1, rate2 } => (rate1, rate2),
            _ => (0.0, 0.0)
        };
        Some(Box::new(DawningFrostEffect {
            rate1, rate2
        }))
    }
}
