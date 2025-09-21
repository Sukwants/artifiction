use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct BloodsoakedRuinsEffect {
    rate: f64,
    requiem_of_ruin: bool,
}

impl BloodsoakedRuinsEffect {
    pub fn new(config: &WeaponConfig) -> BloodsoakedRuinsEffect {
        match *config {
            WeaponConfig::BloodsoakedRuins { rate, requiem_of_ruin } => BloodsoakedRuinsEffect {
                rate,
                requiem_of_ruin
            },
            _ => BloodsoakedRuinsEffect {
                rate: 0.0,
                requiem_of_ruin: false
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for BloodsoakedRuinsEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.set_value_by(AttributeName::EnhanceLunarCharged, "血染荒城被动", (refine * 0.12 + 0.24) * self.rate);
        if self.requiem_of_ruin {
            attribute.set_value_by(AttributeName::CriticalDamageBase, "血染荒城：「荒落的挽歌」", refine * 0.07 + 0.21);
        }
    }
}

pub struct BloodsoakedRuins;

impl WeaponTrait for BloodsoakedRuins {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::BloodsoakedRuins,
        internal_name: "",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate48),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素爆发后的3.5秒内，装备者对敌人造成的月感电反应伤害提高<span style=\"color: #409EFF;\">36%-48%-60%-72%-84%</span>。此外，装备者触发月感电反应后，将获得「荒落的挽歌」：暴击伤害提高<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>，持续6秒；并为装备者恢复<span style=\"color: #409EFF;\">12-13-14-15-16</span>点元素能量，每14秒至多通过这种方式恢复一次元素能量。",
            en: "For 3.5s after using an Elemental Burst, the equipping character's Lunar-Charged DMG dealt to opponents is increased by <span style=\"color: #409EFF;\">36%-48%-60%-72%-84%</span>. Additionally, after triggering a Lunar-Charged reaction, the equipping character will gain Requiem of Ruin: CRIT DMG is increased by <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span> for 6s. They will also regain <span style=\"color: #409EFF;\">12-13-14-15-16</span> Elemental Energy. Elemental Energy can be restored this way once every 14s.",
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "血染荒城",
            en: "Bloodsoaked Ruins"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01,
        ItemConfig {
            name: "requiem_of_ruin",
            title: locale!(
                zh_cn: "「荒落的挽歌」",
                en: "Requiem of Ruin",
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(BloodsoakedRuinsEffect::new(config)))
    }
}
