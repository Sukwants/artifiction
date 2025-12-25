use num_traits::FromPrimitive;
use crate::attribute::*;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::macros::{damage_enum, skill_map};
use crate::common::{ChangeAttribute, Element, MoonglareReaction, ReactionType, Moonsign, SkillType, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::damage::reaction::Reaction;
// use crate::target_functions::target_functions::LaumaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct LaumaSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub z_spiritcall_prayer_dmg: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg: [f64; 15],
    pub e_hold_dmg1: [f64; 15],
    pub e_hold_dmg2: [f64; 15],
    pub e_frostgrove_sanctuary_dmg_atk: [f64; 15],
    pub e_frostgrove_sanctuary_dmg_em: [f64; 15],
    pub e_res: [f64; 15],

    pub q_bloom_increase: [f64; 15],
    pub q_lunar_bloom_increase: [f64; 15],

    pub c2_bloom_increase: f64,
    pub c2_lunar_bloom_increase: f64,
    pub c6_e_dmg: f64,
    pub c6_a_dmg: f64,
}

pub const LAUMA_SKILL: LaumaSkillType = LaumaSkillType {
    // Normal Attack: Peregrination of Linnunrata
    a_dmg1: [0.337024, 0.362301, 0.387578, 0.42128, 0.446557, 0.471834, 0.505536, 0.539238, 0.572941, 0.606643, 0.640346, 0.674048, 0.716176, 0.758304, 0.800432],
    a_dmg2: [0.318048, 0.341902, 0.365755, 0.39756, 0.421414, 0.445267, 0.477072, 0.508877, 0.540682, 0.572486, 0.604291, 0.636096, 0.675852, 0.715608, 0.755364],
    a_dmg3: [0.444968, 0.478341, 0.511713, 0.55621, 0.589583, 0.622955, 0.667452, 0.711949, 0.756446, 0.800942, 0.845439, 0.889936, 0.945557, 1.001178, 1.056799],
    z_spiritcall_prayer_dmg: [1.2904, 1.38718, 1.48396, 1.613, 1.70978, 1.80656, 1.9356, 2.06464, 2.19368, 2.32272, 2.45176, 2.5808, 2.7421, 2.9034, 3.0647],
    x_dmg1: [0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064, 1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984],
    x_dmg2: [1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686, 2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036],
    x_dmg3: [1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632, 2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592],

    // Elemental Skill: Runo: Dawnless Rest of Karsikko
    e_dmg: [1.216, 1.3072, 1.3984, 1.52, 1.6112, 1.7024, 1.824, 1.9456, 2.0672, 2.1888, 2.3104, 2.432, 2.584, 2.736, 2.888],
    e_hold_dmg1: [1.5808, 1.69936, 1.81792, 1.976, 2.09456, 2.21312, 2.3712, 2.52928, 2.68736, 2.84544, 3.00352, 3.1616, 3.3592, 3.5568, 3.7544],
    e_hold_dmg2: [1.52, 1.634, 1.748, 1.9, 2.014, 2.128, 2.28, 2.432, 2.584, 2.736, 2.888, 3.04, 3.23, 3.42, 3.61],
    e_frostgrove_sanctuary_dmg_atk: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    e_frostgrove_sanctuary_dmg_em: [1.92, 2.064, 2.208, 2.4, 2.544, 2.688, 2.88, 3.072, 3.264, 3.456, 3.648, 3.84, 4.08, 4.32, 4.56],
    e_res: [0.025, 0.05, 0.075, 0.1, 0.125, 0.15, 0.175, 0.2, 0.225, 0.25, 0.28, 0.31, 0.34, 0.37, 0.4],

    // Elemental Burst: Runo: All Hearts Become the Beating Moon
    q_bloom_increase: [2.7776, 2.98592, 3.19424, 3.472, 3.68032, 3.88864, 4.1664, 4.44416, 4.72192, 4.99968, 5.27744, 5.5552, 5.9024, 6.2496, 6.5968],
    q_lunar_bloom_increase: [2.2224, 2.38908, 2.55576, 2.778, 2.94468, 3.11136, 3.3336, 3.55584, 3.77808, 4.00032, 4.22256, 4.4448, 4.7226, 5.0004, 5.2782],

    c2_bloom_increase: 5.0,
    c2_lunar_bloom_increase: 4.0,
    c6_e_dmg: 1.85,
    c6_a_dmg: 1.5,
};

pub const LAUMA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Lauma,
    internal_name: "Lauma",
    element: Element::Dendro,
    hp: [829, 2151, 2863, 4283, 4789, 5509, 6183, 6911, 7416, 8151, 8657, 9400, 9905, 10654, 11411],
    atk: [20, 51, 69, 103, 115, 132, 148, 165, 177, 195, 207, 225, 237, 255, 313],
    def: [52, 135, 180, 269, 301, 346, 388, 434, 465, 512, 543, 590, 622, 669, 716],
    sub_stat: CharacterSubStatFamily::ElementalMastery115,
    weapon_type: WeaponType::Catalyst,
    star: 5,
    skill_name1: locale!(
        zh_cn: "林麓旅踏",
        en: "Peregrination of Linnunrata",
    ),
    skill_name2: locale!(
        zh_cn: "圣言述咏·终宵永眠",
        en: "Runo: Dawnless Rest of Karsikko",
    ),
    skill_name3: locale!(
        zh_cn: "圣言述咏·众心为月",
        en: "Runo: All Hearts Become the Beating Moon",
    ),
    name_locale: locale!(
        zh_cn: "菈乌玛",
        en: "Lauma",
    )
};

pub struct LaumaEffect {
    pub activated_q: bool,
    pub activated_res: bool,
    pub level_e: usize,
    pub level_q: usize,
    pub has_p1: bool,
    pub has_p2: bool,
    pub has_c2: bool,
    pub has_c6: bool,
    pub moonsign: Moonsign,
}

impl<A: Attribute> ChangeAttribute<A> for LaumaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "初始精通", 200.0);

        if self.has_p1 {
            if self.moonsign == Moonsign::Nascent {
                attribute.set_value_to_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalDamage, None, None, Some(ReactionType::Bloom),
                )), "天赋：奉向霜夜的明光", 1.0);
                attribute.set_value_to_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalDamage, None, None, Some(ReactionType::Hyperbloom),
                )), "天赋：奉向霜夜的明光", 1.0);
                attribute.set_value_to_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalDamage, None, None, Some(ReactionType::Burgeon),
                )), "天赋：奉向霜夜的明光", 1.0);

                attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalRate, None, None, Some(ReactionType::Bloom),
                )), "天赋：奉向霜夜的明光", 0.15);
                attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalRate, None, None, Some(ReactionType::Hyperbloom),
                )), "天赋：奉向霜夜的明光", 0.15);
                attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalRate, None, None, Some(ReactionType::Burgeon),
                )), "天赋：奉向霜夜的明光", 0.15);
            } else if self.moonsign == Moonsign::Ascendant {
                attribute.set_value_by(AttributeName::CriticalDamageLunarBloom, "天赋：奉向霜夜的明光", 0.2);

                attribute.set_value_by(AttributeName::CriticalLunarBloom, "天赋：奉向霜夜的明光", 0.1);
            }
        }

        if self.has_p2 {
            attribute.add_edge1(
                AttributeName::ElementalMastery,
                AttributeName::BonusElementalSkill,
                Box::new(move |em, _| {
                    (em * 0.0004).min(0.32)
                }),
                Box::new(move |em, _, grad| (0.0, 0.0)),
                "天赋：奉向甘泉的沐濯"
            );
        }

        attribute.add_edge1(
            AttributeName::ElementalMastery,
            AttributeName::IncreaseLunarBloom,
            Box::new(move |em: f64, _| {
                (em * 0.000175).min(0.14)
            }),
            Box::new(move |atk, _, grad| (0.0, 0.0)),
            "天赋：月兆祝赐·千籁恩宠"
        );

        if self.activated_q {
            let q_bloom_increase = LAUMA_SKILL.q_bloom_increase[self.level_q] + if self.has_c2 { LAUMA_SKILL.c2_bloom_increase } else { 0.0 };
            let q_lunar_bloom_increase = LAUMA_SKILL.q_lunar_bloom_increase[self.level_q] + if self.has_c2 { LAUMA_SKILL.c2_lunar_bloom_increase } else { 0.0 };

            attribute.add_edge1(
                AttributeName::ElementalMastery,
                AttributeName::ExtraIncreaseBloom,
                Box::new(move |em, _| {
                    em * q_bloom_increase
                }),
                Box::new(move |em, _, grad| (0.0, 0.0)),
                "元素爆发：圣言述咏·众心为月"
            );
            attribute.add_edge1(
                AttributeName::ElementalMastery,
                AttributeName::ExtraIncreaseHyperBloom,
                Box::new(move |em, _| {
                    em * q_bloom_increase
                }),
                Box::new(move |em, _, grad| (0.0, 0.0)),
                "元素爆发：圣言述咏·众心为月"
            );
            attribute.add_edge1(
                AttributeName::ElementalMastery,
                AttributeName::ExtraIncreaseBurgeon,
                Box::new(move |em, _| {
                    em * q_bloom_increase
                }),
                Box::new(move |em, _, grad| (0.0, 0.0)),
                "元素爆发：圣言述咏·众心为月"
            );
            attribute.add_edge1(
                AttributeName::ElementalMastery,
                AttributeName::ExtraIncreaseLunarBloom,
                Box::new(move |em, _| {
                    em * q_lunar_bloom_increase
                }),
                Box::new(move |em, _, grad| (0.0, 0.0)),
                "元素爆发：圣言述咏·众心为月"
            );
        }

        if self.activated_res {
            let e_res = LAUMA_SKILL.e_res[self.level_e];
            attribute.set_value_by(AttributeName::ResMinusDendro, "元素战技：圣言述咏·终宵永眠", e_res);
            attribute.set_value_by(AttributeName::ResMinusHydro, "元素战技：圣言述咏·终宵永眠", e_res);
        }

        if self.has_c6 && self.moonsign.is_ascendant() {
            attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new(
                AttributeVariableType::MoonglareElevate,
                None,
                None,
                Some(ReactionType::LunarBloom),
            )), "六命擢升", 0.25);
        }
    }
}

damage_enum!(
    LaumaDamageEnum
    A1
    A2
    A3
    ZSpiritcallPrayer
    X1
    X2
    X3
    E
    EHold1
    EHold2
    EFrostgroveSanctuary
    // QBloomIncrease
    // QLunarBloomIncrease
    C6E
    C6A
);

impl LaumaDamageEnum {
    pub fn get_element(&self) -> Element {
        use LaumaDamageEnum::*;
        Element::Dendro
    }

    pub fn get_lunar_type(&self) -> MoonglareReaction {
        use LaumaDamageEnum::*;
        match *self {
            EHold2 | C6E | C6A => MoonglareReaction::LunarBloom,
            _ => MoonglareReaction::None,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use LaumaDamageEnum::*;
        match *self {
            A1 | A2 | A3 => SkillType::NormalAttack,
            ZSpiritcallPrayer => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E | EHold1 | EFrostgroveSanctuary => SkillType::ElementalSkill,
            // QBloomIncrease | QLunarBloomIncrease => SkillType::ElementalBurst,
            EHold2 | C6E | C6A => SkillType::Moonglare,
        }
    }
}

pub struct Lauma;

impl CharacterTrait for Lauma {
    const STATIC_DATA: CharacterStaticData = LAUMA_STATIC_DATA;
    type SkillType = LaumaSkillType;
    const SKILL: Self::SkillType = LAUMA_SKILL;
    type DamageEnumType = LaumaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LaumaDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            ZSpiritcallPrayer locale!(zh_cn: "唤灵之祷伤害", en: "Spiritcall Prayer DMG")
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
            C6A locale!(zh_cn: "六命普通攻击伤害", en: "C6 Normal Attack DMG")
        ),
        skill2: skill_map!(
            LaumaDamageEnum
            E locale!(zh_cn: "点按伤害", en: "Press DMG")
            EHold1 locale!(zh_cn: "长按一段伤害", en: "1-Hit Hold DMG")
            EHold2 locale!(zh_cn: "长按二段伤害", en: "2-Hit Hold DMG")
            EFrostgroveSanctuary locale!(zh_cn: "霜林圣域攻击伤害", en: "Frostgrove Sanctuary Attack DMG")
            C6E locale!(zh_cn: "六命霜林圣域额外伤害", en: "C6 Extra Frostgrove Sanctuary Attack DMG")
        ),
        skill3: skill_map!(
            LaumaDamageEnum
            // QBloomIncrease locale!(zh_cn: "绽放、超绽放、烈绽放反应伤害提升", en: "Bloom, Hyperbloom, and Burgeon DMG Increase")
            // QLunarBloomIncrease locale!(zh_cn: "月绽放反应伤害提升", en: "Lunar-Bloom DMG Increase")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "activated_q",
            title: locale!(
                zh_cn: "苍色祷歌",
                en: "Pale Hymn",
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "activated_res",
            title: locale!(
                zh_cn: "圣言述咏·终宵永眠：减抗",
                en: "Runo: Dawnless Rest of Karsikko - Res Down",
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_CHARACTER),
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: LaumaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use LaumaDamageEnum::*;
        let mut builder = D::new();

        if s.get_skill_type() == SkillType::Moonglare {
            let ratio = match s {
                EHold2 => LAUMA_SKILL.e_hold_dmg2[s2],
                C6E => LAUMA_SKILL.c6_e_dmg,
                C6A => LAUMA_SKILL.c6_a_dmg,
                _ => 0.0
            };

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
            if s == EFrostgroveSanctuary {
                builder.add_atk_ratio("技能攻击倍率", LAUMA_SKILL.e_frostgrove_sanctuary_dmg_atk[s2]);
                builder.add_em_ratio("技能精通倍率", LAUMA_SKILL.e_frostgrove_sanctuary_dmg_em[s2]);
            // } else if s == QBloomIncrease || s == QLunarBloomIncrease {
            //     let ratio = match s {
            //         QBloomIncrease => LAUMA_SKILL.q_bloom_increase[s1],
            //         QLunarBloomIncrease => LAUMA_SKILL.q_lunar_bloom_increase[s1],
            //         _ => 0.0
            //     };
    
            //     builder.add_em_ratio("技能倍率", ratio);
    
            //     if context.character_common_data.constellation >= 2 {
            //         let extra_ratio = match s {
            //             QBloomIncrease => LAUMA_SKILL.c2_bloom_increase,
            //             QLunarBloomIncrease => LAUMA_SKILL.c2_lunar_bloom_increase,
            //             _ => 0.0
            //         };
    
            //         builder.add_em_ratio("二命额外倍率", extra_ratio);
            //     }
            } else {
                let ratio = match s {
                    A1 => LAUMA_SKILL.a_dmg1[s1],
                    A2 => LAUMA_SKILL.a_dmg2[s1],
                    A3 => LAUMA_SKILL.a_dmg3[s1],
                    ZSpiritcallPrayer => LAUMA_SKILL.z_spiritcall_prayer_dmg[s1],
                    X1 => LAUMA_SKILL.x_dmg1[s1],
                    X2 => LAUMA_SKILL.x_dmg2[s1],
                    X3 => LAUMA_SKILL.x_dmg3[s1],
                    E => LAUMA_SKILL.e_dmg[s2],
                    EHold1 => LAUMA_SKILL.e_hold_dmg1[s2],
                    _ => 0.0
                };
    
                builder.add_atk_ratio("技能倍率", ratio);
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
        let (activated_q, activated_res, moonsign) = match config {
            CharacterConfig::Lauma { activated_q, activated_res, moonsign } => (
                *activated_q,
                *activated_res,
                *moonsign,
            ),
            _ => (false, false, Moonsign::None),
        };
        Some(Box::new(LaumaEffect {
            activated_q: activated_q,
            activated_res: activated_res,
            level_e: common_data.skill2,
            level_q: common_data.skill3,
            has_p1: common_data.has_talent1,
            has_p2: common_data.has_talent2,
            has_c2: common_data.constellation >= 2,
            has_c6: common_data.constellation >= 6,
            moonsign: moonsign,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
