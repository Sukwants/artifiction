use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Element;

pub struct BuffLumineElectroC2 {
}

impl<A: Attribute> Buff<A> for BuffLumineElectroC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusElectro, "荧-雷「震怒的苍雷」", 0.15);
    }
}

impl BuffMeta for BuffLumineElectroC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LumineElectroC2,
        name_locale: locale!(
            zh_cn: "荧-雷-「震怒的苍雷」",
            en: "Lumine-Electro-Violet Vehemence"
        ),
        image: BuffImage::Avatar(CharacterName::LumineElectro),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "雷轰电转的威光落雷命中敌人后，会使敌人的雷元素抗性降低15%，持续8秒。",
            en: "When Falling Thunder created by Bellowing Thunder hits an opponent, it will decrease their Electro RES by 15% for 8s."
        )),
        from: BuffFrom::Character(CharacterName::LumineElectro),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffLumineElectroC2 {
        })
    }
}