use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffSethosC4 {
}

impl<A: Attribute> Buff<A> for BuffSethosC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "赛索斯「真念鸵羽集」", 80.0);
    }
}

impl BuffMeta for BuffSethosC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SethosC4,
        name_locale: locale!(
            zh_cn: "赛索斯-「真念鸵羽集」",
            en: "Sethos-Beneficent Plumage"
        ),
        image: BuffImage::Avatar(CharacterName::Sethos),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "赛索斯命座4：贯影箭或瞑弦矢命中2名及以上的敌人时，队伍中附近的所有角色的元素精通提升80点，持续10秒。",
            en: "Sethos C4: When a Shadowpiercing Shot or Dusk Bolt strikes 2 or more opponents, all nearby party members gain 80 Elemental Mastery for 10s."
        )),
        from: BuffFrom::Character(CharacterName::Sethos),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffSethosC4 {
        })
    }
}
