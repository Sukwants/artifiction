use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Moonsign;

pub struct BuffFlinsP3 {
    pub atk: f64,
}

impl<A: Attribute> Buff<A> for BuffFlinsP3 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::IncreaseLunarCharged, "菲林斯「月兆祝赐·旧世潜藏」", (self.atk * 0.00007).min(0.14));
    }
}

impl BuffMeta for BuffFlinsP3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FlinsP3,
        name_locale: locale!(
            zh_cn: "菲林斯-「月兆祝赐·旧世潜藏」",
            en: "Flins-Moonsign Benediction: Old World Secrets"
        ),
        image: BuffImage::Avatar(CharacterName::Flins),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "队伍中的角色触发感电反应时，将转为触发月感电反应，且基于菲林斯的攻击力，提升月感电反应的基础伤害：每100点攻击力都将提升0.7%基础伤害，至多通过这种方式提升14%伤害。",
            en: "When a party member triggers an Electro-Charged reaction, it will be converted into the Lunar-Charged reaction, with every 100 ATK that Flins has increasing Lunar-Charged's Base DMG by 0.7%, up to a maximum of 14%."
        )),
        from: BuffFrom::Character(CharacterName::Flins),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "atk",
            title: locale!(
                zh_cn: "攻击力",
                en: "ATK"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let atk = match *b {
            BuffConfig::FlinsP3 { atk } => atk,
            _ => 0.0
        };
        Box::new(BuffFlinsP3 {
            atk,
        })
    }
}

pub struct BuffFlinsC6 {
    pub moonsign: Moonsign,
}

impl<A: Attribute> Buff<A> for BuffFlinsC6 {
    fn change_attribute(&self, attribute: &mut A) {

        if self.moonsign.is_ascendant() {
            attribute.set_value_by(AttributeName::IncreaseLunarCharged, "菲林斯「歌与亡者之舞」", 0.1);
        }
    }
}

impl BuffMeta for BuffFlinsC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FlinsC6,
        name_locale: locale!(
            zh_cn: "菲林斯-「歌与亡者之舞」",
            en: "Flins-Songs and Dances of Death"
        ),
        image: BuffImage::Avatar(CharacterName::Flins),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "月兆·满辉：队伍中附近的所有角色造成的月感电反应伤害擢升10%。",
            en: "Moonsign: Ascendant Gleam: All nearby party members' Lunar-Charged DMG is elevated by 10%."
        )),
        from: BuffFrom::Character(CharacterName::Flins),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN2
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let moonsign = match *b {
            BuffConfig::FlinsC6 { moonsign } => moonsign,
            _ => Moonsign::None
        };
        Box::new(BuffFlinsC6 {
            moonsign,
        })
    }
}
