use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffOroronC6 {
    stack: f64,
}

impl<A: Attribute> Buff<A> for BuffOroronC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("欧洛伦「致深泉的颂赞」", 0.1 * self.stack);
    }
}

impl BuffMeta for BuffOroronC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::OroronC6,
        name_locale: locale!(
            zh_cn: "欧洛伦-「致深泉的颂赞」",
            en: "Ororon-Ode to Deep Springs"
        ),
        image: BuffImage::Avatar(CharacterName::Ororon),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "欧洛伦命座6：触发突破天赋「夜翳的通感」的「显象超感」后，会使队伍中自己的当前场上角色的攻击力提升10%，持续9秒。该效果至多叠加3层，每层独立计算持续时间。",
            en: "Ororon C6: After triggering Hypersense through the Ascension Talent, Nightshade Synesthesia, your current active character's ATK is increased by 10% for 9s. Max 3 stacks, each stack is counted independently."
        )),
        from: BuffFrom::Character(CharacterName::Ororon),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: crate::common::i18n::locale!(
                zh_cn: "平均层数",
                en: "Average Stacks",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let stack = match *b {
            BuffConfig::OroronC6 { stack } => stack,
            _ => 0.0
        };
        Box::new(BuffOroronC6 {
            stack
        })
    }
}
