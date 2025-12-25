use num_traits::FromPrimitive;
use crate::attribute::*;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::macros::{damage_enum, skill_map};
use crate::common::{ChangeAttribute, Element, MoonglareReaction, Moonsign, SkillType, StatName, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct KleeSkillType {
    pub normal_dmg1: [f64; 15],
    pub normal_dmg2: [f64; 15],
    pub normal_dmg3: [f64; 15],
    pub charged_dmg1: [f64; 15],
    pub plunging_dmg1: [f64; 15],
    pub plunging_dmg2: [f64; 15],
    pub plunging_dmg3: [f64; 15],

    pub elemental_skill_dmg1: [f64; 15],
    pub elemental_skill_dmg2: [f64; 15],

    pub elemental_burst_dmg1: [f64; 15],

    pub c1_dmg_ratio: f64,
    pub c4_dmg: f64,
}

pub const KLEE_SKILL: KleeSkillType = KleeSkillType {
    normal_dmg1: [0.7216, 0.7757, 0.8298, 0.902, 0.9561, 1.0102, 1.0824, 1.1546, 1.2267, 1.2989, 1.3739, 1.4721, 1.5702, 1.6683, 1.7665],
    normal_dmg2: [0.624, 0.6708, 0.7176, 0.78, 0.8268, 0.8736, 0.936, 0.9984, 1.0608, 1.1232, 1.1881, 1.273, 1.3578, 1.4427, 1.5276],
    normal_dmg3: [0.8992, 0.9666, 1.0341, 1.124, 1.1914, 1.2589, 1.3488, 1.4387, 1.5286, 1.6186, 1.7121, 1.8344, 1.9567, 2.079, 2.2012],
    charged_dmg1: [1.5736, 1.6916, 1.8096, 1.967, 2.085, 2.203, 2.3604, 2.5178, 2.6751, 2.8325, 2.9961, 3.2101, 3.4242, 3.6382, 3.8522],
    plunging_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    plunging_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    plunging_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    elemental_skill_dmg1: [0.952, 1.0234, 1.0948, 1.19, 1.2614, 1.3328, 1.428, 1.5232, 1.6184, 1.7136, 1.8088, 1.904, 2.023, 2.142, 2.261],
    elemental_skill_dmg2: [0.328, 0.3526, 0.3772, 0.41, 0.4346, 0.4592, 0.492, 0.5248, 0.5576, 0.5904, 0.6232, 0.656, 0.697, 0.738, 0.779],
    elemental_burst_dmg1: [0.4264, 0.4584, 0.4904, 0.533, 0.565, 0.597, 0.6396, 0.6822, 0.7249, 0.7675, 0.8102, 0.8528, 0.9061, 0.9594, 1.0127],

    c1_dmg_ratio: 1.2,
    c4_dmg: 5.55,
};

pub const KLEE_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Klee,
    internal_name: "Klee",
    element: Element::Pyro,
    hp: [801, 2077, 2764, 4136, 4623, 5319, 5970, 6673, 7161, 7870, 8358, 9076, 9563, 10287, 11018],
    atk: [24, 63, 84, 125, 140, 161, 180, 202, 216, 238, 253, 274, 289, 311, 381],
    def: [48, 124, 165, 247, 276, 318, 357, 399, 428, 470, 500, 542, 572, 615, 659],
    sub_stat: CharacterSubStatFamily::Bonus288(StatName::PyroBonus),
    weapon_type: WeaponType::Catalyst,
    star: 5,
    skill_name1: locale!(
        zh_cn: "砰砰",
        en: "Kaboom!",
    ),
    skill_name2: locale!(
        zh_cn: "蹦蹦炸弹",
        en: "Jumpy Dumpty",
    ),
    skill_name3: locale!(
        zh_cn: "轰轰火花",
        en: "Sparks 'n' Splash",
    ),
    name_locale: locale!(
        zh_cn: "可莉",
        en: "Klee",
    )
};

pub struct KleeEffect {
    pub hexerei_secret_rite: bool,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for KleeEffect {
    fn change_attribute(&self, attribute: &mut A) {
    }
}

damage_enum!(
    KleeDamageEnum
    Normal1
    Normal2
    Normal3
    Charged
    ChargedWithTalent
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    Q1
    C1
    C4
);

impl KleeDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        use KleeDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 => SkillType::NormalAttack,
            Charged | ChargedWithTalent | C1 => SkillType::ChargedAttack,
            Plunging1 => SkillType::PlungingAttackInAction,
            Plunging2 | Plunging3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q1 | C4 => SkillType::ElementalBurst
        }
    }
}

#[derive(Copy, Clone, FromPrimitive)]
pub enum KleeRoleEnum {
    MainPyro
}

pub struct Klee;

impl CharacterTrait for Klee {
    const STATIC_DATA: CharacterStaticData = KLEE_STATIC_DATA;
    type SkillType = KleeSkillType;
    const SKILL: Self::SkillType = KLEE_SKILL;
    type DamageEnumType = KleeDamageEnum;
    type RoleEnum = KleeRoleEnum;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: KleeDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: KleeDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: KleeDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: KleeDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: KleeDamageEnum::ChargedWithTalent as usize, text: locale!(zh_cn: "嘭嘭轰击伤害", en: "Boom-Boom Strike DMG") },
            CharacterSkillMapItem { index: KleeDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: KleeDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: KleeDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
            CharacterSkillMapItem { index: KleeDamageEnum::C1 as usize, text: locale!(zh_cn: "连环轰隆火花伤害", en: "Bombard Opponents DMG") },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: KleeDamageEnum::E1 as usize, text: locale!(zh_cn: "蹦蹦炸弹伤害", en: "Jumpy Dumpty DMG") },
            CharacterSkillMapItem { index: KleeDamageEnum::E2 as usize, text: locale!(zh_cn: "诡雷伤害", en: "Mine DMG") },
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: KleeDamageEnum::Q1 as usize, text: locale!(zh_cn: "轰轰火花伤害", en: "Sparks 'n' Splash DMG") },
            CharacterSkillMapItem { index: KleeDamageEnum::C4 as usize, text: locale!(zh_cn: "轰轰火花结束伤害", en: "Sparks 'n' Splash End DMG") },
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
            name: "boom_badge",
            title: locale!(
                zh_cn: "「轰轰勋章」数量",
                en: "Boom Badge Count"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 0 }
        },
        ItemConfig {
            name: "active",
            title: locale!(
                zh_cn: "位于场上",
                en: "Is active"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "activated_q",
            title: locale!(
                zh_cn: "施放轰轰火花后",
                en: "After Sparks 'n' Splash"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "activated_c1",
            title: locale!(
                zh_cn: "触发连环轰隆火花攻击后",
                en: "After Bombard Opponents Attack"
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: KleeDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let hexerei_secret_rite = match &context.character_common_data.config {
            CharacterConfig::Klee { hexerei_secret_rite } => *hexerei_secret_rite,
            _ => false,
        };

        let (boom_badge, active, activated_q, activated_c1) = match *config {
            CharacterSkillConfig::Klee { boom_badge, active, activated_q, activated_c1 } => (boom_badge, active, activated_q, activated_c1),
            _ => (0, false, false, false)
        };

        use KleeDamageEnum::*;
        let mut builder = D::new();

        if s == C1 && context.character_common_data.constellation < 1 {
            return builder.none();
        }
        if s == C4 && context.character_common_data.constellation < 4 {
            return builder.none();
        }

        if (s == ChargedWithTalent || s == C1) && context.character_common_data.has_talent1 {
            builder.add_extra_bonus("天赋1：砰砰礼物", 0.5);
        }

        if context.character_common_data.constellation >= 1 && activated_c1 {
            builder.add_extra_atk("命座1：连环轰隆", context.attribute.get_atk() * 0.6);
        }

        if context.character_common_data.constellation >= 6 && activated_q {
            builder.add_extra_bonus("命座6：火力全开", 0.5);
        }

        let z_ratio = if hexerei_secret_rite {
            [1.0, 1.15, 1.30, 1.50][boom_badge as usize]
        } else { 1.0 };

        let ratio = match s {
            Normal1 => KLEE_SKILL.normal_dmg1[s1],
            Normal2 => KLEE_SKILL.normal_dmg2[s1],
            Normal3 => KLEE_SKILL.normal_dmg3[s1],
            Charged => KLEE_SKILL.charged_dmg1[s1],
            ChargedWithTalent => KLEE_SKILL.charged_dmg1[s1] * z_ratio,
            Plunging1 => KLEE_SKILL.plunging_dmg1[s1],
            Plunging2 => KLEE_SKILL.plunging_dmg2[s1],
            Plunging3 => KLEE_SKILL.plunging_dmg3[s1],
            E1 => KLEE_SKILL.elemental_skill_dmg1[s2],
            E2 => KLEE_SKILL.elemental_skill_dmg2[s2],
            Q1 => KLEE_SKILL.elemental_burst_dmg1[s3],
            C1 => KLEE_SKILL.charged_dmg1[s1] * z_ratio * KLEE_SKILL.c1_dmg_ratio,
            C4 => KLEE_SKILL.c4_dmg * if active { 2.0 } else { 1.0 },
        };

        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            Element::Pyro,
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let hexerei_secret_rite = match *config {
            CharacterConfig::Klee { hexerei_secret_rite } => hexerei_secret_rite,
            _ => false,
        };
        Some(Box::new(KleeEffect {
            hexerei_secret_rite: hexerei_secret_rite,
            common_data: common_data.clone(),
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
