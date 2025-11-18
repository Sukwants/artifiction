use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Element;

pub struct BuffLumineAnemoC6 {
    pub has_elemental_absorption: bool,
    pub elemental_absorption: Element,
}

impl<A: Attribute> Buff<A> for BuffLumineAnemoC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusAnemo, "荧-风「纠缠的信风」", 0.20);
        if self.has_elemental_absorption {
            attribute.set_value_by(AttributeName::res_minus_name_by_element(self.elemental_absorption), "荧-风「纠缠的信风」", 0.20);
        }
    }
}

impl BuffMeta for BuffLumineAnemoC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LumineAnemoC6,
        name_locale: locale!(
            zh_cn: "荧-风-「纠缠的信风」",
            en: "Lumine-Anemo-Intertwined Winds"
        ),
        image: BuffImage::Avatar(CharacterName::LumineAnemo),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "荧-风命座6：受到风息激荡伤害的目标，风元素抗性下降20%。如果产生了元素转化，那么对应元素抗性也下降20%。",
            en: "Lumine-Anemo C6: Targets who take DMG from Gust Surge have their Anemo RES decreased by 20%. If an Elemental Absorption occrred, then their RES towards the corresponding Element is also decreased by 20%."
        )),
        from: BuffFrom::Character(CharacterName::LumineAnemo),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "has_elemental_absorption",
            title: locale!(zh_cn: "是否发生元素吸收", en: "Whether Elemental Absorption Occurred"),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "elemental_absorption",
            title: locale!(zh_cn: "元素吸收类型", en: "Elemental Absorption Type"),
            config: ItemConfigType::Element4 { default: Element::Cryo }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (has_elemental_absorption, elemental_absorption) = match *b {
            BuffConfig::LumineAnemoC6 { has_elemental_absorption, elemental_absorption } => (has_elemental_absorption, elemental_absorption),
            _ => (false, Element::Cryo)
        };
        Box::new(BuffLumineAnemoC6 {
            has_elemental_absorption,
            elemental_absorption,
        })
    }
}