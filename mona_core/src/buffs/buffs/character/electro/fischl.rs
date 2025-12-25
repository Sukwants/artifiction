use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::Element;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffFischlP3 {
    pub overload: bool,
    pub charged: bool,
    pub has_c6: bool,
}

impl<A: Attribute> Buff<A> for BuffFischlP3 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.overload {
            attribute.add_atk_percentage("菲谢尔「魔女的前夜礼·宵世幻奏」", 0.225 * if self.has_c6 { 2.0 } else { 1.0 });
        }

        if self.charged {
            attribute.set_value_by(AttributeName::ElementalMastery, "菲谢尔「魔女的前夜礼·宵世幻奏」", 90.0 * if self.has_c6 { 2.0 } else { 1.0 });
        }
    }
}

impl BuffMeta for BuffFischlP3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FischlP3,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "菲谢尔-「魔女的前夜礼·宵世幻奏」",
            en: "Fischl-Witch's Eve Rite: Phantasmal Nocturne",
        ),
        image: BuffImage::Avatar(CharacterName::Fischl),
        genre: BuffGenre::Character,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "菲谢尔天赋3：魔导·秘仪：\
                <br>·队伍中附近的角色触发超载反应后的10秒内，菲谢尔与队伍中附近的当前场上其他角色的攻击力提升22.5%；\
                <br>·队伍中附近的角色触发感电或月感电反应后的10秒内，菲谢尔与队伍中附近的当前场上其他角色元素精通提升90点。",
            en: "Fischl Talent3: Hexerei: Secret Rite: \
                <br>· After a nearby party member triggers an Overloaded reaction, Fischl and other nearby active party members have their ATK increased by 22.5% for 10s. \
                <br>· After a nearby party member triggers an Electro-Charged or Lunar-Charged reaction, Fischl and other nearby active party members have their Elemental Mastery increased by 90 for 10s.",
        )),
        from: BuffFrom::Character(CharacterName::Fischl),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "overload",
            title: crate::common::i18n::locale!(
                zh_cn: "是否触发超载反应",
                en: "Whether Overloaded reaction is triggered",
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "charged",
            title: crate::common::i18n::locale!(
                zh_cn: "是否触发感电或月感电反应",
                en: "Whether Electro-Charged or Lunar-Charged reaction is triggered",
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "has_c6",
            title: crate::common::i18n::locale!(
                zh_cn: "六命",
                en: "C6",
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (overload, charged, has_c6) = match *b {
            BuffConfig::FischlP3 { overload, charged, has_c6 } => (overload, charged, has_c6),
            _ => (false, false, false),
        };

        Box::new(BuffFischlP3 {
            overload,
            charged,
            has_c6,
        })
    }
}
