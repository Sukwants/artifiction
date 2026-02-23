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

pub struct GestOfTheMightyWolfEffect {
    pub stack: f64,
    pub hexerei_secret_rite: bool,
}

impl<T: Attribute> WeaponEffect<T> for GestOfTheMightyWolfEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        attribute.set_value_by_t(
            AttributeType::Invisible(InvisibleAttributeType::new_any(AttributeVariableType::Bonus)),
            "狼的武功歌被动",
            (0.055 + 0.02 * refine) * self.stack
        );

        if self.hexerei_secret_rite {
            attribute.set_value_by(
                AttributeName::CriticalDamageBase,
                "狼的武功歌被动",
                (0.055 + 0.02 * refine) * self.stack
            );
        }
    }
}

impl GestOfTheMightyWolfEffect {
    pub fn new(config: &WeaponConfig) -> GestOfTheMightyWolfEffect {
        let (stack, hexerei_secret_rite) = match *config {
            WeaponConfig::GestOfTheMightyWolf { stack, hexerei_secret_rite } => (stack, hexerei_secret_rite),
            _ => (0.0, false)
        };

        GestOfTheMightyWolfEffect {
            stack,
            hexerei_secret_rite,
        }
    }
}

pub struct GestOfTheMightyWolf;

impl WeaponTrait for GestOfTheMightyWolf {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::GestOfTheMightyWolf,
        internal_name: "Claymore_GestOfTheMightyWolf",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate72),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "攻击速度提升10%。普通攻击命中敌人/施放元素战技/开始重击时，装备者将分别获得1/2/2层「四风诗系」：造成的伤害提升 <span style=\"color: #409EFF;\">7.5%-9.5%-11.5%-13.5-15.5%</span>，该效果持续4秒，至多叠加4次，每0.01秒至多触发一次。此外，队伍拥有「魔导·秘仪」效果时，每层「四风诗系」还会使装备者的暴击伤害提高 <span style=\"color: #409EFF;\">7.5%-9.5%-11.5%-13.5-15.5%</span>。",
            en: "Increase ATK SPD by 10%. Every time the equipping character’s Normal Attack(s) hit opponent(s)/casts their Elemental Skill/performs Charged Attack(s), gain 1/2/2 stacks of Four Winds’ Hymn respectively: DMG dealt is increased by <span style=\"color: #409EFF;\">7.5%-9.5%-11.5%-13.5-15.5%</span> for 4s. Max 4 stacks. This effect can be triggered once every 0.01s.Additionally, when the party has the “Hexerei: Secret Rite” effect, each stack of Four Winds’ Hymn will increase the CRIT DMG of the equipping character by <span style=\"color: #409EFF;\">7.5%-9.5%-11.5%-13.5-15.5%</span>."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "狼的武功歌",
            en: "Gest of the Mighty Wolf"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(zh_cn: "被动等效层数", en: "Avg Effect Stack"),
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 4.0 }
        },
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_WEAPON),
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(GestOfTheMightyWolfEffect::new(config)))
    }
}
