use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffColleiC4 {
}

impl<A: Attribute> Buff<A> for BuffColleiC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "柯莱「骞林馈遗」", 60.0);
    }
}

impl BuffMeta for BuffColleiC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ColleiC4,
        name_locale: locale!(
            zh_cn: "柯莱-「骞林馈遗」",
            en: "Collei-Gift of the Woods"
        ),
        image: BuffImage::Avatar(CharacterName::Collei),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "柯莱命座4：施放猫猫秘宝时，将使队伍中附近的所有角色（不包括柯莱自己）的元素精通提升60点，持续12秒。",
            en: "Collei C4: Using Trump-Card Kitty will increase all nearby characters' Elemental Mastery by 60 for 12s (not including Collei herself)."
        )),
        from: BuffFrom::Character(CharacterName::Collei),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffColleiC4 {
        })
    }
}
