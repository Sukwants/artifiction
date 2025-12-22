use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::character::characters::hydro::furina::FURINA_SKILL;

pub struct BuffFurinaQ {
    pub level_q: usize,
    pub fanfare: usize,
    pub rate: f64
}

impl<A: Attribute> Buff<A> for BuffFurinaQ {
    fn change_attribute(&self, attribute: &mut A) {

        attribute.set_value_by(AttributeName::BonusBase, "芙宁娜「万众狂欢」",
            FURINA_SKILL.q_bonus1[self.level_q - 1] * self.fanfare as f64 * self.rate);
    }
}

impl BuffMeta for BuffFurinaQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::FurinaQ,
        name_locale: locale!(
            zh_cn: "芙宁娜-「万众狂欢」",
            en: "Furina-Let the People Rejoice"
        ),
        image: BuffImage::Avatar(CharacterName::Furina),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "芙宁娜元素爆发。凝聚狂欢之意，构筑水沫的舞台，基于芙宁娜的生命值上限，造成水元素范围伤害，并使队伍中附近的角色进入「普世欢腾」状态：持续期间，角色的当前生命值提升或降低时，基于提升或降低数值相对于生命值上限的比例，每1%都将使芙宁娜获得1点「气氛值」。同时，基于芙宁娜持有的「气氛值」，附近的队伍中所有角色造成的伤害提升，受治疗加成提升。持续时间结束时，芙宁娜持有的「气氛值」将被移除。",
            en: "Rouses the impulse to revel, creating a stage of foam that will deal AoE Hydro DMG based on Furina's Max HP and cause nearby party members to enter the Universal Revelry state: During this time, when nearby party members' HP increases or decreases, 1 Fanfare point will be granted to Furina for each percentage point of their Max HP by which their HP changes. At the same time, Furina will increase the DMG dealt by and Incoming Healing Bonus of all nearby party members based on the amount of Fanfare she has. When the duration ends, Furina's Fanfare points will be cleared."
        )),
        from: BuffFrom::Character(CharacterName::Furina),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "level_q",
            title: locale!(
                zh_cn: "技能等级",
                en: "Skill Level"
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        },
        ItemConfig {
            name: "fanfare",
            title: locale!(
                zh_cn: "「气氛值」",
                en: "Fanfare Point"
            ),
            config: ItemConfigType::IntInput { min: 0, max: 400, default: 0 }
        },
        ItemConfig {
            name: "rate",
            title: locale!(
                zh_cn: "比例",
                en: "Rate"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (level_q, fanfare, rate) = match *b {
            BuffConfig::FurinaQ { level_q, fanfare, rate } => (level_q, fanfare, rate),
            _ => (1, 0, 0.0)
        };
        Box::new(BuffFurinaQ {
            level_q, fanfare, rate
        })
    }
}