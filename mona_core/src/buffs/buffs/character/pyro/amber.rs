use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffAmberC6 {
}

impl<A: Attribute> Buff<A> for BuffAmberC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("安柏「疾如野火」", 0.15);
    }
}

impl BuffMeta for BuffAmberC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AmberC6,
        name_locale: locale!(
            zh_cn: "安柏-「疾如野火」",
            en: "Amber-Wildfire"
        ),
        image: BuffImage::Avatar(CharacterName::Amber),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "安柏命座6：使用箭雨后的10秒内，队伍中所有角色的攻击力提升15%。",
            en: "Amber C6: Fiery Rain increases all party members' ATK by 15% for 10s."
        )),
        from: BuffFrom::Character(CharacterName::Amber),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAmberC6 {
        })
    }
}
