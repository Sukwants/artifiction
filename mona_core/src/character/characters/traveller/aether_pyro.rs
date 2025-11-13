use crate::attribute::{self, Attribute, AttributeCommon, AttributeName};
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

pub struct AetherPyroSkillType {
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

    pub e_dmg1: [f64; 15], // Blazing Threshold
    pub e_dmgh: [f64; 15], // Hold DMG
    pub e_dmg2: [f64; 15], // Scorching Threshold

    pub q_dmg: [f64; 15],

    pub c6_dmg: f64,
}

pub const AETHERPYRO_SKILL: AetherPyroSkillType = AetherPyroSkillType {
    // Normal Attack: Foreign Blaze
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

    // Elemental Skill: Flowfire Blade
    e_dmg1: [0.2808, 0.30186, 0.32292, 0.351, 0.37206, 0.39312, 0.4212, 0.44928, 0.47736, 0.50544, 0.53352, 0.5616, 0.5967, 0.6318, 0.6669],
    e_dmgh: [0.988, 1.0621, 1.1362, 1.235, 1.3091, 1.3832, 1.482, 1.5808, 1.6796, 1.7784, 1.8772, 1.976, 2.0995, 2.223, 2.3465],
    e_dmg2: [0.8144, 0.87548, 0.93656, 1.018, 1.07908, 1.14016, 1.2216, 1.30304, 1.38448, 1.46592, 1.54736, 1.6288, 1.7306, 1.8324, 1.9342],

    // Elemental Burst: Plains Scorcher
    q_dmg: [4.272, 4.5924, 4.9128, 5.34, 5.6604, 5.9808, 6.408, 6.8352, 7.2624, 7.6896, 8.1168, 8.544, 9.078, 9.612, 10.146],

    c6_dmg: 0.96,
};

pub const AETHERPYRO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::AetherPyro,
    internal_name: "AetherPyro",
    element: Element::Pyro,
    hp: [912, 2342, 3024, 4529, 5031, 5766, 6411, 7164, 7648, 8401, 8885, 9638, 10122, 10875, 11627],
    atk: [18, 46, 59, 88, 98, 113, 125, 140, 149, 164, 174, 188, 198, 212, 266],
    def: [57, 147, 190, 284, 315, 362, 402, 450, 480, 527, 558, 605, 635, 683, 730],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "异邦烈焰",
        en: "Foreign Blaze",
    ),
    skill_name2: locale!(
        zh_cn: "流火剑",
        en: "Flowfire Blade",
    ),
    skill_name3: locale!(
        zh_cn: "灼火燎原",
        en: "Plains Scorcher",
    ),
    name_locale: locale!(
        zh_cn: "空-火",
        en: "Aether-Pyro",
    )
};

pub struct AetherPyroEffect {
}

impl<A: Attribute> ChangeAttribute<A> for AetherPyroEffect {
    fn change_attribute(&self, attribute: &mut A) {
    }
}

damage_enum!(
    AetherPyroDamageEnum
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
    E1
    EH
    E2
    Q
    C6
);

impl AetherPyroDamageEnum {
    pub fn get_element(&self, fumo: bool) -> Element {
        use AetherPyroDamageEnum::*;
        match *self {
            E1 | EH | E2 | Q | C6 => Element::Pyro,
            A1 | A2 | A3 | A4 | A5 | Z1 | Z2 | X1 | X2 | X3 => if fumo { Element::Pyro } else { Element::Physical },
            _ => Element::Physical,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use AetherPyroDamageEnum::*;
        match *self {
            A1 | A2 | A3 | A4 | A5 | C6 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E1 | EH | E2 => SkillType::ElementalSkill,
            Q => SkillType::ElementalBurst,
        }
    }
}

pub struct AetherPyro;

impl CharacterTrait for AetherPyro {
    const STATIC_DATA: CharacterStaticData = AETHERPYRO_STATIC_DATA;
    type SkillType = AetherPyroSkillType;
    const SKILL: Self::SkillType = AETHERPYRO_SKILL;
    type DamageEnumType = AetherPyroDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            AetherPyroDamageEnum
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
            AetherPyroDamageEnum
            E1 locale!(zh_cn: "焰烈之槛伤害", en: "Blazing Threshold")
            EH locale!(zh_cn: "长按伤害", en: "Hold DMG")
            E2 locale!(zh_cn: "灼火之槛伤害", en: "Scorching Threshold")
        ),
        skill3: skill_map!(
            AetherPyroDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "gosoythoth",
            title: locale!(
                zh_cn: "对抗「古斯托特」化形的蚀灭的源焰之主",
                en: "Opposing the Lord of Eroded Primal Fire incarnated by Gosoythoth",
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "nightsouls_blessing",
            title: locale!(
                zh_cn: "夜魂加持状态",
                en: "Nightsoul's Blessing state",
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "active",
            title: locale!(
                zh_cn: "位于场上",
                en: "Current active",
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "activated_q",
            title: locale!(
                zh_cn: "已施放元素爆发",
                en: "Elemental Burst activated",
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: AetherPyroDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (gosoythoth, nightsouls_blessing, active, activated_q) = match *config {
            CharacterSkillConfig::AetherPyro { gosoythoth, nightsouls_blessing, active, activated_q } => (gosoythoth, nightsouls_blessing, active, activated_q),
            _ => (false, false, false, false)
        };

        use AetherPyroDamageEnum::*;
        let mut builder = D::new();

        if context.character_common_data.constellation >= 1 && active {
            builder.add_extra_bonus("命座1：流光的星火", if nightsouls_blessing { 0.15 } else { 0.06 });
        }

        if context.character_common_data.constellation >= 4 && activated_q {
            builder.add_extra_bonus("命座4：燎灼的烈火", 0.2);
        }

        let infused = context.character_common_data.constellation >= 6 && nightsouls_blessing;

        if infused && matches!(s.get_skill_type(), SkillType::NormalAttack | SkillType::ChargedAttack | SkillType::PlungingAttackInAction | SkillType::PlungingAttackOnGround) {
            builder.add_extra_critical_damage("命座6：永燃的圣火", 0.4);
        }

        if gosoythoth {
            if context.character_common_data.constellation >= 1 {
                builder.add_extra_atk("命座1：流光的星火", context.attribute.get_value(AttributeName::ATKBase) * 0.4);
            }

            if context.character_common_data.constellation >= 2 && nightsouls_blessing {
                builder.add_extra_res_minus("命座2：长明的烛火", 0.4);
            }

            if context.character_common_data.constellation >= 3 {
                builder.add_extra_bonus("命座3：燧传的烽火", 0.4);
            }

            if context.character_common_data.constellation >= 5 {
                builder.add_extra_critical("命座5：不灭的薪火", 0.2);
                builder.add_extra_critical_damage("命座5：不灭的薪火", 0.4);
            }
        }

        let ratio = match s {
            A1 => AETHERPYRO_SKILL.a_dmg1[s1],
            A2 => AETHERPYRO_SKILL.a_dmg2[s1],
            A3 => AETHERPYRO_SKILL.a_dmg3[s1],
            A4 => AETHERPYRO_SKILL.a_dmg4[s1],
            A5 => AETHERPYRO_SKILL.a_dmg5[s1],
            Z1 => AETHERPYRO_SKILL.z_dmg1[s1],
            Z2 => AETHERPYRO_SKILL.z_dmg2[s1],
            X1 => AETHERPYRO_SKILL.x_dmg1[s1],
            X2 => AETHERPYRO_SKILL.x_dmg2[s1],
            X3 => AETHERPYRO_SKILL.x_dmg3[s1],
            E1 => AETHERPYRO_SKILL.e_dmg1[s2],
            EH => AETHERPYRO_SKILL.e_dmgh[s2],
            E2 => AETHERPYRO_SKILL.e_dmg2[s2],
            Q => AETHERPYRO_SKILL.q_dmg[s3],
            C6 => AETHERPYRO_SKILL.c6_dmg,
            _ => 0.0
        };

        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(infused),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        Some(Box::new(AetherPyroEffect {
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
