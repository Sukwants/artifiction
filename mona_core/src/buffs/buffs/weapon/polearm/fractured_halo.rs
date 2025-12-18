use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffFracturedHalo {
    refine: usize
}

impl<A: Attribute> Buff<A> for BuffFracturedHalo {
    fn change_attribute(&self, attribute: &mut A) {

        attribute.set_value_by(AttributeName::EnhanceLunarCharged, "BUFF: 支离轮光「流电圣敕」", self.refine as f64 * 0.1 + 0.3);
    }
}

impl BuffMeta for BuffFracturedHalo {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FracturedHalo,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "支离轮光-「流电圣敕」",
            en: "Fractured Halo-Electrifying Edict",
        ),
        image: BuffImage::Weapon(WeaponName::FracturedHalo),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素战技或元素爆发后的20秒内，攻击力提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>。持续期间内，若装备者创造了护盾，则接下来的20秒内，还会获得「流电圣敕」效果：队伍中附近所有角色触发的月感电反应造成的伤害提升<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>。",
            en: "After an Elemental Skill or Elemental Burst is used, ATK is increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> for 20s. If the equipping character creates a Shield while this effect is active, they will gain the Electrifying Edict effect for 20s: All nearby party members deal <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> more Lunar-Charged DMG.",
        )),
        from: BuffFrom::Weapon(WeaponName::FracturedHalo)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::FracturedHalo { refine } => refine,
            _ => 1
        };

        Box::new(BuffFracturedHalo { refine })
    }
}
