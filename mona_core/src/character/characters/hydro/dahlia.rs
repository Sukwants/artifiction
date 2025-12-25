use crate::attribute::*;
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
use crate::target_functions::target_functions::DahliaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct DahliaSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg31: [f64; 15],
    pub a_dmg32: [f64; 15],
    pub a_dmg4: [f64; 15],
    pub z_dmg1: [f64; 15],
    pub z_dmg2: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg: [f64; 15],

    pub q_dmg: [f64; 15],
    pub q_shield: [f64; 15],
    pub q_shield_fixed: [f64; 15],
}

pub const DAHLIA_SKILL: DahliaSkillType = DahliaSkillType {
    a_dmg1: [0.43547, 0.470915, 0.50636, 0.556996, 0.592441, 0.63295, 0.68865, 0.744349, 0.800049, 0.860812, 0.921575, 0.982338, 1.043102, 1.103865, 1.164628],
    a_dmg2: [0.401001, 0.43364, 0.46628, 0.512908, 0.545548, 0.58285, 0.634141, 0.685432, 0.736722, 0.792676, 0.84863, 0.904583, 0.960537, 1.01649, 1.072444],
    a_dmg31: [0.237446, 0.256773, 0.2761, 0.30371, 0.323037, 0.345125, 0.375496, 0.405867, 0.436238, 0.46937, 0.502502, 0.535634, 0.568766, 0.601898, 0.63503],
    a_dmg32: [0.290164, 0.313782, 0.3374, 0.37114, 0.394758, 0.42175, 0.458864, 0.495978, 0.533092, 0.57358, 0.614068, 0.654556, 0.695044, 0.735532, 0.77602],
    a_dmg4: [0.656576, 0.710018, 0.76346, 0.839806, 0.893248, 0.954325, 1.038306, 1.122286, 1.206267, 1.297882, 1.389497, 1.481112, 1.572728, 1.664343, 1.755958],
    z_dmg1: [0.398765, 0.431222, 0.46368, 0.510048, 0.542506, 0.5796, 0.630605, 0.68161, 0.732614, 0.788256, 0.843898, 0.899539, 0.955181, 1.010822, 1.066464],
    z_dmg2: [0.550675, 0.595498, 0.64032, 0.704352, 0.749174, 0.8004, 0.870835, 0.94127, 1.011706, 1.088544, 1.165382, 1.242221, 1.319059, 1.395898, 1.472736],
    x_dmg1: [0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572, 1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982],
    x_dmg2: [1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646, 2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915],
    x_dmg3: [1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586, 3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041],
    e_dmg: [2.328, 2.5026, 2.6772, 2.91, 3.0846, 3.2592, 3.492, 3.7248, 3.9576, 4.1904, 4.4232, 4.656, 4.947, 5.238, 5.529],
    q_dmg: [4.064, 4.3688, 4.6736, 5.08, 5.3848, 5.6896, 6.096, 6.5024, 6.9088, 7.3152, 7.7216, 8.128, 8.636, 9.144, 9.652],
    q_shield: [0.0336, 0.03612, 0.03864, 0.042, 0.04452, 0.04704, 0.0504, 0.05376, 0.05712, 0.06048, 0.06384, 0.0672, 0.0714, 0.0756, 0.0798],
    q_shield_fixed: [3.235577, 3.5591797, 3.9097495, 4.2872858, 4.6917892, 5.123259, 5.581697, 6.067101, 6.5794714, 7.118809, 7.685114, 8.2783856, 8.898624, 9.5458295, 10.22],
};

pub const DAHLIA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Dahlia,
    internal_name: "Dahlia",
    element: Element::Hydro,
    hp: [1049, 2694, 3477, 5208, 5765, 6631, 7373, 8239, 8796, 9661, 10217, 11083, 11640, 12506, 13371],
    atk: [16,41,53,79,87,100,111,125,133,146,154,168,176,189, 237],
    def: [47,121,156,233,258,297,330,369,394,432,457,496,521,560, 598],
    sub_stat: CharacterSubStatFamily::HP240,
    weapon_type: WeaponType::Sword,
    star: 4,
    skill_name1: locale!(
        zh_cn: "西风剑术 · 祭仪",
        en: "Favonius Bladework - Ritual",
    ),
    skill_name2: locale!(
        zh_cn: "圣浸的礼典",
        en: "Immersive Ordinance",
    ),
    skill_name3: locale!(
        zh_cn: "纯耀的祷咏",
        en: "Radiant Psalter",
    ),
    name_locale: locale!(
        zh_cn: "塔利雅",
        en: "Dahlia",
    )
};

pub struct DahliaEffect {
    pub has_p2: bool,
}

impl<A: Attribute> ChangeAttribute<A> for DahliaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_p2 {
            attribute.set_value_by(AttributeName::ShieldStrength, "塔利雅二命「眷怜启应」", 0.25);
        }
    }
}

damage_enum!(
    DahliaDamageEnum
    A1
    A2
    A31
    A32
    A4
    Z1
    Z2
    X1
    X2
    X3
    E
    Q
    QShield
);

impl DahliaDamageEnum {
    pub fn get_element(&self) -> Element {
        use DahliaDamageEnum::*;
        match *self {
            E | Q | QShield => Element::Hydro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use DahliaDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 | A4 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E => SkillType::ElementalSkill,
            Q | QShield => SkillType::ElementalBurst
        }
    }
}

pub struct Dahlia;

impl CharacterTrait for Dahlia {
    const STATIC_DATA: CharacterStaticData = DAHLIA_STATIC_DATA;
    type SkillType = DahliaSkillType;
    const SKILL: Self::SkillType = DAHLIA_SKILL;
    type DamageEnumType = DahliaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            DahliaDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A31 hit_n_dmg!(3, 1)
            A32 hit_n_dmg!(3, 2)
            A4 hit_n_dmg!(4)
            Z1 charged_dmg!(1)
            Z2 charged_dmg!(2)
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            DahliaDamageEnum
            E locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            DahliaDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
            QShield locale!(zh_cn: "圣眷护盾吸收量", en: "Shield of Sacred Favor DMG Absorption")
        )
    };

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: DahliaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use DahliaDamageEnum::*;
        let mut builder = D::new();

        if s == QShield {
            let ratio = DAHLIA_SKILL.q_shield[s3];
            let fixed = DAHLIA_SKILL.q_shield_fixed[s3];

            builder.add_hp_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);

            builder.shield(&context.attribute, Element::Hydro)
        } else {
            let ratio = match s {
                A1 => DAHLIA_SKILL.a_dmg1[s1],
                A2 => DAHLIA_SKILL.a_dmg2[s1],
                A31 => DAHLIA_SKILL.a_dmg31[s1],
                A32 => DAHLIA_SKILL.a_dmg32[s1],
                Z1 => DAHLIA_SKILL.z_dmg1[s1],
                Z2 => DAHLIA_SKILL.z_dmg2[s1],
                X1 => DAHLIA_SKILL.x_dmg1[s1],
                X2 => DAHLIA_SKILL.x_dmg2[s1],
                X3 => DAHLIA_SKILL.x_dmg3[s1],
                E => DAHLIA_SKILL.e_dmg[s2],
                Q => DAHLIA_SKILL.q_dmg[s3],
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);

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
        Some(Box::new(DahliaEffect {
            has_p2: common_data.has_talent2
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        Box::new(crate::target_functions::target_functions::hydro::dahlia_default::DahliaDefaultTargetFunction)
    }
}
