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
use crate::common::item_config_type::{ConfigElements8Multi, ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct JahodaSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub z_dmg1: [f64; 15],
    pub z_dmg2: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmgs: [f64; 15], // Smoke Bomb DMG
    pub e_dmgt1: [f64; 15], // Unfilled Treasure Flask DMG
    pub e_dmgt2: [f64; 15], // Filled Treasure Flask DMG
    pub e_dmgtc: [f64; 15], // Meowball DMG

    pub q_dmg: [f64; 15],
    pub q_dmgc: [f64; 15], // Purrsonal Coordinated Assistance Robot DMG
    pub q_heal_atk: [f64; 15],
    pub q_heal: [f64; 15],
    pub q_healad_atk: [f64; 15],
    pub q_healad: [f64; 15],
}

pub const JAHODA_SKILL: JahodaSkillType = JahodaSkillType {
    // Normal Attack: Strike While the Arrow's Hot
    a_dmg1: [0.416739, 0.450659, 0.48458, 0.533038, 0.566959, 0.605725, 0.659029, 0.712333, 0.765636, 0.823786, 0.881936, 0.940085, 0.998235, 1.056384, 1.114534],
    a_dmg2: [0.192313, 0.207967, 0.22362, 0.245982, 0.261635, 0.279525, 0.304123, 0.328721, 0.35332, 0.380154, 0.406988, 0.433823, 0.460657, 0.487492, 0.514326],
    a_dmg3: [0.511975, 0.553648, 0.59532, 0.654852, 0.696524, 0.74415, 0.809635, 0.87512, 0.940606, 1.012044, 1.083482, 1.154921, 1.226359, 1.297798, 1.369236],
    z_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    z_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    x_dmg1: [0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064, 1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984],
    x_dmg2: [1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686, 2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036],
    x_dmg3: [1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632, 2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592],

    // Elemental Skill: Savvy Strategy: Splitting the Spoils
    e_dmgs: [1.59, 1.70925, 1.8285, 1.9875, 2.10675, 2.226, 2.385, 2.544, 2.703, 2.862, 3.021, 3.18, 3.37875, 3.5775, 3.77625],
    e_dmgt1: [1.908, 2.0511, 2.1942, 2.385, 2.5281, 2.6712, 2.862, 3.0528, 3.2436, 3.4344, 3.6252, 3.816, 4.0545, 4.293, 4.5315],
    e_dmgt2: [2.12, 2.279, 2.438, 2.65, 2.809, 2.968, 3.18, 3.392, 3.604, 3.816, 4.028, 4.24, 4.505, 4.77, 5.035],
    e_dmgtc: [1.28, 1.376, 1.472, 1.6, 1.696, 1.792, 1.92, 2.048, 2.176, 2.304, 2.432, 2.56, 2.72, 2.88, 3.04],

    // Elemental Burst: Hidden Aces: Seven Tools of the Hunter
    q_dmg: [2.072, 2.2274, 2.3828, 2.59, 2.7454, 2.9008, 3.108, 3.3152, 3.5224, 3.7296, 3.9368, 4.144, 4.403, 4.662, 4.921],
    q_dmgc: [0.172664, 0.185614, 0.198564, 0.21583, 0.22878, 0.24173, 0.258996, 0.276262, 0.293529, 0.310795, 0.328062, 0.345328, 0.366911, 0.388494, 0.410077],
    q_heal_atk: [0.79872, 0.858624, 0.918528, 0.9984, 1.058304, 1.118208, 1.19808, 1.277952, 1.357824, 1.437696, 1.517568, 1.59744, 1.69728, 1.79712, 1.89696],
    q_heal: [500.73764, 550.81836, 605.0725, 663.5, 726.1009, 792.8752, 863.8229, 938.944, 1018.23846, 1101.7063, 1189.3477, 1281.1622, 1377.1503, 1477.3118, 1581.6466],
    q_healad_atk: [0.3072, 0.33024, 0.35328, 0.384, 0.40704, 0.43008, 0.4608, 0.49152, 0.52224, 0.55296, 0.58368, 0.6144, 0.6528, 0.6912, 0.7296],
    q_healad: [192.59721, 211.8596, 232.7272, 255.2, 279.27798, 304.96118, 332.2496, 361.1432, 391.642, 423.74597, 457.45517, 492.76956, 529.68915, 568.214, 608.34393],
};

pub const JAHODA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Jahoda,
    internal_name: "Jahoda",
    element: Element::Anemo,
    hp: [809, 2078, 2682, 4017, 4446, 5114, 5687, 6355, 6784, 7451, 7881, 8549, 8978, 9646, 10313],
    atk: [19, 48, 62, 93, 103, 118, 131, 147, 157, 172, 182, 198, 208, 223, 280],
    def: [49, 125, 161, 242, 267, 308, 342, 382, 408, 448, 474, 514, 540, 580, 620],
    sub_stat: CharacterSubStatFamily::HealingBonus185,
    weapon_type: WeaponType::Bow,
    star: 4,
    skill_name1: locale!(
        zh_cn: "见机行矢",
        en: "Strike While the Arrow's Hot",
    ),
    skill_name2: locale!(
        zh_cn: "奇策·财富分配方案",
        en: "Savvy Strategy: Splitting the Spoils",
    ),
    skill_name3: locale!(
        zh_cn: "秘器·猎人的七道具",
        en: "Hidden Aces: Seven Tools of the Hunter",
    ),
    name_locale: locale!(
        zh_cn: "雅珂达",
        en: "Jahoda",
    )
};

pub struct JahodaEffect {
    pub max_elements: ConfigElements8Multi,
    pub moonsign: Moonsign,
    pub has_c6: bool,
}

impl<A: Attribute> ChangeAttribute<A> for JahodaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_c6 && self.moonsign.is_ascendant() {
            attribute.set_value_by(AttributeName::CriticalBase, "命座6：最渺小的幸运", 0.05);
            attribute.set_value_by(AttributeName::CriticalDamageBase, "命座6：最渺小的幸运", 0.40);
        }
    }
}

damage_enum!(
    JahodaDamageEnum
    A1
    A2
    A3
    Z1
    Z2
    X1
    X2
    X3
    ES
    ET1
    ET2
    ETC
    Q
    QC
    QHeal
    QHealAd
);

impl JahodaDamageEnum {
    pub fn get_element(&self, elemental_absorption: Element) -> Element {
        use JahodaDamageEnum::*;
        match *self {
            A1 | A2 | A3 | Z1 | X1 | X2 | X3 | QHeal | QHealAd => Element::Physical,
            Z2 | ES | ET1 | ET2 | Q | QC => Element::Anemo,
            ETC => elemental_absorption,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use JahodaDamageEnum::*;
        match *self {
            A1 | A2 | A3 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            ES | ET1 | ET2 | ETC => SkillType::ElementalSkill,
            Q | QC | QHeal | QHealAd => SkillType::ElementalBurst,
        }
    }
}

pub struct Jahoda;

impl CharacterTrait for Jahoda {
    const STATIC_DATA: CharacterStaticData = JAHODA_STATIC_DATA;
    type SkillType = JahodaSkillType;
    const SKILL: Self::SkillType = JAHODA_SKILL;
    type DamageEnumType = JahodaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            JahodaDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 locale!(zh_cn: "三段单次伤害", en: "3-Hit Single DMG")
            Z1 charged_dmg!("shoot1")
            Z2 charged_dmg!("shoot2")
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            JahodaDamageEnum
            ES locale!(zh_cn: "烟雾弹伤害", en: "Smoke Bomb DMG")
            ET1 locale!(zh_cn: "秘藏瓶未装满伤害", en: "Unfilled Treasure Flask DMG")
            ET2 locale!(zh_cn: "秘藏瓶装满伤害", en: "Filled Treasure Flask DMG")
            ETC locale!(zh_cn: "猫猫球伤害", en: "Meowball DMG")
        ),
        skill3: skill_map!(
            JahodaDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
            QC locale!(zh_cn: "猫型家用互助协调器伤害", en: "Purrsonal Coordinated Assistance Robot DMG")
            QHeal locale!(zh_cn: "猫型家用互助协调器治疗量", en: "Purrsonal Coordinated Assistance Robot Healing")
            QHealAd locale!(zh_cn: "最低生命值角色额外治疗量", en: "Lowest HP Character Additional Healing")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig {
            name: "max_elements",
            title: locale!(zh_cn: "角色数量最多的元素类型", en: "The element with the highest number of characters"),
            config: ItemConfigType::ElementMulti {
                elements: &[Element::Pyro, Element::Hydro, Element::Electro, Element::Cryo],
                default: ConfigElements8Multi {
                    pyro: false,
                    hydro: false,
                    anemo: false,
                    electro: false,
                    dendro: false,
                    cryo: false,
                    geo: false,
                    physical: false,
                }
            }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "elemental_absorption",
            title: locale!(zh_cn: "元素吸收类型", en: "Elemental Absorption Type"),
            config: ItemConfigType::Element4 { default: Element::Pyro }
        },
        ItemConfig {
            name: "activated_p2",
            title: locale!(zh_cn: "元素精通提升", en: "Elemental Mastery Increase"),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: JahodaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (max_elements, moonsign) = match &context.character_common_data.config {
            CharacterConfig::Jahoda { max_elements, moonsign } => (*max_elements, *moonsign),
            _ => (ConfigElements8Multi::default(), Moonsign::None),
        };

        let (elemental_absorption, activated_p2) = match *config {
            CharacterSkillConfig::Jahoda { elemental_absorption, activated_p2 } => (elemental_absorption, activated_p2),
            _ => (Element::Pyro, false),
        };

        use JahodaDamageEnum::*;
        let mut builder = D::new();

        let (dmg_ratio, heal_ratio) = if context.character_common_data.has_talent1{
            let mut dmg_ratio = 1.0;
            let mut heal_ratio = 1.0;

            if max_elements.pyro {
                dmg_ratio = 1.3;
            } else if max_elements.hydro {
                heal_ratio = 1.2;
            }

            if context.character_common_data.constellation >= 2 && moonsign.is_ascendant() {
                if max_elements.pyro && max_elements.hydro {
                    heal_ratio = 1.2;
                }
            }

            (dmg_ratio, heal_ratio)
        } else { (1.0, 1.0) };

        if context.character_common_data.has_talent2 && activated_p2 {
            builder.add_extra_em("天赋2：蜜莓的嘉赏", 100.0);
        }

        if s == QHeal || s == QHealAd {
            let ratio_base = match s {
                QHeal => JAHODA_SKILL.q_heal[s3],
                QHealAd => JAHODA_SKILL.q_healad[s3],
                _ => 0.0
            };
            let ratio_atk = match s {
                QHeal => JAHODA_SKILL.q_heal_atk[s3],
                QHealAd => JAHODA_SKILL.q_healad_atk[s3],
                _ => 0.0
            };

            builder.add_atk_ratio("额外治疗量", ratio_atk * heal_ratio);
            builder.add_extra_damage("基础治疗量", ratio_base * heal_ratio);

            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                A1 => JAHODA_SKILL.a_dmg1[s1],
                A2 => JAHODA_SKILL.a_dmg2[s1],
                A3 => JAHODA_SKILL.a_dmg3[s1],
                Z1 => JAHODA_SKILL.z_dmg1[s2],
                Z2 => JAHODA_SKILL.z_dmg2[s2],
                X1 => JAHODA_SKILL.x_dmg1[s3],
                X2 => JAHODA_SKILL.x_dmg2[s3],
                X3 => JAHODA_SKILL.x_dmg3[s3],
                ES => JAHODA_SKILL.e_dmgs[s2],
                ET1 => JAHODA_SKILL.e_dmgt1[s2],
                ET2 => JAHODA_SKILL.e_dmgt2[s2],
                ETC => JAHODA_SKILL.e_dmgtc[s2],
                Q => JAHODA_SKILL.q_dmg[s3],
                QC => JAHODA_SKILL.q_dmgc[s3] * dmg_ratio,
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);

            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(elemental_absorption),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (max_elements, moonsign) = match *config {
            CharacterConfig::Jahoda { max_elements, moonsign } => (max_elements, moonsign),
            _ => (ConfigElements8Multi::default(), Moonsign::None),
        };
        Some(Box::new(JahodaEffect {
            max_elements,
            moonsign,
            has_c6: common_data.constellation >= 6,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
