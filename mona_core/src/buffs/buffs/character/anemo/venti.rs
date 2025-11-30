use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::Element;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffVentiP3 {
}

impl<A: Attribute> Buff<A> for BuffVentiP3 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusBase, "温迪「魔女的前夜礼·颂时风若」", 0.5);
    }
}

impl BuffMeta for BuffVentiP3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::VentiP3,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "温迪-「魔女的前夜礼·颂时风若」",
            en: "Venti-Witch's Eve Rite: Temporal Wind's Eulogy",
        ),
        image: BuffImage::Avatar(CharacterName::Venti),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "温迪天赋3：魔导·秘仪：元素爆发风神之诗创造的暴风之眼存在期间，附近的当前场上角色触发扩散反应后的4秒内，该角色造成的伤害提升50%。",
            en: "Venti P3: While the Stormeye created by the Elemental Burst Wind's Grand Ode is active, for 4s after a nearby active character triggers a Swirl reaction, that character's DMG is increased by 50%.",
        )),
        from: BuffFrom::Character(CharacterName::Venti),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffVentiP3 {
        })
    }
}

pub struct BuffVentiC2 {
}

impl<A: Attribute> Buff<A> for BuffVentiC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusAnemo, "温迪「眷恋的泠风」", 0.24);
        attribute.set_value_by(AttributeName::ResMinusPhysical, "温迪「眷恋的泠风」", 0.24);
    }
}

impl BuffMeta for BuffVentiC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::VentiC2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "温迪-「眷恋的泠风」",
            en: "Venti-Breeze of Reminiscence",
        ),
        image: BuffImage::Avatar(CharacterName::Venti),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "温迪命座2：高天之歌会使敌人的风元素抗性与物理抗性降低24%，持续10秒。",
            en: "Venti C2: Skyward Sonnet decreases opponents' Anemo RES and Physical RES by 24% for 10s.",
        )),
        from: BuffFrom::Character(CharacterName::Venti),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffVentiC2 {
        })
    }
}

pub struct BuffVentiC4 {
}

impl<A: Attribute> Buff<A> for BuffVentiC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusAnemo, "温迪「自由的凛风」", 0.25);
    }
}

impl BuffMeta for BuffVentiC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::VentiC4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "温迪-「自由的凛风」",
            en: "Venti-Breeze of Reminiscence",
        ),
        image: BuffImage::Avatar(CharacterName::Venti),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "温迪命座4：温迪施放元素战技高天之歌或元素爆发风神之诗后，温迪与队伍中自己的当前场上其他角色获得25%风元素伤害加成，持续10秒。",
            en: "Venti C4: After unleashing his Elemental Skill Skyward Sonnet or Elemental Burst Wind's Grand Ode, Venti and his own active party members gain a 25% Anemo DMG Bonus for 10s.",
        )),
        from: BuffFrom::Character(CharacterName::Venti),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffVentiC4 {
        })
    }
}


pub struct BuffVentiC6 {
    pub elemental_absorption: Option<Element>,
}

impl<A: Attribute> Buff<A> for BuffVentiC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusAnemo, "温迪「抗争的暴风」", 0.2);
        if self.elemental_absorption != None {
            attribute.set_value_by(AttributeName::res_minus_name_by_element(self.elemental_absorption.unwrap()), "温迪「抗争的暴风」", 0.2);
        }
    }
}

impl BuffMeta for BuffVentiC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::VentiC6,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "温迪-「抗争的暴风」",
            en: "Venti-Storm of Defiance",
        ),
        image: BuffImage::Avatar(CharacterName::Venti),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "温迪命座6：温迪六命BUFF。受风神之诗伤害的敌人，风元素抗性降低20％。若产生了元素转化，则使转换的元素抗性也降低20％。",
            en: "温迪命座6：温迪六命BUFF。受风神之诗伤害的敌人，风元素抗性降低20％。若产生了元素转化，则使转换的元素抗性也降低20％。",
        )),
        from: BuffFrom::Character(CharacterName::Venti),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "elemental_absorption",
            title: locale!(
                zh_cn: "元素转化",
                en: "Elemental Absorption"
            ),
            config: ItemConfigType::ElementOptional { 
                elements: &[Element::Pyro, Element::Hydro, Element::Electro, Element::Cryo], 
                default: None, 
            }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let elemental_absorption = match *b {
            BuffConfig::VentiC6 { elemental_absorption } => elemental_absorption,
            _ => None
        };

        Box::new(BuffVentiC6 {
            elemental_absorption
        })
    }
}
