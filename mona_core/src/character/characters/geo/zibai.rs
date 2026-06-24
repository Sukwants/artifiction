use crate::{buffs::buffs::common, character::{character_common_data, characters::prelude::*}};

pub struct ZibaiSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub a_dmg4: [f64; 15],
    pub z_dmg: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg_a1: [f64; 15],
    pub e_dmg_a2: [f64; 15],
    pub e_dmg_a3: [f64; 15],
    pub e_dmg_a4: [f64; 15],
    pub e_dmg_a4e: [f64; 15],
    pub e_dmg_z: [f64; 15],
    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],

    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],
}

pub const ZIBAI_SKILL: ZibaiSkillType = ZibaiSkillType {
    // Normal Attack: Golden Blade's Petaled Touch
    a_dmg1: [0.505542, 0.546691, 0.58784, 0.646624, 0.687773, 0.7348, 0.799462, 0.864125, 0.928787, 0.999328, 1.069869, 1.14041, 1.21095, 1.281491, 1.352032],
    a_dmg2: [0.465527, 0.503418, 0.54131, 0.595441, 0.633333, 0.676638, 0.736182, 0.795726, 0.85527, 0.920227, 0.985184, 1.050141, 1.115099, 1.180056, 1.245013],
    a_dmg3: [0.308882, 0.334023, 0.359165, 0.395082, 0.420223, 0.448956, 0.488464, 0.527973, 0.567481, 0.61058, 0.65368, 0.69678, 0.73988, 0.78298, 0.826079],
    a_dmg4: [0.778954, 0.842357, 0.90576, 0.996336, 1.059739, 1.1322, 1.231834, 1.331467, 1.431101, 1.539792, 1.648483, 1.757174, 1.865866, 1.974557, 2.083248],
    z_dmg: [0.73659, 0.796545, 0.8565, 0.94215, 1.002105, 1.070625, 1.16484, 1.259055, 1.35327, 1.45605, 1.55883, 1.66161, 1.76439, 1.86717, 1.96995],
    x_dmg1: [0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572, 1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982],
    x_dmg2: [1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646, 2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915],
    x_dmg3: [1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586, 3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041],

    // Elemental Skill: Heaven and Earth Made Manifest
    e_dmg_a1: [0.565792, 0.608226, 0.650661, 0.70724, 0.749674, 0.792109, 0.848688, 0.905267, 0.961846, 1.018426, 1.075005, 1.131584, 1.202308, 1.273032, 1.343756],
    e_dmg_a2: [0.521007, 0.560083, 0.599158, 0.651259, 0.690335, 0.72941, 0.781511, 0.833612, 0.885712, 0.937813, 0.989914, 1.042014, 1.10714, 1.172266, 1.237392],
    e_dmg_a3: [0.345694, 0.371621, 0.397548, 0.432117, 0.458044, 0.483971, 0.51854, 0.55311, 0.587679, 0.622248, 0.656818, 0.691387, 0.734599, 0.777811, 0.821022],
    e_dmg_a4: [0.871788, 0.937172, 1.002556, 1.089735, 1.155119, 1.220503, 1.307682, 1.394861, 1.48204, 1.569218, 1.656397, 1.743576, 1.852549, 1.961523, 2.070496],
    e_dmg_z: [0.6595, 0.708962, 0.758425, 0.824375, 0.873838, 0.9233, 0.98925, 1.0552, 1.12115, 1.1871, 1.25305, 1.319, 1.401437, 1.483875, 1.566312],
    e_dmg1: [1.72528, 1.854676, 1.984072, 2.1566, 2.285996, 2.415392, 2.58792, 2.760448, 2.932976, 3.105504, 3.278032, 3.45056, 3.66622, 3.88188, 4.09754],
    e_dmg2: [1.40968, 1.515406, 1.621132, 1.7621, 1.867826, 1.973552, 2.11452, 2.255488, 2.396456, 2.537424, 2.678392, 2.81936, 2.99557, 3.17178, 3.34799],
    e_dmg_a4e: [0.29456, 0.316652, 0.338744, 0.3682, 0.390292, 0.412384, 0.44184, 0.471296, 0.500752, 0.530208, 0.559664, 0.58912, 0.62594, 0.66276, 0.69958],

    // Elemental Burst: Tri-Sphere Eminence
    q_dmg1: [1.2696, 1.36482, 1.46004, 1.587, 1.68222, 1.77744, 1.9044, 2.03136, 2.15832, 2.28528, 2.41224, 2.5392, 2.6979, 2.8566, 3.0153],
    q_dmg2: [1.77744, 1.910748, 2.044056, 2.2218, 2.355108, 2.488416, 2.66616, 2.843904, 3.021648, 3.199392, 3.377136, 3.55488, 3.77706, 3.99924, 4.22142],
};

pub const ZIBAI_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Zibai,
    internal_name: "Zibai",
    element: Element::Geo,
    hp: [1006, 2609, 3471, 5194, 5807, 6681, 7498, 8381, 8994, 9885, 10497, 11399, 12011, 12919, 13838],
    atk: [18, 45, 60, 90, 101, 116, 130, 146, 157, 172, 183, 198, 209, 225, 275],
    def: [74, 193, 257, 385, 430, 495, 555, 621, 666, 732, 777, 844, 890, 957, 1025],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Sword,
    star: 5,
    skill_name1: locale!(
        zh_cn: "金铗点桂",
        en: "Golden Blade's Petaled Touch",
    ),
    skill_name2: locale!(
        zh_cn: "天地忽然身",
        en: "Heaven and Earth Made Manifest",
    ),
    skill_name3: locale!(
        zh_cn: "三垣威仪法",
        en: "Tri-Sphere Eminence",
    ),
    name_locale: locale!(
        zh_cn: "兹白",
        en: "Zibai",
    )
};

pub struct ZibaiEffect {
    pub moonsign: Moonsign,
    pub geo_count: usize,
    pub hydro_count: usize,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for ZibaiEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.common_data.has_talent2 {
            if self.geo_count > 1 {
                attribute.add_def_percentage("兹白天赋2", (self.geo_count - 1) as f64 * 0.15);
            }
            if self.hydro_count > 0 {
                attribute.set_value_by(AttributeName::ElementalMastery, "兹白天赋2", self.hydro_count as f64 * 60.0);
            }
        }
        
        attribute.add_edge_s1to1(
            CharacterSelector::select_all(attribute),
            AttributeType::Panel(AttributeName::DEF),
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ElevativeBase, ReactionType::LunarCrystallize)),
            Arc::new(|def: f64, _| (def / 100.0 * 0.007).min(0.14) ),
            "兹白天赋3",
            EdgePriority::Invisible,
        );
    }
}

damage_enum!(
    ZibaiDamageEnum
    A1
    A2
    A31
    A32
    A4
    Z1
    Z2
    X1
    X2
    X3
    EA1
    EA2
    EA31
    EA32
    EA4
    EA4E
    EZ1
    EZ2
    E1
    E2
    Q1
    Q2
);

impl ZibaiDamageEnum {
    pub fn get_element(&self) -> Element {
        use ZibaiDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 | A4 | Z1 | Z2 | X1 | X2 | X3 => Element::Physical,
            EA1 | EA2 | EA31 | EA32 | EA4 | EA4E | EZ1 | EZ2 | E1 | E2 | Q1 | Q2 => Element::Geo,
        }
    }

    pub fn get_lunar_type(&self) -> ElevativeReaction {
        use ZibaiDamageEnum::*;
        match *self {
            EA4E | E2 | Q2 => ElevativeReaction::LunarCrystallize,
            _ => ElevativeReaction::None,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use ZibaiDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 | A4 | EA1 | EA2 | EA31 | EA32 | EA4 => SkillType::NormalAttack,
            Z1 | Z2 | EZ1 | EZ2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q1 => SkillType::ElementalBurst,
            EA4E | E2 | Q2 => SkillType::Elevative,
        }
    }
}

pub struct Zibai;

impl CharacterTrait for Zibai {
    const STATIC_DATA: CharacterStaticData = ZIBAI_STATIC_DATA;
    type SkillType = ZibaiSkillType;
    const SKILL: Self::SkillType = ZIBAI_SKILL;
    type DamageEnumType = ZibaiDamageEnum;
    type RoleEnum = ();

    const DEFAULT_TAGS: Option<&'static [CharacterTag]> = Some(
        &[CharacterTag::Moonsign]
    );

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ZibaiDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A31 hit_n_dmg!(3, 1)
            A32 hit_n_dmg!(3, 2)
            A4 hit_n_dmg!(4)
            Z1 charged_dmg!(1)
            Z2 charged_dmg!(2)
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            ZibaiDamageEnum
            EA1 locale!(zh_cn: "月转时隙一段伤害", en: "Lunar Phase Shift 1-Hit DMG")
            EA2 locale!(zh_cn: "月转时隙二段伤害", en: "Lunar Phase Shift 2-Hit DMG")
            EA31 locale!(zh_cn: "月转时隙三段伤害-1", en: "Lunar Phase Shift 3-Hit DMG-1")
            EA32 locale!(zh_cn: "月转时隙三段伤害-2", en: "Lunar Phase Shift 3-Hit DMG-2")
            EA4 locale!(zh_cn: "月转时隙四段伤害", en: "Lunar Phase Shift 4-Hit DMG")
            EZ1 locale!(zh_cn: "月转时隙重击伤害-1", en: "Lunar Phase Shift Charged Attack DMG-1")
            EZ2 locale!(zh_cn: "月转时隙重击伤害-2", en: "Lunar Phase Shift Charged Attack DMG-2")
            E1 locale!(zh_cn: "灵驹飞踏第一段伤害", en: "Spirit Steed's Stride 1-Hit DMG")
            E2 locale!(zh_cn: "灵驹飞踏第二段伤害", en: "Spirit Steed's Stride 2-Hit DMG")
            EA4E locale!(zh_cn: "月转时隙第四段额外伤害", en: "Lunar Phase Shift 4-Hit Ascendant Gleam DMG")
        ),
        skill3: skill_map!(
            ZibaiDamageEnum
            Q1 locale!(zh_cn: "技能第一段伤害", en: "Skill 1-Hit DMG")
            Q2 locale!(zh_cn: "技能第二段伤害", en: "Skill 2-Hit DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig {
            name: "geo_count",
            title: locale!(
                zh_cn: "岩元素角色数量",
                en: "Number of Geo Characters"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 1 }
        },
        ItemConfig {
            name: "hydro_count",
            title: locale!(
                zh_cn: "水元素角色数量",
                en: "Number of Hydro Characters"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 0 }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "lunar_phase_shift",
            title: locale!(
                zh_cn: "处于「月转时隙」模式",
                en: "In the Lunar Phase Shift mode"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "activated_c1",
            title: locale!(
                zh_cn: "一命加成",
                en: "Activate C1 Bonus"
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "activated_c4",
            title: locale!(
                zh_cn: "四命加成",
                en: "Activate C4 Bonus"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "stack_c6",
            title: locale!(
                zh_cn: "六命效果层数",
                en: "Number of C6 effect stacks"
            ),
            config: ItemConfigType::Int { min: 0, max: 30, default: 30 }
        },
    ]);

    fn change_attribute<A: Attribute>(attribute: &mut A, common_data: &CharacterCommonData, skill_config: &CharacterSkillConfig) {
        let (moonsign, geo_count, hydro_count) = match &common_data.config {
            CharacterConfig::Zibai { moonsign, geo_count, hydro_count } => (*moonsign, *geo_count, *hydro_count),
            _ => (Moonsign::None, 0, 0),
        };

        let (lunar_phase_shift, activated_c1, activated_c4, stack_c6) = match &skill_config {
            CharacterSkillConfig::Zibai { lunar_phase_shift, activated_c1, activated_c4, stack_c6 } => (*lunar_phase_shift, *activated_c1, *activated_c4, *stack_c6),
            _ => (false, false, false, 0),
        };

        if common_data.constellation >= 2 && lunar_phase_shift {
            attribute.set_value_by_s(
                CharacterSelector::select_all(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                    AttributeVariableType::ReactionEnhance,
                    ReactionType::LunarCrystallize,
                )),
                "兹白命座2",
                0.30,
            );
        }

        if common_data.constellation >= 6 && lunar_phase_shift {
            attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::ReactionEnhance,
                ReactionType::LunarCrystallize,
            )), "兹白命座6", stack_c6 as f64 * 0.016);
        }
    }

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ZibaiDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (moonsign, geo_count, hydro_count) = match &context.character_common_data.config {
            CharacterConfig::Zibai { moonsign, geo_count, hydro_count } => (*moonsign, *geo_count, *hydro_count),
            _ => (Moonsign::None, 0, 0),
        };

        let (lunar_phase_shift, activated_c1, activated_c4, stack_c6) = match &config {
            CharacterSkillConfig::Zibai { lunar_phase_shift, activated_c1, activated_c4, stack_c6 } => (*lunar_phase_shift, *activated_c1, *activated_c4, *stack_c6),
            _ => (false, false, false, 0),
        };

        use ZibaiDamageEnum::*;
        let mut builder = D::new();

        let ratio = match s {
            A1 => ZIBAI_SKILL.a_dmg1[s1],
            A2 => ZIBAI_SKILL.a_dmg2[s1],
            A31 => ZIBAI_SKILL.a_dmg3[s1],
            A32 => ZIBAI_SKILL.a_dmg3[s1],
            A4 => ZIBAI_SKILL.a_dmg4[s1],
            Z1 => ZIBAI_SKILL.z_dmg[s2],
            Z2 => ZIBAI_SKILL.z_dmg[s2],
            X1 => ZIBAI_SKILL.x_dmg1[s3],
            X2 => ZIBAI_SKILL.x_dmg2[s3],
            X3 => ZIBAI_SKILL.x_dmg3[s3],
            EA1 => ZIBAI_SKILL.e_dmg_a1[s2],
            EA2 => ZIBAI_SKILL.e_dmg_a2[s2],
            EA31 => ZIBAI_SKILL.e_dmg_a3[s2],
            EA32 => ZIBAI_SKILL.e_dmg_a3[s2],
            EA4 => ZIBAI_SKILL.e_dmg_a4[s2],
            EA4E => ZIBAI_SKILL.e_dmg_a4e[s2] * if context.character_common_data.constellation >= 4 && activated_c4 { 2.50 } else { 1.0 },
            EZ1 => ZIBAI_SKILL.e_dmg_z[s2],
            EZ2 => ZIBAI_SKILL.e_dmg_z[s2],
            E1 => ZIBAI_SKILL.e_dmg1[s2],
            E2 => ZIBAI_SKILL.e_dmg2[s2],
            Q1 => ZIBAI_SKILL.q_dmg1[s3],
            Q2 => ZIBAI_SKILL.q_dmg2[s3],
        };

        if s == E2 {
            if context.character_common_data.constellation >= 1 && activated_c1 {
                builder.add_extra_reaction_enhance("兹白命座1", 2.20);
            }

            if context.character_common_data.has_talent1 {
                if context.character_common_data.constellation >= 2 && moonsign.is_ascendant() {
                    builder.add_extra_reaction_extra("兹白天赋1", context.attribute.get_def() * 6.10);
                } else {
                    builder.add_extra_reaction_extra("兹白天赋1", context.attribute.get_def() * 0.60);
                }
            }

        }

        if s == EA4E && !moonsign.is_ascendant() {
            return builder.none();
        }

        match s {
            A1 | A2 | A31 | A32 | A4 | Z1 | Z2 | X1 | X2 | X3 => {
                builder.add_atk_ratio("技能倍率", ratio);
            }
            EA1 | EA2 | EA31 | EA32 | EA4 | EA4E | EZ1 | EZ2 | E1 | E2 | Q1 | Q2 => {
                builder.add_def_ratio("技能倍率", ratio);
            }
        }

        if s.get_lunar_type() != ElevativeReaction::None {
            builder.elevative(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                s.get_lunar_type(),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
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
        let (moonsign, geo_count, hydro_count) = match *config {
            CharacterConfig::Zibai { moonsign, geo_count, hydro_count } => (moonsign, geo_count, hydro_count),
            _ => (Moonsign::None, 0, 0),
        };
        Some(Box::new(ZibaiEffect {
            moonsign,
            geo_count,
            hydro_count,
            common_data: common_data.clone(),
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
