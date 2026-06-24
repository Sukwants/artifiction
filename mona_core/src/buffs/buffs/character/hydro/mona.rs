use crate::buffs::buffs::prelude::*;
use crate::character::characters::Mona;
use crate::character::traits::CharacterTrait;
use crate::enemies::Enemy;

pub struct BuffMonaQ {
    pub c4: bool,
    pub skill3: usize,
    pub is_hexerei: bool,
}

impl<A: Attribute> Buff<A> for BuffMonaQ {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = Mona::SKILL.elemental_burst_bonus[self.skill3 - 1];
        attribute.set_value_by(AttributeName::BonusBase, "BUFF: 莫娜「星异」", bonus);
        if self.c4 {
            attribute.set_value_by(AttributeName::CriticalBase, "BUFF: 莫娜四命「灭绝的预言」", 0.15);

            if self.is_hexerei {
                attribute.set_value_by(AttributeName::CriticalDamageBase, "BUFF: 莫娜四命「灭绝的预言」", 0.15);
            }
        }
    }
}

impl BuffMeta for BuffMonaQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MonaQ,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "莫娜-「星异」",
            en: "Mona-Omen",
        ),
        image: BuffImage::Avatar(CharacterName::Mona),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "莫娜Q技能：对敌人施加星异的伤害加成效果，并以此提高这一次造成的伤害。\
                <br>命座4：队伍中所有角色攻击处于星异状态下的敌人时，暴击率提升15%；队伍中所有魔导角色攻击处于星异状态下的敌人时，暴击伤害提升15%。",
            en: "Mona Elemental Burst: Applies an Omen to the opponent, which gives a DMG Bonus, also increasing the DMG of the attack that causes it. \
                <br>When any party member attacks an opponent affected by an Omen, their CRIT Rate is increased by 15%. When any Hexerei party member attacks an opponent affected by an Omen, their CRIT DMG is increased by 15%.",
        )),
        from: BuffFrom::Character(CharacterName::Mona),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "skill3",
            title: crate::common::i18n::locale!(
                zh_cn: "Q技能等级",
                en: "Q Level",
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 9 }
        },
        ItemConfig {
            name: "c4",
            title: crate::common::i18n::locale!(
                zh_cn: "是否4命",
                en: "C4",
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig::IS_HEXEREI(false, ItemConfig::PRIORITY_BUFF),
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (c4, skill3, is_hexerei) = match *b {
            BuffConfig::MonaQ { c4, skill3, is_hexerei } => (c4, skill3, is_hexerei),
            _ => (false, 1, false)
        };
        Box::new(BuffMonaQ {
            c4, skill3, is_hexerei
        })
    }
}


pub struct BuffMonaC1 {
    pub off_field: bool,
}

impl<A: Attribute> Buff<A> for BuffMonaC1 {
    fn change_attribute(&self, attribute: &mut A) {
        let val = if self.off_field { 0.24 } else { 0.15 };
        attribute.set_value_by(AttributeName::EnhanceElectroCharged, "BUFF: 莫娜一命「沉没的预言」", val);
        attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, ReactionType::LunarCharged)), "BUFF: 莫娜一命「沉没的预言」", val);
        attribute.set_value_by(AttributeName::EnhanceVaporize, "BUFF: 莫娜一命「沉没的预言」", val);
        attribute.set_value_by(AttributeName::EnhanceSwirlHydro, "BUFF: 莫娜一命「沉没的预言」", val);
    }
}

impl BuffMeta for BuffMonaC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MonaC1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "莫娜-「沉没的预言」",
            en: "Mona-Prophecy of Submersion",
        ),
        image: BuffImage::Avatar(CharacterName::Mona),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "莫娜命座1：队伍中自己的角色攻击命中处于星异状态下的敌人后的8秒内，水元素相关反应的效果提升：\
                <br>·感电反应造成的伤害提升15%，月感电反应造成的伤害提升15%，蒸发反应造成的伤害提升15%，水元素扩散反应造成的伤害提升15%；",
            en: "Mona C1: When any of your own party members hits an opponent affected by an Omen, the effects of Hydro-related Elemental Reactions are enhanced for 8s: \
                <br>· Electro-Charged DMG increases by 15%. \
                <br>· Lunar-Charged DMG increases by 15%. \
                <br>· Vaporize DMG increases by 15%. \
                <br>· Hydro Swirl DMG increases by 15%.",
        )),
        from: BuffFrom::Character(CharacterName::Mona),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "off_field",
            title: crate::common::i18n::locale!(
                zh_cn: "处于后台",
                en: "Off Field",
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let off_field = match *b {
            BuffConfig::MonaC1 { off_field } => off_field,
            _ => false
        };
        Box::new(BuffMonaC1 {
            off_field,
        })
    }
}


pub struct BuffMonaTalent3 {
    pub hexerei_secret_rite: bool,
    pub stack: f64,
}

impl<A: Attribute> Buff<A> for BuffMonaTalent3 {
    fn change_attribute(&self, attribute: &mut A) {
        let val = self.stack * 0.05;
        if self.hexerei_secret_rite {
            attribute.set_value_by_t(
                AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::ReactionEnhance,
                    None, None, Some(ReactionType::Vaporize),
                )),
                "莫娜天赋3",
                val,
            );
        }
    }
}

impl BuffMeta for BuffMonaTalent3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MonaTalent3,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "莫娜-「魔女的前夜礼·天步真原」",
            en: "Mona-Witch's Eve Rite: Genesis of Starsigns",
        ),
        image: BuffImage::Avatar(CharacterName::Mona),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "莫娜天赋3：魔导·秘仪：队伍中自己的其他角色对敌人触发蒸发反应时，将消耗所有的「水星天的辉光」，每消耗一层都会使本次蒸发反应造成的伤害提升5%。",
            en: "Mona Talent 3: Hexerei: Secret Rite: When other party members trigger a Vaporize reaction on an enemy, all Astral Glow of Mercury stacks are consumed. Each stack consumed increases the damage of that Vaporize reaction by 5%.",
        )),
        from: BuffFrom::Character(CharacterName::Mona),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_BUFF),
        ItemConfig {
            name: "stack",
            title: crate::common::i18n::locale!(
                zh_cn: "「水星天的辉光」平均层数",
                en: "Avg. Astral Glow of Mercury Stacks",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (hexerei_secret_rite, stack) = match *b {
            BuffConfig::MonaTalent3 { hexerei_secret_rite, stack } => (hexerei_secret_rite, stack),
            _ => (false, 0.0)
        };
        Box::new(BuffMonaTalent3 {
            hexerei_secret_rite,
            stack,
        })
    }
}
