use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffBarbaraC2 {
}

impl<A: Attribute> Buff<A> for BuffBarbaraC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusHydro, "芭芭拉「元气迸发」", 0.15);
    }
}

impl BuffMeta for BuffBarbaraC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BarbaraC2,
        name_locale: locale!(
            zh_cn: "芭芭拉-「元气迸发」",
            en: "Barbara-Vitality Burst"
        ),
        image: BuffImage::Avatar(CharacterName::Barbara),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "芭芭拉命座2：演唱，开始♪技能持续期间，当前场上自己的角色获得15%水元素伤害加成。",
            en: "Barbara C2: During the ability Let the Show Begin♪'s duration, your active character gains a 15% Hydro DMG Bonus."
        )),
        from: BuffFrom::Character(CharacterName::Barbara),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffBarbaraC2 {
        })
    }
}
