use crate::artifacts::ArtifactSetName;
use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffSongOfDaysPast4 {
    pub heal_amount: f64,
}

impl<A: Attribute> Buff<A> for BuffSongOfDaysPast4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ExtraDmgBase, "BUFF: 昔时之歌4", self.heal_amount * 0.08);
    }
}

impl BuffMeta for BuffSongOfDaysPast4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SongOfDaysPast4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "昔时之歌4",
            en: "Song Of Days Past 4",
        ),
        image: BuffImage::Artifact(ArtifactSetName::SongOfDaysPast),
        genre: BuffGenre::Artifact,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "装备者对队伍中的角色进行治疗时，将产生持续6秒的渴盼效果，记录治疗的生命值回复量（包括溢出值）。持续时间结束时，渴盼效果将转变为「彼时的浪潮」效果：队伍中自己的当前场上角色的普通攻击、重击、下落攻击、元素战技与元素爆发命中敌人时，将基于渴盼效果所记录的回复量的8%提高造成的伤害，「彼时的浪潮」将在生效5次或10秒后移除。一次渴盼效果至多记录15000点回复量，同时至多存在一个，能够记录多个装备者的产生的回复量；装备者处于队伍后台时，依然能触发该效果。",
            en: "When the equipping character heals a party member, the Yearning effect will be created for 6s, which records the total amount of healing provided (including overflow healing). When the duration expires, the Yearning effect will be transformed into the \"Waves of Days Past\" effect: When your active party member hits an opponent with a Normal Attack, Charged Attack, Plunging Attack, Elemental Skill, or Elemental Burst, the DMG dealt will be increased by 8% of the total healing amount recorded by the Yearning effect. The \"Waves of Days Past\" effect is removed after it has taken effect 5 times or after 10s. A single instance of the Yearning effect can record up to 15,000 healing, and only a single instance can exist at once, but it can record the healing from multiple equipping characters. Equipping characters on standby can still trigger this effect."
        )),
        from: BuffFrom::Artifact(ArtifactSetName::SongOfDaysPast)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "heal_amount",
            title: crate::common::i18n::locale!(
                zh_cn: "记录治疗量",
                en: "Record Healing Amount",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 15000.0, default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let heal_amount = match *b {
            BuffConfig::SongOfDaysPast4 { heal_amount } => heal_amount,
            _ => 0.0
        };
        Box::new(BuffSongOfDaysPast4 { heal_amount })
    }
}