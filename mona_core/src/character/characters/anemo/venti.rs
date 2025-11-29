use num_traits::FromPrimitive;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
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

pub struct VentiSkillType {
    pub a_dmg11: [f64; 15],
    pub a_dmg12: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub a_dmg41: [f64; 15],
    pub a_dmg42: [f64; 15],
    pub a_dmg5: [f64; 15],
    pub a_dmg6: [f64; 15],
    pub z_dmg1: [f64; 15],
    pub z_dmg2: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],
    pub a_dmgw_ratio: [f64; 15],
    pub a_dmgw_c1_ratio: f64,
    pub z_dmg_c1_ratio: f64,
    
    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg_c2_ratio: f64,
    
    pub q_dmg: [f64; 15],
    pub q_dmga: [f64; 15],

}

pub const VENTI_SKILL: VentiSkillType = VentiSkillType {
    // Normal Attack: Divine Marksmanship
    a_dmg11: [0.2038, 0.2204, 0.237, 0.2607, 0.2773, 0.2963, 0.3223, 0.3484, 0.3745, 0.4029, 0.4355, 0.4738, 0.5121, 0.5505, 0.5923],
    a_dmg12: [0.2038, 0.2204, 0.237, 0.2607, 0.2773, 0.2963, 0.3223, 0.3484, 0.3745, 0.4029, 0.4355, 0.4738, 0.5121, 0.5505, 0.5923],
    a_dmg2: [0.4438, 0.4799, 0.516, 0.5676, 0.6037, 0.645, 0.7018, 0.7585, 0.8153, 0.8772, 0.9482, 1.0316, 1.115, 1.1985, 1.2895],
    a_dmg3: [0.5237, 0.5664, 0.609, 0.6699, 0.7125, 0.7613, 0.8282, 0.8952, 0.9622, 1.0353, 1.119, 1.2175, 1.316, 1.4145, 1.5219],
    a_dmg41: [0.2606, 0.2818, 0.303, 0.3333, 0.3545, 0.3787, 0.4121, 0.4454, 0.4787, 0.5151, 0.5568, 0.6058, 0.6548, 0.7037, 0.7572],
    a_dmg42: [0.2606, 0.2818, 0.303, 0.3333, 0.3545, 0.3787, 0.4121, 0.4454, 0.4787, 0.5151, 0.5568, 0.6058, 0.6548, 0.7037, 0.7572],
    a_dmg5: [0.5065, 0.5478, 0.589, 0.6479, 0.6891, 0.7363, 0.801, 0.8658, 0.9306, 1.0013, 1.0823, 1.1775, 1.2728, 1.368, 1.4719],
    a_dmg6: [0.7095, 0.7673, 0.825, 0.9075, 0.9653, 1.0313, 1.122, 1.2128, 1.3035, 1.4025, 1.5159, 1.6493, 1.7827, 1.9161, 2.0617],
    z_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9371, 1.0196, 1.1021, 1.1845, 1.2745],
    z_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.361, 2.5296, 2.6982, 2.8669, 3.0355],
    x_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    x_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    x_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],
    a_dmgw_ratio: [1.6, 1.7, 1.8, 1.9, 2.0, 2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7, 2.8, 2.9, 3.0],
    a_dmgw_c1_ratio: 0.2,
    z_dmg_c1_ratio: 0.33,

    // Elemental Skill: Skyward Sonnet
    e_dmg1: [2.76, 2.967, 3.174, 3.45, 3.657, 3.864, 4.14, 4.416, 4.692, 4.968, 5.244, 5.52, 5.865, 6.21, 6.555],
    e_dmg2: [3.8, 4.085, 4.37, 4.75, 5.035, 5.32, 5.7, 6.08, 6.46, 6.84, 7.22, 7.6, 8.075, 8.55, 9.025],
    e_dmg_c2_ratio: 3.0,

    // Elemental Burst: Wind's Grand Ode
    q_dmg: [0.376, 0.4042, 0.4324, 0.47, 0.4982, 0.5264, 0.564, 0.6016, 0.6392, 0.6768, 0.7144, 0.752, 0.799, 0.846, 0.893],
    q_dmga: [0.188, 0.2021, 0.2162, 0.235, 0.2491, 0.2632, 0.282, 0.3008, 0.3196, 0.3384, 0.3572, 0.376, 0.3995, 0.423, 0.4465],

};

pub const VENTI_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Venti,
    internal_name: "Venti",
    element: Element::Anemo,
    hp: [820, 2127, 2830, 4234, 4734, 5446, 6112, 6832, 7331, 8058, 8557, 9292, 9791, 10531, 11280],
    atk: [20, 53, 71, 106, 118, 136, 153, 171, 183, 201, 214, 232, 245, 263, 323],
    def: [52, 135, 180, 269, 301, 346, 388, 434, 465, 512, 543, 590, 622, 669, 716],
    sub_stat: CharacterSubStatFamily::Recharge320,
    weapon_type: WeaponType::Bow,
    star: 5,
    skill_name1: locale!(
        zh_cn: "神代射术",
        en: "Normal Attack: Divine Marksmanship",
    ),
    skill_name2: locale!(
        zh_cn: "高天之歌",
        en: "Skyward Sonnet",
    ),
    skill_name3: locale!(
        zh_cn: "风神之诗",
        en: "Wind's Grand Ode",
    ),
    name_locale: locale!(
        zh_cn: "温迪",
        en: "Venti",
    )
};

pub struct VentiEffect {
    pub hexerei_secret_rite: bool,
    pub elemental_absorption: Option<Element>,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for VentiEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.common_data.constellation >= 2 {
            attribute.set_value_by(AttributeName::ResMinusAnemo, "命座2：眷恋的泠风", 0.24);
            attribute.set_value_by(AttributeName::ResMinusPhysical, "命座2：眷恋的泠风", 0.24);
        }

        if self.common_data.constellation >= 4 {
            attribute.set_value_by(AttributeName::BonusAnemo, "命座4：自由的凛风", 0.25);
        }

        if self.common_data.constellation >= 6 {
            attribute.set_value_by(AttributeName::ResMinusAnemo, "命座6：抗争的暴风", 0.2);
            if self.elemental_absorption != None {
                attribute.set_value_by(AttributeName::res_minus_name_by_element(self.elemental_absorption.unwrap()), "命座6：抗争的暴风", 0.2);
            }
            attribute.set_value_by(AttributeName::CriticalDamageBase, "命座6：抗争的暴风", 1.0);
        }
    }
}

damage_enum!(
    VentiDamageEnum
    A11
    A12
    A2
    A3
    A41
    A42
    A5
    A6
    Z1
    Z2
    X1
    X2
    X3
    AWC1
    ZC1
    E1
    E2
    Q
    QA
);

impl VentiDamageEnum {
    pub fn get_element(&self, activated_q: bool, elemental_absorption: Option<Element>) -> Element {
        use VentiDamageEnum::*;
        match *self {
            A11 | A12 | A2 | A3 | A41 | A42 | A5 | A6 => if activated_q { Element::Anemo } else { Element::Physical },
            Z1 | X1 | X2 | X3 => Element::Physical,
            Z2 | AWC1 | ZC1 | E1 | E2 | Q => Element::Anemo,
            QA => elemental_absorption.unwrap_or(Element::Anemo),
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use VentiDamageEnum::*;
        match *self {
            A11 | A12 | A2 | A3 | A41 | A42 | A5 | A6 | ZC1 | AWC1 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q | QA => SkillType::ElementalBurst,
        }
    }
}

pub struct Venti;

impl CharacterTrait for Venti {
    const STATIC_DATA: CharacterStaticData = VENTI_STATIC_DATA;
    type SkillType = VentiSkillType;
    const SKILL: Self::SkillType = VENTI_SKILL;
    type DamageEnumType = VentiDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            VentiDamageEnum
            A11 hit_n_dmg!(1, 1)
            A12 hit_n_dmg!(1, 2)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            A3 hit_n_dmg!(3)
            A41 hit_n_dmg!(4, 1)
            A42 hit_n_dmg!(4, 2)
            A5 hit_n_dmg!(5)
            A6 hit_n_dmg!(6)
            Z1 charged_dmg!("shoot1")
            Z2 charged_dmg!("shoot2")
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
            AWC1 locale!(zh_cn: "飓风箭额外箭矢伤害", en: "Windsunder Extra Arrow DMG")
            ZC1 locale!(zh_cn: "分裂箭伤害", en: "Additional Arrow DMG")
        ),
        skill2: skill_map!(
            VentiDamageEnum
            E1 locale!(zh_cn: "点按伤害", en: "Press DMG")
            E2 locale!(zh_cn: "长按伤害", en: "Hold DMG")
        ),
        skill3: skill_map!(
            VentiDamageEnum
            Q locale!(zh_cn: "持续伤害", en: "DoT")
            QA locale!(zh_cn: "附加元素伤害", en: "Additional Elemental DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig {
            name: "elemental_absorption",
            title: locale!(
                zh_cn: "元素转化",
                en: "Elemental Absorption"
            ),
            config: ItemConfigType::ElementOptional { 
                elements: &[Element::Pyro, Element::Hydro, Element::Electro, Element::Cryo], 
                default: None, 
            }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "activated_q",
            title: locale!(
                zh_cn: "暴风之眼存在",
                en: "Stormeye active"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "active",
            title: locale!(
                zh_cn: "位于场上",
                en: "Is active"
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "breeze_blow",
            title: locale!(
                zh_cn: "风起之时",
                en: "Wherever a Breeze Blows"
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: VentiDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (hexerei_secret_rite, elemental_absorption) = match &context.character_common_data.config {
            CharacterConfig::Venti { hexerei_secret_rite, elemental_absorption } => (*hexerei_secret_rite, *elemental_absorption),
            _ => (false, None),
        };

        let (activated_q, active, breeze_blow) = match *config {
            CharacterSkillConfig::Venti { activated_q, active, breeze_blow } => (activated_q, active, breeze_blow),
            _ => (false, false, false)
        };

        use VentiDamageEnum::*;
        let mut builder = D::new();

        if activated_q && active && elemental_absorption != None {
            builder.add_extra_bonus("天赋3：魔女的前夜礼·颂时风若", 0.5);
        }

        let ratio = match s {
            A11 => VENTI_SKILL.a_dmg11[s1],
            A12 => VENTI_SKILL.a_dmg12[s1],
            A2 => VENTI_SKILL.a_dmg2[s1],
            A3 => VENTI_SKILL.a_dmg3[s1],
            A41 => VENTI_SKILL.a_dmg41[s1],
            A42 => VENTI_SKILL.a_dmg42[s1],
            A5 => VENTI_SKILL.a_dmg5[s1],
            A6 => VENTI_SKILL.a_dmg6[s1],
            Z1 => VENTI_SKILL.z_dmg1[s2],
            Z2 => VENTI_SKILL.z_dmg2[s2],
            X1 => VENTI_SKILL.x_dmg1[s1],
            X2 => VENTI_SKILL.x_dmg2[s1],
            X3 => VENTI_SKILL.x_dmg3[s1],
            AWC1 => VENTI_SKILL.a_dmgw_ratio[s1],
            ZC1 => VENTI_SKILL.z_dmg_c1_ratio,
            E1 => VENTI_SKILL.e_dmg1[s2],
            E2 => VENTI_SKILL.e_dmg2[s2],
            Q => VENTI_SKILL.q_dmg[s3],
            QA => if elemental_absorption != None { VENTI_SKILL.q_dmga[s3] } else { 0.0 },
        } * match s.get_skill_type() {
            SkillType::NormalAttack => if activated_q && hexerei_secret_rite { VENTI_SKILL.a_dmgw_ratio[s1] } else { 1.0 },
            SkillType::ElementalSkill => if context.character_common_data.constellation >= 2 && breeze_blow { 3.0 } else { 1.0 },
            SkillType::ElementalBurst => if hexerei_secret_rite { 1.35 } else { 1.0 }
            _ => 1.0,
        };

        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(activated_q, elemental_absorption),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (hexerei_secret_rite, elemental_absorption) = match *config {
            CharacterConfig::Venti { hexerei_secret_rite, elemental_absorption } => (hexerei_secret_rite, elemental_absorption),
            _ => (false, None),
        };
        Some(Box::new(VentiEffect {
            hexerei_secret_rite: hexerei_secret_rite,
            elemental_absorption: elemental_absorption,
            common_data: common_data.clone(),
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
