use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffLaylaC4 {
    pub hp: f64,
}

impl<A: Attribute> Buff<A> for BuffLaylaC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ExtraDmgNormalAttack, "莱依拉「星示昭明」", self.hp * 0.05);
        attribute.set_value_by(AttributeName::ExtraDmgChargedAttack, "莱依拉「星示昭明」", self.hp * 0.05);
    }
}

impl BuffMeta for BuffLaylaC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LaylaC4,
        name_locale: locale!(
            zh_cn: "莱依拉-「星示昭明」",
            en: "Layla-Starry Illumination"
        ),
        image: BuffImage::Avatar(CharacterName::Layla),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "莱依拉命座4：垂裳端凝之夜开始发射一轮飞星时，将为附近的队伍中所有角色赋予「启明」效果，使普通攻击与重击造成的伤害提升，提升值相当于莱依拉生命值上限的5%。",
            en: "Layla C4: When Nights of Formal Focus starts to fire off Shooting Stars, it will grant all nearby party members the Dawn Star effect, causing their Normal and Charged Attack DMG to increase based on 5% of Layla's Max HP."
        )),
        from: BuffFrom::Character(CharacterName::Layla),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp",
            title: locale!(
                zh_cn: "生命值上限",
                en: "Max HP"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let hp = match *b {
            BuffConfig::LaylaC4 { hp } => hp,
            _ => 0.0
        };
        Box::new(BuffLaylaC4 {
            hp
        })
    }
}
