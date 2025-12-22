use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffAthameArtis {
    pub refine: usize,
    pub hexerei_secret_rite: bool,
}

impl<A: Attribute> Buff<A> for BuffAthameArtis {
    fn change_attribute(&self, attribute: &mut A) {
        let refine: f64 = self.refine as f64;

        attribute.add_atk_percentage("黑蚀「白昼之刃」", (refine * 0.05 + 0.15) * if self.hexerei_secret_rite { 1.75 } else { 1.0 });
    }
}

impl BuffMeta for BuffAthameArtis {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AthameArtis,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "黑蚀-「白昼之刃」",
            en: "Athame Artis-the Daylight Hours",
        ),
        image: BuffImage::Weapon(WeaponName::AthameArtis),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "元素爆发命中敌人时，将获得「白昼之刃」效果：除装备者以外，队伍中附近的当前场上角色攻击力提升 <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> ，持续3秒。\
                <br>此外，队伍拥有「魔导·秘仪」效果时，「白昼之刃」的效果额外提升75%。",
            en: "When an Elemental Burst hits an opponent, gain the Blade of the Daylight Hours effect: Nearby active party members other than the equipping character have their ATK increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> for 3s. \
                <br>Additionally, when the party possesses Hexerei: Secret Rite effects, the effects of Blade of the Daylight Hours are increased by an additional 75%.",
        )),
        from: BuffFrom::Weapon(WeaponName::AthameArtis)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_WEAPON),
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (refine, hexerei_secret_rite) = match *b {
            BuffConfig::AthameArtis { refine, hexerei_secret_rite } => (refine, hexerei_secret_rite),
            _ => (1, false)
        };

        Box::new(BuffAthameArtis { refine, hexerei_secret_rite })
    }
}
