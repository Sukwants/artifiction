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

pub struct AetherDendroSkillType {
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

    pub q_dmgc: [f64; 15], // Lea Lotus Lamp Attack DMG
    pub q_dmge: [f64; 15], // Explosion DMG
}

pub const AETHERDENDRO_SKILL: AetherDendroSkillType = AetherDendroSkillType {
    // Normal Attack: Foreign Fieldcleaver
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

    // Elemental Skill: Razorgrass Blade
    e_dmg: [2.304, 2.4768, 2.6496, 2.88, 3.0528, 3.2256, 3.456, 3.6864, 3.9168, 4.1472, 4.3776, 4.608, 4.896, 5.184, 5.472],

    // Elemental Burst: Surgent Manifestation
    q_dmgc: [0.8016, 0.86172, 0.92184, 1.002, 1.06212, 1.12224, 1.2024, 1.28256, 1.36272, 1.44288, 1.52304, 1.6032, 1.7034, 1.8036, 1.9038],
    q_dmge: [4.008, 4.3086, 4.6092, 5.01, 5.3106, 5.6112, 6.012, 6.4128, 6.8136, 7.2144, 7.6152, 8.016, 8.517, 9.018, 9.519],
};

pub const AETHERDENDRO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::AetherDendro,
    internal_name: "AetherDendro",
    element: Element::Dendro,
    hp: [912, 2342, 3024, 4529, 5031, 5766, 6411, 7164, 7648, 8401, 8885, 9638, 10122, 10875, 11627],
    atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212, 266],
    def: [57, 147, 190, 284, 315, 362, 402, 450, 480, 527, 558, 605, 635, 683, 730],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "异邦草翦",
        en: "Foreign Fieldcleaver",
    ),
    skill_name2: locale!(
        zh_cn: "草缘剑",
        en: "Razorgrass Blade",
    ),
    skill_name3: locale!(
        zh_cn: "偃草若化",
        en: "Surgent Manifestation",
    ),
    name_locale: locale!(
        zh_cn: "空-草",
        en: "Aether-Dendro",
    )
};

pub struct AetherDendroEffect {
}

impl<A: Attribute> ChangeAttribute<A> for AetherDendroEffect {
    fn change_attribute(&self, attribute: &mut A) {
    }
}

damage_enum!(
    AetherDendroDamageEnum
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
    QC
    QE
);

impl AetherDendroDamageEnum {
    pub fn get_element(&self) -> Element {
        use AetherDendroDamageEnum::*;
        match *self {
            E | QC | QE => Element::Dendro,
            _ => Element::Physical,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use AetherDendroDamageEnum::*;
        match *self {
            A1 | A2 | A3 | A4 | A5 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E => SkillType::ElementalSkill,
            QC | QE => SkillType::ElementalBurst,
        }
    }
}

pub struct AetherDendro;

impl CharacterTrait for AetherDendro {
    const STATIC_DATA: CharacterStaticData = AETHERDENDRO_STATIC_DATA;
    type SkillType = AetherDendroSkillType;
    const SKILL: Self::SkillType = AETHERDENDRO_SKILL;
    type DamageEnumType = AetherDendroDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            AetherDendroDamageEnum
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
            AetherDendroDamageEnum
            E locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
        skill3: skill_map!(
            AetherDendroDamageEnum
            QC locale!(zh_cn: "草灯莲攻击伤害", en: "Lea Lotus Lamp Attack DMG")
            QE locale!(zh_cn: "激烈爆发伤害", en: "Explosion DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "overflowing_lotuslight_count",
            title: locale!(
                zh_cn: "莲光遍照层数",
                en: "Overflowing Lotuslight Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 10, default: 0 }
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: AetherDendroDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let overflowing_lotuslight_count = match config {
            CharacterSkillConfig::AetherDendro { overflowing_lotuslight_count } => *overflowing_lotuslight_count,
            _ => 0
        };

        use AetherDendroDamageEnum::*;
        let mut builder = D::new();

        let mut em = context.attribute.get_value(AttributeName::ElementalMastery);

        if context.character_common_data.has_talent1 {
            builder.add_extra_em("天赋1：蔓生的埜草", (overflowing_lotuslight_count.min(10) as f64) * 6.0);
            em += (overflowing_lotuslight_count.min(10) as f64) * 6.0;
        }

        if context.character_common_data.has_talent2 {
            match s.get_skill_type() {
                SkillType::ElementalSkill => {
                    builder.add_extra_bonus("天赋2：繁庑的丛草", em * 0.0015);
                },
                SkillType::ElementalBurst => {
                    builder.add_extra_bonus("天赋2：繁庑的丛草", em * 0.001);
                },
                _ => {}
            }
        }

        if context.character_common_data.constellation >= 6 && overflowing_lotuslight_count > 0 {
            builder.add_extra_bonus("命座6：蕴思的霜草", 0.12);
        }

        let ratio = match s {
            A1 => AETHERDENDRO_SKILL.a_dmg1[s1],
            A2 => AETHERDENDRO_SKILL.a_dmg2[s1],
            A3 => AETHERDENDRO_SKILL.a_dmg3[s1],
            A4 => AETHERDENDRO_SKILL.a_dmg4[s1],
            A5 => AETHERDENDRO_SKILL.a_dmg5[s1],
            Z1 => AETHERDENDRO_SKILL.z_dmg1[s1],
            Z2 => AETHERDENDRO_SKILL.z_dmg2[s1],
            X1 => AETHERDENDRO_SKILL.x_dmg1[s1],
            X2 => AETHERDENDRO_SKILL.x_dmg2[s1],
            X3 => AETHERDENDRO_SKILL.x_dmg3[s1],
            E => AETHERDENDRO_SKILL.e_dmg[s2],
            QC => AETHERDENDRO_SKILL.q_dmgc[s3],
            QE => AETHERDENDRO_SKILL.q_dmge[s3],
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
        Some(Box::new(AetherDendroEffect {
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
