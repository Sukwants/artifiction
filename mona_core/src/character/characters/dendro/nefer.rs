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

pub struct ReferSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub a_dmg4: [f64; 15],
    pub z_dmg: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg_atk: [f64; 15],
    pub e_dmg_em: [f64; 15],
    pub e_dmg1_atk: [f64; 15],
    pub e_dmg1_em: [f64; 15],
    pub e_dmg2_atk: [f64; 15],
    pub e_dmg2_em: [f64; 15],
    pub e_dmgs1: [f64; 15],
    pub e_dmgs2: [f64; 15],
    pub e_dmgs3: [f64; 15],

    pub q_dmg1_atk: [f64; 15],
    pub q_dmg1_em: [f64; 15],
    pub q_dmg2_atk: [f64; 15],
    pub q_dmg2_em: [f64; 15],

    pub c6_dmg1: f64,
    pub c6_dmg2: f64,
}

pub const REFER_SKILL: ReferSkillType = ReferSkillType {
    // Normal Attack: Striking Serpent (values from provided page, converted to decimal)
    a_dmg1: [0.380712, 0.409265, 0.437819, 0.47589, 0.504443, 0.532997, 0.571068, 0.609139, 0.64721, 0.685282, 0.723353, 0.761424, 0.809013, 0.856602, 0.904191],
    a_dmg2: [0.37564, 0.403813, 0.431986, 0.46955, 0.497723, 0.525896, 0.56346, 0.601024, 0.638588, 0.676152, 0.713716, 0.75128, 0.798235, 0.84519, 0.892145],
    a_dmg3: [0.2524, 0.27133, 0.29026, 0.3155, 0.33443, 0.35336, 0.3786, 0.40384, 0.42908, 0.45432, 0.47956, 0.5048, 0.53635, 0.5679, 0.59945],
    a_dmg4: [0.609944, 0.65569, 0.701436, 0.76243, 0.808176, 0.853922, 0.914916, 0.97591, 1.036905, 1.097899, 1.158894, 1.219888, 1.296131, 1.372374, 1.448617],
    z_dmg: [1.3088, 1.40696, 1.50512, 1.636, 1.73416, 1.83232, 1.9632, 2.09408, 2.22496, 2.35584, 2.48672, 2.6176, 2.7812, 2.9448, 3.1084],
    x_dmg1: [0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064, 1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984],
    x_dmg2: [1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686, 2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036],
    x_dmg3: [1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632, 2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592],

    // Elemental Skill: Runo: Dawnless Rest of Karsikko
    e_dmg_atk: [0.764384, 0.821128, 0.878416, 0.9548, 1.012088, 1.069376, 1.14576, 1.222144, 1.298528, 1.374912, 1.451296, 1.52768, 1.62316, 1.71864, 1.81412],
    e_dmg_em: [1.52768, 1.642256, 1.756832, 1.9096, 2.024176, 2.138752, 2.29152, 2.444288, 2.597056, 2.749824, 2.902592, 3.05536, 3.24632, 3.43728, 3.62824],
    e_dmg1_atk: [0.2464, 0.26488, 0.28336, 0.308, 0.32648, 0.34496, 0.3696, 0.39424, 0.41888, 0.44352, 0.46816, 0.4928, 0.5236, 0.5544, 0.5852],
    e_dmg1_em: [0.4928, 0.52976, 0.56672, 0.616, 0.65296, 0.68992, 0.7392, 0.78848, 0.83776, 0.88704, 0.93632, 0.9856, 1.0472, 1.1088, 1.1704],
    e_dmg2_atk: [0.32032, 0.344344, 0.368368, 0.4004, 0.424424, 0.448448, 0.48048, 0.512512, 0.544544, 0.576576, 0.608608, 0.64064, 0.68068, 0.72072, 0.76076],
    e_dmg2_em: [0.64064, 0.688688, 0.737376, 0.8008, 0.849, 0.897696, 0.96096, 1.025024, 1.089088, 1.153152, 1.217216, 1.28128, 1.36136, 1.44144, 1.52152],
    e_dmgs1: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    e_dmgs2: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    e_dmgs3: [1.28, 1.376, 1.472, 1.6, 1.696, 1.792, 1.92, 2.048, 2.176, 2.304, 2.432, 2.56, 2.72, 2.88, 3.04],

    // Elemental Burst: Runo: All Hearts Become the Beating Moon
    q_dmg1_atk: [2.2464, 2.41488, 2.58336, 2.808, 2.97648, 3.14496, 3.3696, 3.59424, 3.81888, 4.04352, 4.26816, 4.4928, 4.7736, 5.0544, 5.3352],
    q_dmg1_em: [4.4928, 4.82976, 5.16672, 5.616, 5.95296, 6.28992, 6.7392, 7.18848, 7.63776, 8.08704, 8.53632, 8.9856, 9.5472, 10.1088, 10.6704],
    q_dmg2_atk: [3.3696, 3.62232, 3.87504, 4.212, 4.46472, 4.71744, 5.0544, 5.39136, 5.72832, 6.06528, 6.40224, 6.7392, 7.1604, 7.5816, 8.0028],
    q_dmg2_em: [6.7392, 7.24464, 7.75008, 8.424, 8.92944, 9.43488, 10.1088, 10.78272, 11.45664, 12.13056, 12.80448, 13.4784, 14.3208, 15.1632, 16.0056],

    c6_dmg1: 0.85,
    c6_dmg2: 1.2,
};

pub const REFER_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Nefer,
    internal_name: "Nefer",
    element: Element::Dendro,
    hp: [989, 2565, 3413, 5107, 5710, 6569, 7373, 8241, 8844, 9720, 10322, 11208, 11811, 12704, 12704],
    atk: [27, 70, 93, 138, 155, 178, 200, 223, 240, 264, 280, 304, 320, 344, 344],
    def: [62, 161, 215, 321, 359, 413, 464, 519, 556, 612, 649, 705, 743, 799, 799],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Catalyst,
    star: 5,
    skill_name1: locale!(
        zh_cn: "游虵吐信",
        en: "Striking Serpent",
    ),
    skill_name2: locale!(
        zh_cn: "弈术·千夜一舞",
        en: "Senet Strategy: Dance of a Thousand Nights",
    ),
    skill_name3: locale!(
        zh_cn: "圣约·真眸幻戏",
        en: "Sacred Vow: True Eye's Phantasm",
    ),
    name_locale: locale!(
        zh_cn: "奈芙尔",
        en: "Nefer",
    )
};

pub struct ReferEffect {
    pub has_c6: bool,
    pub moonsign: Moonsign,
}

impl<A: Attribute> ChangeAttribute<A> for ReferEffect {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "初始精通", 100.0);

        attribute.add_edge1(
            AttributeName::ElementalMastery,
            AttributeName::IncreaseLunarBloom,
            Box::new(move |em: f64, _| {
                (em * 0.000175).min(0.14)
            }),
            Box::new(move |atk, _, grad| (0.0, 0.0)),
            "天赋：月兆祝赐·廊下暮影"
        );

        if self.has_c6 && self.moonsign.is_ascendant() {
            attribute.set_value_by(AttributeName::IncreaseLunarBloom, "六命：决胜于逆转之时", 0.15);
        }
    }
}

damage_enum!(
    ReferDamageEnum
    A1
    A2
    A3
    A4
    Z
    X1
    X2
    X3
    E
    E1
    E2
    ES1
    ES2
    ES3
    Q1
    Q2
    C61
    C62
);

impl ReferDamageEnum {
    pub fn get_element(&self) -> Element {
        use ReferDamageEnum::*;
        Element::Dendro
    }

    pub fn get_lunar_type(&self) -> MoonglareReaction {
        use ReferDamageEnum::*;
        match *self {
            ES1 | ES2 | ES3 | C61 | C62 => MoonglareReaction::LunarBloom,
            _ => MoonglareReaction::None,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use ReferDamageEnum::*;
        match *self {
            A1 | A2 | A3 | A4 => SkillType::NormalAttack,
            Z => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E | E1 | E2 => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst,
            ES1 | ES2 | ES3 | C61 | C62 => SkillType::Moonglare,
        }
    }
}

pub struct Nefer;

impl CharacterTrait for Nefer {
    const STATIC_DATA: CharacterStaticData = REFER_STATIC_DATA;
    type SkillType = ReferSkillType;
    const SKILL: Self::SkillType = REFER_SKILL;
    type DamageEnumType = ReferDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ReferDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            A4 hit_n_dmg!(4)
            Z charged_dmg!()
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            ReferDamageEnum
            E locale!(zh_cn: "点按伤害", en: "Press DMG")
            E1 locale!(zh_cn: "幻戏自身一段伤害", en: "Phantasm Performance 1-Hit DMG (Nefer)")
            E2 locale!(zh_cn: "幻戏自身二段伤害", en: "Phantasm Performance 2-Hit DMG (Nefer)")
            ES1 locale!(zh_cn: "幻戏虚影一段", en: "Phantasm Performance 1-Hit DMG (Shade)")
            ES2 locale!(zh_cn: "幻戏虚影二段", en: "Phantasm Performance 2-Hit DMG (Shade)")
            ES3 locale!(zh_cn: "幻戏虚影三段", en: "Phantasm Performance 3-Hit DMG (Shade)")
            C61 locale!(zh_cn: "六命二段替换伤害", en: "C6 2-Hit Replacement DMG")
            C62 locale!(zh_cn: "六命额外伤害", en: "C6 Extra DMG")
        ),
        skill3: skill_map!(
            ReferDamageEnum
            Q1 hit_n_dmg!(1)
            Q2 hit_n_dmg!(2)
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_CHARACTER),
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "veil_of_falsehood",
            title: locale!(
                zh_cn: "「伪秘之帷」层数",
                en: "Veil of Falsehood count"
            ),
            config: ItemConfigType::Int { min: 0, max: 5, default: 3 }
        },
        ItemConfig {
            name: "shadow_dance",
            title: locale!(
                zh_cn: "「影舞」状态",
                en: "Shadow Dance state"
            ),
            config: ItemConfigType::Bool { default: true }
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ReferDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (veil_of_falsehood, shadow_dance) = match *config {
            CharacterSkillConfig::Nefer { veil_of_falsehood, shadow_dance } => (veil_of_falsehood, shadow_dance),
            _ => (0, false)
        };

        use ReferDamageEnum::*;
        let mut builder = D::new();

        if context.character_common_data.constellation >= 4 && shadow_dance {
            builder.add_extra_res_minus("四命：眩惑入谜局之网", 0.2);
        }

        if context.character_common_data.has_talent1 {
            if veil_of_falsehood >= 5 && context.character_common_data.constellation >= 2 {
                builder.add_extra_em("二命：明察为筹算之先", 200.0);
            } else if veil_of_falsehood >= 3{
                builder.add_extra_em("天赋：月下的豪赌", 100.0);
            }
        }

        if s.get_skill_type() == SkillType::Moonglare {
            let ratio = (match s {
                ES1 => REFER_SKILL.e_dmgs1[s2],
                ES2 => REFER_SKILL.e_dmgs2[s2],
                ES3 => REFER_SKILL.e_dmgs3[s2],
                C61 => REFER_SKILL.c6_dmg1,
                C62 => REFER_SKILL.c6_dmg2,
                _ => 0.0
            }
                + if context.character_common_data.constellation >= 1 { 0.6 } else { 0.0 })
                * (1.0 + veil_of_falsehood as f64 * 0.08).min(1.4);

            builder.add_em_ratio("技能倍率", ratio);

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
            let atk_ratio = match s {
                A1 => REFER_SKILL.a_dmg1[s1],
                A2 => REFER_SKILL.a_dmg2[s1],
                A3 => REFER_SKILL.a_dmg3[s1],
                Z => REFER_SKILL.z_dmg[s1],
                X1 => REFER_SKILL.x_dmg1[s1],
                X2 => REFER_SKILL.x_dmg2[s1],
                X3 => REFER_SKILL.x_dmg3[s1],
                E => REFER_SKILL.e_dmg_atk[s2],
                E1 => REFER_SKILL.e_dmg1_atk[s2],
                E2 => REFER_SKILL.e_dmg2_atk[s2],
                Q1 => REFER_SKILL.q_dmg1_atk[s3],
                Q2 => REFER_SKILL.q_dmg2_atk[s3],
                _ => 0.0
            };

            let em_ratio = match s {
                E => REFER_SKILL.e_dmg_em[s2],
                E1 => REFER_SKILL.e_dmg1_em[s2],
                E2 => REFER_SKILL.e_dmg2_em[s2],
                Q1 => REFER_SKILL.q_dmg1_em[s3],
                Q2 => REFER_SKILL.q_dmg2_em[s3],
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", atk_ratio);
            if em_ratio > 0.0 {
                builder.add_em_ratio("技能倍率", em_ratio);
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
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let moonsign = match config {
            CharacterConfig::Nefer { moonsign } => *moonsign,
            _ => Moonsign::None,
        };
        Some(Box::new(ReferEffect {
            has_c6: common_data.constellation >= 6,
            moonsign: moonsign,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
