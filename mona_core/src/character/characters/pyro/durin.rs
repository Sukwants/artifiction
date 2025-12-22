use num_traits::FromPrimitive;
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

pub struct DurinSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg31: [f64; 15],
    pub a_dmg32: [f64; 15],
    pub a_dmg4: [f64; 15],
    pub z_dmg: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmgp: [f64; 15], // Transmutation: Confirmation of Purity DMG
    pub e_dmgd1: [f64; 15], // Transmutation: Denial of Darkness DMG
    pub e_dmgd2: [f64; 15],
    pub e_dmgd3: [f64; 15],

    pub q_dmgp1: [f64; 15], // Principle of Purity
    pub q_dmgp2: [f64; 15],
    pub q_dmgp3: [f64; 15],
    pub q_dmgp: [f64; 15],
    pub q_dmgd1: [f64; 15], // Principle of Darkness
    pub q_dmgd2: [f64; 15],
    pub q_dmgd3: [f64; 15],
    pub q_dmgd: [f64; 15],
}

pub const DURIN_SKILL: DurinSkillType = DurinSkillType {
    // Normal Attack: Radiant Wingslash
    a_dmg1: [0.456505, 0.493663, 0.53082, 0.583902, 0.621059, 0.663525, 0.721915, 0.780305, 0.838696, 0.902394, 0.966092, 1.029791, 1.093489, 1.157188, 1.220886],
    a_dmg2: [0.410048, 0.443424, 0.4768, 0.52448, 0.557856, 0.596, 0.648448, 0.700896, 0.753344, 0.81056, 0.867776, 0.924992, 0.982208, 1.039424, 1.09664],
    a_dmg31: [0.291617, 0.315354, 0.33909, 0.373, 0.396735, 0.423863, 0.461162, 0.498462, 0.535762, 0.576453, 0.617144, 0.657835, 0.698525, 0.739216, 0.779907],
    a_dmg32: [0.291617, 0.315354, 0.33909, 0.373, 0.396735, 0.423863, 0.461162, 0.498462, 0.535762, 0.576453, 0.617144, 0.657835, 0.698525, 0.739216, 0.779907],
    a_dmg4: [0.711521, 0.769436, 0.82735, 0.910085, 0.968, 1.034188, 1.125196, 1.216205, 1.307213, 1.406495, 1.505777, 1.605059, 1.704341, 1.803623, 1.902905],
    z_dmg: [1.13434, 1.22667, 1.319, 1.4509, 1.54323, 1.64875, 1.79384, 1.93893, 2.08402, 2.2423, 2.40058, 2.55886, 2.71714, 2.87542, 3.0337],
    x_dmg1: [0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572, 1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982],
    x_dmg2: [1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646, 2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915],
    x_dmg3: [1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586, 3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041],

    // Elemental Skill: Binary Form: Convergence and Division
    e_dmgp: [1.056, 1.1352, 1.2144, 1.32, 1.3992, 1.4784, 1.584, 1.6896, 1.7952, 1.9008, 2.0064, 2.112, 2.244, 2.376, 2.508], 
    e_dmgd1: [0.7224, 0.77658, 0.83076, 0.903, 0.95718, 1.01136, 1.0836, 1.15584, 1.22808, 1.30032, 1.37256, 1.4448, 1.5351, 1.6254, 1.7157],
    e_dmgd2: [0.532, 0.5719, 0.6118, 0.665, 0.7049, 0.7448, 0.798, 0.8512, 0.9044, 0.9576, 1.0108, 1.064, 1.1305, 1.197, 1.2635], 
    e_dmgd3: [0.6464, 0.69488, 0.74336, 0.808, 0.85648, 0.90496, 0.9696, 1.03424, 1.09888, 1.16352, 1.22816, 1.2928, 1.3736, 1.4544, 1.5352],

    // Elemental Burst: Principle of Purity: As the Light Shifts
    q_dmgp1: [1.1896, 1.27882, 1.36804, 1.487, 1.57622, 1.66544, 1.7844, 1.90336, 2.02232, 2.14128, 2.26024, 2.3792, 2.5279, 2.6766, 2.8253],
    q_dmgp2: [0.964, 1.0363, 1.1086, 1.205, 1.2773, 1.3496, 1.446, 1.5424, 1.6388, 1.7352, 1.8316, 1.928, 2.0485, 2.169, 2.2895],
    q_dmgp3: [1.1184, 1.20228, 1.28616, 1.398, 1.48188, 1.56576, 1.6776, 1.78944, 1.90128, 2.01312, 2.12496, 2.2368, 2.3766, 2.5164, 2.6562],
    q_dmgp: [0.9464, 1.01738, 1.08836, 1.183, 1.25398, 1.32496, 1.4196, 1.51424, 1.60888, 1.70352, 1.79816, 1.8928, 2.0111, 2.1294, 2.2477],
    q_dmgd1: [1.2544, 1.34848, 1.44256, 1.568, 1.66208, 1.75616, 1.8816, 2.00704, 2.13248, 2.25792, 2.38336, 2.5088, 2.6656, 2.8224, 2.9792],
    q_dmgd2: [1.0176, 1.09392, 1.17024, 1.272, 1.34832, 1.42464, 1.5264, 1.62816, 1.72992, 1.83168, 1.93344, 2.0352, 2.1624, 2.2896, 2.4168],
    q_dmgd3: [1.1184, 1.20228, 1.28616, 1.398, 1.48188, 1.56576, 1.6776, 1.78944, 1.90128, 2.01312, 2.12496, 2.2368, 2.3766, 2.5164, 2.6562],
    q_dmgd: [1.2984, 1.39578, 1.49316, 1.623, 1.72038, 1.81776, 1.9476, 2.07744, 2.20728, 2.33712, 2.46696, 2.5968, 2.7591, 2.9214, 3.0837],
};

pub const DURIN_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Durin,
    internal_name: "Durin",
    element: Element::Pyro,
    hp: [968, 2510, 3340, 4997, 5587, 6428, 7214, 8063, 8653, 9510, 10099, 10966, 11556, 12430, 13313],
    atk: [27, 70, 93, 139, 156, 179, 201, 225, 241, 265, 282, 306, 322, 347, 425],
    def: [64, 166, 221, 331, 370, 425, 477, 533, 572, 629, 668, 726, 765, 822, 881],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "芒焰之翼斩",
        en: "Radiant Wingslash",
    ),
    skill_name2: locale!(
        zh_cn: "二元式·聚分熔炼",
        en: "Binary Form: Convergence and Division",
    ),
    skill_name3: locale!(
        zh_cn: "白化法·如光流变",
        en: "Principle of Purity: As the Light Shifts",
    ),
    name_locale: locale!(
        zh_cn: "杜林",
        en: "Durin",
    )
};

pub struct DurinEffect {
    pub hexerei_secret_rite: bool,
    pub essential_transmutation: usize,
    pub has_p1: bool,
    pub has_c4: bool,
}

impl<A: Attribute> ChangeAttribute<A> for DurinEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_p1 {
            if self.essential_transmutation == 0 {
                attribute.set_value_by(AttributeName::ResMinusPyro, "天赋1：光灵遵神数显现", if self.hexerei_secret_rite { 0.35 } else { 0.20 });
            } else {
                attribute.set_value_by(AttributeName::EnhanceMelt, "天赋1：光灵遵神数显现", if self.hexerei_secret_rite { 0.7 } else { 0.4 });
                attribute.set_value_by(AttributeName::EnhanceVaporize, "天赋1：光灵遵神数显现", if self.hexerei_secret_rite { 0.7 } else { 0.4 });
            }
        }
    }
}

damage_enum!(
    DurinDamageEnum
    A1
    A2
    A31
    A32
    A4
    Z
    X1
    X2
    X3
    EP
    ED1
    ED2
    ED3
    QP1
    QP2
    QP3
    QP
    QD1
    QD2
    QD3
    QD
);

impl DurinDamageEnum {
    pub fn get_element(&self) -> Element {
        use DurinDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 | A4 | Z | X1 | X2 | X3 => Element::Physical,
            EP | ED1 | ED2 | ED3 | QP1 | QP2 | QP3 | QP | QD1 | QD2 | QD3 | QD => Element::Pyro,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use DurinDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 | A4 => SkillType::NormalAttack,
            Z => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            EP | ED1 | ED2 | ED3 => SkillType::ElementalSkill,
            QP1 | QP2 | QP3 | QP | QD1 | QD2 | QD3 | QD => SkillType::ElementalBurst,
        }
    }
}

pub struct Durin;

impl CharacterTrait for Durin {
    const STATIC_DATA: CharacterStaticData = DURIN_STATIC_DATA;
    type SkillType = DurinSkillType;
    const SKILL: Self::SkillType = DURIN_SKILL;
    type DamageEnumType = DurinDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            DurinDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A31 hit_n_dmg!(3, 1)
            A32 hit_n_dmg!(3, 2)
            A4 hit_n_dmg!(4)
            Z charged_dmg!()
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            DurinDamageEnum
            EP locale!(zh_cn: "转变·白化之是伤害", en: "Transmutation: Confirmation of Purity DMG")
            ED1 locale!(zh_cn: "转变·黑度之否一段伤害", en: "Transmutation: Denial of Darkness DMG 1")
            ED2 locale!(zh_cn: "转变·黑度之否二段伤害", en: "Transmutation: Denial of Darkness DMG 2")
            ED3 locale!(zh_cn: "转变·黑度之否三段伤害", en: "Transmutation: Denial of Darkness DMG 3")
        ),
        skill3: skill_map!(
            DurinDamageEnum
            QP1 locale!(zh_cn: "白化法·如光流变一段伤害", en: "Principle of Purity: As the Light Shifts DMG 1")
            QP2 locale!(zh_cn: "白化法·如光流变二段伤害", en: "Principle of Purity: As the Light Shifts DMG 2")
            QP3 locale!(zh_cn: "白化法·如光流变三段伤害", en: "Principle of Purity: As the Light Shifts DMG 3")
            QP locale!(zh_cn: "白焰之龙伤害", en: "Dragon of White Flame DMG")
            QD1 locale!(zh_cn: "黑度法·如星阴燃一段伤害", en: "Principle of Darkness: As the Stars Smolder DMG 1")
            QD2 locale!(zh_cn: "黑度法·如星阴燃二段伤害", en: "Principle of Darkness: As the Stars Smolder DMG 2")
            QD3 locale!(zh_cn: "黑度法·如星阴燃三段伤害", en: "Principle of Darkness: As the Stars Smolder DMG 3")
            QD locale!(zh_cn: "黑焰之龙伤害", en: "Dragon of Dark Decay DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig::IS_HEXEREI(true, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig {
            name: "essential_transmutation",
            title: locale!(
                zh_cn: "精质转变",
                en: "Essential Transmutation"
            ),
            config: ItemConfigType::Option2 { options_zh: "白化之是,黑度之否", options_en: "Confirmation of Purity,Denial of Darkness", default: 0 }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "activated_res",
            title: locale!(
                zh_cn: "白焰之龙减抗",
                en: "Dragon of White Flame RES Reduction"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "primordial_fusion",
            title: locale!(
                zh_cn: "存在「肇象」",
                en: "Has primordial Fusion"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "cycle_of_enlightenment",
            title: locale!(
                zh_cn: "存在「轮变启迪」",
                en: "Has Cycle of Enlightenment"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "activated_reaction",
            title: locale!(
                zh_cn: "触发火元素相关反应",
                en: "Activated Pyro Element Reactions"
            ),
            config: ItemConfigType::Bool { default: true }
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: DurinDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (hexerei_secret_rite, essential_transmutation) = match &context.character_common_data.config {
            CharacterConfig::Durin { hexerei_secret_rite, essential_transmutation } => (*hexerei_secret_rite, *essential_transmutation),
            _ => (false, 0),
        };

        let (activated_res, primordial_fusion, cycle_of_enlightenment, activated_reaction) = match *config {
            CharacterSkillConfig::Durin { activated_res, primordial_fusion, cycle_of_enlightenment, activated_reaction } => (activated_res, primordial_fusion, cycle_of_enlightenment, activated_reaction),
            _ => (false, false, false, false)
        };

        use DurinDamageEnum::*;
        let mut builder = D::new();

        if context.character_common_data.constellation >= 1 && cycle_of_enlightenment {
            if essential_transmutation == 1 && s.get_skill_type() == SkillType::ElementalBurst {
                builder.add_extra_damage("命座1：红土之逆", context.attribute.get_atk() * 1.50);
            }
        }

        if context.character_common_data.constellation >= 2 && activated_reaction {
            if s.get_element() == Element::Pyro {
                builder.add_extra_bonus("命座2：无底之想", 0.5);
            }
        }

        if context.character_common_data.constellation >= 4 && s.get_skill_type() == SkillType::ElementalBurst {
            builder.add_extra_bonus("命座4：流溢之原", 0.4);
        }

        if context.character_common_data.constellation >= 6 {
            if essential_transmutation == 0 {
                builder.add_extra_def_penetration("命座6：双重诞生", 0.3);
                builder.add_extra_def_minus("命座6：双重诞生", 0.3);
            } else {
                builder.add_extra_def_penetration("命座6：双重诞生", 0.7);
            }
        }

        let ratio = match s {
            A1 => DURIN_SKILL.a_dmg1[s1],
            A2 => DURIN_SKILL.a_dmg2[s1],
            A31 => DURIN_SKILL.a_dmg31[s1],
            A32 => DURIN_SKILL.a_dmg32[s1],
            A4 => DURIN_SKILL.a_dmg4[s1],
            Z => DURIN_SKILL.z_dmg[s1],
            X1 => DURIN_SKILL.x_dmg1[s1],
            X2 => DURIN_SKILL.x_dmg2[s1],
            X3 => DURIN_SKILL.x_dmg3[s1],
            EP => DURIN_SKILL.e_dmgp[s2],
            ED1 => DURIN_SKILL.e_dmgd1[s2],
            ED2 => DURIN_SKILL.e_dmgd2[s2],
            ED3 => DURIN_SKILL.e_dmgd3[s2],
            QP1 => DURIN_SKILL.q_dmgp1[s3],
            QP2 => DURIN_SKILL.q_dmgp2[s3],
            QP3 => DURIN_SKILL.q_dmgp3[s3],
            QP => DURIN_SKILL.q_dmgp[s3],
            QD1 => DURIN_SKILL.q_dmgd1[s3],
            QD2 => DURIN_SKILL.q_dmgd2[s3],
            QD3 => DURIN_SKILL.q_dmgd3[s3],
            QD => DURIN_SKILL.q_dmgd[s3],
        };

        let extra_ratio = if context.character_common_data.has_talent2 && primordial_fusion {
            (context.attribute.get_atk() / 100.0 * 0.03).min(0.75)
        } else { 0.0 } * ratio;

        builder.add_atk_ratio("技能倍率", ratio);
        if extra_ratio > 0.0 {
            builder.add_atk_ratio("天赋2：混沌如黑夜构成", extra_ratio);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (hexerei_secret_rite, essential_transmutation) = match *config {
            CharacterConfig::Durin { hexerei_secret_rite, essential_transmutation } => (hexerei_secret_rite, essential_transmutation),
            _ => (false, 0),
        };
        Some(Box::new(DurinEffect {
            hexerei_secret_rite: hexerei_secret_rite,
            essential_transmutation: essential_transmutation,
            has_p1: common_data.has_talent1,
            has_c4: common_data.constellation >= 4,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
