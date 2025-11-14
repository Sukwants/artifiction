use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Element;

pub struct BuffAetherGeoC1 {
}

impl<A: Attribute> Buff<A> for BuffAetherGeoC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::CriticalBase, "空-岩「巍然的青岩」", 0.10);
    }
}

impl BuffMeta for BuffAetherGeoC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AetherGeoC1,
        name_locale: locale!(
            zh_cn: "空-岩-「巍然的青岩」",
            en: "Aether-Geo-Invincible Stonewall"
        ),
        image: BuffImage::Avatar(CharacterName::AetherGeo),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "队伍中角色处于岩潮叠嶂的岩嶂包围中时，暴击率提升10%，并提高抗打断能力。",
            en: "Party members within the radius of Wake of Earth have their CRIT Rate increased by 10% and have increased resistance against interruption."
        )),
        from: BuffFrom::Character(CharacterName::AetherGeo),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAetherGeoC1 {
        })
    }
}