use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffFantasticalBlessings {
}

impl<A: Attribute> Buff<A> for BuffFantasticalBlessings {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_hp_percentage("幻境祝福", 0.2);
        attribute.add_atk_percentage("幻境祝福", 0.2);
        attribute.add_def_percentage("幻境祝福", 0.2);
    }
}

impl BuffMeta for BuffFantasticalBlessings {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FantasticalBlessings,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "幻境祝福",
            en: "Fantastical Blessings",
        ),
        image: BuffImage::Misc("imaginarium_theater"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "解锁幻想真境剧诗后，将出现每期更新的幻境祝福，为当期开幕角色提供增益效果。当期开幕角色编入队伍后，该角色生命值上限、攻击力与防御力提升 20%。",
            en: "After unlocking the Imaginarium Theater, the Fantastical Blessings available will refresh each season, offering buffs for the Opening Characters. After the Opening Characters join your party, the character's Max HP, ATK, and DEF are increased by 20%.",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffFantasticalBlessings {})
    }
}