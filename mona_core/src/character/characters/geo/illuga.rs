use crate::{buffs::buffs::common, character::{character_common_data, characters::prelude::*}};

pub struct IllugaSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub a_dmg4: [f64; 15],
    pub z_dmg: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg_p_em: [f64; 15],
    pub e_dmg_p_def: [f64; 15],
    pub e_dmg_h_em: [f64; 15],
    pub e_dmg_h_def: [f64; 15],

    pub q_dmg_em: [f64; 15],
    pub q_dmg_def: [f64; 15],
    pub q_increase_geo: [f64; 15],
    pub q_increase_lunar: [f64; 15],
}

pub const ILLUGA_SKILL: IllugaSkillType = IllugaSkillType {
    // Normal Attack: Oathkeeper's Spear
    a_dmg1: [0.473662, 0.512216, 0.55077, 0.605847, 0.644401, 0.688462, 0.749047, 0.809632, 0.870217, 0.936309, 1.002401, 1.068494, 1.134586, 1.200679, 1.266771],
    a_dmg2: [0.485255, 0.524753, 0.56425, 0.620675, 0.660172, 0.705313, 0.76738, 0.829448, 0.891515, 0.959225, 1.026935, 1.094645, 1.162355, 1.230065, 1.297775],
    a_dmg3: [0.31433, 0.339915, 0.3655, 0.40205, 0.427635, 0.456875, 0.49708, 0.537285, 0.57749, 0.62135, 0.66521, 0.70907, 0.75293, 0.79679, 0.84065],
    a_dmg4: [0.762786, 0.824873, 0.88696, 0.975656, 1.037743, 1.1087, 1.206266, 1.303831, 1.401397, 1.507832, 1.614267, 1.720702, 1.827138, 1.933573, 2.04],
    z_dmg: [1.11026, 1.20063, 1.291, 1.4201, 1.51047, 1.61375, 1.75576, 1.89777, 2.03978, 2.1947, 2.34962, 2.50454, 2.65946, 2.81438, 2.9693],
    x_dmg1: [0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572, 1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982],
    x_dmg2: [1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646, 2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915],
    x_dmg3: [1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586, 3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041],

    // Elemental Skill: Dawnbearing Songbird
    e_dmg_h_em: [4.8256, 5.18752, 5.54944, 6.032, 6.39392, 6.75584, 7.2384, 7.72096, 8.20352, 8.68608, 9.16864, 9.6512, 10.2544, 10.8576, 11.4608],
    e_dmg_h_def: [2.4128, 2.59376, 2.77472, 3.016, 3.19696, 3.37792, 3.6192, 3.86048, 4.10176, 4.34304, 4.58432, 4.8256, 5.1272, 5.4288, 5.7304],
    e_dmg_p_em: [6.032, 6.4844, 6.9368, 7.54, 7.9924, 8.4448, 9.048, 9.6512, 10.2544, 10.8576, 11.4608, 12.064, 12.818, 13.572, 14.326],
    e_dmg_p_def: [3.016, 3.2422, 3.4684, 3.77, 3.9962, 4.2224, 4.524, 4.8256, 5.1272, 5.4288, 5.7304, 6.032, 6.409, 6.786, 7.163],

    // Elemental Burst: Shadowless Reflection
    q_dmg_em: [8.272, 8.8924, 9.5128, 10.34, 10.9604, 11.5808, 12.408, 13.2352, 14.0624, 14.8896, 15.7168, 16.544, 17.578, 18.612, 19.646],
    q_dmg_def: [4.136, 4.4462, 4.7564, 5.17, 5.4802, 5.7904, 6.204, 6.6176, 7.0312, 7.4448, 7.8584, 8.272, 8.789, 9.306, 9.823],
    q_increase_geo: [0.336, 0.3612, 0.3864, 0.42, 0.4452, 0.4704, 0.504, 0.5376, 0.5712, 0.6048, 0.6384, 0.672, 0.714, 0.756, 0.798],
    q_increase_lunar: [2.2592, 2.42864, 2.59808, 2.824, 2.99344, 3.16288, 3.3888, 3.61472, 3.84064, 4.06656, 4.29248, 4.5184, 4.8008, 5.0832, 5.3656],
};

pub const ILLUGA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Illuga,
    internal_name: "Illuga",
    element: Element::Geo,
    hp: [1006, 2609, 3471, 5194, 5807, 6681, 7498, 8381, 8994, 9885, 10497, 11399, 12011, 12919, 13838],
    atk: [18, 45, 60, 90, 101, 116, 130, 146, 157, 172, 183, 198, 209, 225, 275],
    def: [74, 193, 257, 385, 430, 495, 555, 621, 666, 732, 777, 844, 890, 957, 1025],
    sub_stat: CharacterSubStatFamily::ElementalMastery96,
    weapon_type: WeaponType::Polearm,
    star: 4,
    skill_name1: locale!(
        zh_cn: "守誓枪术",
        en: "Oathkeeper's Spear",
    ),
    skill_name2: locale!(
        zh_cn: "衔莺破晓",
        en: "Dawnbearing Songbird",
    ),
    skill_name3: locale!(
        zh_cn: "鉴照无影",
        en: "Shadowless Reflection",
    ),
    name_locale: locale!(
        zh_cn: "叶洛亚",
        en: "Illuga",
    )
};

pub struct IllugaEffect {
    pub moonsign: Moonsign,
    pub hydro_geo_count: usize,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for IllugaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.common_data.has_talent1 {
            attribute.set_value_by_s(
                CharacterSelector::select_all_except_self(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::CriticalRate, Element::Geo)),
                "叶洛亚天赋1",
                if self.common_data.constellation >= 6 { 0.10 } else { 0.05 },
            );
            attribute.set_value_by_s(
                CharacterSelector::select_all_except_self(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::CriticalDamage, Element::Geo)),
                "叶洛亚天赋1",
                if self.common_data.constellation >= 6 { 0.30 } else { 0.10 },
            );

            if self.moonsign.is_ascendant() {
                attribute.set_value_by_s(
                    CharacterSelector::select_all_except_self(attribute),
                    AttributeType::Panel(AttributeName::ElementalMastery),
                    "叶洛亚天赋1",
                if self.common_data.constellation >= 6 { 80.0 } else { 50.0 },
                );
            }
        }
    }
}

damage_enum!(
    IllugaDamageEnum
    A1
    A2
    A31
    A32
    A4
    Z
    X1
    X2
    X3
    EP
    EH
    Q
    QIGeo
    QILunar
    C2
);

impl IllugaDamageEnum {
    pub fn get_element(&self) -> Element {
        use IllugaDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 | A4 | Z | X1 | X2 | X3 => Element::Physical,
            EP | EH | Q | C2 => Element::Geo,
            QIGeo | QILunar => panic!()
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use IllugaDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 | A4  => SkillType::NormalAttack,
            Z => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            EP | EH => SkillType::ElementalSkill,
            Q | C2 => SkillType::ElementalBurst,
            QIGeo | QILunar => panic!()
        }
    }
}

pub struct Illuga;

impl CharacterTrait for Illuga {
    const STATIC_DATA: CharacterStaticData = ILLUGA_STATIC_DATA;
    type SkillType = IllugaSkillType;
    const SKILL: Self::SkillType = ILLUGA_SKILL;
    type DamageEnumType = IllugaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            IllugaDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A31 hit_n_dmg!(3, 1)
            A32 hit_n_dmg!(3, 2)
            A4 hit_n_dmg!(4)
            Z charged_dmg!()
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            IllugaDamageEnum
            EP locale!(zh_cn: "点按伤害", en: "Press DMG")
            EH locale!(zh_cn: "长按伤害", en: "Hold DMG")
        ),
        skill3: skill_map!(
            IllugaDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
            QIGeo locale!(zh_cn: "岩元素伤害增加", en: "Geo DMG Bonus")
            QILunar locale!(zh_cn: "月结晶反应伤害增加", en: "Lunar-Crystallize Reaction DMG Bonus")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig {
            name: "hydro_geo_count",
            title: locale!(
                zh_cn: "水元素或岩元素角色数量",
                en: "Number of Hydro or Geo Characters"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 1 }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "nightingales_song",
            title: locale!(
                zh_cn: "存在「夜莺之歌」",
                en: "Has Nightingale's Song"
            ),
            config: ItemConfigType::Bool { default: true }
        },
    ]);

    fn change_attribute<A: Attribute>(attribute: &mut A, common_data: &CharacterCommonData, skill_config: &CharacterSkillConfig) {
        let (s1, s2, s3) = common_data.get_3_skill();

        let (moonsign, hydro_geo_count) = match &common_data.config {
            CharacterConfig::Illuga { moonsign, hydro_geo_count } => (*moonsign, *hydro_geo_count),
            _ => (Moonsign::None, 0),
        };

        let nightingales_song = match &skill_config {
            CharacterSkillConfig::Illuga { nightingales_song } => *nightingales_song,
            _ => false,
        };

        if nightingales_song {
            let extra_geo_ratio = if common_data.has_talent2 { [0.0, 0.07, 0.14, 0.24][hydro_geo_count.min(3)] } else { 0.0 };
            let extra_lunar_ratio = if common_data.has_talent2 { [0.0, 0.48, 0.96, 1.60][hydro_geo_count.min(3)] } else { 0.0 };
            
            attribute.add_edge_s1to1(
                CharacterSelector::select_all_onfield(attribute),
                AttributeType::Panel(AttributeName::ElementalMastery),
                AttributeType::Invisible(InvisibleAttributeType::new_element(
                    AttributeVariableType::BaseDamage,
                    Element::Geo,
                )),
                Arc::new(move |em: f64, _| em * (ILLUGA_SKILL.q_increase_geo[s3] + extra_geo_ratio) ),
                "叶洛亚Q技能",
                EdgePriority::Invisible,
            );
            attribute.add_edge_s1to1(
                CharacterSelector::select_all_onfield(attribute),
                AttributeType::Panel(AttributeName::ElementalMastery),
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                    AttributeVariableType::MoonglareBase,
                    ReactionType::LunarCrystallize,
                )),
                Arc::new(move |em: f64, _| em * (ILLUGA_SKILL.q_increase_lunar[s3] + extra_lunar_ratio) ),
                "叶洛亚Q技能",
                EdgePriority::Invisible,
            );
        }

        if common_data.constellation >= 4 && nightingales_song {
            attribute.set_value_by_s(
                CharacterSelector::select_all_onfield(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::ReactionEnhance,
                ReactionType::LunarCrystallize, 
            )), "叶洛亚命座4", 200.0);
        }
    }

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: IllugaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (moonsign, hydro_geo_count) = match &context.character_common_data.config {
            CharacterConfig::Illuga { moonsign, hydro_geo_count } => (*moonsign, *hydro_geo_count),
            _ => (Moonsign::None, 0),
        };

        let nightingales_song = match &config {
            CharacterSkillConfig::Illuga { nightingales_song } => *nightingales_song,
            _ => false,
        };

        use IllugaDamageEnum::*;
        let mut builder = D::new();

        let atk_ratio = match s {
            A1 => ILLUGA_SKILL.a_dmg1[s1],
            A2 => ILLUGA_SKILL.a_dmg2[s1],
            A31 => ILLUGA_SKILL.a_dmg3[s1],
            A32 => ILLUGA_SKILL.a_dmg3[s1],
            A4 => ILLUGA_SKILL.a_dmg4[s1],
            Z => ILLUGA_SKILL.z_dmg[s2],
            X1 => ILLUGA_SKILL.x_dmg1[s3],
            X2 => ILLUGA_SKILL.x_dmg2[s3],
            X3 => ILLUGA_SKILL.x_dmg3[s3],
            _ => 0.0,
        };
        let em_ratio = match s {
            EP => ILLUGA_SKILL.e_dmg_p_em[s2],
            EH => ILLUGA_SKILL.e_dmg_h_em[s2],
            Q => ILLUGA_SKILL.q_dmg_em[s3],
            QIGeo => ILLUGA_SKILL.q_increase_geo[s3],
            QILunar => ILLUGA_SKILL.q_increase_lunar[s3],
            _ => 0.0,
        };
        let def_ratio = match s {
            EP => ILLUGA_SKILL.e_dmg_p_def[s2],
            EH => ILLUGA_SKILL.e_dmg_h_def[s2],
            Q => ILLUGA_SKILL.q_dmg_def[s3],
            _ => 0.0,
        };

        if atk_ratio > 0.0 { builder.add_atk_ratio("技能倍率", atk_ratio); }
        if em_ratio > 0.0 { builder.add_em_ratio("技能倍率", em_ratio); }
        if def_ratio > 0.0 { builder.add_def_ratio("技能倍率", def_ratio); }

        if s == QIGeo || s == QILunar {
            if nightingales_song {
                if context.character_common_data.has_talent2 {
                    if s == QIGeo {
                        builder.add_em_ratio("天赋2额外倍率", [0.0, 0.07, 0.14, 0.24][hydro_geo_count.min(3)]);
                    } else if s == QILunar {
                        builder.add_em_ratio("天赋2额外倍率", [0.0, 0.48, 0.96, 1.60][hydro_geo_count.min(3)]);
                    }
                }

                builder.number(
                    &context.attribute,
                )
            } else {
                builder.none()
            }
        } else {
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
        let (moonsign, hydro_geo_count) = match *config {
            CharacterConfig::Illuga { moonsign, hydro_geo_count } => (moonsign, hydro_geo_count),
            _ => (Moonsign::None, 0),
        };
        Some(Box::new(IllugaEffect {
            moonsign,
            hydro_geo_count,
            common_data: common_data.clone(),
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
