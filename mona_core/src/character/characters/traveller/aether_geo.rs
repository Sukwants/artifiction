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

pub struct AetherGeoSkillType {
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

    pub p2_dmg: f64,
}

pub const AETHERGEO_SKILL: AetherGeoSkillType = AetherGeoSkillType {
    // Normal Attack: Foreign Rockblade
    a_dmg1: [0.44462, 0.48081, 0.517, 0.5687, 0.60489, 0.64625, 0.70312, 0.76, 0.81686, 0.8789, 0.94094, 1.00298, 1.06502, 1.12706, 1.1891],
    a_dmg2: [0.4343, 0.46965, 0.505, 0.5555, 0.59085, 0.63125, 0.6868, 0.74235, 0.7979, 0.8585, 0.9191, 0.9797, 1.0403, 1.1009, 1.1615],
    a_dmg3: [0.52976, 0.57288, 0.616, 0.6776, 0.72072, 0.77, 0.83776, 0.90552, 0.97328, 1.0472, 1.12112, 1.19504, 1.26896, 1.34288, 1.4168],
    a_dmg4: [0.58308, 0.63054, 0.678, 0.7458, 0.79326, 0.8475, 0.92208, 0.99666, 1.07124, 1.1526, 1.23396, 1.31532, 1.39668, 1.47804, 1.5594],
    a_dmg5: [0.70778, 0.76539, 0.823, 0.9053, 0.96291, 1.02875, 1.11928, 1.20981, 1.30034, 1.3991, 1.49786, 1.59662, 1.69538, 1.79414, 1.8929],
    z_dmg1: [0.559, 0.6045, 0.65, 0.715, 0.7605, 0.8125, 0.884, 0.9555, 1.027, 1.105, 1.183, 1.261, 1.339, 1.417, 1.495],
    z_dmg2: [0.60716, 0.65658, 0.706, 0.7766, 0.82602, 0.8825, 0.96016, 1.03782, 1.11548, 1.2002, 1.28492, 1.36964, 1.45436, 1.53908, 1.6238],
    x_dmg1: [0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572, 1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982],
    x_dmg2: [1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646, 2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915],
    x_dmg3: [1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586, 3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041],

    // Elemental Skill: Starfell Sword
    e_dmg: [2.48, 2.666, 2.852, 3.1, 3.286, 3.472, 3.72, 3.968, 4.216, 4.464, 4.712, 4.96, 5.27, 5.58, 5.89],

    // Elemental Burst: Wake of Earth
    q_dmg: [1.48, 1.591, 1.702, 1.85, 1.961, 2.072, 2.22, 2.368, 2.516, 2.664, 2.812, 2.96, 3.145, 3.33, 3.515],

    p2_dmg: 0.6,
};

pub const AETHERGEO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::AetherGeo,
    internal_name: "AetherGeo",
    element: Element::Geo,
    hp: [912, 2342, 3024, 4529, 5031, 5766, 6411, 7164, 7648, 8401, 8885, 9638, 10122, 10875, 11627],
    atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212, 266],
    def: [57, 147, 190, 284, 315, 362, 402, 450, 480, 527, 558, 605, 635, 683, 730],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "异邦岩锋",
        en: "Foreign Rockblade",
    ),
    skill_name2: locale!(
        zh_cn: "星陨剑",
        en: "Starfell Sword",
    ),
    skill_name3: locale!(
        zh_cn: "岩潮叠嶂",
        en: "Wake of Earth",
    ),
    name_locale: locale!(
        zh_cn: "空-岩",
        en: "Aether-Geo",
    )
};

pub struct AetherGeoEffect {
}

impl<A: Attribute> ChangeAttribute<A> for AetherGeoEffect {
    fn change_attribute(&self, attribute: &mut A) {
    }
}

damage_enum!(
    AetherGeoDamageEnum
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
    P2
);

impl AetherGeoDamageEnum {
    pub fn get_element(&self) -> Element {
        use AetherGeoDamageEnum::*;
        match *self {
            E | Q | P2 => Element::Geo,
            _ => Element::Physical,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use AetherGeoDamageEnum::*;
        match *self {
            A1 | A2 | A3 | A4 | A5 | P2 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E => SkillType::ElementalSkill,
            Q => SkillType::ElementalBurst,
        }
    }
}

pub struct AetherGeo;

impl CharacterTrait for AetherGeo {
    const STATIC_DATA: CharacterStaticData = AETHERGEO_STATIC_DATA;
    type SkillType = AetherGeoSkillType;
    const SKILL: Self::SkillType = AETHERGEO_SKILL;
    type DamageEnumType = AetherGeoDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            AetherGeoDamageEnum
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
            AetherGeoDamageEnum
            E locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            AetherGeoDamageEnum
            Q locale!(zh_cn: "地震波单次伤害", en: "DMG Per Shockwave")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: AetherGeoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use AetherGeoDamageEnum::*;
        let mut builder = D::new();

        let ratio = match s {
            A1 => AETHERGEO_SKILL.a_dmg1[s1],
            A2 => AETHERGEO_SKILL.a_dmg2[s1],
            A3 => AETHERGEO_SKILL.a_dmg3[s1],
            A4 => AETHERGEO_SKILL.a_dmg4[s1],
            A5 => AETHERGEO_SKILL.a_dmg5[s1],
            Z1 => AETHERGEO_SKILL.z_dmg1[s1],
            Z2 => AETHERGEO_SKILL.z_dmg2[s1],
            X1 => AETHERGEO_SKILL.x_dmg1[s1],
            X2 => AETHERGEO_SKILL.x_dmg2[s1],
            X3 => AETHERGEO_SKILL.x_dmg3[s1],
            E => AETHERGEO_SKILL.e_dmg[s2],
            Q => AETHERGEO_SKILL.q_dmg[s3],
            P2 => AETHERGEO_SKILL.p2_dmg,
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
        Some(Box::new(AetherGeoEffect {
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
