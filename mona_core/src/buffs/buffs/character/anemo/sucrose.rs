use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffSucroseTalent1;

impl<A: Attribute> Buff<A> for BuffSucroseTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "砂糖「触媒置换术」", 50.0);
    }
}

impl BuffMeta for BuffSucroseTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SucroseTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "砂糖-「触媒置换术」",
            en: "Sucrose-Catalyst Conversion",
        ),
        image: BuffImage::Avatar(CharacterName::Sucrose),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "砂糖天赋1：砂糖触发扩散反应时，使队伍中所有对应元素类型的角色（不包括砂糖自己）元素精通提升50，持续8秒。",
            en: "Sucrose Talent1: When Sucrose triggers a Swirl reaction, all characters in the party with the matching element (excluding Sucrose) have their Elemental Mastery increased by 50 for 8s.",
        )),
        from: BuffFrom::Character(CharacterName::Sucrose),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffSucroseTalent1)
    }
}


pub struct BuffSucroseTalent2 {
    pub em: f64
}

impl<A: Attribute> Buff<A> for BuffSucroseTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        let v = self.em * 0.2;
        attribute.set_value_by(AttributeName::ElementalMastery, "砂糖「小小的慧风」", v);
    }
}

impl BuffMeta for BuffSucroseTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SucroseTalent2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "砂糖-「小小的慧风」",
            en: "Sucrose-Mollis Favonius",
        ),
        image: BuffImage::Avatar(CharacterName::Sucrose),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "砂糖天赋2：风灵作成·陆叁零捌或禁·风灵作成·染伍同构贰型命中敌人时，基于砂糖元素精通的20%,为队伍中所有角色（不包括砂糖自己）提供元素精通加成，持续8秒。",
            en: "Sucrose Talent2: When Astable Anemohypostasis Creation - 6308 or Forbidden Creation - Isomer 75 / Type II hits an opponent, increases all party members' (excluding Sucrose) Elemental Mastery by an amount equal to 20% of Sucrose's Elemental Mastery for 8s.",
        )),
        from: BuffFrom::Character(CharacterName::Sucrose),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: crate::common::i18n::locale!(
                zh_cn: "砂糖的元素精通",
                en: "Sucrose's EM",
            ),
            config: ItemConfigType::FloatInput { default: 200.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::SucroseTalent2 { em } => em,
            _ => 0.0
        };

        Box::new(BuffSucroseTalent2 {
            em
        })
    }
}

pub struct BuffSucroseTalent3 {
    pub is_hexerei: bool,
}

impl<A: Attribute> Buff<A> for BuffSucroseTalent3 {
    fn change_attribute(&self, attribute: &mut A) {
        let val = if self.is_hexerei { 0.0714285 + 0.0571428 } else { 0.0571428 };

        attribute.set_value_by(AttributeName::BonusNormalAttack, "砂糖「魔女的前夜礼·七循之理」", val);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "砂糖「魔女的前夜礼·七循之理」", val);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "砂糖「魔女的前夜礼·七循之理」", val);
        attribute.set_value_by(AttributeName::BonusElementalSkill, "砂糖「魔女的前夜礼·七循之理」", val);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "砂糖「魔女的前夜礼·七循之理」", val);
    }
}

impl BuffMeta for BuffSucroseTalent3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SucroseTalent3,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "砂糖-「魔女的前夜礼·七循之理」",
            en: "Sucrose-Witch's Eve Rite: Sevenfold Transmutation",
        ),
        image: BuffImage::Avatar(CharacterName::Sucrose),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "砂糖天赋3：魔导·秘仪：\
                <br>·召唤小型风灵后的15秒内，队伍中附近的角色的普通攻击、重击、下落攻击、元素战技和元素爆发造成的伤害提升5.71428%。\
                <br>·召唤大型风灵后的20秒内，队伍中附近的魔导角色的普通攻击、重击、下落攻击、元素战技和元素爆发造成的伤害提升7.14285%。",
            en: "Sucrose Talent3: Secret Rite: \
                <br>· After creating a Small Wind Spirit, nearby party members' Normal Attack, Charged Attack, Plunging Attack, Elemental Skill, and Elemental Burst DMG are increased by 5.71428% for 15s. \
                <br>· After creating a Large Wind Spirit, nearby Hexerei party members' Normal Attack, Charged Attack, Plunging Attack, Elemental Skill, and Elemental Burst DMG are increased by 7.14285% for 20s.",
        )),
        from: BuffFrom::Character(CharacterName::Sucrose),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::IS_HEXEREI(false, ItemConfig::PRIORITY_BUFF),
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let is_hexerei = match *b {
            BuffConfig::SucroseTalent3 { is_hexerei } => is_hexerei,
            _ => false
        };

        Box::new(BuffSucroseTalent3 {
            is_hexerei
        })
    }
}

pub struct BuffSucroseC6 {
    pub element: Element,
    pub is_hexerei: bool,
}

impl<A: Attribute> Buff<A> for BuffSucroseC6 {
    fn change_attribute(&self, attribute: &mut A) {
        let name = AttributeName::bonus_name_by_element(self.element);
        attribute.set_value_by(name, "砂糖「混元熵增论」", 0.2);

        if self.is_hexerei {
            attribute.set_value_by(name, "砂糖「混元熵增论」", 0.0857142);
        }
    }
}

impl BuffMeta for BuffSucroseC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SucroseC6,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "砂糖-「混元熵增论」",
            en: "Sucrose-Chaotic Entropy",
        ),
        image: BuffImage::Avatar(CharacterName::Sucrose),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "砂糖命座6：禁·风灵作成·柒伍同构贰型如果发生了元素转化，则使队伍中所有角色在技能持续时间内获得20%的对应元素伤害加成，并使队伍中附近的魔导角色额外获得8.57142%的对应元素伤害加成。",
            en: "Sucrose C6: If Forbidden Creation - Isomer 75 / Type II triggers an Elemental Absorption, all party members gain a 20% Elemental DMG Bonus for the corresponding absorbed element, and nearby Hexerei characters in the party gain an additional 8.57142% Elemental DMG Bonus for the corresponding absorbed element during its duration.",
        )),
        from: BuffFrom::Character(CharacterName::Sucrose),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "element",
            title: crate::common::i18n::locale!(
                zh_cn: "扩散类型",
                en: "Swirl Type",
            ),
            config: ItemConfigType::Element4 { default: Element::Electro }
        },
        ItemConfig::IS_HEXEREI(false, ItemConfig::PRIORITY_BUFF),
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (element, is_hexerei) = match *b {
            BuffConfig::SucroseC6 { element, is_hexerei } => (element, is_hexerei),
            _ => (Element::Electro, false)
        };

        Box::new(BuffSucroseC6 {
            element, is_hexerei
        })
    }
}
