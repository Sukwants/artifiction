use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffStarcallersWatch {
    pub refine: usize,
}

impl<A: Attribute> Buff<A> for BuffStarcallersWatch {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusBase ,"BUFF: 祭星者之望-「照夜之镜」", self.refine as f64 * 0.07 + 0.21);
    }
}

impl BuffMeta for BuffStarcallersWatch {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::StarcallersWatch,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "祭星者之望-「照夜之镜」",
            en: "Starcallers Watch-Mirror of Night",
        ),
        image: BuffImage::Weapon(WeaponName::StarcallersWatch),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "装备者创造护盾后的15秒内，获得「照夜之镜」效果：队伍中自己的当前场上角色对附近的敌人造成的伤害提升<span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span>，每14秒至多获得一次「照夜之镜」效果。",
            en: "Gain the \"Mirror of Night\" effect within 15s after the equipping character creates a shield: The current active party member deals <span style=\"color: #409EFF;\">28%-35%-42%-49%-56%</span> increased DMG to nearby opponents. You can gain the \"Mirror of Night\" effect once every 14s.",
        )),
        from: BuffFrom::Weapon(WeaponName::StarcallersWatch),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let refine = match *b {
            BuffConfig::StarcallersWatch { refine } => refine,
            _ => 1
        };

        Box::new(BuffStarcallersWatch {
            refine
        })
    }
}

