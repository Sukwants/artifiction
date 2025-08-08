use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::macros::{damage_enum, skill_map};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
// use crate::target_functions::target_functions::IneffaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct IneffaSkillType {
    pub p1_dmg: f64,
}

pub const INEFFA_SKILL: IneffaSkillType = IneffaSkillType {
    p1_dmg: 0.65,
};

pub const INEFFA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Ineffa,
    internal_name: "Ineffa",
    element: Element::Electro,
    hp: [982, 2547, 3389, 5071, 5669, 6523, 7320, 8182, 8780, 9650, 10249, 11128, 11727, 12613],
    atk: [26,67,89,133,148,171,192,214,230,253,268,291,307,330],
    def: [64,167,222,333,372,428,480,537,576,633,673,730,770,828],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Catalyst,
    star: 5,
    skill_name1: locale!(
        zh_cn: "除尘旋刃",
        en: "Cyclonic Duster",
    ),
    skill_name2: locale!(
        zh_cn: "涤净模式 · 稳态载频",
        en: "Cleaning Mode: Carrier Frequency",
    ),
    skill_name3: locale!(
        zh_cn: "至高律令 · 全域扫灭",
        en: "Supreme Instruction: Cyclonic Exterminator",
    ),
    name_locale: locale!(
        zh_cn: "伊涅芙",
        en: "Ineffa",
    )
};

pub struct IneffaEffect {
}

impl<A: Attribute> ChangeAttribute<A> for IneffaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        unimplemented!()
    }
}

damage_enum!(
    IneffaDamageEnum
    Talent1
);

impl IneffaDamageEnum {
    pub fn get_element(&self) -> Element {
        use IneffaDamageEnum::*;
        match *self {
            Talent1 => Element::Electro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use IneffaDamageEnum::*;
        match *self {
            Talent1 => SkillType::NoneType,
        }
    }
}

pub struct Ineffa;

impl CharacterTrait for Ineffa {
    const STATIC_DATA: CharacterStaticData = INEFFA_STATIC_DATA;
    type SkillType = IneffaSkillType;
    const SKILL: Self::SkillType = INEFFA_SKILL;
    type DamageEnumType = IneffaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            IneffaDamageEnum
        ),
        skill2: skill_map!(
            IneffaDamageEnum
            Talent1 locale!(zh_cn: "薇尔琪塔额外放电攻击", en: "Birgitta Additional Discharge Attack")
        ),
        skill3: skill_map!(
            IneffaDamageEnum
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: IneffaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use IneffaDamageEnum::*;
        let mut builder = D::new();

        if s == Talent1 {
            let ratio = match s {
                Talent1 => INEFFA_SKILL.p1_dmg,
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);

            builder.moonglare(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        } else {
            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        None
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
