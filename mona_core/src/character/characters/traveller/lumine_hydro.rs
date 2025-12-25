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

pub struct LumineHydroSkillType {
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

    pub e_dmg: [f64; 15],
    pub e_dmg_d: [f64; 15], // Dewdrop DMG
    pub e_dmg_st: [f64; 15], // Spiritbreath Thorn DMG
    pub e_dmg_ad: [f64; 15], // Suffusion DMG Bonus

    pub q_dmg: [f64; 15],
}

pub const LUMINEHYDRO_SKILL: LumineHydroSkillType = LumineHydroSkillType {
    // Normal Attack: Foreign Stream
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

    // Elemental Skill: Aquacrest Saber
    e_dmg: [1.8928, 2.03476, 2.17672, 2.366, 2.50796, 2.64992, 2.8392, 3.02848, 3.21776, 3.40704, 3.59632, 3.7856, 4.0222, 4.2588, 4.4954],
    e_dmg_d: [0.328, 0.3526, 0.3772, 0.41, 0.4346, 0.4592, 0.492, 0.5248, 0.5576, 0.5904, 0.6232, 0.656, 0.697, 0.738, 0.779],
    e_dmg_st: [0.328, 0.3526, 0.3772, 0.41, 0.4346, 0.4592, 0.492, 0.5248, 0.5576, 0.5904, 0.6232, 0.656, 0.697, 0.738, 0.779],
    e_dmg_ad: [0.0064, 0.00688, 0.00736, 0.008, 0.00848, 0.00896, 0.0096, 0.01024, 0.01088, 0.01152, 0.01216, 0.0128, 0.0136, 0.0144, 0.0152],

    // Elemental Burst: Rising Waters
    q_dmg: [1.018664, 1.095064, 1.171464, 1.27333, 1.34973, 1.42613, 1.527996, 1.629862, 1.731729, 1.833595, 1.935462, 2.037328, 2.164661, 2.291994, 2.419327],
};

pub const LUMINEHYDRO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::LumineHydro,
    internal_name: "LumineHydro",
    element: Element::Hydro,
    hp: [912, 2342, 3024, 4529, 5031, 5766, 6411, 7164, 7648, 8401, 8885, 9638, 10122, 10875, 11627],
    atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212, 266],
    def: [57, 147, 190, 284, 315, 362, 402, 450, 480, 527, 558, 605, 635, 683, 730],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "异邦激流",
        en: "Foreign Stream",
    ),
    skill_name2: locale!(
        zh_cn: "水纹剑",
        en: "Aquacrest Saber",
    ),
    skill_name3: locale!(
        zh_cn: "扬水制流",
        en: "Rising Waters",
    ),
    name_locale: locale!(
        zh_cn: "荧-水",
        en: "Lumine-Hydro",
    )
};

pub struct LumineHydroEffect {
}

impl<A: Attribute> ChangeAttribute<A> for LumineHydroEffect {
    fn change_attribute(&self, attribute: &mut A) {
    }
}

damage_enum!(
    LumineHydroDamageEnum
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
    E
    ED
    EST
    Q
);

impl LumineHydroDamageEnum {
    pub fn get_element(&self) -> Element {
        use LumineHydroDamageEnum::*;
        match *self {
            E | ED | EST | Q => Element::Hydro,
            _ => Element::Physical,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use LumineHydroDamageEnum::*;
        match *self {
            A1 | A2 | A3 | A4 | A5 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E | ED | EST => SkillType::ElementalSkill,
            Q => SkillType::ElementalBurst,
        }
    }
}

pub struct LumineHydro;

impl CharacterTrait for LumineHydro {
    const STATIC_DATA: CharacterStaticData = LUMINEHYDRO_STATIC_DATA;
    type SkillType = LumineHydroSkillType;
    const SKILL: Self::SkillType = LUMINEHYDRO_SKILL;
    type DamageEnumType = LumineHydroDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LumineHydroDamageEnum
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
        ),
        skill2: skill_map!(
            LumineHydroDamageEnum
            E locale!(zh_cn: "喷发激流伤害", en: "Torrent Surge DMG")
            ED locale!(zh_cn: "露滴伤害", en: "Dewdrop DMG")
            EST locale!(zh_cn: "灵息之刺伤害", en: "Spiritbreath Thorn DMG")
        ),
        skill3: skill_map!(
            LumineHydroDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp_above_half",
            title: locale!(
                zh_cn: "生命值高于50%",
                en: "HP Above 50%",
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "consumed_hp_times",
            title: locale!(
                zh_cn: "元素战技消耗生命值次数",
                en: "Elemental Skill Consumed HP Times",
            ),
            config: ItemConfigType::Int { min: 0, max: 5, default: 0 }
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: LumineHydroDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (hp_above_half, consumed_hp_times) = match *config {
            CharacterSkillConfig::LumineHydro { hp_above_half, consumed_hp_times } => (hp_above_half, consumed_hp_times),
            _ => (false, 0)
        };

        use LumineHydroDamageEnum::*;
        let mut builder = D::new();

        if s == ED {
            if hp_above_half {
                builder.add_hp_ratio("充盈伤害额外倍率", LUMINEHYDRO_SKILL.e_dmg_ad[s2]);
            }
        }

        if s == E {
            if consumed_hp_times > 0 {
                let consumed = context.attribute.get_hp() * 0.04 * (consumed_hp_times as f64);
                builder.add_extra_damage("天赋2：澄明的净水", (consumed * 0.45).min(5000.0));
            }
        }

        let ratio = match s {
            A1 => LUMINEHYDRO_SKILL.a_dmg1[s1],
            A2 => LUMINEHYDRO_SKILL.a_dmg2[s1],
            A3 => LUMINEHYDRO_SKILL.a_dmg3[s1],
            A4 => LUMINEHYDRO_SKILL.a_dmg4[s1],
            A5 => LUMINEHYDRO_SKILL.a_dmg5[s1],
            Z1 => LUMINEHYDRO_SKILL.z_dmg1[s1],
            Z2 => LUMINEHYDRO_SKILL.z_dmg2[s1],
            X1 => LUMINEHYDRO_SKILL.x_dmg1[s1],
            X2 => LUMINEHYDRO_SKILL.x_dmg2[s1],
            X3 => LUMINEHYDRO_SKILL.x_dmg3[s1],
            E => LUMINEHYDRO_SKILL.e_dmg[s2],
            ED => LUMINEHYDRO_SKILL.e_dmg_d[s2],
            EST => LUMINEHYDRO_SKILL.e_dmg_st[s2],
            Q => LUMINEHYDRO_SKILL.q_dmg[s3],
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

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(LumineHydroEffect {
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
