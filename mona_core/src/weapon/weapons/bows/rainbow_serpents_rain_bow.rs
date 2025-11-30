use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct RainbowSerpentsRainBowEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for RainbowSerpentsRainBowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.add_atk_percentage("虹蛇的雨弦「圣座外的星语」", (0.21 + 0.07 * refine) * self.rate);
    }
}

pub struct RainbowSerpentsRainBow;

impl WeaponTrait for RainbowSerpentsRainBow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::RainbowSerpentsRainBow,
        internal_name: "Bow_RainbowSerpentsRainBow",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge100),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "装备者处于队伍后台时，装备者的攻击命中敌人后的8秒内，攻击力提升 <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span> 。",
            en: "ATK is increased by <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span> for 8s after the equipping character's attacks hit an opponent while the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "虹蛇的雨弦",
            en: "Rainbow Serpent's Rain Bow"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01,
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::RainbowSerpentsRainBow { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(RainbowSerpentsRainBowEffect {
            rate
        }))
    }
}