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
// use crate::target_functions::target_functions::FlinsDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct FlinsSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub a_dmg4: [f64; 15],
    pub a_dmg5: [f64; 15],
    pub z_dmg: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],
    pub e_dmg4: [f64; 15],
    pub e_dmg5: [f64; 15],
    pub e_dmgz: [f64; 15],
    pub e_ns_dmg: [f64; 15], // Northland Spearstorm DMG

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
    pub q_dmg3: [f64; 15],
    pub q_ts_dmg: [f64; 15], // Thunderous Symphony DMG
    pub q_tsa_dmg: [f64; 15], // Thunderous Symphony Additional DMG

    pub c2_dmg: f64,
}

pub const FLINS_SKILL: FlinsSkillType = FlinsSkillType {
    // Normal Attack: Pocztowy Demonspear
    a_dmg1: [0.44726, 0.483665, 0.52007, 0.572077, 0.608482, 0.650088, 0.707295, 0.764503, 0.821711, 0.884119, 0.946527, 1.008936, 1.071344, 1.133753, 1.196161],
    a_dmg2: [0.451483, 0.488231, 0.52498, 0.577478, 0.614227, 0.656225, 0.713973, 0.771721, 0.829468, 0.892466, 0.955464, 1.018461, 1.081459, 1.144456, 1.207454],
    a_dmg3: [0.559198, 0.604714, 0.65023, 0.715253, 0.760769, 0.812787, 0.884313, 0.955838, 1.027363, 1.105391, 1.183419, 1.261446, 1.339474, 1.417501, 1.495529],
    a_dmg4: [0.320389, 0.346467, 0.372545, 0.4098, 0.435878, 0.465681, 0.506661, 0.547641, 0.588621, 0.633327, 0.678032, 0.722737, 0.767443, 0.812148, 0.856853],
    a_dmg5: [0.767946, 0.830453, 0.89296, 0.982256, 1.044763, 1.1162, 1.214426, 1.312651, 1.410877, 1.518032, 1.625187, 1.732342, 1.839498, 1.946653, 2.053808],
    z_dmg: [1.03028, 1.11414, 1.198, 1.3178, 1.40166, 1.4975, 1.62928, 1.76106, 1.89284, 2.0366, 2.18036, 2.32412, 2.46788, 2.61164, 2.7554],
    x_dmg1: [0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572, 1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982],
    x_dmg2: [1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646, 2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915],
    x_dmg3: [1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586, 3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041],

    // Elemental Skill: Ancient Rite: Arcane Light
    e_dmg1: [0.58248, 0.626166, 0.669852, 0.7281, 0.771786, 0.815472, 0.87372, 0.931968, 0.990216, 1.048464, 1.106712, 1.16496, 1.23777, 1.31058, 1.38339],
    e_dmg2: [0.587976, 0.632074, 0.676172, 0.73497, 0.779068, 0.823166, 0.881964, 0.940762, 0.999559, 1.058357, 1.117154, 1.175952, 1.249449, 1.322946, 1.396443],
    e_dmg3: [0.728256, 0.782875, 0.837494, 0.91032, 0.964939, 1.019558, 1.092384, 1.16521, 1.238035, 1.310861, 1.383686, 1.456512, 1.547544, 1.638576, 1.729608],
    e_dmg4: [0.417252, 0.448546, 0.47984, 0.521565, 0.552859, 0.584153, 0.625878, 0.667603, 0.709328, 0.751054, 0.792779, 0.834504, 0.88666, 0.938817, 0.990973],
    e_dmg5: [1.000112, 1.07512, 1.150129, 1.25014, 1.325148, 1.400157, 1.500168, 1.600179, 1.70019, 1.800202, 1.900213, 2.000224, 2.125238, 2.250252, 2.375266],
    e_dmgz: [1.1496, 1.23582, 1.32204, 1.437, 1.52322, 1.60944, 1.7244, 1.83936, 1.95432, 2.06928, 2.18424, 2.2992, 2.4429, 2.5866, 2.7303],
    e_ns_dmg: [1.784, 1.9178, 2.0516, 2.23, 2.3638, 2.4976, 2.676, 2.8544, 3.0328, 3.2112, 3.3896, 3.568, 3.791, 4.014, 4.237],

    // Elemental Burst: Ancient Ritual: Cometh the Night
    q_dmg1: [2.5984, 2.79328, 2.98816, 3.248, 3.44288, 3.63776, 3.8976, 4.15744, 4.41728, 4.67712, 4.93696, 5.1968, 5.5216, 5.8464, 6.1712],
    q_dmg2: [0.1624, 0.17458, 0.18676, 0.203, 0.21518, 0.22736, 0.2436, 0.25984, 0.27608, 0.29232, 0.30856, 0.3248, 0.3451, 0.3654, 0.3857],
    q_dmg3: [1.16928, 1.256976, 1.344672, 1.4616, 1.549296, 1.636992, 1.75392, 1.870848, 1.987776, 2.104704, 2.221632, 2.33856, 2.48472, 2.63088, 2.77704],
    q_ts_dmg: [0.71456, 0.768152, 0.821744, 0.8932, 0.946792, 1.000384, 1.07184, 1.143296, 1.214752, 1.286208, 1.357664, 1.42912, 1.51844, 1.60776, 1.69708],
    q_tsa_dmg: [1.03936, 1.117312, 1.195264, 1.2992, 1.377152, 1.455104, 1.55904, 1.662976, 1.766912, 1.870848, 1.974784, 2.07872, 2.20864, 2.33856, 2.46848],

    c2_dmg: 0.5,
};

pub const FLINS_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Flins,
    internal_name: "Flins",
    element: Element::Electro,
    hp: [972, 2522, 3356, 5022, 5614, 6459, 7249, 8103, 8695, 9557, 10149, 11020, 11613, 12491, 13379],
    atk: [27, 71, 94, 141, 158, 182, 204, 228, 245, 269, 286, 310, 327, 352, 431],
    def: [63, 163, 217, 325, 363, 418, 469, 524, 563, 619, 657, 713, 752, 809, 866],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Polearm,
    star: 5,
    skill_name1: locale!(
        zh_cn: "扈圣魔枪",
        en: "Cyclonic Duster",
    ),
    skill_name2: locale!(
        zh_cn: "古律 · 孤灯遗秘",
        en: "Ancient Rite: Arcane Light",
    ),
    skill_name3: locale!(
        zh_cn: "旧仪 · 夜客致访",
        en: "Ancient Ritual: Cometh the Night",
    ),
    name_locale: locale!(
        zh_cn: "菲林斯",
        en: "Flins",
    )
};

pub struct FlinsEffect {
    pub has_p1: bool,
    pub has_p2: bool,
    pub has_c2: bool,
    pub has_c4: bool,
    pub has_c6: bool,
    pub moonsign: Moonsign,
}

impl<A: Attribute> ChangeAttribute<A> for FlinsEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_p1 && self.moonsign.is_ascendant() {
            attribute.set_value_by(AttributeName::EnhanceLunarCharged, "菲林斯天赋：寒冬的交响", 0.2);
        }

        if self.has_p2 {
            attribute.add_edge1(
                AttributeName::ATK,
                AttributeName::ElementalMastery,
                Box::new(move |atk, _| {
                    (atk * 0.08).min(160.0)
                }),
                Box::new(move |atk, _, grad| (0.0, 0.0)),
                "菲林斯天赋：幽焰的呢喃"
            );
        }

        attribute.add_edge1(
            AttributeName::ATK,
            AttributeName::IncreaseLunarCharged,
            Box::new(move |atk, _| {
                (atk * 0.00007).min(0.14)
            }),
            Box::new(move |atk, _, grad| (0.0, 0.0)),
            "菲林斯天赋：月兆祝赐·旧世潜藏"
        );

        if self.has_c2 && self.moonsign.is_ascendant() {
            attribute.set_value_by(AttributeName::ResMinusElectro, "菲林斯命座：渡越魍魉之墙", 0.25);
        }

        if self.has_c4 {
            attribute.add_atk_percentage("菲林斯命座：荒山嘶啭之夜", 0.2);
            if self.has_p2 {
                attribute.add_edge1(
                    AttributeName::ATK,
                    AttributeName::ElementalMastery,
                    Box::new(move |atk, _| {
                        (atk * 0.1).min(220.0)
                    }),
                    Box::new(move |atk, _, grad| (0.0, 0.0)),
                    "菲林斯命座：荒山嘶啭之夜"
                );
            }
        }

        if self.has_c6 {
            attribute.set_value_by(AttributeName::IncreaseLunarCharged, "菲林斯命座：歌与亡者之舞", if self.moonsign.is_ascendant() { 0.45 } else { 0.35 });
        }
    }
}

damage_enum!(
    FlinsDamageEnum
    A1
    A2
    A3
    A4
    A5
    Z
    X1
    X2
    X3
    E1
    E2
    E3
    E4
    E5
    ENS
    Q1
    Q2
    Q3
    QTS
    QTSA
    C2
    LunarCharged
);

impl FlinsDamageEnum {
    pub fn get_element(&self) -> Element {
        use FlinsDamageEnum::*;
        match *self {
            E1 | E2 | E3 | E4 | E5| ENS | Q1 | Q2 | Q3 | QTS | QTSA | C2 | LunarCharged => Element::Electro,
            _ => Element::Physical,
        }
    }

    pub fn get_lunar_type(&self) -> MoonglareReaction {
        use FlinsDamageEnum::*;
        match *self {
            LunarCharged => MoonglareReaction::LunarChargedReaction,
            Q2 | Q3 | QTS | QTSA | C2 => MoonglareReaction::LunarCharged,
            _ => MoonglareReaction::None,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use FlinsDamageEnum::*;
        match *self {
            A1 | A2 | A3 | A4 | A5 => SkillType::NormalAttack,
            Z => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E1 | E2 | E3 | E4 | E5 | ENS => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
            Q2 | Q3 | QTS | QTSA | C2 | LunarCharged => SkillType::Moonglare,
        }
    }
}

pub struct Flins;

impl CharacterTrait for Flins {
    const STATIC_DATA: CharacterStaticData = FLINS_STATIC_DATA;
    type SkillType = FlinsSkillType;
    const SKILL: Self::SkillType = FLINS_SKILL;
    type DamageEnumType = FlinsDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            FlinsDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            A4 hit_n_dmg!(4)
            A5 hit_n_dmg!(5)
            Z charged_dmg!()
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            FlinsDamageEnum
            E1 locale!(zh_cn: "一段伤害", en: "1-Hit DMG")
            E2 locale!(zh_cn: "二段伤害", en: "2-Hit DMG")
            E3 locale!(zh_cn: "三段伤害", en: "3-Hit DMG")
            E4 locale!(zh_cn: "三段伤害", en: "3-Hit DMG")
            E5 locale!(zh_cn: "三段伤害", en: "3-Hit DMG")
            ENS locale!(zh_cn: "北国枪阵伤害", en: "Northland Spearstorm DMG")
            C2 locale!(zh_cn: "二命额外伤害", en: "C2 extra DMG")
            LunarCharged locale!(zh_cn: "月感电伤害", en: "Lunar-Charged DMG")
        ),
        skill3: skill_map!(
            FlinsDamageEnum
            Q1 locale!(zh_cn: "技能初始伤害", en: "Initial Skill DMG")
            Q2 locale!(zh_cn: "中间段月感电伤害", en: "Middle Phase Lunar-Charged DMG")
            Q3 locale!(zh_cn: "最终段月感电伤害", en: "Final Phase Lunar-Charged DMG")
            QTS locale!(zh_cn: "雷霆交响伤害", en: "Thunderous Symphony DMG")
            QTSA locale!(zh_cn: "雷霆交响额外伤害", en: "Thunderous Symphony Additional DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN2
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: FlinsDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use FlinsDamageEnum::*;
        let mut builder = D::new();

        if s.get_skill_type() == SkillType::Moonglare {
            let ratio = match s {
                Q2 => FLINS_SKILL.q_dmg2[s3],
                Q3 => FLINS_SKILL.q_dmg3[s3],
                QTS => FLINS_SKILL.q_ts_dmg[s3],
                QTSA => FLINS_SKILL.q_tsa_dmg[s3],
                C2 => FLINS_SKILL.c2_dmg,
                LunarCharged => 0.0,
                _ => 0.0
            };

            builder.add_atk_ratio("技能倍率", ratio);

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
            let ratio = match s {
                A1 => FLINS_SKILL.a_dmg1[s1],
                A2 => FLINS_SKILL.a_dmg2[s1],
                A3 => FLINS_SKILL.a_dmg3[s1] * 2.0,
                A4 => FLINS_SKILL.a_dmg4[s1],
                A5 => FLINS_SKILL.a_dmg5[s1],
                Z => FLINS_SKILL.z_dmg[s1],
                X1 => FLINS_SKILL.x_dmg1[s1],
                X2 => FLINS_SKILL.x_dmg2[s1],
                X3 => FLINS_SKILL.x_dmg3[s1],
                E1 => FLINS_SKILL.e_dmg1[s2],
                E2 => FLINS_SKILL.e_dmg2[s2],
                E3 => FLINS_SKILL.e_dmg3[s2],
                E4 => FLINS_SKILL.e_dmg4[s2],
                E5 => FLINS_SKILL.e_dmg5[s2],
                ENS => FLINS_SKILL.e_ns_dmg[s2],
                Q1 => FLINS_SKILL.q_dmg1[s3],
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
        Some(Box::new(FlinsEffect {
            has_p1: common_data.has_talent1,
            has_p2: common_data.has_talent2,
            has_c2: common_data.constellation >= 2,
            has_c4: common_data.constellation >= 4,
            has_c6: common_data.constellation >= 6,
            moonsign: match config {
                    CharacterConfig::Flins { moonsign } => *moonsign,
                    _ => Moonsign::None,
                }
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
