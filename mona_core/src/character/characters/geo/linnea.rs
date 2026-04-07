use crate::{character::characters::prelude::*, utils};

pub struct LinneaSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub z_dmg1: [f64; 15],
    pub z_dmg2: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],
    pub e_dmg3: [f64; 15],

    pub q_heal: [f64; 15],
    pub q_heal_def: [f64; 15],
    pub q_heal_c: [f64; 15],
    pub q_heal_c_def: [f64; 15],
}

pub const LINNEA_SKILL: LinneaSkillType = LinneaSkillType {
    // Normal Attack: Capture Protocol
    a_dmg1: [0.59, 0.638, 0.686, 0.7546, 0.8026, 0.8575, 0.933, 1.0084, 1.0839, 1.1662, 1.2485, 1.3309, 1.4132, 1.4955, 1.5778],
    a_dmg2: [0.5115, 0.5532, 0.5948, 0.6543, 0.6959, 0.7435, 0.8089, 0.8743, 0.9398, 1.0111, 1.0825, 1.1539, 1.2253, 1.2966, 1.368],
    a_dmg3: [0.8163, 0.8828, 0.9492, 1.0441, 1.1106, 1.1865, 1.2909, 1.3953, 1.4997, 1.6136, 1.7275, 1.8414, 1.9554, 2.0693, 2.1832],
    z_dmg1: [0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894, 1.0506, 1.1118, 1.173],
    z_dmg2: [1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635, 2.79, 2.945],
    x_dmg1: [0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.826, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027, 1.282, 1.3612, 1.4405, 1.5198],
    x_dmg2: [1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.797, 1.9423, 2.0877, 2.2462, 2.4048, 2.5634, 2.7219, 2.8805, 3.039],
    x_dmg3: [1.4193, 1.5349, 1.6504, 1.8154, 1.931, 2.063, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037, 3.2018, 3.3998, 3.5979, 3.7959],

    // Elemental Skill: Countermeasure: Lumi's Battle Cry!
    e_dmg1: [0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16, 2.28],
    e_dmg2: [1.0, 1.075, 1.15, 1.25, 1.325, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.125, 2.25, 2.375],
    e_dmg3: [4.0, 4.3, 4.6, 5.0, 5.3, 5.6, 6.0, 6.4, 6.8, 7.2, 7.6, 8.0, 8.5, 9.0, 9.5],

    // Elemental Burst: Memo: Survival Guide in Extreme Conditions
    q_heal: [770.38, 847.42, 930.89, 1020.78, 1117.09, 1219.82, 1328.98, 1444.55, 1566.54, 1694.95, 1829.79, 1971.04, 2118.72, 2272.82, 2433.33],
    q_heal_def: [1.6, 1.72, 1.84, 2.0, 2.12, 2.24, 2.4, 2.56, 2.72, 2.88, 3.04, 3.2, 3.4, 3.6, 3.8],
    q_heal_c: [154.08, 169.48, 186.18, 204.16, 223.42, 243.96, 265.8, 288.91, 313.31, 338.99, 365.96, 394.21, 423.74, 454.56, 486.67],
    q_heal_c_def: [0.32, 0.344, 0.368, 0.4, 0.424, 0.448, 0.48, 0.512, 0.544, 0.576, 0.608, 0.64, 0.68, 0.72, 0.76],
};

pub const LINNEA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Linnea,
    internal_name: "Linnea",
    element: Element::Geo,
    hp: [770, 1998, 2659, 3978, 4447, 5117, 5742, 6419, 6888, 7570, 8040, 8730, 9199, 9895, 10598],
    atk: [11, 29, 39, 58, 65, 74, 83, 93, 100, 110, 117, 127, 133, 144, 176],
    def: [71, 183, 244, 365, 408, 469, 526, 588, 631, 694, 737, 800, 843, 907, 971],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Bow,
    star: 5,
    skill_name1: locale!(
        zh_cn: "捕获方案",
        en: "Capture Protocol",
    ),
    skill_name2: locale!(
        zh_cn: "对策·露米呀吼吼！",
        en: "Countermeasure: Lumi's Battle Cry!",
    ),
    skill_name3: locale!(
        zh_cn: "备忘·绝境生存指南",
        en: "Memo: Survival Guide in Extreme Conditions",
    ),
    name_locale: locale!(
        zh_cn: "莉奈娅",
        en: "Linnea",
    )
};

pub struct LinneaEffect {
    pub moonsign: Moonsign,
    pub moondrift_harmony: bool,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for LinneaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.common_data.has_talent1 {
            attribute.set_value_by_s(
                CharacterSelector::select_all(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_element(
                    AttributeVariableType::ResMinus,
                    Element::Geo,
                )),
                "莉奈娅天赋1",
                if self.moonsign.is_ascendant() { 0.30 } else { 0.15 },
            );
        }

        let has_on_field_moonsign = attribute.get_characters().iter().any(|character| {
            character.team_id == attribute.get_character().team_id
                && character.on_field
                && character.tags.contains(&CharacterTag::Moonsign)
        });
        if has_on_field_moonsign && self.common_data.has_talent2 {
            attribute.add_edge_s1to1(
                CharacterSelector::select_onfield(attribute),
                AttributeType::Panel(AttributeName::DEF),
                AttributeType::Panel(AttributeName::ElementalMastery),
                Arc::new(|def: f64, _| def * 0.05 ),
                "莉奈娅天赋2",
                EdgePriority::Common,
            )
        } else {
            attribute.add_edge_t1(
                AttributeType::Panel(AttributeName::DEF),
                AttributeType::Panel(AttributeName::ElementalMastery),
                Arc::new(|def: f64, _| def * 0.05 ),
                "莉奈娅天赋2",
                EdgePriority::Common,
            )
        }

        attribute.add_edge_s1to1(
            CharacterSelector::select_all(attribute),
            AttributeType::Panel(AttributeName::DEF),
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::MoonglareBase, ReactionType::LunarCrystallize)),
            Arc::new(|def: f64, _| (def / 100.0 * 0.007).min(0.14) ),
            "莉奈娅天赋3",
            EdgePriority::Invisible,
        );

        if self.moondrift_harmony && self.common_data.constellation >= 2 {
            attribute.set_value_by_s(
                CharacterSelector::select_element(attribute, Element::Hydro),
                AttributeType::Panel(AttributeName::CriticalDamageBase),
                "莉奈娅命座2",
                0.4
            );
            attribute.set_value_by_s(
                CharacterSelector::select_element(attribute, Element::Geo),
                AttributeType::Panel(AttributeName::CriticalDamageBase),
                "莉奈娅命座2",
                0.4
            );
        }

        if self.moondrift_harmony && self.common_data.constellation >= 4 {
            attribute.add_edge_s1ton(
                CharacterSelector::select_onfield(attribute),
                AttributeType::Panel(AttributeName::DEFBase),
                AttributeType::Panel(AttributeName::DEFPercentage),
                Arc::new(|def_base: f64, _| def_base * 0.25),
                "莉奈娅命座4",
                EdgePriority::Base,
            );
            attribute.add_edge_t1(
                AttributeType::Panel(AttributeName::DEFBase),
                AttributeType::Panel(AttributeName::DEFPercentage),
                Arc::new(|def_base: f64, _| def_base * 0.25),
                "莉奈娅命座4",
                EdgePriority::Base,
            );
        }

        if self.common_data.constellation >= 6 {
            attribute.set_value_by_s(
                CharacterSelector::select_all(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                    AttributeVariableType::MoonglareElevate,
                    ReactionType::LunarCrystallize,
                )),
                "莉奈娅命座6",
                0.25
            );
        }
    }
}

damage_enum!(
    LinneaDamageEnum
    A1
    A2
    A3
    Z1
    Z2
    X1
    X2
    X3
    E1
    E2
    E3
    Q
    QC
);

impl LinneaDamageEnum {
    pub fn get_element(&self) -> Element {
        use LinneaDamageEnum::*;
        match *self {
            A1 | A2 | A3 | Z1 | X1 | X2 | X3 => Element::Physical,
            Z2 | E1 | E2 | E3 | Q | QC => Element::Geo,
        }
    }

    pub fn get_lunar_type(&self) -> MoonglareReaction {
        use LinneaDamageEnum::*;
        match *self {
            E2 | E3 => MoonglareReaction::LunarCrystallize,
            _ => MoonglareReaction::None,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use LinneaDamageEnum::*;
        match *self {
            A1 | A2 | A3 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E1 => SkillType::ElementalSkill,
            Q | QC => SkillType::ElementalBurst,
            E2 | E3 => SkillType::Moonglare,
        }
    }
}

pub struct Linnea;

impl CharacterTrait for Linnea {
    const STATIC_DATA: CharacterStaticData = LINNEA_STATIC_DATA;
    type SkillType = LinneaSkillType;
    const SKILL: Self::SkillType = LINNEA_SKILL;
    type DamageEnumType = LinneaDamageEnum;
    type RoleEnum = ();

    const DEFAULT_TAGS: Option<&'static [CharacterTag]> = Some(
        &[CharacterTag::Moonsign]
    );

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            LinneaDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            Z1 charged_dmg!("shoot1")
            Z2 charged_dmg!("shoot2")
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            LinneaDamageEnum
            E1 locale!(zh_cn: "露米捶捶乱打伤害", en: "Lumi Pound-Pound Pummeler DMG")
            E2 locale!(zh_cn: "露米加力重锤伤害", en: "Lumi Heavy Overdrive Hammer DMG")
            E3 locale!(zh_cn: "露米百万吨重锤伤害", en: "Lumi Million Ton Crush DMG")
        ),
        skill3: skill_map!(
            LinneaDamageEnum
            Q locale!(zh_cn: "首次治疗量", en: "Initial Healing Amount")
            QC locale!(zh_cn: "持续治疗量", en: "Continuous Healing")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig {
            name: "moondrift_harmony",
            title: locale!(
                zh_cn: "月笼谐奏",
                en: "Moondrift Harmony",
            ),
            config: ItemConfigType::Bool { default: true }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "field_catalog",
            title: locale!(
                zh_cn: "「历览编录」",
                en: "Field Catalog"
            ),
            config: ItemConfigType::Int { min: 0, max: 18, default: 18 }
        },
    ]);

    fn change_attribute<A: Attribute>(attribute: &mut A, common_data: &CharacterCommonData, skill_config: &CharacterSkillConfig) {
        let (moonsign, moondrift_harmony) = match &common_data.config {
            CharacterConfig::Linnea { moonsign, moondrift_harmony } => (*moonsign, *moondrift_harmony),
            _ => (Moonsign::None, false),
        };

        let field_catalog = match &skill_config {
            CharacterSkillConfig::Linnea { field_catalog } => *field_catalog,
            _ => 0,
        };

        if common_data.constellation >= 1 && field_catalog > 0 {
            attribute.add_edge_s1to1(
                CharacterSelector::select_all(attribute),
                AttributeType::Panel(AttributeName::DEF),
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                    AttributeVariableType::ReactionExtra,
                    ReactionType::LunarCrystallize,
                )),
                if common_data.constellation >= 6 { Arc::new(|def: f64, _| def * 0.75 * 1.5) } else { Arc::new(|def: f64, _| def * 0.75) },
                "莉奈娅命座1",
                EdgePriority::Invisible,
            );
        }
    }

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: LinneaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (moonsign, moondrift_harmony) = match &context.character_common_data.config {
            CharacterConfig::Linnea { moonsign, moondrift_harmony } => (*moonsign, *moondrift_harmony),
            _ => (Moonsign::None, false),
        };

        let field_catalog = match &config {
            CharacterSkillConfig::Linnea { field_catalog } => *field_catalog,
            _ => 0,
        };

        use LinneaDamageEnum::*;
        let mut builder = D::new();

        if s == Q || s == QC {
            let ratio = match s {
                Q => LINNEA_SKILL.q_heal_def[s3],
                QC => LINNEA_SKILL.q_heal_c_def[s3],
                _ => unreachable!(),
            };
            let extra = match s {
                Q => LINNEA_SKILL.q_heal[s3],
                QC => LINNEA_SKILL.q_heal_c[s3],
                _ => unreachable!(),
            };
            builder.add_def_ratio("额外治疗量", ratio);
            builder.add_base("基础治疗量", extra);

            builder.heal(&context.attribute)
        } else {
            let ratio = match s {
                A1 => LINNEA_SKILL.a_dmg1[s1],
                A2 => LINNEA_SKILL.a_dmg2[s1],
                A3 => LINNEA_SKILL.a_dmg3[s1],
                Z1 => LINNEA_SKILL.z_dmg1[s2],
                Z2 => LINNEA_SKILL.z_dmg2[s2],
                X1 => LINNEA_SKILL.x_dmg1[s3],
                X2 => LINNEA_SKILL.x_dmg2[s3],
                X3 => LINNEA_SKILL.x_dmg3[s3],
                E1 => LINNEA_SKILL.e_dmg1[s2],
                E2 => LINNEA_SKILL.e_dmg2[s2],
                E3 => LINNEA_SKILL.e_dmg3[s2],
                _ => unreachable!(),
            };
    
            if s == E3 && context.character_common_data.constellation >= 2 {
                builder.add_extra_critical_damage("莉奈娅命座2", 1.5);
            }
    
            match s {
                A1 | A2 | A3 | Z1 | Z2 | X1 | X2 | X3 => {
                    builder.add_atk_ratio("技能倍率", ratio);
                },
                E1 | E2 | E3 => {
                    builder.add_def_ratio("技能倍率", ratio);
                },
                _ => unreachable!(),
            }
    
            if s.get_lunar_type() != MoonglareReaction::None {
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

    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (moonsign, moondrift_harmony) = match *config {
            CharacterConfig::Linnea { moonsign, moondrift_harmony } => (moonsign, moondrift_harmony),
            _ => (Moonsign::None, false),
        };
        Some(Box::new(LinneaEffect {
            moonsign,
            moondrift_harmony,
            common_data: common_data.clone(),
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
