use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::{Moonsign, ReactionType};
use crate::character::characters::dendro::lauma::LAUMA_SKILL;

pub struct BuffLaumaE {
    pub level_e: usize,
}

impl<A: Attribute> Buff<A> for BuffLaumaE {
    fn change_attribute(&self, attribute: &mut A) {
        let e_res = LAUMA_SKILL.e_res[self.level_e - 1];
        attribute.set_value_by(AttributeName::ResMinusDendro, "菈乌玛「圣言述咏·终宵永眠」", e_res);
        attribute.set_value_by(AttributeName::ResMinusHydro, "菈乌玛「圣言述咏·终宵永眠」", e_res);
    }
}

impl BuffMeta for BuffLaumaE {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LaumaE,
        name_locale: locale!(
            zh_cn: "菈乌玛-「圣言述咏·终宵永眠」",
            en: "Lauma-Runo: Dawnless Rest of Karsikko"
        ),
        image: BuffImage::Avatar(CharacterName::Lauma),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "菈乌玛E技能：菈乌玛的元素战技或霜林圣域的攻击命中敌人时，将使该敌人的草元素抗性与水元素抗性降低，持续10秒。",
            en: "Lauma Elemental Skill: When Lauma's Elemental Skill or attacks from Frostgrove Sanctuary hit an opponent, that opponent's Dendro RES and Hydro RES will be decreased for 10s."
        )),
        from: BuffFrom::Character(CharacterName::Lauma),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level_e",
            title: crate::common::i18n::locale!(
                zh_cn: "元素战技技能等级",
                en: "Element Skill Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let level_e = match *b {
            BuffConfig::LaumaE { level_e } => level_e,
            _ => 0
        };
        Box::new(BuffLaumaE {
            level_e,
        })
    }
}

pub struct BuffLaumaQ {
    pub em: f64,
    pub level_q: usize,
    pub has_c2: bool,
}

impl<A: Attribute> Buff<A> for BuffLaumaQ {
    fn change_attribute(&self, attribute: &mut A) {
        let q_bloom_increase = LAUMA_SKILL.q_bloom_increase[self.level_q - 1] + if self.has_c2 { LAUMA_SKILL.c2_bloom_increase } else { 0.0 };
        let q_lunar_bloom_increase = LAUMA_SKILL.q_lunar_bloom_increase[self.level_q - 1] + if self.has_c2 { LAUMA_SKILL.c2_lunar_bloom_increase } else { 0.0 };

        attribute.set_value_by(AttributeName::ExtraIncreaseBloom,"菈乌玛「苍色祷歌」",self.em * q_bloom_increase);
        attribute.set_value_by(AttributeName::ExtraIncreaseHyperBloom,"菈乌玛「苍色祷歌」",self.em * q_bloom_increase);
        attribute.set_value_by(AttributeName::ExtraIncreaseBurgeon,"菈乌玛「苍色祷歌」",self.em * q_bloom_increase);
        attribute.set_value_by(AttributeName::ExtraIncreaseLunarBloom,"菈乌玛「苍色祷歌」",self.em * q_lunar_bloom_increase);
    }
}

impl BuffMeta for BuffLaumaQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LaumaQ,
        name_locale: locale!(
            zh_cn: "菈乌玛-「苍色祷歌」",
            en: "Lauma-Pale Hymn"
        ),
        image: BuffImage::Avatar(CharacterName::Lauma),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "菈乌玛Q技能：队伍中附近的角色造成绽放、超绽放、烈绽放、月绽放反应伤害时，将消耗一层「苍色祷歌」，提升造成的伤害，提升值基于菈乌玛的元素精通。上述伤害同时命中多名敌人时，会依据命中敌人的数量消耗「苍色祷歌」层数。\
                <br>菈乌玛命座2：「苍色祷歌」的效果提升：队伍中附近的所有角色触发绽放、超绽放、烈绽放反应时造成的伤害额外提升，提升值相当于菈乌玛元素精通的500%；队伍中附近的所有角色造成的月绽放反应伤害额外提升，提升值相当于菈乌玛元素精通的400%。",
            en: "Lauma Elemental Burst: When nearby party members deal Bloom, Hyperbloom, Burgeon, or Lunar-Bloom DMG, 1 stack of Pale Hymn will be consumed and the DMG dealt will be increased based on Lauma's Elemental Mastery. If this DMG hits multiple opponents at once, then multiple stacks of Pale Hymn will be consumed, depending on how many opponents are hit.\
                <br>Lauma C2: Pale Hymn effects are increased: All nearby party members' Bloom, Hyperbloom, and Burgeon DMG is further increased by 500% of Lauma's Elemental Mastery, and their Lunar-Bloom DMG is further increased by 400% of Lauma's Elemental Mastery."
        )),
        from: BuffFrom::Character(CharacterName::Lauma),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "元素精通",
                en: "Elemental Mastery"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
        ItemConfig {
            name: "level_q",
            title: crate::common::i18n::locale!(
                zh_cn: "元素爆发技能等级",
                en: "Element Burst Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        },
        ItemConfig {
            name: "has_c2",
            title: locale!(
                zh_cn: "C2",
                en: "C2"
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (em, level_q, has_c2) = match *b {
            BuffConfig::LaumaQ { em, level_q, has_c2 } => (em, level_q, has_c2),
            _ => (0.0, 0, false)
        };
        Box::new(BuffLaumaQ {
            em,
            level_q,
            has_c2,
        })
    }
}

pub struct BuffLaumaP1 {
    pub moonsign: Moonsign,
}

impl<A: Attribute> Buff<A> for BuffLaumaP1 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.moonsign == Moonsign::Nascent {
                attribute.set_value_to_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalDamage, None, None, Some(ReactionType::Bloom),
                )), "菈乌玛「奉向霜夜的明光」", 1.0);
                attribute.set_value_to_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalDamage, None, None, Some(ReactionType::Hyperbloom),
                )), "菈乌玛「奉向霜夜的明光」", 1.0);
                attribute.set_value_to_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalDamage, None, None, Some(ReactionType::Burgeon),
                )), "菈乌玛「奉向霜夜的明光」", 1.0);

                attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalRate, None, None, Some(ReactionType::Bloom),
                )), "菈乌玛「奉向霜夜的明光」", 0.15);
                attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalRate, None, None, Some(ReactionType::Hyperbloom),
                )), "菈乌玛「奉向霜夜的明光」", 0.15);
                attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalRate, None, None, Some(ReactionType::Burgeon),
                )), "菈乌玛「奉向霜夜的明光」", 0.15);
        }
        if self.moonsign == Moonsign::Ascendant {
            attribute.set_value_by(AttributeName::CriticalDamageLunarBloom, "菈乌玛「奉向霜夜的明光」", 0.2);

            attribute.set_value_by(AttributeName::CriticalLunarBloom, "菈乌玛「奉向霜夜的明光」", 0.1);
        }
    }
}

impl BuffMeta for BuffLaumaP1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LaumaP1,
        name_locale: locale!(
            zh_cn: "菈乌玛-「奉向霜夜的明光」",
            en: "Lauma-Runo-Light for the Frosty Night"
        ),
        image: BuffImage::Avatar(CharacterName::Lauma),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "菈乌玛天赋1：菈乌玛施放元素战技圣言述咏·终宵永眠后的20秒内，将依据队伍的月兆，分别产生不同的强化效果，不同月兆等级提供的强化效果无法叠加。\
                <br>月兆·初辉：队伍中附近的所有角色触发的绽放、超绽放、烈绽放反应造成的伤害能够造成暴击，暴击率固定为15%，暴击伤害固定为100%。该效果提供的暴击率可以与使对应元素反应能够造成暴击的同类效果提供的暴击率叠加。\
                <br>月兆·满辉：队伍中附近的所有角色造成的月绽放反应伤害，暴击率提升10%，暴击伤害提升20%。",
            en: "Lauma Talent1: For the next 20s after Lauma uses her Elemental Skill Runo: Dawnless Rest of Karsikko, corresponding differing buff effects will be granted depending on the party's Moonsign. The buffs provided by different Moonsign levels cannot stack.\
                <br>Moonsign: Nascent Gleam: Bloom, Hyperbloom, and Burgeon DMG dealt by all nearby party members can score CRIT Hits, with CRIT Rate fixed at 15%, and CRIT DMG fixed at 100%. CRIT Rate from this effect stacks with CRIT Rate from similar effects that allow these Elemental Reactions to CRIT.\
                <br>Moonsign: Ascendant Gleam: All nearby party members' Lunar-Bloom DMG CRIT Rate +10%, CRIT DMG +20%."
        )),
        from: BuffFrom::Character(CharacterName::Lauma),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_BUFF),
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let moonsign = match *b {
            BuffConfig::LaumaP1 { moonsign } => moonsign,
            _ => Moonsign::None
        };
        Box::new(BuffLaumaP1 {
            moonsign,
        })
    }
}

pub struct BuffLaumaP3 {
    pub em: f64,
}

impl<A: Attribute> Buff<A> for BuffLaumaP3 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::IncreaseLunarBloom, "菈乌玛「月兆祝赐·千籁恩宠」", (self.em * 0.000175).min(0.14));
    }
}

impl BuffMeta for BuffLaumaP3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LaumaP3,
        name_locale: locale!(
            zh_cn: "菈乌玛-「月兆祝赐·千籁恩宠」",
            en: "Lauma-Moonsign Benediction: Nature's Chorus"
        ),
        image: BuffImage::Avatar(CharacterName::Lauma),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "菈乌玛天赋3：队伍中的角色触发绽放反应时，将转为触发月绽放反应，且基于菈乌玛的元素精通，提升队伍中角色造成的月绽放反应的基础伤害：每点元素精通都将提升0.0175%月绽放反应的基础伤害，至多通过这种方式提升14%伤害。",
            en: "Lauma Talent3: When a party member triggers a Bloom reaction, it will be converted into the Lunar-Bloom reaction, with every point of Elemental Mastery that Lauma has increasing Lunar-Bloom's Base DMG by 0.0175%, up to a maximum of 14%."
        )),
        from: BuffFrom::Character(CharacterName::Lauma),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "元素精通",
                en: "Elemental Mastery"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::LaumaP3 { em } => em,
            _ => 0.0
        };
        Box::new(BuffLaumaP3 {
            em,
        })
    }
}