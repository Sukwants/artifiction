use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffLynetteP1 {
    pub el_count: usize,
}

impl<A: Attribute> Buff<A> for BuffLynetteP1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("琳妮特「巧施协同」", (self.el_count as f64 * 0.04 + 0.04).min(0.2));
    }
}

impl BuffMeta for BuffLynetteP1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LynetteP1,
        name_locale: locale!(
            zh_cn: "琳妮特-「巧施协同」",
            en: "Lynette-Sophisticated Synergy"
        ),
        image: BuffImage::Avatar(CharacterName::Lynette),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "琳妮特天赋1：施放魔术·运变惊奇后的10秒内，队伍中分别存在1/2/3/4种元素类型的角色时，队伍中所有角色的攻击力分别提升8%/12%/16%/20%。",
            en: "Lynette Talent 1: Within 10s after using Magic Trick: Astonishing Shift, when there are 1/2/3/4 Elemental Types in the party, all party members' ATK will be increased by 8%/12%/16%/20% respectively."
        )),
        from: BuffFrom::Character(CharacterName::Lynette),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "el_count",
            title: locale!(
                zh_cn: "队伍中角色元素种类数",
                en: "Elemental Types Count in the Party"
            ),
            config: ItemConfigType::Int { min: 1, max: 4, default: 1 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let el_count = match *b {
            BuffConfig::LynetteP1 { el_count } => el_count,
            _ => 0
        };
        Box::new(BuffLynetteP1 {
            el_count
        })
    }
}
