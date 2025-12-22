use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffAThousandFloatingDreams {
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffAThousandFloatingDreams {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery ,"BUFF: 千夜浮梦-「千夜的曙歌」", self.refine as f64 * 2.0 + 38.0);
    }
}

impl BuffMeta for BuffAThousandFloatingDreams {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AThousandFloatingDreams,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "千夜浮梦-「千夜的曙歌」",
            en: "A Thousand Floating Dreams-A Thousand Nights' Dawnsong",
        ),
        image: BuffImage::Weapon(WeaponName::AThousandFloatingDreams),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "队伍中装备者以外的附近角色的元素精通提升<span style=\"color: #409EFF;\">40-42-44-46-48</span>点，多件同名武器产生的此效果可以叠加。",
            en: "All nearby party members other than the equipping character will have their Elemental Mastery increased by <span style=\"color: #409EFF;\">40-42-44-46-48</span>. Multiple such effects from multiple such weapons can stack.",
        )),
        from: BuffFrom::Weapon(WeaponName::AThousandFloatingDreams),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::AThousandFloatingDreams { refine } => refine,
            _ => 1
        };

        Box::new(BuffAThousandFloatingDreams {
            refine
        })
    }
}

