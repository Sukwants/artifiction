use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Moonsign;

pub struct BuffNeferP3 {
    pub em: f64,
}

impl<A: Attribute> Buff<A> for BuffNeferP3 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::IncreaseLunarBloom, "奈芙尔「月兆祝赐·廊下暮影」", (self.em * 0.000175).min(0.14));
    }
}

impl BuffMeta for BuffNeferP3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NeferP3,
        name_locale: locale!(
            zh_cn: "奈芙尔-「月兆祝赐·廊下暮影」",
            en: "Nefer-Moonsign Benediction: Old World Secrets"
        ),
        image: BuffImage::Avatar(CharacterName::Nefer),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "队伍中的角色触发绽放反应时，将转为触发月绽放反应，且基于奈芙尔的元素精通，提升队伍中角色造成的月绽放反应的基础伤害：每点元素精通都将提升0.0175%月绽放反应的基础伤害，至多通过这种方式提升14%伤害。",
            en: "When a party member triggers a Bloom reaction, it will be converted into the Lunar-Bloom reaction, with every point of Elemental Mastery that Nefer has increasing Lunar-Bloom's Base DMG by 0.0175%, up to a maximum of 14%."
        )),
        from: BuffFrom::Character(CharacterName::Nefer),
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
            BuffConfig::NeferP3 { em } => em,
            _ => 0.0
        };
        Box::new(BuffNeferP3 {
            em,
        })
    }
}