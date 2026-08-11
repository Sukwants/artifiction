use crate::buffs::buffs::prelude::*;
use crate::character::characters::anemo::yumemizuki_mizuki::YUMEMIZUKI_MIZUKI_SKILL;


pub struct BuffYumemizukiMizukiP3 {
    pub em: f64,
}

impl<A: Attribute> Buff<A> for BuffYumemizukiMizukiP3 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_to(AttributeName::ElementalMastery, "梦见月瑞希天赋3", self.em * 0.1);
    }
}

impl BuffMeta for BuffYumemizukiMizukiP3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YumemizukiMizukiP3,
        name_locale: locale!(
            zh_cn: "梦见月瑞希-「廓然梦生」",
            en: "Yumemizuki Mizuki-'Vast Be the Dream'"
        ),
        image: BuffImage::Avatar(CharacterName::YumemizukiMizuki),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "处于梦浮状态下时，队伍中附近的角色的元素精通提升，提升值相当于梦见月瑞希自己的元素精通的10%。",
            en: "While in the Dreamdrifter state, the Elemental Mastery of nearby party characters is increased by 10% of Yumemizuki Mizuki's Elemental Mastery."
        )),
        from: BuffFrom::Character(CharacterName::YumemizukiMizuki),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "梦见月瑞希的元素精通",
                en: "EM of Mizuki"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5000.0, default: 1000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::YumemizukiMizukiP3 { em } => em,
            _ => 0.0
        };
        Box::new(BuffYumemizukiMizukiP3 {
            em
        })
    }
}

pub struct BuffYumemizukiMizukiC1 {
    pub em: f64,
}

impl<A: Attribute> Buff<A> for BuffYumemizukiMizukiC1 {
    fn change_attribute(&self, attribute: &mut A) {
        for reaction in &[ReactionType::CryoSwirl, ReactionType::PyroSwirl, ReactionType::HydroSwirl, ReactionType::ElectroSwirl] {
            attribute.set_value_to_t(
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionExtra, *reaction)),
                "梦见月瑞希命座1",
                11.0 * self.em,
            );
        }
        // 星扩散反应伤害提升：相当于梦见月瑞希元素精通的550%
        attribute.set_value_to_t(
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionExtra, ReactionType::StellarSwirl)),
            "梦见月瑞希命座1",
            5.5 * self.em,
        );
    }
}

impl BuffMeta for BuffYumemizukiMizukiC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YumemizukiMizukiC1,
        name_locale: locale!(
            zh_cn: "梦见月瑞希-「宿雾若水遥」",
            en: "Yumemizuki Mizuki-'In Mist-Like Waters'"
        ),
        image: BuffImage::Avatar(CharacterName::YumemizukiMizuki),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "梦见月瑞希处于梦浮状态下时，每3.5秒将对附近的敌人施加持续3秒的「二十三夜待」效果。处于二十三夜待状态下的敌人受到风元素伤害而触发扩散或星扩散反应时，将移除该效果，使此次扩散/星扩散反应对该敌人造成的伤害提升，提升值分别相当于梦见月瑞希元素精通的1100%/550%。",
            en: "When Yumemizuki Mizuki is in the Dreamdrifter state, she will continuously apply the \"Twenty-Three Nights' Awaiting\" effect to nearby opponents for 3s every 3.5s. When an opponent is affected by Anemo DMG-triggered Swirl or Stellar Swirl reactions while the aforementioned effect is active, the effect will be canceled and this Swirl/Stellar Swirl instance has its DMG against this opponent increased by 1,100%/550% of Mizuki's Elemental Mastery."
        )),
        from: BuffFrom::Character(CharacterName::YumemizukiMizuki),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "梦见月瑞希的元素精通",
                en: "EM of Mizuki"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5000.0, default: 1000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::YumemizukiMizukiC1 { em } => em,
            _ => 0.0
        };
        Box::new(BuffYumemizukiMizukiC1 {
            em
        })
    }
}

pub struct BuffYumemizukiMizukiC2 {
    pub em: f64,
}

impl<A: Attribute> Buff<A> for BuffYumemizukiMizukiC2 {
    fn change_attribute(&self, attribute: &mut A) {
        let bonus = self.em * 0.0004;

        attribute.set_value_by(AttributeName::BonusPyro, "梦见月瑞希命座2", bonus);
        attribute.set_value_by(AttributeName::BonusHydro, "梦见月瑞希命座2", bonus);
        attribute.set_value_by(AttributeName::BonusCryo, "梦见月瑞希命座2", bonus);
        attribute.set_value_by(AttributeName::BonusElectro, "梦见月瑞希命座2", bonus);

        let res_minus = 0.2;
        for element in [Element::Pyro, Element::Hydro, Element::Cryo, Element::Electro, Element::Anemo].iter() {
            attribute.set_value_to_t(
                AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::ResMinus, *element)),
                "梦见月瑞希命座2",
                res_minus,
            );
        }
    }
}

impl BuffMeta for BuffYumemizukiMizukiC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YumemizukiMizukiC2,
        name_locale: locale!(
            zh_cn: "梦见月瑞希-「缠忆君影梦相见」",
            en: "Yumemizuki Mizuki-'Your Echo I Meet in Dreams'"
        ),
        image: BuffImage::Avatar(CharacterName::YumemizukiMizuki),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "进入梦浮状态时，梦见月瑞希的每点元素精通，会为附近的队伍中所有其他角色提供0.04%火元素、水元素、冰元素与雷元素伤害加成，效果持续至梦浮状态结束。此外，梦见月瑞希处于梦浮状态下时，附近敌人的火元素抗性、水元素抗性、冰元素抗性、雷元素抗性与风元素抗性降低20%。",
            en: "When entering the Dreamdrifter state, every Elemental Mastery point Yumemizuki Mizuki has will increase all nearby party members' Pyro, Hydro, Cryo, and Electro DMG Bonuses by 0.04% until the Dreamdrifter state ends. Additionally, while Yumemizuki Mizuki is in the Dreamdrifter state, the Pyro RES, Hydro RES, Cryo RES, Electro RES, and Anemo RES of nearby opponents are lowered by 20%."
        )),
        from: BuffFrom::Character(CharacterName::YumemizukiMizuki),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "梦见月瑞希的元素精通",
                en: "EM of Mizuki"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 5000.0, default: 1000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::YumemizukiMizukiC2 { em } => em,
            _ => 0.0
        };
        Box::new(BuffYumemizukiMizukiC2 {
            em
        })
    }
}

pub struct BuffYumemizukiMizukiC6;

impl<A: Attribute> Buff<A> for BuffYumemizukiMizukiC6 {
    fn change_attribute(&self, attribute: &mut A) {
        // 命座6：扩散/星扩散反应造成的伤害可暴击（应用该 buff 视为当前角色受瑞希命座6影响）
        for reaction in [ReactionType::CryoSwirl, ReactionType::PyroSwirl, ReactionType::HydroSwirl, ReactionType::ElectroSwirl] {
            attribute.set_value_to_t(
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::CriticalRate, reaction)),
                "梦见月瑞希命座6",
                YUMEMIZUKI_MIZUKI_SKILL.c6_crit_rate_base,
            );
            attribute.set_value_to_t(
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::CriticalDamage, reaction)),
                "梦见月瑞希命座6",
                YUMEMIZUKI_MIZUKI_SKILL.c6_crit_dmg_base,
            );
        }
        // 星扩散在扩散固定值基础上提升
        attribute.set_value_to_t(
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::CriticalRate, ReactionType::StellarSwirl)),
            "梦见月瑞希命座6",
            YUMEMIZUKI_MIZUKI_SKILL.c6_crit_rate_base + YUMEMIZUKI_MIZUKI_SKILL.c6_crit_rate_stellar,
        );
        attribute.set_value_to_t(
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::CriticalDamage, ReactionType::StellarSwirl)),
            "梦见月瑞希命座6",
            YUMEMIZUKI_MIZUKI_SKILL.c6_crit_dmg_base + YUMEMIZUKI_MIZUKI_SKILL.c6_crit_dmg_stellar,
        );
    }
}

impl BuffMeta for BuffYumemizukiMizukiC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YumemizukiMizukiC6,
        name_locale: locale!(
            zh_cn: "梦见月瑞希-「慕念萦心间」",
            en: "Yumemizuki Mizuki-'The Heart Lingers Long'"
        ),
        image: BuffImage::Avatar(CharacterName::YumemizukiMizuki),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "梦见月瑞希命座6：梦见月瑞希处于梦浮状态下时，队伍中附近的角色触发的扩散反应造成的伤害能够造成暴击，暴击率固定为30%，暴击伤害固定为100%，并使这些角色造成的星扩散反应伤害暴击率提升10%，暴击伤害提升20%。",
            en: "Yumemizuki Mizuki Constellation 6: While Yumemizuki Mizuki is in the Dreamdrifter state, Swirl DMG dealt by nearby party members can score CRIT Hits, with CRIT Rate fixed at 30%, and CRIT DMG fixed at 100%. The CRIT Rate of any Stellar Swirl DMG dealt by these party members is also increased by 10%, while CRIT DMG is increased by 20%."
        )),
        from: BuffFrom::Character(CharacterName::YumemizukiMizuki),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffYumemizukiMizukiC6)
    }
}
