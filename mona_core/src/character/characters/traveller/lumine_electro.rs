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

pub struct LumineElectroSkillType {
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

    pub q_dmg: [f64; 15],
    pub q_dmgt: [f64; 15], // Falling Thunder DMG
}

pub const LUMINEELECTRO_SKILL: LumineElectroSkillType = LumineElectroSkillType {
    // Normal Attack: Foreign Thundershock
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

    // Elemental Skill: Lightning Blade
    e_dmg: [0.78664, 0.845638, 0.904636, 0.9833, 1.042298, 1.101296, 1.17996, 1.258624, 1.337288, 1.415952, 1.494616, 1.57328, 1.67161, 1.76994, 1.86827],

    // Elemental Burst: Bellowing Thunder
    q_dmg: [1.144, 1.2298, 1.3156, 1.43, 1.5158, 1.6016, 1.716, 1.8304, 1.9448, 2.0592, 2.1736, 2.288, 2.431, 2.574, 2.717],
    q_dmgt: [0.328, 0.3526, 0.3772, 0.41, 0.4346, 0.4592, 0.492, 0.5248, 0.5576, 0.5904, 0.6232, 0.656, 0.697, 0.738, 0.779],
};

pub const LUMINEELECTRO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::LumineElectro,
    internal_name: "LumineElectro",
    element: Element::Electro,
    hp: [912, 2342, 3024, 4529, 5031, 5766, 6411, 7164, 7648, 8401, 8885, 9638, 10122, 10875, 11627],
    atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212, 266],
    def: [57, 147, 190, 284, 315, 362, 402, 450, 480, 527, 558, 605, 635, 683, 730],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "异邦惊雷",
        en: "Foreign Thundershock",
    ),
    skill_name2: locale!(
        zh_cn: "雷影剑",
        en: "Lightning Blade",
    ),
    skill_name3: locale!(
        zh_cn: "雷轰电转",
        en: "Bellowing Thunder",
    ),
    name_locale: locale!(
        zh_cn: "荧-雷",
        en: "Lumine-Electro",
    )
};

pub struct LumineElectroEffect {
    pub has_p2: bool,
    pub has_c2: bool,
    pub abundance_amulet_count: usize,
}

impl<A: Attribute> ChangeAttribute<A> for LumineElectroEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_c2 {
            attribute.set_value_by(AttributeName::ResMinusElectro, "二命：震怒的苍雷", 0.15);
        }

        if self.abundance_amulet_count > 0 {
            attribute.set_value_by(AttributeName::RechargeExtra, "元素战技：丰穰勾玉", 0.2);

            if self.has_p2 {
                attribute.add_edge1(
                    AttributeName::Recharge,
                    AttributeName::RechargeExtra,
                    Box::new(move |rec, _| {
                        rec * 0.1
                    }),
                    Box::new(move |rec, _, grad| (0.0, 0.0)),
                    "天赋2：回响的轰雷"
                );
            }
        }
    }
}

damage_enum!(
    LumineElectroDamageEnum
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
    Q
    QT
    QC6
);

impl LumineElectroDamageEnum {
    pub fn get_element(&self) -> Element {
        use LumineElectroDamageEnum::*;
        match *self {
            E | Q | QT | QC6 => Element::Electro,
            _ => Element::Physical,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use LumineElectroDamageEnum::*;
        match *self {
            A1 | A2 | A3 | A4 | A5 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E => SkillType::ElementalSkill,
            Q | QT | QC6 => SkillType::ElementalBurst,
        }
    }
}

pub struct LumineElectro;

impl CharacterTrait for LumineElectro {
    const STATIC_DATA: CharacterStaticData = LUMINEELECTRO_STATIC_DATA;
    type SkillType = LumineElectroSkillType;
    const SKILL: Self::SkillType = LUMINEELECTRO_SKILL;
    type DamageEnumType = LumineElectroDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LumineElectroDamageEnum
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
            LumineElectroDamageEnum
            E locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            LumineElectroDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
            QT locale!(zh_cn: "威光落雷伤害", en: "Falling Thunder DMG")
            QC6 locale!(zh_cn: "威光落雷六命增强伤害", en: "Falling Thunder DMG C6 Enhanced")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "abundance_amulet_count",
            title: locale!(
                zh_cn: "丰穰勾玉吸收数量",
                en: "Abundance Amulet Absorption Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: LumineElectroDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use LumineElectroDamageEnum::*;
        let mut builder = D::new();

        let ratio = match s {
            A1 => LUMINEELECTRO_SKILL.a_dmg1[s1],
            A2 => LUMINEELECTRO_SKILL.a_dmg2[s1],
            A3 => LUMINEELECTRO_SKILL.a_dmg3[s1],
            A4 => LUMINEELECTRO_SKILL.a_dmg4[s1],
            A5 => LUMINEELECTRO_SKILL.a_dmg5[s1],
            Z1 => LUMINEELECTRO_SKILL.z_dmg1[s1],
            Z2 => LUMINEELECTRO_SKILL.z_dmg2[s1],
            X1 => LUMINEELECTRO_SKILL.x_dmg1[s1],
            X2 => LUMINEELECTRO_SKILL.x_dmg2[s1],
            X3 => LUMINEELECTRO_SKILL.x_dmg3[s1],
            E => LUMINEELECTRO_SKILL.e_dmg[s2],
            Q => LUMINEELECTRO_SKILL.q_dmg[s3],
            QT => LUMINEELECTRO_SKILL.q_dmgt[s3],
            QC6 => LUMINEELECTRO_SKILL.q_dmgt[s3] * 2.0,
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
        Some(Box::new(LumineElectroEffect {
            has_p2: common_data.has_talent2,
            has_c2: common_data.constellation >= 2,
            abundance_amulet_count: match config {
                CharacterConfig::LumineElectro { abundance_amulet_count } => *abundance_amulet_count,
                _ => 0
            },
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
