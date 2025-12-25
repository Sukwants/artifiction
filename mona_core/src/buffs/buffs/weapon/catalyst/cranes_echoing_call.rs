use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffCranesEchoingCall {
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffCranesEchoingCall {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusPlungingAttack ,"BUFF: 鹤鸣余音-「云笈降真要诀」", self.refine as f64 * 0.13 + 0.15);
    }
}

impl BuffMeta for BuffCranesEchoingCall {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::CranesEchoingCall,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "鹤鸣余音-「云笈降真要诀」",
            en: "Crane's Echoing Call-Cloudfall Axiom",
        ),
        image: BuffImage::Weapon(WeaponName::CranesEchoingCall),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "装备者下落攻击命中敌人后，队伍中附近的所有角色下落攻击造成的伤害提高<span style=\"color: #409EFF;\">28%-41%-54%-67%-80%</span>，持续20秒。",
            en: "After the equipping character hits an opponent with a Plunging Attack, all nearby party members' Plunging Attacks deal <span style=\"color: #409EFF;\">28%-41%-54%-67%-80%</span> increased DMG for 20s.",
        )),
        from: BuffFrom::Weapon(WeaponName::CranesEchoingCall),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::CranesEchoingCall { refine } => refine,
            _ => 1
        };

        Box::new(BuffCranesEchoingCall {
            refine
        })
    }
}

