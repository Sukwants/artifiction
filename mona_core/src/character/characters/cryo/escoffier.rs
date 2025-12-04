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
// use crate::target_functions::target_functions::EscoffierDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct EscoffierSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg31: [f64; 15],
    pub a_dmg32: [f64; 15],
    pub z_dmg: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg: [f64; 15],
    pub e_con_dmg: [f64; 15],

    pub q_dmg: [f64; 15],
    pub q_heal_atk: [f64; 15],
    pub q_heal: [f64; 15],

    pub p1_heal: f64,

    pub c6_dmg: f64,
}

pub const ESCOFFIER_SKILL: EscoffierSkillType = EscoffierSkillType {
    // Normal Attack: Kitchen Skills
    a_dmg1: [0.51551, 0.55747, 0.59943, 0.659373, 0.701333, 0.749287, 0.815225, 0.881162, 0.947099, 1.019031, 1.090963, 1.162894, 1.234826, 1.306757, 1.378689],
    a_dmg2: [0.475933, 0.514671, 0.55341, 0.608751, 0.64749, 0.691762, 0.752638, 0.813513, 0.874388, 0.940797, 1.007206, 1.073615, 1.140025, 1.206434, 1.272843],
    a_dmg31: [0.33, 0.356855, 0.383715, 0.422087, 0.448947, 0.479644, 0.521852, 0.564061, 0.60627, 0.652316, 0.698361, 0.744407, 0.790453, 0.836499, 0.882544],
    a_dmg32: [0.403327, 0.436156, 0.468985, 0.515884, 0.548712, 0.586231, 0.63782, 0.689408, 0.740996, 0.797274, 0.853553, 0.909831, 0.966109, 1.022387, 1.078666],
    z_dmg: [1.15412, 1.24806, 1.342, 1.4762, 1.57014, 1.6775, 1.82512, 1.97274, 2.12036, 2.2814, 2.44244, 2.60348, 2.76452, 2.92556, 3.0866],
    x_dmg1: [0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572, 1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982],
    x_dmg2: [1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646, 2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915],
    x_dmg3: [1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586, 3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041],

    // Elemental Skill: Low-Temperature Cooking
    e_dmg: [0.504, 0.5418, 0.5796, 0.63, 0.6678, 0.7056, 0.756, 0.8064, 0.8568, 0.9072, 0.9576, 1.008, 1.071, 1.134, 1.197],
    e_con_dmg: [1.2, 1.29, 1.38, 1.5, 1.59, 1.68, 1.8, 1.92, 2.04, 2.16, 2.28, 2.4, 2.55, 2.7, 2.85],

    // Elemental Burst: Scoring Cuts
    q_dmg: [5.928, 6.3726, 6.8172, 7.41, 7.8546, 8.2992, 8.892, 9.4848, 10.0776, 10.6704, 11.2632, 11.856, 12.597, 13.338, 14.079],
    q_heal_atk: [1.72032, 1.849344, 1.978368, 2.1504, 2.279424, 2.408448, 2.58048, 2.752512, 2.924544, 3.096576, 3.268608, 3.44064, 3.65568, 3.87072, 4.08576],
    q_heal: [1078.5255, 1186.3931, 1303.2495, 1429.095, 1563.9294, 1707.7528, 1860.5652, 2022.3665, 2193.1567, 2372.936, 2561.704, 2759.4614, 2966.2075, 3181.9426, 3406.6665],

    p1_heal: 1.3824,

    c6_dmg: 5.0,
};

pub const ESCOFFIER_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Escoffier,
    internal_name: "Escoffier",
    element: Element::Cryo,
    hp: [1039, 2695, 3586, 5366, 5999, 6902, 7747, 8659, 9292, 10213, 10846, 11777, 12410, 13348, 14297],
    atk: [27, 70, 93, 139, 156, 179, 201, 225, 241, 265, 282, 306, 322, 347, 425],
    def: [57, 148, 197, 294, 329, 378, 425, 475, 509, 560, 595, 646, 680, 732, 784],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Polearm,
    star: 5,
    skill_name1: locale!(
        zh_cn: "后厨手艺",
        en: "Normal Attack: Kitchen Skills",
    ),
    skill_name2: locale!(
        zh_cn: "低温烹饪",
        en: "Low-Temperature Cooking",
    ),
    skill_name3: locale!(
        zh_cn: "花刀技法",
        en: "Scoring Cuts",
    ),
    name_locale: locale!(
        zh_cn: "爱可菲",
        en: "Escoffier",
    ),
};

pub struct EscoffierEffect {
    pub hydro_cryo_count: usize,
    pub has_p2: bool,
    pub has_c1: bool,
    pub has_c4: bool,
}

impl<A: Attribute> ChangeAttribute<A> for EscoffierEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_p2 {
            attribute.set_value_by(AttributeName::ResMinusHydro, "爱可菲「灵感浸入调味」", [0.00, 0.05, 0.10, 0.15, 0.55][self.hydro_cryo_count]);
            attribute.set_value_by(AttributeName::ResMinusCryo, "爱可菲「灵感浸入调味」", [0.00, 0.05, 0.10, 0.15, 0.55][self.hydro_cryo_count]);

            if self.has_c1 && self.hydro_cryo_count >= 4 {
                attribute.set_value_by(AttributeName::CriticalDamageCryo, "爱可菲「味蕾绽放的餐前旋舞」", 0.6);
            }
        }
    }
}

damage_enum!(
    EscoffierDamageEnum
    A1
    A2
    A31
    A32
    Z
    X1
    X2
    X3
    E
    ECon
    Q
    QHeal
    P1Heal
    C6
);

impl EscoffierDamageEnum {
    pub fn get_element(&self) -> Element {
        use EscoffierDamageEnum::*;
        match *self {
            E | ECon | Q | C6 => Element::Cryo,
            _ => Element::Physical,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use EscoffierDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 => SkillType::NormalAttack,
            Z => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E | ECon | C6 => SkillType::ElementalSkill,
            Q => SkillType::ElementalBurst,
            QHeal | P1Heal => SkillType::NoneType,
        }
    }
}

pub struct Escoffier;

impl CharacterTrait for Escoffier {
    const STATIC_DATA: CharacterStaticData = ESCOFFIER_STATIC_DATA;
    type SkillType = EscoffierSkillType;
    const SKILL: Self::SkillType = ESCOFFIER_SKILL;
    type DamageEnumType = EscoffierDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            EscoffierDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A31 hit_n_dmg!(3, 1)
            A32 hit_n_dmg!(3, 2)
            Z charged_dmg!()
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            EscoffierDamageEnum
            E locale!(zh_cn: "技能伤害", en: "Skill DMG")
            ECon locale!(zh_cn: "冻霜芭菲伤害", en: "Frosty Parfait DMG")
            C6 locale!(zh_cn: "六命额外伤害", en: "C6 extra DMG")
        ),
        skill3: skill_map!(
            EscoffierDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
            QHeal locale!(zh_cn: "技能治疗量", en: "Skill Heal Amount")
            P1Heal locale!(zh_cn: "「康复食疗」治疗量", en: "Recovery Effect Heal Amount")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hydro_cryo_count",
            title: locale!(
                zh_cn: "队伍中冰水元素角色数量",
                en: "Count of Hydro/Cryo characters in the team",
            ),
            config: ItemConfigType::Int { min: 1, max: 4, default: 1 },
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: EscoffierDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use EscoffierDamageEnum::*;
        let mut builder = D::new();

        if s == QHeal || s == P1Heal {
            if s == QHeal {
                builder.add_atk_ratio("额外治疗量", ESCOFFIER_SKILL.q_heal_atk[s3]);
                builder.add_extra_damage("基础治疗量", ESCOFFIER_SKILL.q_heal[s3]);
            } else if s == P1Heal {
                if context.character_common_data.constellation >= 4 {
                    builder.add_extra_critical("爱可菲「迷迭生香的配比秘方」", context.attribute.get_value(AttributeName::CriticalBase));
                    builder.add_extra_critical_damage("爱可菲「迷迭生香的配比秘方」", 1.0);
                }

                builder.add_atk_ratio("额外治疗量", ESCOFFIER_SKILL.p1_heal);
            }

            builder.heal(
                &context.attribute,
            )
        } else {
            let ratio = match s {
                A1 => ESCOFFIER_SKILL.a_dmg1[s1],
                A2 => ESCOFFIER_SKILL.a_dmg2[s1],
                A31 => ESCOFFIER_SKILL.a_dmg31[s1],
                A32 => ESCOFFIER_SKILL.a_dmg32[s1],
                Z => ESCOFFIER_SKILL.z_dmg[s1],
                X1 => ESCOFFIER_SKILL.x_dmg1[s1],
                X2 => ESCOFFIER_SKILL.x_dmg2[s1],
                X3 => ESCOFFIER_SKILL.x_dmg3[s1],
                E => ESCOFFIER_SKILL.e_dmg[s2],
                ECon => ESCOFFIER_SKILL.e_con_dmg[s2],
                Q => ESCOFFIER_SKILL.q_dmg[s3],
                C6 => ESCOFFIER_SKILL.c6_dmg,
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
        Some(Box::new(EscoffierEffect {
            hydro_cryo_count: match config {
                    CharacterConfig::Escoffier { hydro_cryo_count } => *hydro_cryo_count,
                    _ => 0,
                },
            has_p2: common_data.has_talent2,
            has_c1: common_data.constellation >= 1,
            has_c4: common_data.constellation >= 4,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
