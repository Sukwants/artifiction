use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::macros::{damage_enum, skill_map};
use crate::common::{ChangeAttribute, Element, MoonglareReaction, SkillType, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
// use crate::target_functions::target_functions::IneffaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct IneffaSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg31: [f64; 15],
    pub a_dmg32: [f64; 15],
    pub a_dmg4: [f64; 15],
    pub z_dmg: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg: [f64; 15],
    pub e_birgitta_dmg: [f64; 15],
    pub e_shield_base: [f64; 15],
    pub e_shield_additional: [f64; 15],

    pub q_dmg: [f64; 15],

    pub p1_dmg: f64,
    pub c2_dmg: f64,
    pub c6_dmg: f64,
}

pub const INEFFA_SKILL: IneffaSkillType = IneffaSkillType {
    // Normal Attack: Cyclonic Duster
    a_dmg1: [0.348352, 0.376706, 0.40506, 0.445566, 0.47392, 0.506325, 0.550882, 0.595438, 0.64, 0.688602, 0.737209, 0.785816, 0.834424, 0.883031, 0.931638],
    a_dmg2: [0.342211, 0.370066, 0.39792, 0.437712, 0.465566, 0.4974, 0.541171, 0.584942, 0.628714, 0.676464, 0.724214, 0.771965, 0.819715, 0.867466, 0.915216],
    a_dmg31: [0.227556, 0.246078, 0.2646, 0.29106, 0.309582, 0.33075, 0.359856, 0.388962, 0.418068, 0.44982, 0.481572, 0.513324, 0.545076, 0.576828, 0.60858],
    a_dmg32: [0.227556, 0.246078, 0.2646, 0.29106, 0.309582, 0.33075, 0.359856, 0.388962, 0.418068, 0.44982, 0.481572, 0.513324, 0.545076, 0.576828, 0.60858],
    a_dmg4: [0.560677, 0.606314, 0.65195, 0.717145, 0.762782, 0.814938, 0.886652, 0.958367, 1.030081, 1.108315, 1.186549, 1.264783, 1.343017, 1.421251, 1.499485],
    z_dmg: [0.94944, 1.02672, 1.104, 1.2144, 1.29168, 1.38, 1.50144, 1.62288, 1.74432, 1.8768, 2.00928, 2.14176, 2.27424, 2.40672, 2.5392],
    x_dmg1: [0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572, 1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982],
    x_dmg2: [1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646, 2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915],
    x_dmg3: [1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586, 3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041],

    // Elemental Skill: Cleaning Mode: Carrier Frequency
    e_dmg: [0.864, 0.9288, 0.9936, 1.08, 1.1448, 1.2096, 1.296, 1.3824, 1.4688, 1.5552, 1.6416, 1.728, 1.836, 1.944, 2.052],
    e_birgitta_dmg: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    e_shield_base: [1386.6759, 1525.3628, 1675.6068, 1837.4082, 2010.7667, 2195.6826, 2392.1558, 2600.186, 2819.7734, 3050.9182, 3293.6204, 3547.8796, 3813.696, 4091.0698, 4380.0],
    e_shield_additional: [2.21184, 2.377728, 2.543616, 2.7648, 2.930688, 3.096576, 3.31776, 3.538944, 3.760128, 3.981312, 4.202496, 4.42368, 4.70016, 4.97664, 5.25312],

    // Elemental Burst: Supreme Instruction: Cyclonic Exterminator
    q_dmg: [6.768, 7.2756, 7.7832, 8.46, 8.9676, 9.4752, 10.152, 10.8288, 11.5056, 12.1824, 12.8592, 13.536, 14.382, 15.228, 16.074],

    p1_dmg: 0.65,
    c2_dmg: 3.0,
    c6_dmg: 1.35,
};

pub const INEFFA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Ineffa,
    internal_name: "Ineffa",
    element: Element::Electro,
    hp: [982, 2547, 3389, 5071, 5669, 6523, 7320, 8182, 8780, 9650, 10249, 11128, 11727, 12613],
    atk: [26, 67, 89, 133, 148, 171, 192, 214, 230, 253, 268, 291, 307, 330],
    def: [64, 167, 222, 333, 372, 428, 480, 537, 576, 633, 673, 730, 770, 828],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Polearm,
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
    pub has_p2: bool,
    pub has_c1: bool,
}

impl<A: Attribute> ChangeAttribute<A> for IneffaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_p2 {
            attribute.add_edge1(
                AttributeName::ATK,
                AttributeName::ElementalMastery,
                Box::new(move |atk, _| {
                    atk * 0.06
                }),
                Box::new(move |atk, _, grad| (0.0, 0.0)),
                "伊涅芙天赋：全相重构协议"
            );
        }

        attribute.add_edge1(
            AttributeName::ATK,
            AttributeName::IncreaseLunarCharged,
            Box::new(move |atk, _| {
                atk * 0.00007
            }),
            Box::new(move |atk, _, grad| (0.0, 0.0)),
            "伊涅芙天赋：月兆祝赐·象拟中继"
        );

        if self.has_c1 {
            attribute.add_edge1(
                AttributeName::ATK,
                AttributeName::EnhanceLunarCharged,
                Box::new(move |atk, _| {
                    (atk * 0.00025).min(0.5)
                }),
                Box::new(move |atk, _, grad| (0.0, 0.0)),
                "伊涅芙命座：循环整流引擎"
            );
        }
    }
}

damage_enum!(
    IneffaDamageEnum
    A1
    A2
    A31
    A32
    A4
    Z
    X1
    X2
    X3
    E
    EContinued
    EShield
    Q
    P1
    C2
    C6
    LunarCharged
);

impl IneffaDamageEnum {
    pub fn get_element(&self) -> Element {
        use IneffaDamageEnum::*;
        match *self {
            E | EContinued | EShield | Q | P1 | C2 | C6 | LunarCharged => Element::Electro,
            _ => Element::Physical,
        }
    }

    pub fn get_lunar_type(&self) -> MoonglareReaction {
        use IneffaDamageEnum::*;
        match *self {
            LunarCharged => MoonglareReaction::LunarChargedReaction,
            P1 | C2 | C6 => MoonglareReaction::LunarCharged,
            _ => MoonglareReaction::None,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use IneffaDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 | A4 => SkillType::NormalAttack,
            Z => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E | EContinued | EShield => SkillType::ElementalSkill,
            Q => SkillType::ElementalBurst,
            P1 | C2 | C6 | LunarCharged => SkillType::Moonglare,
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
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A31 hit_n_dmg!(3, 1)
            A32 hit_n_dmg!(3, 2)
            A4 hit_n_dmg!(3)
            Z charged_dmg!()
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            IneffaDamageEnum
            E locale!(zh_cn: "技能伤害", en: "Skill DMG")
            EContinued locale!(zh_cn: "薇尔琪塔放电伤害", en: "Birgitta Discharge DMG")
            EShield locale!(zh_cn: "护盾吸收量", en: "Shield DMG Absorption")
            P1 locale!(zh_cn: "薇尔琪塔额外放电伤害", en: "Birgitta Additional Discharge DMG")
            C2 locale!(zh_cn: "二命额外伤害", en: "C2 extra DMG")
            C6 locale!(zh_cn: "六命额外伤害", en: "C6 extra DMG")
            LunarCharged locale!(zh_cn: "月感电伤害", en: "Lunar-Charged DMG")
        ),
        skill3: skill_map!(
            IneffaDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
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

        if s == EShield {
            builder.add_atk_ratio("护盾基础吸收量", INEFFA_SKILL.e_shield_additional[s2]);
            builder.add_extra_damage("护盾额外吸收量", INEFFA_SKILL.e_shield_base[s2]);
            builder.shield(
                &context.attribute,
                s.get_element(),
            )
        } else if s == P1 || s == C2 || s == C6 || s == LunarCharged {
            let ratio = match s {
                P1 => INEFFA_SKILL.p1_dmg,
                C2 => INEFFA_SKILL.c2_dmg,
                C6 => INEFFA_SKILL.c6_dmg,
                LunarCharged => 0.0,
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);

            builder.moonglare(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                s.get_lunar_type(),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        } else {
            let ratio = match s {
                A1 => INEFFA_SKILL.a_dmg1[s1],
                A2 => INEFFA_SKILL.a_dmg2[s1],
                A31 => INEFFA_SKILL.a_dmg31[s1],
                A32 => INEFFA_SKILL.a_dmg32[s1],
                A4 => INEFFA_SKILL.a_dmg4[s1],
                Z => INEFFA_SKILL.z_dmg[s1],
                X1 => INEFFA_SKILL.x_dmg1[s1],
                X2 => INEFFA_SKILL.x_dmg2[s1],
                X3 => INEFFA_SKILL.x_dmg3[s1],
                E => INEFFA_SKILL.e_dmg[s2],
                EContinued => INEFFA_SKILL.e_birgitta_dmg[s2],
                Q => INEFFA_SKILL.q_dmg[s3],
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
        Some(Box::new(IneffaEffect {
            has_p2: common_data.has_talent2,
            has_c1: common_data.constellation >= 1,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
