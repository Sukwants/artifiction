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

pub struct ReliquaryOfTruthEffect {
    pub effect1: bool,
    pub effect2: bool,
}

impl<A: Attribute> WeaponEffect<A> for ReliquaryOfTruthEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::CriticalBase, "真语秘匣被动", 0.06 + 0.02 * refine);

        let rate = if self.effect1 && self.effect2 { 1.5 } else { 1.0 };

        if self.effect1 {
            attribute.set_value_by(AttributeName::ElementalMastery, "真语秘匣「伪言之秘」", (60.0 + 20.0 * refine) * rate);
        }

        if self.effect2 {
            attribute.set_value_by(AttributeName::CriticalDamageBase, "真语秘匣「真识之月」", (0.18 + 0.06 * refine) * rate);
        }
    }
}

pub struct ReliquaryOfTruth;

impl WeaponTrait for ReliquaryOfTruth {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::ReliquaryOfTruth,
        internal_name: "Catalyst_ReliquaryOfTruth",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "暴击率提升 <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> 。施放元素战技时，装备者获得「伪言之秘」效果：元素精通提升 <span style=\"color: #409EFF;\">80-100-120-140-160</span> 点，持续12秒。装备者对敌人造成月绽放反应伤害时，装备者获得「真识之月」效果：暴击伤害提升 <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> ，持续4秒。「伪言之秘」效果与「真识之月」效果同时存在时，这些效果分别提升50%。",
            en: "CRIT Rate is increased by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> . When the equipping character unleashes an Elemental Skill, they gain the Secret of Lies effect: Elemental Mastery is increased by <span style=\"color: #409EFF;\">80-100-120-140-160</span> for 12s. When the equipping character deals Lunar-Bloom DMG to an opponent, they gain the Moon of Truth effect: CRIT DMG is increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> for 4s. When both the Secret of Lies and Moon of Truth effects are active at the same time, the results of both effects will be increased by 50%."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "真语秘匣",
            en: "Reliquary of Truth"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "effect1",
            title: locale!(
                zh_cn: "「伪言之秘」",
                en: "Secret of Lies",
            ),
            config: ItemConfigType::Bool {  default: false },
        },
        ItemConfig {
            name: "effect2",
            title: locale!(
                zh_cn: "「真识之月」",
                en: "Moon of Truth"
            ),
            config: ItemConfigType::Bool {  default: false },
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (effect1, effect2) = match *config {
            WeaponConfig::ReliquaryOfTruth { effect1, effect2 } => (effect1, effect2),
            _ => (false, false)
        };

        Some(Box::new(ReliquaryOfTruthEffect {
            effect1,
            effect2,
        }))
    }
}