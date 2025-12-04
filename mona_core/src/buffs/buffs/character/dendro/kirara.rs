use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffKiraraC6 {
}

impl<A: Attribute> Buff<A> for BuffKiraraC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPyro, "绮良良「沿途百景会心」", 0.12);
        attribute.set_value_by(AttributeName::BonusHydro, "绮良良「沿途百景会心」", 0.12);
        attribute.set_value_by(AttributeName::BonusAnemo, "绮良良「沿途百景会心」", 0.12);
        attribute.set_value_by(AttributeName::BonusElectro, "绮良良「沿途百景会心」", 0.12);
        attribute.set_value_by(AttributeName::BonusDendro, "绮良良「沿途百景会心」", 0.12);
        attribute.set_value_by(AttributeName::BonusCryo, "绮良良「沿途百景会心」", 0.12);
        attribute.set_value_by(AttributeName::BonusGeo, "绮良良「沿途百景会心」", 0.12);
    }
}

impl BuffMeta for BuffKiraraC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::KiraraC6,
        name_locale: locale!(
            zh_cn: "绮良良-「沿途百景会心」",
            en: "Kirara-Countless Sights to See"
        ),
        image: BuffImage::Avatar(CharacterName::Kirara),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "绮良良命座6：绮良良施放元素战技或元素爆发后的15秒内，附近的队伍中所有角色获得12%所有元素伤害加成。",
            en: "Kirara C6: All nearby party members gain a 12% All Elemental DMG Bonus for 15s after Kirara uses her Elemental Skill or Burst."
        )),
        from: BuffFrom::Character(CharacterName::Kirara),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffKiraraC6 {
        })
    }
}
