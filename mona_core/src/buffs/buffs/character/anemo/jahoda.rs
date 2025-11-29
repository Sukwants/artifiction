use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::{Element, Moonsign};
use crate::common::i18n::locale;
use crate::common::item_config_type::{ConfigElements8Multi, ItemConfig, ItemConfigType};

pub struct BuffJahodaP2 {
}

impl<A: Attribute> Buff<A> for BuffJahodaP2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "雅珂达「蜜莓的嘉赏」", 100.0);
    }
}

impl BuffMeta for BuffJahodaP2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::JahodaP2,
        name_locale: locale!(
            zh_cn: "雅珂达-「蜜莓的嘉赏」",
            en: "Jahoda-Sweet Berry Bounty"
        ),
        image: BuffImage::Avatar(CharacterName::Jahoda),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "雅珂达天赋2：元素爆发秘器·猎人的七道具中的猫型家用互助协调器对队伍中自己的当前场上角色触发治疗时，若受到治疗的角色生命值高于70%，则该角色的元素精通提升100点，持续6秒。",
            en: "Jahoda Talent2: When a Purrsonal Coordinated Assistance Robot from the Elemental Burst Hidden Aces: Seven Tools of the Hunter triggers healing on an active team member, if that character's HP is above 70%, their Elemental Mastery will be increased by 100 for 6s."
        )),
        from: BuffFrom::Character(CharacterName::Jahoda),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffJahodaP2 {
        })
    }
}

pub struct BuffJahodaC6 {
    pub moonsign: Moonsign,
}

impl<A: Attribute> Buff<A> for BuffJahodaC6 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.moonsign.is_ascendant() {
            attribute.set_value_by(AttributeName::CriticalBase, "雅珂达「最渺小的幸运」", 0.05);
            attribute.set_value_by(AttributeName::CriticalDamageBase, "雅珂达「最渺小的幸运」", 0.40);
        }
    }
}

impl BuffMeta for BuffJahodaC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::JahodaC6,
        name_locale: locale!(
            zh_cn: "雅珂达-「最渺小的幸运」",
            en: "Jahoda-The Littlest Luck"
        ),
        image: BuffImage::Avatar(CharacterName::Jahoda),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "雅珂达命座6：月兆·满辉：元素战技奇策·财富分配方案中的呼噜噜秘藏瓶装满后的20秒内，队伍中附近的月兆角色的暴击率提升5%，暴击伤害提升40%。",
            en: "Jahoda C6: Moonsign: Ascendant Gleam: After the Purr-loined Treasure Flask from the Elemental Skill Savvy Strategy: Splitting the Spoils is full, nearby Moonsign characters in your party have their CRIT Rate increased by 5% and CRIT DMG increased by 40% for 20s."
        )),
        from: BuffFrom::Character(CharacterName::Jahoda),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_BUFF),
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let moonsign = match *b {
            BuffConfig::JahodaC6 { moonsign } => moonsign,
            _ => Moonsign::None,
        };
        Box::new(BuffJahodaC6 {
            moonsign
        })
    }
}