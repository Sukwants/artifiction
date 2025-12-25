use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffNightweaversLookingGlass {
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffNightweaversLookingGlass {
    fn change_attribute(&self, attribute: &mut A) {
        let refine = self.refine as f64;

        attribute.set_value_by(AttributeName::EnhanceBloom, "纺夜天镜「千年的祷咏歌」", 0.9 + 0.3 * refine);
        attribute.set_value_by(AttributeName::EnhanceHyperbloom, "纺夜天镜「千年的祷咏歌」", 0.6 + 0.2 * refine);
        attribute.set_value_by(AttributeName::EnhanceBurgeon, "纺夜天镜「千年的祷咏歌」", 0.6 + 0.2 * refine);
        attribute.set_value_by(AttributeName::EnhanceLunarBloom, "纺夜天镜「千年的祷咏歌」", 0.3 + 0.1 * refine);
    }
}

impl BuffMeta for BuffNightweaversLookingGlass {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NightweaversLookingGlass,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "纺夜天镜-「千年的祷咏歌」",
            en: "Nightweaver's Looking Glass-Millennial Hymn",
        ),
        image: BuffImage::Weapon(WeaponName::NightweaversLookingGlass),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "「终北圣言」效果与「朔月诗篇」效果同时存在时，队伍中附近的所有角色触发的绽放反应造成的伤害提升 <span style=\"color: #409EFF;\">120%-150%-180%-210%-240%</span> ，超绽放、烈绽放反应造成的伤害提升 <span style=\"color: #409EFF;\">80%-100%-120%-140%-160%</span> ，月绽放反应伤害提升 <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> ，该效果无法叠加。装备者处于队伍后台时，依然能触发上述效果。",
            en: "When both Prayer of the Far North and New Moon Verse are in effect, all nearby party members' Bloom DMG is increased by <span style=\"color: #409EFF;\">120%-150%-180%-210%-240%</span> , their Hyperbloom and Burgeon DMG is increased by <span style=\"color: #409EFF;\">80%-100%-120%-140%-160%</span> , and their Lunar-Bloom DMG is increased by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> . This effect cannot stack. The aforementioned effects can be triggered even if the equipping character is off-field.",
        )),
        from: BuffFrom::Weapon(WeaponName::NightweaversLookingGlass),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::NightweaversLookingGlass { refine } => refine,
            _ => 1
        };

        Box::new(BuffNightweaversLookingGlass {
            refine
        })
    }
}

