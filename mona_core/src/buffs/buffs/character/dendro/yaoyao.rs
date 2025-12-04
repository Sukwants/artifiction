use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffYaoyaoC1 {
}

impl<A: Attribute> Buff<A> for BuffYaoyaoC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusDendro, "瑶瑶「妙受琼阁」", 0.15);
    }
}

impl BuffMeta for BuffYaoyaoC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::YaoyaoC1,
        name_locale: locale!(
            zh_cn: "瑶瑶-「妙受琼阁」",
            en: "Yaoyao-Adeptus' Tutelage"
        ),
        image: BuffImage::Avatar(CharacterName::Yaoyao),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "瑶瑶命座1：白玉萝卜炸裂时，处在其影响范围内的当前场上角色获得15%草元素伤害加成，持续8秒。",
            en: "Yaoyao C1: When White Jade Radishes explode, active characters within their AoE will gain 15% Dendro DMG Bonus for 8s."
        )),
        from: BuffFrom::Character(CharacterName::Yaoyao),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffYaoyaoC1 {
        })
    }
}
