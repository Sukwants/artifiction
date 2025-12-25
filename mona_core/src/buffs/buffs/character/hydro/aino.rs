use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Moonsign;

pub struct BuffAinoC1 {
}

impl<A: Attribute> Buff<A> for BuffAinoC1 {
    fn change_attribute(&self, attribute: &mut A) {

        attribute.set_value_by(AttributeName::ElementalMastery, "爱诺「灰与力场的平衡理论」", 80.0)
    }
}

impl BuffMeta for BuffAinoC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AinoC1,
        name_locale: locale!(
            zh_cn: "爱诺-「灰与力场的平衡理论」",
            en: "Aino-The Theory of Ash–Field Equilibrium"
        ),
        image: BuffImage::Avatar(CharacterName::Aino),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "爱诺施放元素战技妙思捕手或元素爆发精密水冷仪后，爱诺自身的元素精通提升80点，附近的当前场上角色的元素精通提升80点，持续15秒。",
            en: "After Aino uses her Elemental Skill Musecatcher or her Elemental Burst Precision Hydronic Cooler, her Elemental Mastery will be increased by 80. The Elemental Mastery of nearby active party members will be increased by 80 for 15s."
        )),
        from: BuffFrom::Character(CharacterName::Aino),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAinoC1 {})
    }
}
pub struct BuffAinoC6 {
    moonsign: Moonsign,
}

impl<A: Attribute> Buff<A> for BuffAinoC6 {
    fn change_attribute(&self, attribute: &mut A) {

        let val = match self.moonsign {
            Moonsign::Ascendant => 0.35,
            Moonsign::Nascent => 0.15,
            Moonsign::None => 0.0
        };

        attribute.set_value_by(AttributeName::EnhanceElectroCharged, "爱诺「天才之为构造之责任」", val);
        attribute.set_value_by(AttributeName::EnhanceBloom, "爱诺「天才之为构造之责任」", val);
        attribute.set_value_by(AttributeName::EnhanceLunarCharged, "爱诺「天才之为构造之责任」", val);
        attribute.set_value_by(AttributeName::EnhanceLunarBloom, "爱诺「天才之为构造之责任」", val);
    }
}

impl BuffMeta for BuffAinoC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AinoC6,
        name_locale: locale!(
            zh_cn: "爱诺-「天才之为构造之责任」",
            en: "Aino-The Burden of Creative Genius"
        ),
        image: BuffImage::Avatar(CharacterName::Aino),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "施放元素爆发精密水冷仪后的15秒内，附近的当前场上角色触发的感电、绽放、月感电、月绽放造成的伤害提升15%。月兆·满辉：上述反应造成的伤害额外提升20%。",
            en: "For the next 15s after using the Elemental Burst Precision Hydronic Cooler, DMG from nearby active characters' Electro-Charged, Bloom, Lunar-Charged, and Lunar-Bloom reactions is increased by 15%. Moonsign: Ascendant Gleam: DMG from the aforementioned reactions will be further increased by 20%."
        )),
        from: BuffFrom::Character(CharacterName::Aino),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_BUFF),
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let moonsign = match *b {
            BuffConfig::AinoC6 { moonsign } => moonsign,
            _ => Moonsign::None,
        };
        Box::new(BuffAinoC6 { moonsign })
    }
}