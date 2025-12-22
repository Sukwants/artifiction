use crate::attribute::*;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct SymphonistOfScentsEffect {
    offfield_rate: f64,
    healing_rate: f64
}

impl SymphonistOfScentsEffect {
    pub fn new(config: &WeaponConfig) -> SymphonistOfScentsEffect {
        match *config {
            WeaponConfig::SymphonistOfScents { offfield_rate, healing_rate } => SymphonistOfScentsEffect {
                offfield_rate,
                healing_rate
            },
            _ => SymphonistOfScentsEffect {
                offfield_rate: 0.0,
                healing_rate: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for SymphonistOfScentsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let atk_bonus = refine * 0.03 + 0.09;
        let atk_bonus_offfield = refine * 0.03 + 0.09;
        let atk_bonus_healing = refine * 0.08 + 0.24;

        attribute.add_atk_percentage("香韵奏者被动等效", atk_bonus + atk_bonus_offfield * self.offfield_rate + atk_bonus_healing * self.healing_rate);
    }
}

pub struct SymphonistOfScents;

impl WeaponTrait for SymphonistOfScents {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SymphonistOfScents,
        internal_name: "Polearm_SymphonistOfScents",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "攻击力提升 <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> ；当装备此武器的角色处于队伍后台时，攻击力额外提升 <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> 。进行治疗后，装备者与受到治疗的角色会获得「甘美回奏」的效果，攻击力提升 <span style=\"color: #409EFF;\">32%-40%-48%-56%-64%</span> ，持续3秒。装备者处于队伍后台时，依然能触发上述效果。",
            en: "ATK is increased by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> . When the equipping character is off-field, ATK is increased by an additional <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> . After initiating healing, the equipping character and the character(s) they have healed will obtain the \"Sweet Echoes\" effect, increasing their ATK by <span style=\"color: #409EFF;\">32%-40%-48%-56%-64%</span> for 3s. This effect can be triggered even if the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "香韵奏者",
            en: "Symphonist of Scents"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "offfield_rate",
            title: locale!(
                zh_cn: "后台时间比例",
                en: "Off-Field Time Ratio"
            ),
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "healing_rate",
            title: locale!(
                zh_cn: "治疗时间比例",
                en: "Healing Time Ratio"
            ),
            config: ItemConfig::RATE01_TYPE
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SymphonistOfScentsEffect::new(config)))
    }
}
