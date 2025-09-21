use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Moonsign;

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
