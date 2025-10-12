use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffSymphonistOfScents {
    refine: usize
}

impl<A: Attribute> Buff<A> for BuffSymphonistOfScents {
    fn change_attribute(&self, attribute: &mut A) {

        attribute.add_atk_percentage("BUFF: 香韵奏者「甘美回奏」", self.refine as f64 * 0.08 + 0.24);
    }
}

impl BuffMeta for BuffSymphonistOfScents {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SymphonistOfScents,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "香韵奏者-「甘美回奏」",
            en: "Symphonist of Scents-Seasoned Symphony",
        ),
        image: BuffImage::Weapon(WeaponName::SymphonistOfScents),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "进行治疗后，装备者与受到治疗的角色会获得「甘美回奏」的效果，攻击力提升 <span style=\"color: #409EFF;\">32%-40%-48%-56%-64%</span> ，持续3秒。装备者处于队伍后台时，依然能触发上述效果。",
            en: "After initiating healing, the equipping character and the character(s) they have healed will obtain the \"Sweet Echoes\" effect, increasing their ATK by <span style=\"color: #409EFF;\">32%-40%-48%-56%-64%</span> for 3s. This effect can be triggered even if the equipping character is off-field.",
        )),
        from: BuffFrom::Weapon(WeaponName::SymphonistOfScents)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::SymphonistOfScents { refine } => refine,
            _ => 1
        };

        Box::new(BuffSymphonistOfScents { refine })
    }
}
