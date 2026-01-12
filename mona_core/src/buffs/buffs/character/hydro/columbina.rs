use crate::buffs::buffs::prelude::*;
use crate::character::characters::columbina::COLUMBINA_SKILL;

pub struct BuffColumbinaQ {
    pub level_q: usize,
}

impl<A: Attribute> Buff<A> for BuffColumbinaQ {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Invisible(InvisibleAttributeType::new(
                AttributeVariableType::ReactionEnhance,
                None, None, Some(ReactionType::LunarCharged),
            )), "哥伦比娅Q技能", COLUMBINA_SKILL.q_enhance[self.level_q - 1]);
        attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Invisible(InvisibleAttributeType::new(
                AttributeVariableType::ReactionEnhance,
                None, None, Some(ReactionType::LunarBloom),
            )), "哥伦比娅Q技能", COLUMBINA_SKILL.q_enhance[self.level_q - 1]);
        attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Invisible(InvisibleAttributeType::new(
                AttributeVariableType::ReactionEnhance,
                None, None, Some(ReactionType::LunarCrystallize),
            )), "哥伦比娅Q技能", COLUMBINA_SKILL.q_enhance[self.level_q - 1]);
    }
}

impl BuffMeta for BuffColumbinaQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ColumbinaQ,
        name_locale: locale!(
            zh_cn: "哥伦比娅-「她的乡愁」",
            en: "Columbina-Moonlit Melancholy"
        ),
        image: BuffImage::Avatar(CharacterName::Columbina),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "哥伦比娅Q技能：以无秽的新月之名连接山与海，将身边的大地短暂变换为月之领域。\
                <br>月之领域：当前场上角色处于月之领域中时，队伍中的所有角色造成的月曜反应伤害将会提升。",
            en: "Columbina Skill Q: Hills and tides unite under the silver light of a pristine new moon. Temporarily transforms the surrounding terrain into Lunar Domain.\
                <br>Lunar Domain: When current active characters are within the Lunar Domain, Lunar Reaction DMG inflicted by all party members will increase."
        )),
        from: BuffFrom::Character(CharacterName::Columbina),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level_q",
            title: locale!(
                zh_cn: "技能等级",
                en: "Skill Level"
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let level_q = match *b {
            BuffConfig::ColumbinaQ { level_q } => level_q,
            _ => 0
        };
        Box::new(BuffColumbinaQ {
            level_q,
        })
    }
}


pub struct BuffColumbinaP3 {
    pub hp: f64,
}

impl<A: Attribute> Buff<A> for BuffColumbinaP3 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_s(
            CharacterSelector::select_all(attribute),
            AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::MoonglareBase,
                    None, None, None
                )),
            "哥伦比娅天赋3",
            (self.hp / 1000.0 * 0.002).min(0.07)
        );
    }
}

impl BuffMeta for BuffColumbinaP3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ColumbinaP3,
        name_locale: locale!(
            zh_cn: "哥伦比娅-「月兆祝赐·借汝月光」",
            en: "Columbina-Moonsign Benediction: Moonlight, Lent Unto You"
        ),
        image: BuffImage::Avatar(CharacterName::Columbina),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "哥伦比娅天赋3：队伍中的角色触发感电/绽放/水元素结晶反应时，将转为触发月感电/月绽放/月结晶反应，且基于哥伦比娅的生命值上限，提升队伍中角色造成的月曜反应的基础伤害：每1000点生命值上限都将提升0.2%月曜反应的基础伤害，至多通过这种方式提升7%伤害。",
            en: "Columbina Talent 3: When a party member triggers an Electro-Charged/Bloom/Hydro-Crystallize reaction, it will be converted into a Lunar-Charged/Lunar-Bloom/Lunar-Crystallize reaction. Lunar Reaction DMG inflicted by party members gain a Base DMG increase based on Columbina's Max HP. For every 1,000 points of Max HP, the Base DMG of Lunar reactions is increased by 0.2%, up to a maximum of 7%."
        )),
        from: BuffFrom::Character(CharacterName::Columbina),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp",
            title: locale!(
                zh_cn: "生命值上限",
                en: "Max HP"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let hp = match *b {
            BuffConfig::ColumbinaP3 { hp } => hp,
            _ => 0.0
        };
        Box::new(BuffColumbinaP3 {
            hp,
        })
    }
}


pub struct BuffColumbinaC2 {
    pub moonsign: Moonsign,
    pub main_element: Option<Element>,
    pub hp: f64,
}

impl<A: Attribute> Buff<A> for BuffColumbinaC2 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.moonsign.is_ascendant() {
            if let Some(element) = self.main_element {
                match element {
                    Element::Electro => {
                        attribute.set_value_by_s(
                            CharacterSelector::select_onfield(attribute),
                            AttributeType::Panel(AttributeName::ATKFixed),
                            "哥伦比娅命座2",
                            self.hp * 0.01,
                        );
                    },
                    Element::Dendro => {
                        attribute.set_value_by_s(
                            CharacterSelector::select_onfield(attribute),
                            AttributeType::Panel(AttributeName::ElementalMastery),
                            "哥伦比娅命座2",
                            self.hp * 0.0035,
                        );
                    },
                    Element::Geo => {
                        attribute.set_value_by_s(
                            CharacterSelector::select_onfield(attribute),
                            AttributeType::Panel(AttributeName::DEFFixed),
                            "哥伦比娅命座2",
                            self.hp * 0.01,
                        );
                    },
                    _ => panic!()
                }
            }
        }
    }
}

impl BuffMeta for BuffColumbinaC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ColumbinaC2,
        name_locale: locale!(
            zh_cn: "哥伦比娅-「为夜增辉，与君遥伴」",
            en: "Columbina-Not in Lone Splendor"
        ),
        image: BuffImage::Avatar(CharacterName::Columbina),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "哥伦比娅命座2：月兆·满辉：\
                <br>皎辉效果持续期间，若本次触发引力干涉时，为哥伦比娅积攒最多引力值的月曜反应类型为：\
                <br>·月感电反应：队伍中自己的当前场上角色攻击力提升，提升值相当于哥伦比娅生命值上限的1%；\
                <br>·月绽放反应：队伍中自己的当前场上角色元素精通提升，提升值相当于哥伦比娅生命值上限的0.35%；\
                <br>·月结晶反应：队伍中自己的当前场上角色防御力提升，提升值相当于哥伦比娅生命值上限的1%。",
            en: "Columbina C2: Moonsign: Ascendant Gleam:\
                <br>When Lunar Brilliance is active, if, during the current trigger of Gravity Interference, the Lunar reaction type that has accumulated the most Gravity for Columbina is:\
                <br>· Lunar-Charged: The party's current active character gains ATK equal to 1% of Columbina's Max HP.\
                <br>· Lunar-Bloom: Elemental Mastery of the party's current active character increases. Increase in Elemental Mastery is equal to 0.35% of Columbina's Max HP.\
                <br>· Lunar-Crystallize: DEF of the party's current active character increases. Increase in DEF is equal to 1% of Columbina's Max HP."
        )),
        from: BuffFrom::Character(CharacterName::Columbina),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_BUFF),
        ItemConfig {
            name: "main_element",
            title: locale!(
                zh_cn: "引力干涉属性",
                en: "Gravity Interference Element"
            ),
            config: ItemConfigType::ElementOptional { elements: &[Element::Electro, Element::Dendro, Element::Geo], default: None }
        },
        ItemConfig {
            name: "hp",
            title: locale!(
                zh_cn: "生命值上限",
                en: "Max HP"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (moonsign, main_element, hp) = match *b {
            BuffConfig::ColumbinaC2 { moonsign, main_element, hp } => (moonsign, main_element, hp),
            _ => (Moonsign::Nascent, None, 0.0)
        };
        Box::new(BuffColumbinaC2 {
            moonsign,
            main_element,
            hp,
        })
    }
}


pub struct BuffColumbinaC6 {
    pub reacted_element: ConfigElements8Multi,
}

impl<A: Attribute> Buff<A> for BuffColumbinaC6 {
    fn change_attribute(&self, attribute: &mut A) {
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
}

impl BuffMeta for BuffColumbinaC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ColumbinaC6,
        name_locale: locale!(
            zh_cn: "哥伦比娅-「夜昏且暗，且随月光」",
            en: "Columbina-Through Darkness Led by Moonlight"
        ),
        image: BuffImage::Avatar(CharacterName::Columbina),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "哥伦比娅命座6：处于月之领域中的所有角色触发月曜反应后的8秒内，依据参与反应的元素类型，使队伍中的所有角色造成的对应元素类型伤害的暴击伤害提升80%。同种元素类型的暴击伤害提升效果无法叠加。",
            en: "Columbina C6: For 8s after characters in the Lunar Domain trigger a Lunar reaction, based on the elements involved in the reaction, the CRIT DMG of the corresponding Elemental DMG is increased by 80%. The CRIT DMG-increasing effects for the same Elemental Type do not stack."
        )),
        from: BuffFrom::Character(CharacterName::Columbina),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
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

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let reacted_element = match *b {
            BuffConfig::ColumbinaC6 { reacted_element } => reacted_element,
            _ => ConfigElements8Multi::default()
        };
        Box::new(BuffColumbinaC6 {
            reacted_element,
        })
    }
}


pub struct BuffColumbinaC {
    pub constellation: usize,
}

impl<A: Attribute> Buff<A> for BuffColumbinaC {
    fn change_attribute(&self, attribute: &mut A) {
        let mut val = 0.0;

        if self.constellation >= 1 { val += 0.015; }
        if self.constellation >= 2 { val += 0.07; }
        if self.constellation >= 3 { val += 0.015; }
        if self.constellation >= 4 { val += 0.015; }
        if self.constellation >= 5 { val += 0.015; }
        if self.constellation >= 6 { val += 0.07; }

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

impl BuffMeta for BuffColumbinaC {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ColumbinaC,
        name_locale: locale!(
            zh_cn: "哥伦比娅-「空月归乡」",
            en: "Columbina-Welkin Moon's Homecoming"
        ),
        image: BuffImage::Avatar(CharacterName::Columbina),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "哥伦比娅命座1：队伍中附近的所有角色造成的月曜反应伤害擢升1.5%。\
                <br>哥伦比娅命座2：队伍中附近的所有角色造成的月曜反应伤害擢升7%。\
                <br>哥伦比娅命座3：队伍中附近的所有角色造成的月曜反应伤害擢升1.5%。\
                <br>哥伦比娅命座4：队伍中附近的所有角色造成的月曜反应伤害擢升1.5%。\
                <br>哥伦比娅命座5：队伍中附近的所有角色造成的月曜反应伤害擢升1.5%。\
                <br>哥伦比娅命座6：队伍中附近的所有角色造成的月曜反应伤害擢升7%。",
            en: "Columbina C1: All nearby party members' Lunar Reaction DMG is elevated by 1.5%.\
                <br>Columbina C2: All nearby party members' Lunar Reaction DMG is elevated by 7%.\
                <br>Columbina C3: All nearby party members' Lunar Reaction DMG is elevated by 1.5%.\
                <br>Columbina C4: All nearby party members' Lunar Reaction DMG is elevated by 1.5%.\
                <br>Columbina C5: All nearby party members' Lunar Reaction DMG is elevated by 1.5%.\
                <br>Columbina C6: All nearby party members' Lunar Reaction DMG is elevated by 7%.",
        )),
        from: BuffFrom::Character(CharacterName::Columbina),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "constellation",
            title: locale!(
                zh_cn: "命座",
                en: "Constellation"
            ),
            config: ItemConfigType::Int { min: 0, max: 6, default: 0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let constellation = match *b {
            BuffConfig::ColumbinaC { constellation } => constellation,
            _ => 0
        };
        Box::new(BuffColumbinaC {
            constellation,
        })
    }
}

