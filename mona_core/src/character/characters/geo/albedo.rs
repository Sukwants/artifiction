use num_traits::FromPrimitive;
use crate::attribute::*;
use crate::buffs::buffs::common;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::macros::{damage_enum, skill_map};
use crate::common::{ChangeAttribute, DamageResult, Element, MoonglareReaction, Moonsign, SkillType, StatName, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct AlbedoSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub normal_dmg4: [f64; 15],
    pub normal_dmg5: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub charged_dmg2: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],
    pub elemental_burst_dmg2: [f64; 15],

    pub c2_dmg: f64,
}

const ALBEDO_SKILL: AlbedoSkillType = AlbedoSkillType {
    normal_dmg1: [0.3674, 0.3973, 0.4272, 0.4699, 0.4998, 0.534, 0.581, 0.628, 0.675, 0.7262, 0.785, 0.8541, 0.9231, 0.9922, 1.0676],
    normal_dmg2: [0.3674, 0.3973, 0.4272, 0.4699, 0.4998, 0.534, 0.581, 0.628, 0.675, 0.7262, 0.785, 0.8541, 0.9231, 0.9922, 1.0676],
    normal_dmg3: [0.4745, 0.5132, 0.5518, 0.607, 0.6456, 0.6898, 0.7504, 0.8111, 0.8718, 0.9381, 1.0139, 1.1032, 1.1924, 1.2816, 1.3789],
    normal_dmg4: [0.4975, 0.538, 0.5785, 0.6364, 0.6768, 0.7231, 0.7868, 0.8504, 0.914, 0.9835, 1.063, 1.1565, 1.2501, 1.3436, 1.4457],
    normal_dmg5: [0.6207, 0.6713, 0.7218, 0.794, 0.8445, 0.9022, 0.9816, 1.061, 1.1404, 1.227, 1.3263, 1.443, 1.5597, 1.6764, 1.8038],
    charged_dmg1: [0.473, 0.5115, 0.55, 0.605, 0.6435, 0.6875, 0.748, 0.8085, 0.869, 0.935, 1.0106, 1.0996, 1.1885, 1.2774, 1.3745],
    charged_dmg2: [0.602, 0.651, 0.7, 0.77, 0.819, 0.875, 0.952, 1.029, 1.106, 1.19, 1.2862, 1.3994, 1.5126, 1.6258, 1.7493],
    plunging_dmg1: [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.011, 1.0928, 1.1746, 1.2638, 1.353, 1.4422, 1.5314, 1.6206, 1.7098],
    plunging_dmg2: [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.527, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189],
    plunging_dmg3: [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.602, 3.8248, 4.0476, 4.2704],
    elemental_skill_dmg1: [1.304, 1.4018, 1.4996, 1.63, 1.7278, 1.8256, 1.956, 2.0864, 2.2168, 2.3472, 2.4776, 2.608, 2.771, 2.934, 3.097],
    elemental_skill_dmg2: [1.336, 1.4362, 1.5364, 1.67, 1.7702, 1.8704, 2.004, 2.1376, 2.2712, 2.4048, 2.5384, 2.672, 2.839, 3.006, 3.173],
    elemental_burst_dmg1: [3.672, 3.9474, 4.2228, 4.59, 4.8654, 5.1408, 5.508, 5.8752, 6.2424, 6.6096, 6.9768, 7.344, 7.803, 8.262, 8.721],
    elemental_burst_dmg2: [0.72, 0.774, 0.828, 0.9, 0.954, 1.008, 1.08, 1.152, 1.224, 1.296, 1.368, 1.44, 1.53, 1.62, 1.71],

    c2_dmg: 3.0,
};

const ALBEDO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Albedo,
    internal_name: "Albedo",
    element: Element::Geo,
    hp: [1030, 2671, 3554, 5317, 5944, 6839, 7675, 8579, 9207, 10119, 10746, 11669, 12296, 13226, 14166],
    atk: [20, 51, 68, 101, 113, 130, 146, 163, 175, 192, 204, 222, 233, 251, 308],
    def: [68, 177, 235, 352, 394, 453, 508, 568, 610, 670, 712, 773, 815, 876, 938],
    sub_stat: CharacterSubStatFamily::Bonus288(StatName::GeoBonus),
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "西风剑术·白",
        en: "Normal Attack: Favonius Bladework - Weiss",
    ),
    skill_name2: locale!(
        zh_cn: "创生法·拟造阳华",
        en: "Abiogenesis: Solar Isotoma",
    ),
    skill_name3: locale!(
        zh_cn: "诞生式·大地之潮",
        en: "Rite of Progeniture: Tectonic Tide",
    ),
    name_locale: locale!(
        zh_cn: "阿贝多",
        en: "Albedo",
    )
};

pub struct AlbedoEffect {
    pub hexerei_secret_rite: bool,
    pub is_hexerei: bool,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for AlbedoEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.hexerei_secret_rite {
            let mut add_effect = |at: AttributeName| {
                attribute.add_edge1(
                    AttributeName::DEF,
                    AttributeName::BonusNormalAttack,
                    if self.is_hexerei {
                        Box::new(move |def, _| { (def / 1000.0 * 0.14).min(0.42) })
                    } else {
                        Box::new(move |def, _| { (def / 1000.0 * 0.04).min(0.12) })
                    },
                    Box::new(move |def, _, grad| (0.0, 0.0)),
                    "天赋3：魔女的前夜礼·白芒之书"
                );
            };
            add_effect(AttributeName::BonusNormalAttack);
            add_effect(AttributeName::BonusChargedAttack);
            add_effect(AttributeName::BonusPlungingAttack);
            add_effect(AttributeName::BonusElementalSkill);
            add_effect(AttributeName::BonusElementalBurst);
        }

        if self.common_data.constellation >= 1 {
            attribute.add_def_percentage("命座1：伊甸之花", 0.5);
        }

        if self.common_data.constellation >= 4 {
            if self.hexerei_secret_rite {
                attribute.set_value_by(AttributeName::BonusPlungingAttack, "命座4：神性之陨", 0.6);
            } else {
                attribute.set_value_by(AttributeName::BonusPlungingAttack, "命座4：神性之陨", 0.3);
            }
        }
    }
}

damage_enum!(
    AlbedoDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Normal5
    Charged11
    Charged12
    Plunging1
    Plunging2
    Plunging3
    E1
    ETransientBlossom
    Q1
    QFatalBlossom
    C2
);

impl AlbedoDamageEnum {
    pub fn is_def_ratio(&self) -> bool {
        use AlbedoDamageEnum::*;
        *self == ETransientBlossom || *self == C2
    }

    pub fn get_skill_type(&self) -> SkillType {
        use AlbedoDamageEnum::*;
        match *self {
            Charged11 | Charged12 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | ETransientBlossom => SkillType::ElementalSkill,
            Q1 | QFatalBlossom | C2 => SkillType::ElementalBurst,
            _ => SkillType::NormalAttack
        }
    }

    pub fn get_element(&self) -> Element {
        use AlbedoDamageEnum::*;
        match *self {
            E1 | ETransientBlossom | Q1 | QFatalBlossom | C2 => Element::Geo,
            _ => Element::Physical
        }
    }
}

pub struct Albedo;

#[derive(Copy, Clone, FromPrimitive)]
pub enum AlbedoRoleEnum {
    Sub
}

impl CharacterTrait for Albedo {
    const STATIC_DATA: CharacterStaticData = ALBEDO_STATIC_DATA;
    type SkillType = AlbedoSkillType;
    const SKILL: Self::SkillType = ALBEDO_SKILL;
    type DamageEnumType = AlbedoDamageEnum;
    type RoleEnum = AlbedoRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: AlbedoDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: AlbedoDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: AlbedoDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: AlbedoDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: AlbedoDamageEnum::Normal5 as usize, text: hit_n_dmg!(5) },
            CharacterSkillMapItem { index: AlbedoDamageEnum::Charged11 as usize, text: charged_dmg!(1) },
            CharacterSkillMapItem { index: AlbedoDamageEnum::Charged12 as usize, text: charged_dmg!(2) },
            CharacterSkillMapItem { index: AlbedoDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: AlbedoDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: AlbedoDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: AlbedoDamageEnum::E1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: AlbedoDamageEnum::ETransientBlossom as usize, text: locale!(zh_cn: "刹那之花伤害", en: "Transient Blossom DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: AlbedoDamageEnum::Q1 as usize, text: locale!(zh_cn: "爆发伤害", en: "Burst DMG") },
            CharacterSkillMapItem { index: AlbedoDamageEnum::QFatalBlossom as usize, text: locale!(zh_cn: "生灭之花", en: "Fatal Blossom DMG") },
            CharacterSkillMapItem { index: AlbedoDamageEnum::C2 as usize, text: locale!(zh_cn: "生灭之花-显生之宙", en: "Fatal Blossom DMG-Opening of Phanerozoic") },
        ])
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig::IS_HEXEREI(true, ItemConfig::PRIORITY_CHARACTER),
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "lower50",
            title: locale!(
                zh_cn: "敌人生命值低于50%",
                en: "Enemy HP below 50%"
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "activated_q",
            title: locale!(
                zh_cn: "施放元素爆发后",
                en: "After Elemental Burst"
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "fatal_count",
            title: locale!(
                zh_cn: "生灭计数",
                en: "Fatal Reckoning"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 0 }
        },
        ItemConfig {
            name: "crystallize_shield",
            title: locale!(
                zh_cn: "处于结晶反应产生的护盾庇护下",
                en: "Protected by a shield created by Crystallize"
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s = num::FromPrimitive::from_usize(s).unwrap();

        let s1 = context.character_common_data.skill1;
        let s2 = context.character_common_data.skill2;
        let s3 = context.character_common_data.skill3;

        let (hexerei_secret_rite, is_hexerei) = match &context.character_common_data.config {
            CharacterConfig::Albedo { hexerei_secret_rite, is_hexerei } => (*hexerei_secret_rite, *is_hexerei),
            _ => (false, false),
        };

        let (lower50, activated_q, fatal_count, crystallize_shield) = match *config {
            CharacterSkillConfig::Albedo { lower50, activated_q, fatal_count, crystallize_shield } => (lower50, activated_q, fatal_count, crystallize_shield),
            _ => (false, false, 0, false)
        };

        use AlbedoDamageEnum::*;
        let ratio = match s {
            Normal1 => ALBEDO_SKILL.normal_dmg1[s1],
            Normal2 => ALBEDO_SKILL.normal_dmg2[s1],
            Normal3 => ALBEDO_SKILL.normal_dmg3[s1],
            Normal4 => ALBEDO_SKILL.normal_dmg4[s1],
            Normal5 => ALBEDO_SKILL.normal_dmg5[s1],
            Charged11 => ALBEDO_SKILL.charged_dmg1[s1],
            Charged12 => ALBEDO_SKILL.charged_dmg2[s1],
            Plunging1 => ALBEDO_SKILL.plunging_dmg1[s1],
            Plunging2 => ALBEDO_SKILL.plunging_dmg2[s1],
            Plunging3 => ALBEDO_SKILL.plunging_dmg3[s1],
            E1 => ALBEDO_SKILL.elemental_skill_dmg1[s2],
            ETransientBlossom => ALBEDO_SKILL.elemental_skill_dmg2[s2],
            Q1 => ALBEDO_SKILL.elemental_burst_dmg1[s3],
            QFatalBlossom => ALBEDO_SKILL.elemental_burst_dmg2[s3],
            C2 => ALBEDO_SKILL.c2_dmg,
        };

        let mut builder = D::new();

        if s == C2 && context.character_common_data.constellation < 2 {
            return builder.none();
        }

        if s.is_def_ratio() {
            builder.add_def_ratio("技能倍率", ratio)
        } else {
            builder.add_atk_ratio("技能倍率", ratio)
        }

        if s == ETransientBlossom && context.character_common_data.has_talent1 && lower50{
            builder.add_extra_bonus("天赋1：白垩色的威压", 0.25);
            if hexerei_secret_rite {
                builder.add_def_ratio("天赋1：白垩色的威压", 2.40);
            }
        }

        if context.character_common_data.has_talent2 && activated_q {
            builder.add_extra_em("天赋2：瓶中人的天慧", 125.0);
        }

        if s.get_skill_type() == SkillType::ElementalBurst && context.character_common_data.constellation >= 2 {
            if (s == Q1 || s == QFatalBlossom) && context.character_common_data.constellation >= 6 {
                builder.add_def_ratio("命座2：显生之宙", 0.3 * 4.0);
            } else if fatal_count > 0 {
                builder.add_def_ratio("命座2：显生之宙", 0.3 * fatal_count as f64);
            }
        }

        if context.character_common_data.constellation >= 6 {
            if crystallize_shield {
                builder.add_extra_bonus("命座6：无垢之土", 0.17);
            }

            if (s == QFatalBlossom || s == C2) && activated_q {
                builder.add_def_ratio("命座6：无垢之土", 2.50);
            }
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
        let (hexerei_secret_rite, is_hexerei) = match *config {
            CharacterConfig::Albedo { hexerei_secret_rite, is_hexerei } => (hexerei_secret_rite, is_hexerei),
            _ => (false, false),
        };
        Some(Box::new(AlbedoEffect {
            hexerei_secret_rite: hexerei_secret_rite,
            is_hexerei: is_hexerei,
            common_data: common_data.clone(),
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
