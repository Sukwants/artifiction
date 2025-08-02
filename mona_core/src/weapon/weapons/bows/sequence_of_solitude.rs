use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct SequenceOfSolitude;

impl WeaponTrait for SequenceOfSolitude {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SequenceOfSolitude,
        internal_name: "Bow_SequenceOfSolitude",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::HP90),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "攻击命中敌人时，在目标位置基于生命值上限的 <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>，造成范围伤害。该效果每15秒至多触发一次。",
            en: "When an attack hits an opponent, deal AoE DMG equal to <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> of Max HP at the target's location. This effect can be triggered once every 15s."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "冷寂迸音",
            en: "Sequence of Solitude"
        )
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
