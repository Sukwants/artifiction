use crate::attribute::*;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::macros::{damage_enum, skill_map};
use crate::common::{ChangeAttribute, Element, MoonglareReaction, Moonsign, SkillType, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct LumineAnemoSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub a_dmg4: [f64; 15],
    pub a_dmg5: [f64; 15],
    pub z_dmg1: [f64; 15],
    pub z_dmg2: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmgc1: [f64; 15], // Initial Cutting DMG
    pub e_dmgc2: [f64; 15], // Max Cutting DMG
    pub e_dmgs1: [f64; 15], // Initial Storm DMG
    pub e_dmgs2: [f64; 15], // Max Storm DMG
    pub e_dmga_rate: f64,   // The ratio for Elemental Absorpted DMG to Anemo DMG

    pub q_dmg: [f64; 15],
    pub q_dmga: [f64; 15], // Additional Elemental DMG

    pub p2_dmg: f64,
}

pub const LUMINEANEMO_SKILL: LumineAnemoSkillType = LumineAnemoSkillType {
    // Normal Attack: Foreign Ironwind
    a_dmg1: [0.44462, 0.48081, 0.517, 0.5687, 0.60489, 0.64625, 0.70312, 0.76, 0.81686, 0.8789, 0.94094, 1.00298, 1.06502, 1.12706, 1.1891],
    a_dmg2: [0.4343, 0.46965, 0.505, 0.5555, 0.59085, 0.63125, 0.6868, 0.74235, 0.7979, 0.8585, 0.9191, 0.9797, 1.0403, 1.1009, 1.1615],
    a_dmg3: [0.52976, 0.57288, 0.616, 0.6776, 0.72072, 0.77, 0.83776, 0.90552, 0.97328, 1.0472, 1.12112, 1.19504, 1.26896, 1.34288, 1.4168],
    a_dmg4: [0.58308, 0.63054, 0.678, 0.7458, 0.79326, 0.8475, 0.92208, 0.99666, 1.07124, 1.1526, 1.23396, 1.31532, 1.39668, 1.47804, 1.5594],
    a_dmg5: [0.70778, 0.76539, 0.823, 0.9053, 0.96291, 1.02875, 1.11928, 1.20981, 1.30034, 1.3991, 1.49786, 1.59662, 1.69538, 1.79414, 1.8929],
    z_dmg1: [0.559, 0.6045, 0.65, 0.715, 0.7605, 0.8125, 0.884, 0.9555, 1.027, 1.105, 1.183, 1.261, 1.339, 1.417, 1.495],
    z_dmg2: [0.7224, 0.7812, 0.84, 0.924, 0.9828, 1.05, 1.1424, 1.2348, 1.3272, 1.428, 1.5288, 1.6296, 1.7304, 1.8312, 1.932],
    x_dmg1: [0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572, 1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982],
    x_dmg2: [1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646, 2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915],
    x_dmg3: [1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586, 3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041],

    // Elemental Skill: Palm Vortex
    e_dmgc1: [0.12, 0.129, 0.138, 0.15, 0.159, 0.168, 0.18, 0.192, 0.204, 0.216, 0.228, 0.24, 0.255, 0.27, 0.285],
    e_dmgc2: [0.168, 0.1806, 0.1932, 0.21, 0.2226, 0.2352, 0.252, 0.2688, 0.2856, 0.3024, 0.3192, 0.336, 0.357, 0.378, 0.399],
    e_dmgs1: [1.76, 1.892, 2.024, 2.2, 2.332, 2.464, 2.64, 2.816, 2.992, 3.168, 3.344, 3.52, 3.74, 3.96, 4.18],
    e_dmgs2: [1.92, 2.064, 2.208, 2.4, 2.544, 2.688, 2.88, 3.072, 3.264, 3.456, 3.648, 3.84, 4.08, 4.32, 4.56],
    e_dmga_rate: 0.25, // 天赋描述中无相关倍率，根据实测数据推断染色伤害为风元素伤害的 1/4

    // Elemental Burst: Gust Surge
    q_dmg: [0.808, 0.8686, 0.9292, 1.01, 1.0706, 1.1312, 1.212, 1.2928, 1.3736, 1.4544, 1.5352, 1.616, 1.717, 1.818, 1.919],
    q_dmga: [0.248, 0.2666, 0.2852, 0.31, 0.3286, 0.3472, 0.372, 0.3968, 0.4216, 0.4464, 0.4712, 0.496, 0.527, 0.558, 0.589],

    p2_dmg: 0.6,
};

pub const LUMINEANEMO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::LumineAnemo,
    internal_name: "LumineAnemo",
    element: Element::Anemo,
    hp: [912, 2342, 3024, 4529, 5031, 5766, 6411, 7164, 7648, 8401, 8885, 9638, 10122, 10875, 11627],
    atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212, 266],
    def: [57, 147, 190, 284, 315, 362, 402, 450, 480, 527, 558, 605, 635, 683, 730],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "异邦铁风",
        en: "Foreign Ironwind",
    ),
    skill_name2: locale!(
        zh_cn: "风涡剑",
        en: "Palm Vortex",
    ),
    skill_name3: locale!(
        zh_cn: "风息激荡",
        en: "Gust Surge",
    ),
    name_locale: locale!(
        zh_cn: "荧-风",
        en: "Lumine-Anemo",
    )
};

pub struct LumineAnemoEffect {
    pub has_c2: bool,
    pub has_c6: bool,
}

impl<A: Attribute> ChangeAttribute<A> for LumineAnemoEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_c2 {
            attribute.set_value_by(AttributeName::Recharge, "二命：革新的旋风", 0.16);
        }

        if self.has_c6 {
            attribute.set_value_by(AttributeName::ResMinusAnemo, "六命：革新的旋风", 0.2);
        }
    }
}

damage_enum!(
    LumineAnemoDamageEnum
    A1
    A2
    A3
    A4
    A5
    Z1
    Z2
    X1
    X2
    X3
    EC1
    EC2
    ES1
    ES2
    EC1A
    EC2A
    ES1A
    ES2A
    Q
    QA
    P2
);

impl LumineAnemoDamageEnum {
    pub fn get_element(&self, elemental_absorption: Element) -> Element {
        use LumineAnemoDamageEnum::*;
        match *self {
            EC1 | EC2 | ES1 | ES2 | Q | P2 => Element::Anemo,
            EC1A | EC2A | ES1A | ES2A | QA => elemental_absorption,
            _ => Element::Physical,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use LumineAnemoDamageEnum::*;
        match *self {
            A1 | A2 | A3 | A4 | A5 | P2 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            EC1 | EC2 | ES1 | ES2 | EC1A | EC2A | ES1A | ES2A => SkillType::ElementalSkill,
            Q | QA => SkillType::ElementalBurst,
        }
    }
}

pub struct LumineAnemo;

impl CharacterTrait for LumineAnemo {
    const STATIC_DATA: CharacterStaticData = LUMINEANEMO_STATIC_DATA;
    type SkillType = LumineAnemoSkillType;
    const SKILL: Self::SkillType = LUMINEANEMO_SKILL;
    type DamageEnumType = LumineAnemoDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LumineAnemoDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            A4 hit_n_dmg!(4)
            A5 hit_n_dmg!(5)
            Z1 charged_dmg!(1)
            Z2 charged_dmg!(2)
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
            P2 locale!(zh_cn: "裂空之风", en: "Slitting Wind")
        ),
        skill2: skill_map!(
            LumineAnemoDamageEnum
            EC1 locale!(zh_cn: "初始切割伤害", en: "Initial Cutting DMG")
            EC2 locale!(zh_cn: "最大切割伤害", en: "Max Cutting DMG")
            ES1 locale!(zh_cn: "初始爆风伤害", en: "Initial Storm DMG")
            ES2 locale!(zh_cn: "最大爆风伤害", en: "Max Storm DMG")
            EC1A locale!(zh_cn: "初始切割染色伤害", en: "Initial Cutting DMG (Absorbed)")
            EC2A locale!(zh_cn: "最大切割染色伤害", en: "Max Cutting DMG (Absorbed)")
            ES1A locale!(zh_cn: "初始爆风染色伤害", en: "Initial Storm DMG (Absorbed)")
            ES2A locale!(zh_cn: "最大爆风染色伤害", en: "Max Storm DMG (Absorbed)")
        ),
        skill3: skill_map!(
            LumineAnemoDamageEnum
            Q locale!(zh_cn: "龙卷风伤害", en: "Tornado DMG")
            QA locale!(zh_cn: "附加元素伤害", en: "Additional Elemental DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "elemental_absorption",
            title: locale!(zh_cn: "元素吸收类型", en: "Elemental Absorption Type"),
            config: ItemConfigType::Element4 { default: Element::Cryo }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: LumineAnemoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let elemental_absorption = match *config {
            CharacterSkillConfig::LumineAnemo { elemental_absorption } => elemental_absorption,
            _ => Element::Cryo,
        };

        use LumineAnemoDamageEnum::*;
        let mut builder = D::new();

        if context.character_common_data.constellation >= 6 && s.get_element(elemental_absorption) != Element::Anemo {
            builder.add_extra_res_minus("六命：革新的旋风", 0.2);
        }

        let ratio = match s {
            A1 => LUMINEANEMO_SKILL.a_dmg1[s1],
            A2 => LUMINEANEMO_SKILL.a_dmg2[s1],
            A3 => LUMINEANEMO_SKILL.a_dmg3[s1],
            A4 => LUMINEANEMO_SKILL.a_dmg4[s1],
            A5 => LUMINEANEMO_SKILL.a_dmg5[s1],
            Z1 => LUMINEANEMO_SKILL.z_dmg1[s1],
            Z2 => LUMINEANEMO_SKILL.z_dmg2[s1],
            X1 => LUMINEANEMO_SKILL.x_dmg1[s1],
            X2 => LUMINEANEMO_SKILL.x_dmg2[s1],
            X3 => LUMINEANEMO_SKILL.x_dmg3[s1],
            EC1 => LUMINEANEMO_SKILL.e_dmgc1[s2],
            EC2 => LUMINEANEMO_SKILL.e_dmgc2[s2],
            ES1 => LUMINEANEMO_SKILL.e_dmgs1[s2],
            ES2 => LUMINEANEMO_SKILL.e_dmgs2[s2],
            EC1A => LUMINEANEMO_SKILL.e_dmgc1[s2] * LUMINEANEMO_SKILL.e_dmga_rate,
            EC2A => LUMINEANEMO_SKILL.e_dmgc2[s2] * LUMINEANEMO_SKILL.e_dmga_rate,
            ES1A => LUMINEANEMO_SKILL.e_dmgs1[s2] * LUMINEANEMO_SKILL.e_dmga_rate,
            ES2A => LUMINEANEMO_SKILL.e_dmgs2[s2] * LUMINEANEMO_SKILL.e_dmga_rate,
            Q => LUMINEANEMO_SKILL.q_dmg[s3],
            QA => LUMINEANEMO_SKILL.q_dmga[s3],
            P2 => LUMINEANEMO_SKILL.p2_dmg,
            _ => 0.0
        };

        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(elemental_absorption),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(LumineAnemoEffect {
            has_c2: common_data.constellation >= 2,
            has_c6: common_data.constellation >= 6,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
