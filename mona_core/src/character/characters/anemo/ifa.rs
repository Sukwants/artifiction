use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::macros::{damage_enum, skill_map};
use crate::common::{ChangeAttribute, Element, SkillType, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::target_functions::IfaDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct IfaSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub z_dmg: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg: [f64; 15],
    pub e_heal: [f64; 15],
    pub e_heal_fixed: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const IFA_SKILL: IfaSkillType = IfaSkillType {
    a_dmg1: [0.536072, 0.576277, 0.616483, 0.67009, 0.710295, 0.750501, 0.804108, 0.857715, 0.911322, 0.96493, 1.018537, 1.072144, 1.139153, 1.206162, 1.273171],
    a_dmg2: [0.474672, 0.510272, 0.545873, 0.59334, 0.62894, 0.664541, 0.712008, 0.759475, 0.806942, 0.85441, 0.901877, 0.949344, 1.008678, 1.068012, 1.127346],
    a_dmg3: [0.747584, 0.803653, 0.859722, 0.93448, 0.990549, 1.046618, 1.121376, 1.196134, 1.270893, 1.345651, 1.42041, 1.495168, 1.588616, 1.682064, 1.775512],
    z_dmg: [1.4704, 1.58068, 1.69096, 1.838, 1.94828, 2.05856, 2.2056, 2.35264, 2.49968, 2.64672, 2.79376, 2.9408, 3.1246, 3.3084, 3.4922],
    x_dmg1: [0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064, 1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984],
    x_dmg2: [1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686, 2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036],
    x_dmg3: [1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632, 2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592],
    e_dmg: [1.3336, 1.43362, 1.53364, 1.667, 1.76702, 1.86704, 2.0004, 2.13376, 2.26712, 2.40048, 2.53384, 2.6672, 2.8339, 3.0006, 3.1673],
    e_heal: [0.2016, 0.21672, 0.23184, 0.252, 0.26712, 0.28224, 0.3024, 0.32256, 0.34272, 0.36288, 0.38304, 0.4032, 0.4284, 0.4536, 0.4788],
    e_heal_fixed: [48.14847, 52.96399, 58.1808, 63.7989, 69.8183, 76.23898, 83.06097, 90.28424, 97.90881, 105.93467, 114.36182, 123.19027, 132.42001, 142.05104, 152.08337],
    q_dmg1: [5.0848, 5.46616, 5.84752, 6.356, 6.73736, 7.11872, 7.6272, 8.13568, 8.64416, 9.15264, 9.66112, 10.1696, 10.8052, 11.4408, 12.0764],
    q_dmg2: [1.0896, 1.17132, 1.25304, 1.362, 1.44372, 1.52544, 1.6344, 1.74336, 1.85232, 1.96128, 2.07024, 2.1792, 2.3154, 2.4516, 2.5878]
};

pub const IFA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Ifa,
    internal_name: "Ifa",
    element: Element::Anemo,
    hp: [845, 2171, 2803, 4198, 4647, 5345, 5943, 6641, 7090, 7787, 8236, 8934, 9383, 10081],
    atk: [15,38,50,74,82,95,105,118,125,138,146,158,166,178],
    def: [51,130,168,252,279,321,357,399,426,468,495,537,563,605],
    sub_stat: CharacterSubStatFamily::ElementalMastery96,
    weapon_type: WeaponType::Catalyst,
    star: 4,
    skill_name1: locale!(
        zh_cn: "祛风妙仪",
        en: "Rite of Dispelling Winds",
    ),
    skill_name2: locale!(
        zh_cn: "空天疾护",
        en: "Airborne Disease Prevention",
    ),
    skill_name3: locale!(
        zh_cn: "复合镇静域",
        en: "Compound Sedation Field",
    ),
    name_locale: locale!(
        zh_cn: "伊法",
        en: "Ifa",
    )
};

pub struct IfaEffect {
    pub nightsoul_total: usize,
    pub has_p1: bool,
    pub has_p2: bool,
    pub has_c2: bool,
    pub has_c4: bool,
}

impl<A: Attribute> ChangeAttribute<A> for IfaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        let mut essentials: usize = 0;
        if self.has_p1 {
            essentials += self.nightsoul_total * 1;
        }
        if self.has_c2 {
            essentials += (self.nightsoul_total - 60) * 4;
            essentials = essentials.min(200);
        } else {
            essentials = essentials.min(150);
        }
        attribute.set_value_by(AttributeName::EnhanceSwirlBase, "伊法天赋：场中医者视野", (essentials as f64) * 0.015);
        attribute.set_value_by(AttributeName::EnhanceElectroCharged, "伊法天赋：场中医者视野", (essentials as f64) * 0.015);
        // attribute.set_value_by(AttributeName::EnhanceLunarCharged, "伊法天赋：场中医者视野", essentials * 0.002);

        if self.has_p2 {
            attribute.set_value_by(AttributeName::ElementalMastery, "伊法天赋：互助救援协议", 80.0);
        }
        if self.has_c4 {
            attribute.set_value_by(AttributeName::ElementalMastery, "伊法四命：糜烂应体的置换", 100.0);
        }
    }
}

damage_enum!(
    IfaDamageEnum
    A1
    A2
    A3
    Z
    X1
    X2
    X3
    E
    EHeal
    Q1
    Q2
);

impl IfaDamageEnum {
    pub fn get_element(&self) -> Element {
        use IfaDamageEnum::*;
        match *self {
            _ => Element::Anemo
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use IfaDamageEnum::*;
        match *self {
            A1 | A2 | A3 => SkillType::NormalAttack,
            Z => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E | EHeal => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst
        }
    }
}

pub struct Ifa;

impl CharacterTrait for Ifa {
    const STATIC_DATA: CharacterStaticData = IFA_STATIC_DATA;
    type SkillType = IfaSkillType;
    const SKILL: Self::SkillType = IFA_SKILL;
    type DamageEnumType = IfaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            IfaDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            Z charged_dmg!()
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            IfaDamageEnum
            E locale!(zh_cn: "秘药弹伤害", en: "Tonicshot DMG")
            EHeal locale!(zh_cn: "秘药弹命中治疗量", en: "Tonicshot Healing On Hit")
        ),
        skill3: skill_map!(
            IfaDamageEnum
            Q1 locale!(zh_cn: "技能伤害", en: "Skill DMG")
            Q2 locale!(zh_cn: "镇静标记伤害", en: "Sedation Mark DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "nightsoul_total",
            title: locale!(
                zh_cn: "队伍中所有角色夜魂值的总和",
                en: "Nightsoul Points in the Entire Party"
            ),
            config: ItemConfigType::Int { min: 0, max: 200, default: 0 }
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: IfaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use IfaDamageEnum::*;
        let mut builder = D::new();

        if s == EHeal {
            let ratio = IFA_SKILL.e_heal[s2];
            let fixed = IFA_SKILL.e_heal_fixed[s2];

            builder.add_em_ratio("技能倍率", ratio);
            builder.add_extra_damage("技能倍率", fixed);

            return builder.heal(&context.attribute);
        } else {
            let ratio = match s {
                A1 => IFA_SKILL.a_dmg1[s1],
                A2 => IFA_SKILL.a_dmg2[s1],
                A3 => IFA_SKILL.a_dmg3[s1],
                Z => IFA_SKILL.z_dmg[s1],
                X1 => IFA_SKILL.x_dmg1[s1],
                X2 => IFA_SKILL.x_dmg2[s1],
                X3 => IFA_SKILL.x_dmg3[s1],
                E => IFA_SKILL.e_dmg[s2],
                Q1 => IFA_SKILL.q_dmg1[s3],
                Q2 => IFA_SKILL.q_dmg2[s3],
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
        let nightsoul_total = match config {
            CharacterConfig::Ifa { nightsoul_total } => *nightsoul_total,
            _ => 0,
        };

        Some(Box::new(IfaEffect {
            nightsoul_total,
            has_p1: common_data.has_talent1,
            has_p2: common_data.has_talent2,
            has_c2: common_data.constellation >= 2,
            has_c4: common_data.constellation >= 4,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        Box::new(crate::target_functions::target_functions::anemo::ifa_default::IfaDefaultTargetFunction)
    }
}
