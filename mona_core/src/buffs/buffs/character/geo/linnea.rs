use crate::buffs::buffs::prelude::*;

pub struct BuffLinneaP1 {
    pub moonsign: Moonsign,
}

impl<A: Attribute> Buff<A> for BuffLinneaP1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_t(
            AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::ResMinus, Element::Geo)),
            "莉奈娅天赋1",
            if self.moonsign.is_ascendant() { 0.3 } else { 0.15 },
        );
    }
}

impl BuffMeta for BuffLinneaP1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LinneaP1,
        name_locale: locale!(
            zh_cn: "莉奈娅-「野外观察手记」",
            en: "Linnea-Field Observation Notes"
        ),
        image: BuffImage::Avatar(CharacterName::Linnea),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "莉奈娅天赋1：露米在场上时，露米附近敌人的岩元素抗性会降低15%。\
                <br>月兆·满辉：莉奈娅的元素战技对策·露米呀吼吼！和元素爆发备忘·绝境生存指南获得强化，呼唤露米上场后，露米附近敌人的岩元素抗性还会进一步降低15%。",
            en: "Linnea Talent1: When Lumi is present on the field, the Geo RES of opponents near Lumi will decrease by 15%.\
                <br>Moonsign: Ascendant Gleam: Linnea's Elemental Skill Countermeasure: Lumi's Battle Cry! and Elemental Burst Memo: Survival Guide in Extreme Conditions are enhanced. After summoning Lumi, the Geo RES of opponents near Lumi will be further decreased by 15%."
        )),
        from: BuffFrom::Character(CharacterName::Linnea),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_BUFF),
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let moonsign = match *b {
            BuffConfig::LinneaP1 { moonsign } => moonsign,
            _ => Moonsign::None
        };
        Box::new(BuffLinneaP1 {
            moonsign,
        })
    }
}

pub struct BuffLinneaP2 {
    pub def: f64,
}

impl<A: Attribute> Buff<A> for BuffLinneaP2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_t(
            AttributeType::Panel(AttributeName::ElementalMastery),
            "莉奈娅天赋2",
            self.def * 0.05
        );
    }
}

impl BuffMeta for BuffLinneaP2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LinneaP2,
        name_locale: locale!(
            zh_cn: "莉奈娅-「万类博物图鉴」",
            en: "Linnea-Universal Naturalist Archive"
        ),
        image: BuffImage::Avatar(CharacterName::Linnea),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "莉奈娅天赋2：若队伍中自己的当前场上角色为月兆角色：莉奈娅会提升该角色的元素精通，提升值基于莉奈娅防御力的5%",
            en: "Linnea Talent2: If your current active character is a Moonsign character: Linnea will increase the Elemental Mastery of that character. Increase in Elemental Mastery is based on 5% of Linnea's DEF.z"
        )),
        from: BuffFrom::Character(CharacterName::Linnea),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_BUFF),
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let def = match *b {
            BuffConfig::LinneaP2 { def } => def,
            _ => 0.0
        };
        Box::new(BuffLinneaP2 {
            def,
        })
    }
}

pub struct BuffLinneaP3 {
    pub def: f64,
}

impl<A: Attribute> Buff<A> for BuffLinneaP3 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_t(
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::MoonglareBase, ReactionType::LunarCrystallize
            )),
            "莉奈娅天赋3",
            (self.def / 100.0 * 0.007).min(0.14),
        );
    }
}

impl BuffMeta for BuffLinneaP3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LinneaP3,
        name_locale: locale!(
            zh_cn: "莉奈娅-「月兆祝赐·栖地考察」",
            en: "Linnea-Moonsign Benediction: Habitat Survey"
        ),
        image: BuffImage::Avatar(CharacterName::Linnea),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "莉奈娅天赋3：队伍中的角色触发水元素结晶反应时，将转为触发月结晶反应，且基于莉奈娅的防御力，提升队伍中角色造成的月结晶反应的基础伤害：每100点防御力都将提升0.7%月结晶反应的基础伤害，至多通过这种方式提升14%伤害。",
            en: "Linnea Talent3: When a party member triggers a Hydro Crystallize reaction, it will be converted into the Lunar-Crystallize reaction, with every 100 DEF that Linnea has increasing Lunar-Crystallize's Base DMG by 0.7%, up to a maximum of 14%."
        )),
        from: BuffFrom::Character(CharacterName::Linnea),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "def",
            title: locale!(
                zh_cn: "防御力",
                en: "DEF"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let def = match *b {
            BuffConfig::LinneaP3 { def } => def,
            _ => 0.0
        };
        Box::new(BuffLinneaP3 {
            def,
        })
    }
}

pub struct BuffLinneaC1 {
    pub def: f64,
    pub has_c6: bool,
}

impl<A: Attribute> Buff<A> for BuffLinneaC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_t(
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::ReactionExtra, ReactionType::LunarCrystallize
            )),
            "莉奈娅命座1",
            self.def * 0.75 * if self.has_c6 { 1.5 } else { 1.0 },
        );
    }
}

impl BuffMeta for BuffLinneaC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LinneaC1,
        name_locale: locale!(
            zh_cn: "莉奈娅-「未完成的分类」",
            en: "Linnea-Provisional Classification"
        ),
        image: BuffImage::Avatar(CharacterName::Linnea),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "莉奈娅命座1：队伍中附近的角色造成月结晶反应伤害时，将消耗一层「历览编录」，提升造成的伤害，提升值相当于莉奈娅防御力的75%。\
                <br>命座6：消耗「历览编录」时，将消耗原本2倍的层数，并使伤害提升效果提高至原本的150%。",
            en: "Linnea C1: When nearby party members deal Lunar-Crystallize Reaction DMG, consume 1 stack of Field Catalog to increase the DMG dealt. The increase in DMG is equal to 75% of Linnea's DEF.\
                <br>C6: When consuming Field Catalog, consume twice the original number of stacks, such that the increase in DMG will be boosted to 150% of the original value."
        )),
        from: BuffFrom::Character(CharacterName::Linnea),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "def",
            title: locale!(
                zh_cn: "防御力",
                en: "DEF"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
        ItemConfig {
            name: "has_c6",
            title: locale!(
                zh_cn: "命座6",
                en: "C6"
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (def, has_c6) = match *b {
            BuffConfig::LinneaC1 { def, has_c6 } => (def, has_c6),
            _ => (0.0, false)
        };
        Box::new(BuffLinneaC1 {
            def,
            has_c6,
        })
    }
}

pub struct BuffLinneaC2 {
}

impl<A: Attribute> Buff<A> for BuffLinneaC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_t(
            AttributeType::Panel(AttributeName::CriticalDamageBase),
            "莉奈娅命座2",
            0.4,
        );
    }
}

impl BuffMeta for BuffLinneaC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LinneaC2,
        name_locale: locale!(
            zh_cn: "莉奈娅-「喜或悲的谕告」",
            en: "Linnea-Tidings of Joy and Sorrow"
        ),
        image: BuffImage::Avatar(CharacterName::Linnea),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "莉奈娅命座2：触发月笼谐奏后的8秒内，队伍中所有元素类型为水元素与岩元素的角色的暴击伤害提升40%。",
            en: "Linnea C2: Within 8s after triggering Moondrift Harmony, all Hydro and Geo party members have their CRIT DMG increased by 40%."
        )),
        from: BuffFrom::Character(CharacterName::Linnea),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffLinneaC2 {
        })
    }
}

pub struct BuffLinneaC4 {
}

impl<A: Attribute> Buff<A> for BuffLinneaC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_def_percentage(
            "莉奈娅命座4",
            0.25,
        );
    }
}

impl BuffMeta for BuffLinneaC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LinneaC4,
        name_locale: locale!(
            zh_cn: "莉奈娅-「专家的直感觉」",
            en: "Linnea-Expert Instinct"
        ),
        image: BuffImage::Avatar(CharacterName::Linnea),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "莉奈娅命座4：触发月笼谐奏后的5秒内，莉奈娅与队伍中自己的当前场上角色的防御力分别提升25%。",
            en: "Linnea C4: Within 5s after Moondrift Harmony is triggered, increase the DEF of Linnea and your current active character by 25% respectively."
        )),
        from: BuffFrom::Character(CharacterName::Linnea),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffLinneaC4 {
        })
    }
}

pub struct BuffLinneaC6 {
}

impl<A: Attribute> Buff<A> for BuffLinneaC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_t(
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::MoonglareElevate,ReactionType::LunarCrystallize
            )),
            "莉奈娅命座6",
            0.25
        );
    }
}

impl BuffMeta for BuffLinneaC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LinneaC6,
        name_locale: locale!(
            zh_cn: "莉奈娅-「专家的直感觉」",
            en: "Linnea-Expert Instinct"
        ),
        image: BuffImage::Avatar(CharacterName::Linnea),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "莉奈娅命座6：月兆·满辉：队伍中附近的角色造成的月结晶反应伤害擢升25%。",
            en: "Linnea C6: Moonsign: Ascendant Gleam: Lunar-Crystallize Reaction DMG dealt by nearby party members is elevated by 25%."
        )),
        from: BuffFrom::Character(CharacterName::Linnea),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffLinneaC6 {
        })
    }
}