use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffLanyanC4 {
}

impl<A: Attribute> Buff<A> for BuffLanyanC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "蓝砚「『揽龙鹰兮结血珠』」", 60.0);
    }
}

impl BuffMeta for BuffLanyanC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LanyanC4,
        name_locale: locale!(
            zh_cn: "蓝砚-「『揽龙鹰兮结血珠』」",
            en: "Lan Yan-\"With Drakefalcon's Blood-Pearls Adorned\""
        ),
        image: BuffImage::Avatar(CharacterName::Lanyan),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "蓝砚命座4：施放元素爆发鹍弦踏月出之后的12秒内，队伍中附近所有角色的元素精通提升60点。",
            en: "Lan Yan C4: After Lan Yan uses her Elemental Burst Lustrous Moonrise, the Elemental Mastery of all nearby party members increases by 60 for 12s."
        )),
        from: BuffFrom::Character(CharacterName::Lanyan),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffLanyanC4 {
        })
    }
}
