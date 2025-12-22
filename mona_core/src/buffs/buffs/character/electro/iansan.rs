use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::character::characters::electro::iansan::IANSAN_SKILL;

pub struct BuffIansanQ {
    pub nightsoul_point: usize,
    pub atk: f64,
    pub level_q: usize,
}

impl<A: Attribute> Buff<A> for BuffIansanQ {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ATKFixed, "伊安珊「力的三原理」", if self.nightsoul_point >= 42 {
            (self.atk * 0.27).min(IANSAN_SKILL.max_atk_bonus[self.level_q - 1])
        } else {
            (self.atk * (self.nightsoul_point as f64 * 0.005)).min(IANSAN_SKILL.max_atk_bonus[self.level_q - 1])
        });
    }
}

impl BuffMeta for BuffIansanQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::IansanQ,
        name_locale: locale!(
            zh_cn: "伊安珊-「力的三原理」",
            en: "Iansan-The Three Principles of Power"
        ),
        image: BuffImage::Avatar(CharacterName::Iansan),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "伊安珊Q技能：动能标示将跟随角色行动，并根据伊安珊的夜魂值，以不同的方式使队伍中自己的当前场上角色的攻击力提升：\
                <br>· 若伊安珊的夜魂值少于42点，将基于伊安珊的夜魂值与攻击力，获得攻击力加成；\
                <br>· 若伊安珊拥有至少42点夜魂值，动能标示将切换至「炽烈声援！」模式，基于伊安珊的攻击力，获得更高的攻击力加成。",
            en: "Iansan Elemental Burst: The Kinetic Energy Scale will follow the character around, boosting the ATK of your current active party member in different ways based on Iansan's Nightsoul points.\
                <br>· If Iansan has less than 42 Nightsoul points, the ATK bonus will be based on her Nightsoul points and ATK.\
                <br>· If Iansan has at least 42 Nightsoul points, the Kinetic Energy Scale will switch to the \"Ardent Support!\" state, in which the ATK bonus will be stronger and based on her ATK alone."
        )),
        from: BuffFrom::Character(CharacterName::Iansan),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "nightsoul_point",
            title: locale!(
                zh_cn: "夜魂值",
                en: "Nightsoul points"
            ),
            config: ItemConfigType::Int { min: 0, max: 54, default: 0 }
        },
        ItemConfig {
            name: "atk",
            title: locale!(
                zh_cn: "攻击力",
                en: "ATK"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
        ItemConfig {
            name: "level_q",
            title: locale!(
                zh_cn: "技能等级",
                en: "Skill Level"
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (nightsoul_point, atk, level_q) = match *b {
            BuffConfig::IansanQ { nightsoul_point, atk, level_q } => (nightsoul_point, atk, level_q),
            _ => (0, 0.0, 10)
        };
        Box::new(BuffIansanQ {
            nightsoul_point,
            atk,
            level_q,
        })
    }
}

pub struct BuffIansanC2 {
}

impl<A: Attribute> Buff<A> for BuffIansanC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.add_atk_percentage("伊安珊「偷懒是健身大忌!」", 0.3);
    }
}

impl BuffMeta for BuffIansanC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::IansanC2,
        name_locale: locale!(
            zh_cn: "伊安珊-「偷懒是健身大忌!」",
            en: "Iansan-Laziness is the Enemy!"
        ),
        image: BuffImage::Avatar(CharacterName::Iansan),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "伊安珊命座2：「标准动作」效果持续期间，若伊安珊处于队伍后台，还会使队伍中自己的当前场上角色的攻击力提升30%。该效果需要解锁突破天赋「强化抗阻练习」。",
            en: "Iansan C2: If Iansan is off-field while Precise Movement is active, she will also increase your current active character's ATK by 30%. You must first unlock the Ascension Talent \"Enhanced Resistance Training\" to access the above effect."
        )),
        from: BuffFrom::Character(CharacterName::Iansan),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffIansanC2 {
        })
    }
}


pub struct BuffIansanC6 {
}

impl<A: Attribute> Buff<A> for BuffIansanC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusBase, "伊安珊「极限发力」", 0.25);
    }
}

impl BuffMeta for BuffIansanC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::IansanC6,
        name_locale: locale!(
            zh_cn: "伊安珊-「『沃陆之邦』的训教」",
            en: "Iansan-Teachings of the Collective of Plenty"
        ),
        image: BuffImage::Avatar(CharacterName::Iansan),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "伊安珊命座6：伊安珊触发恢复夜魂值的效果时，若夜魂值恢复量溢出，将获得「极限发力」效果，使队伍中自己的当前场上角色造成的伤害提升25%，持续3秒。",
            en: "Iansan C6: When Iansan triggers Nightsoul point restoration, if there is any overflow, she will gain the Extreme Force effect, which increases the DMG dealt by your current active character by 25% for 3s."
        )),
        from: BuffFrom::Character(CharacterName::Iansan),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffIansanC6 {
        })
    }
}
