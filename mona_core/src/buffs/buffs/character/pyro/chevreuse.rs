use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffChevreuseP1 {
}

impl<A: Attribute> Buff<A> for BuffChevreuseP1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusPyro, "夏沃蕾「尖兵协同战法」", 0.4);
        attribute.set_value_by(AttributeName::ResMinusElectro, "夏沃蕾「尖兵协同战法」", 0.4);
    }
}

impl BuffMeta for BuffChevreuseP1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ChevreuseP1,
        name_locale: locale!(
            zh_cn: "夏沃蕾-「尖兵协同战法」",
            en: "Chevreuse-Vanguard's Coordinated Tactics"
        ),
        image: BuffImage::Avatar(CharacterName::Chevreuse),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "夏沃蕾天赋1：队伍中所有角色的元素类型均为火元素与雷元素，并且至少有一名火元素角色、一名雷元素角色时：夏沃蕾将为队伍中附近的角色施加「协同战法」：角色触发超载反应后，受本次反应影响的敌人的火元素与雷元素抗性降低40%，持续6秒。",
            en: "Chevreuse Talent 1: When all party members are Pyro and Electro characters and there is at least one Pyro and one Electro character each in the party: Chevreuse grants \"Coordinated Tactics\" to nearby party members: After a character triggers the Overloaded reaction, the Pyro and Electro RES of the opponent(s) affected by this Overloaded reaction will be decreased by 40% for 6s."
        )),
        from: BuffFrom::Character(CharacterName::Chevreuse),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffChevreuseP1 {
        })
    }
}

pub struct BuffChevreuseP2 {
    pub hp: f64,
}

impl<A: Attribute> Buff<A> for BuffChevreuseP2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("夏沃蕾「纵阵武力统筹」", (self.hp / 1000.0 * 0.01).min(0.4));
    }
}

impl BuffMeta for BuffChevreuseP2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ChevreuseP2,
        name_locale: locale!(
            zh_cn: "夏沃蕾-「纵阵武力统筹」",
            en: "Chevreuse-Vertical Force Coordination"
        ),
        image: BuffImage::Avatar(CharacterName::Chevreuse),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "夏沃蕾天赋2：夏沃蕾发射近迫式急促拦射的「超量装药弹头」后，基于夏沃蕾的生命值上限，每1000点生命值上限都将使队伍中附近的所有火元素与雷元素角色的攻击力提升1%，持续30秒，至多通过这种方式提升40%攻击力。",
            en: "Chevreuse Talent 2: After Chevreuse fires an Overcharged Ball using Short-Range Rapid Interdiction Fire, nearby Pyro and Electro characters in the party gain 1% increased ATK for every 1,000 Max HP Chevreuse has for 30s. ATK can be increased by up to 40% in this way."
        )),
        from: BuffFrom::Character(CharacterName::Chevreuse),
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
            BuffConfig::ChevreuseP2 { hp } => hp,
            _ => 0.0
        };
        Box::new(BuffChevreuseP2 {
            hp
        })
    }
}


pub struct BuffChevreuseC6 {
    pub stack: f64,
}

impl<A: Attribute> Buff<A> for BuffChevreuseC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPyro, "夏沃蕾「终结罪恶的追缉」", self.stack * 0.2);
        attribute.set_value_by(AttributeName::BonusElectro, "夏沃蕾「终结罪恶的追缉」", self.stack * 0.2);
    }
}

impl BuffMeta for BuffChevreuseC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ChevreuseC6,
        name_locale: locale!(
            zh_cn: "夏沃蕾-「终结罪恶的追缉」",
            en: "Chevreuse-In Pursuit of Ending Evil"
        ),
        image: BuffImage::Avatar(CharacterName::Chevreuse),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "夏沃蕾命座6：队伍中的角色受到「近迫式急促拦射」的治疗后，获得20%火元素伤害加成与雷元素伤害加成，持续8秒，此效果至多叠加3层，每层独立计算持续时间。",
            en: "Chevreuse C6: After a party member is healed by Short-Range Rapid Interdiction Fire, they gain a 20% Pyro DMG Bonus and Electro DMG Bonus for 8s. Max 3 stacks. Each stack's duration is counted independently."
        )),
        from: BuffFrom::Character(CharacterName::Chevreuse),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "平均层数",
                en: "Average Stacks"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let stack = match *b {
            BuffConfig::ChevreuseC6 { stack } => stack,
            _ => 0.0
        };
        Box::new(BuffChevreuseC6 {
            stack
        })
    }
}
