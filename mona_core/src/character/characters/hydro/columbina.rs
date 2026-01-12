use crate::character::characters::prelude::*;

pub struct ColumbinaSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub z_dmg: [f64; 15],
    pub z_dmgm: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg: [f64; 15],
    pub e_dmggc: [f64; 15], // Gravity Ripple: Continuous DMG
    pub e_dmg_electro: [f64; 15], // Gravity Interference: Lunar-Charged DMG
    pub e_dmg_dendro: [f64; 15], // Gravity Interference: Lunar-Bloom DMG
    pub e_dmg_geo: [f64; 15], // Gravity Interference: Lunar-Crystallize DMG

    pub q_dmg: [f64; 15],
    pub q_enhance: [f64; 15],
}

pub const COLUMBINA_SKILL: ColumbinaSkillType = ColumbinaSkillType {
    // Normal Attack: Moondew Cascade
    a_dmg1: [0.46792, 0.503014, 0.538108, 0.5849, 0.62, 0.655088, 0.70188, 0.748672, 0.795464, 0.842256, 0.889048, 0.93584, 0.99433, 1.05282, 1.11131],
    a_dmg2: [0.366256, 0.393725, 0.421194, 0.45782, 0.485289, 0.512758, 0.549384, 0.58601, 0.622635, 0.659261, 0.695886, 0.732512, 0.778294, 0.824076, 0.869858],
    a_dmg3: [0.58484, 0.628703, 0.672566, 0.73105, 0.774913, 0.818776, 0.87726, 0.935744, 0.994228, 1.052712, 1.111196, 1.16968, 1.242785, 1.31589, 1.388995],
    z_dmg: [1.1608, 1.24786, 1.33492, 1.451, 1.53806, 1.62512, 1.7412, 1.85728, 1.97336, 2.08944, 2.20552, 2.3216, 2.4667, 2.6118, 2.7569],
    z_dmgm: [0.015112, 0.016245, 0.017379, 0.01889, 0.020023, 0.021157, 0.022668, 0.024179, 0.02569, 0.027202, 0.028713, 0.030224, 0.032113, 0.034002, 0.035891],
    x_dmg1: [0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064, 1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984],
    x_dmg2: [1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686, 2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036],
    x_dmg3: [1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632, 2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592],

    // Elemental Skill: Eternal Tides
    e_dmg: [0.1672, 0.17974, 0.19228, 0.209, 0.22154, 0.23408, 0.2508, 0.26752, 0.28424, 0.30096, 0.31768, 0.3344, 0.3553, 0.3762, 0.3971], 
    e_dmggc: [0.0936, 0.10062, 0.10764, 0.117, 0.12402, 0.13104, 0.1404, 0.14976, 0.15912, 0.16848, 0.17784, 0.1872, 0.1989, 0.2106, 0.2223],
    e_dmg_electro: [0.04704, 0.050568, 0.054096, 0.0588, 0.062328, 0.065856, 0.07056, 0.075264, 0.079968, 0.084672, 0.089376, 0.09408, 0.09996, 0.10584, 0.11172],
    e_dmg_dendro: [0.01408, 0.015136, 0.016192, 0.0176, 0.018656, 0.019712, 0.02112, 0.022528, 0.023936, 0.025344, 0.026752, 0.02816, 0.02992, 0.03168, 0.03344],
    e_dmg_geo: [0.08824, 0.094858, 0.101476, 0.1103, 0.116918, 0.123536, 0.13236, 0.141184, 0.15, 0.158832, 0.167656, 0.17648, 0.18751, 0.19854, 0.20957],

    // Elemental Burst: Moonlit Melancholy
    q_dmg: [0.3224, 0.34658, 0.37076, 0.403, 0.42718, 0.45136, 0.4836, 0.51584, 0.54808, 0.58032, 0.61256, 0.6448, 0.6851, 0.7254, 0.7657],
    q_enhance: [0.13, 0.16, 0.19, 0.22, 0.25, 0.28, 0.31, 0.34, 0.37, 0.4, 0.43, 0.46, 0.49, 0.52, 0.55],
};

pub const COLUMBINA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Columbina,
    internal_name: "Columbina",
    element: Element::Hydro,
    hp: [1144, 2967, 3948, 5908, 6605, 7599, 8528, 9533, 10230, 11243, 11940, 12965, 13662, 14695, 15740],
    atk: [7, 19, 26, 38, 43, 49, 56, 62, 67, 73, 78, 84, 89, 96, 117],
    def: [40, 104, 138, 207, 231, 266, 299, 334, 358, 394, 418, 454, 479, 515, 552],
    sub_stat: CharacterSubStatFamily::CriticalRate192,
    weapon_type: WeaponType::Catalyst,
    star: 5,
    skill_name1: locale!(
        zh_cn: "月露泼降",
        en: "Moondew Cascade",
    ),
    skill_name2: locale!(
        zh_cn: "万古潮汐",
        en: "Eternal Tides",
    ),
    skill_name3: locale!(
        zh_cn: "她的乡愁",
        en: "Moonlit Melancholy",
    ),
    name_locale: locale!(
        zh_cn: "哥伦比娅",
        en: "Columbina",
    )
};

pub struct ColumbinaEffect {
    pub moonsign: Moonsign,
    pub main_element: Option<Element>,
    pub reacted_element: ConfigElements8Multi,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for ColumbinaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.common_data.constellation >= 2 {
            attribute.add_hp_percentage("哥伦比娅命座2", 0.4);

            if self.moonsign.is_ascendant() {
                if let Some(element) = self.main_element {
                    match element {
                        Element::Electro => {
                            attribute.add_edge_s1to1(
                                CharacterSelector::select_onfield(attribute),
                                AttributeType::Panel(AttributeName::HP),
                                AttributeType::Panel(AttributeName::ATKFixed),
                                Arc::new(move |hp: f64, _| hp * 0.01),
                                "哥伦比娅命座2",
                                EdgePriority::Common,
                            );
                        },
                        Element::Dendro => {
                            attribute.add_edge_s1to1(
                                CharacterSelector::select_onfield(attribute),
                                AttributeType::Panel(AttributeName::HP),
                                AttributeType::Panel(AttributeName::ElementalMastery),
                                Arc::new(move |hp: f64, _| hp * 0.0035),
                                "哥伦比娅命座2",
                                EdgePriority::Common,
                            );
                        },
                        Element::Geo => {
                            attribute.add_edge_s1to1(
                                CharacterSelector::select_onfield(attribute),
                                AttributeType::Panel(AttributeName::HP),
                                AttributeType::Panel(AttributeName::DEFFixed),
                                Arc::new(move |hp: f64, _| hp * 0.01),
                                "哥伦比娅命座2",
                                EdgePriority::Common,
                            );
                        },
                        _ => panic!()
                    }
                }
            }
        }

        if self.common_data.constellation >= 6 {
            if self.reacted_element.hydro {
                attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Invisible(InvisibleAttributeType::new(
                        AttributeVariableType::CriticalDamage,
                        Some(Element::Hydro), None, None
                    )), "哥伦比娅命座6", 0.8);
            }
            if self.reacted_element.electro {
                attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Invisible(InvisibleAttributeType::new(
                        AttributeVariableType::CriticalDamage,
                        Some(Element::Electro), None, None
                    )), "哥伦比娅命座6", 0.8);
            }
            if self.reacted_element.dendro {
                attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Invisible(InvisibleAttributeType::new(
                        AttributeVariableType::CriticalDamage,
                        Some(Element::Dendro), None, None
                    )), "哥伦比娅命座6", 0.8);
            }
            if self.reacted_element.geo {
                attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Invisible(InvisibleAttributeType::new(
                        AttributeVariableType::CriticalDamage,
                        Some(Element::Geo), None, None
                    )), "哥伦比娅命座6", 0.8);
            }
        }

        attribute.add_edge_s1to1(
            CharacterSelector::select_all(attribute),
            AttributeType::Panel(AttributeName::HP),
            AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::MoonglareBase,
                    None, None, None
                )),
            Arc::new(|hp: f64, _| (hp / 1000.0 * 0.002).min(0.07) ),
            "哥伦比娅天赋3",
            EdgePriority::Invisible,
        );

        if self.common_data.constellation >= 1 {
            let mut val = 0.0;

            if self.common_data.constellation >= 1 { val += 0.015; }
            if self.common_data.constellation >= 2 { val += 0.07; }
            if self.common_data.constellation >= 3 { val += 0.015; }
            if self.common_data.constellation >= 4 { val += 0.015; }
            if self.common_data.constellation >= 5 { val += 0.015; }
            if self.common_data.constellation >= 6 { val += 0.07; }

            attribute.set_value_by_s(
                CharacterSelector::select_all(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::MoonglareElevate,
                    None, None, None
                )),
                "哥伦比娅命座",
                val
            );
        }
    }
}

damage_enum!(
    ColumbinaDamageEnum
    A1
    A2
    A3
    Z
    ZM
    X1
    X2
    X3
    E
    EGC
    EGI
    Q
);

impl ColumbinaDamageEnum {
    pub fn get_element(&self, main_element: Option<Element>) -> Element {
        use ColumbinaDamageEnum::*;
        match *self {
            A1 | A2 | A3 | Z | X1 | X2 | X3 | E | EGC | Q => Element::Hydro,
            ZM => Element::Dendro,
            EGI => main_element.unwrap(),
        }
    }

    pub fn get_lunar_type(&self, main_element: Option<Element>) -> MoonglareReaction {
        use ColumbinaDamageEnum::*;
        match *self {
            ZM => MoonglareReaction::LunarBloom,
            EGI => match main_element {
                Some(Element::Electro) => MoonglareReaction::LunarCharged,
                Some(Element::Dendro) => MoonglareReaction::LunarBloom,
                Some(Element::Geo) => MoonglareReaction::LunarCrystallize,
                _ => MoonglareReaction::None,
            },
            _ => MoonglareReaction::None,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use ColumbinaDamageEnum::*;
        match *self {
            A1 | A2 | A3 => SkillType::NormalAttack,
            Z => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E | EGC => SkillType::ElementalSkill,
            Q => SkillType::ElementalBurst,
            ZM | EGI => SkillType::Moonglare,
        }
    }
}

pub struct Columbina;

impl CharacterTrait for Columbina {
    const STATIC_DATA: CharacterStaticData = COLUMBINA_STATIC_DATA;
    type SkillType = ColumbinaSkillType;
    const SKILL: Self::SkillType = COLUMBINA_SKILL;
    type DamageEnumType = ColumbinaDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ColumbinaDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            Z charged_dmg!()
            ZM locale!(zh_cn: "月露涤荡伤害", en: "Moondew Cleanse DMG")
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            ColumbinaDamageEnum
            E locale!(zh_cn: "技能伤害", en: "Skill DMG")
            EGC locale!(zh_cn: "引力涟漪·持续伤害", en: "Gravity Ripple: Continuous DMG")
            EGI locale!(zh_cn: "引力干涉", en: "Gravity Interference")
        ),
        skill3: skill_map!(
            ColumbinaDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig {
            name: "main_element",
            title: locale!(
                zh_cn: "引力干涉属性",
                en: "Gravity Interference Element"
            ),
            config: ItemConfigType::ElementOptional { elements: &[Element::Electro, Element::Dendro, Element::Geo], default: None }
        },
        ItemConfig {
            name: "reacted_element",
            title: locale!(
                zh_cn: "月曜反应元素",
                en: "Lunar Reaction Element"
            ),
            config: ItemConfigType::ElementMulti { elements: &[Element::Hydro, Element::Electro, Element::Dendro, Element::Geo], default: ConfigElements8Multi {
                pyro: false,
                hydro: true,
                anemo: false,
                electro: false,
                dendro: false,
                cryo: false,
                geo: false,
                physical: false,
            }}
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "activated_q",
            title: locale!(
                zh_cn: "处于月之领域中",
                en: "In Moonlit Domain"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "stack_p1",
            title: locale!(
                zh_cn: "天赋一层数",
                en: "Talent 1 Stack Count"
            ),
            config: ItemConfigType::Int { min: 0, max: 3, default: 3 }
        },
        ItemConfig {
            name: "activated_c4",
            title: locale!(
                zh_cn: "四命加成",
                en: "Activate C4 Bonus"
            ),
            config: ItemConfigType::Bool { default: true }
        },
    ]);

    fn change_attribute<A: Attribute>(attribute: &mut A, common_data: &CharacterCommonData, skill_config: &CharacterSkillConfig) {
        let (moonsign, main_element, reacted_element) = match &common_data.config {
            CharacterConfig::Columbina { moonsign, main_element, reacted_element } => (*moonsign, *main_element, *reacted_element),
            _ => (Moonsign::None, None, ConfigElements8Multi::default()),
        };

        let (activated_q, stack_p1, activated_c4) = match *skill_config {
            CharacterSkillConfig::Columbina { activated_q, stack_p1, activated_c4 } => (activated_q, stack_p1, activated_c4),
            _ => (false, 0, false)
        };

        if activated_q {
            attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::ReactionEnhance,
                    None, None, Some(ReactionType::LunarCharged),
                )), "哥伦比娅Q技能", COLUMBINA_SKILL.q_enhance[common_data.skill3 as usize]);
            attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::ReactionEnhance,
                    None, None, Some(ReactionType::LunarBloom),
                )), "哥伦比娅Q技能", COLUMBINA_SKILL.q_enhance[common_data.skill3 as usize]);
            attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::ReactionEnhance,
                    None, None, Some(ReactionType::LunarCrystallize),
                )), "哥伦比娅Q技能", COLUMBINA_SKILL.q_enhance[common_data.skill3 as usize]);
        }

        if common_data.has_talent1 && stack_p1 > 0 {
            attribute.set_value_by(AttributeName::CriticalBase, "哥伦比娅天赋1", stack_p1 as f64 * 0.05);
        }
    }

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ColumbinaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (moonsign, main_element, reacted_element) = match &context.character_common_data.config {
            CharacterConfig::Columbina { moonsign, main_element, reacted_element } => (*moonsign, *main_element, *reacted_element),
            _ => (Moonsign::None, None, ConfigElements8Multi::default()),
        };

        let (activated_q, stack_p1, activated_c4) = match *config {
            CharacterSkillConfig::Columbina { activated_q, stack_p1, activated_c4 } => (activated_q, stack_p1, activated_c4),
            _ => (false, 0, false)
        };

        use ColumbinaDamageEnum::*;
        let mut builder = D::new();

        let ratio = match s {
            A1 => COLUMBINA_SKILL.a_dmg1[s1],
            A2 => COLUMBINA_SKILL.a_dmg2[s1],
            A3 => COLUMBINA_SKILL.a_dmg3[s1],
            Z => COLUMBINA_SKILL.z_dmg[s1],
            ZM => COLUMBINA_SKILL.z_dmgm[s1],
            X1 => COLUMBINA_SKILL.x_dmg1[s1],
            X2 => COLUMBINA_SKILL.x_dmg2[s1],
            X3 => COLUMBINA_SKILL.x_dmg3[s1],
            E => COLUMBINA_SKILL.e_dmg[s2],
            EGC => COLUMBINA_SKILL.e_dmggc[s2],
            EGI => match main_element {
                Some(Element::Electro) => COLUMBINA_SKILL.e_dmg_electro[s2],
                Some(Element::Dendro) => COLUMBINA_SKILL.e_dmg_dendro[s2],
                Some(Element::Geo) => COLUMBINA_SKILL.e_dmg_geo[s2],
                _ => 0.0,
            },
            Q => COLUMBINA_SKILL.q_dmg[s3],
        };

        if s.get_skill_type() == SkillType::Moonglare || s.get_skill_type() == SkillType::ElementalSkill || s.get_skill_type() == SkillType::ElementalBurst {
            builder.add_hp_ratio("技能倍率", ratio);
        } else {
            builder.add_atk_ratio("技能倍率", ratio);
        }

        if s == EGI {
            if context.character_common_data.constellation >= 4 && activated_c4 {
                if let Some(element) = main_element {
                    builder.add_extra_reaction_extra("哥伦比娅命座4",
                        match element {
                            Element::Electro => 0.125,
                            Element::Dendro => 0.025,
                            Element::Geo => 0.125,
                            _ => 0.0,
                        } * context.attribute.get_hp()
                    );
                }
            }
        }

        if s == ZM || s == EGI {
            if s.get_lunar_type(main_element) != MoonglareReaction::None {
                builder.moonglare(
                    &context.attribute,
                    &context.enemy,
                    s.get_element(main_element),
                    s.get_lunar_type(main_element),
                    s.get_skill_type(),
                    context.character_common_data.level,
                    fumo,
                )
            } else {
                builder.none()
            }
        } else {
            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(main_element),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (moonsign, main_element, reacted_element) = match *config {
            CharacterConfig::Columbina { moonsign, main_element, reacted_element } => (moonsign, main_element, reacted_element),
            _ => (Moonsign::None, None, ConfigElements8Multi::default()),
        };
        Some(Box::new(ColumbinaEffect {
            moonsign,
            main_element,
            reacted_element,
            common_data: common_data.clone(),
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
