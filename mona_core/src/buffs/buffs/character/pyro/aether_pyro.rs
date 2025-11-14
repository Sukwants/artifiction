use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffAetherPyroC1 {
    pub nightsouls_blessing: bool,
}

impl<A: Attribute> Buff<A> for BuffAetherPyroC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusBase, "空-火「流光的星火」", if self.nightsouls_blessing { 0.15 } else { 0.06 });
    }
}

impl BuffMeta for BuffAetherPyroC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AetherPyroC1,
        name_locale: locale!(
            zh_cn: "空-火-「流光的星火」",
            en: "Aether-Pyro-Starfire's Flowing Light"
        ),
        image: BuffImage::Avatar(CharacterName::AetherPyro),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "焰烈之槛或灼火之槛存在期间，当前场上角色造成的伤害提升6%；若当前场上角色处于夜魂加持状态，造成的伤害还会提升9%。",
            en: "While Blazing Threshold or Scorching Threshold are active, the current active character deals 6% increased DMG. If said character is in the Nightsoul's Blessing state, they will deal an additional 9% DMG."
        )),
        from: BuffFrom::Character(CharacterName::AetherPyro),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "nightsouls_blessing",
            title: locale!(
                zh_cn: "夜魂加持状态",
                en: "Nightsoul's Blessing state",
            ),
            config: ItemConfigType::Bool { default: true }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let nightsouls_blessing = match *b {
            BuffConfig::AetherPyroC1 { nightsouls_blessing } => nightsouls_blessing,
            _ => false
        };
        Box::new(BuffAetherPyroC1 {
            nightsouls_blessing,
        })
    }
}

pub struct BuffAetherPyroC2 {
    pub stack: usize,
}

impl<A: Attribute> Buff<A> for BuffAetherPyroC2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ResMinusPyro, "空-火「长明的烛火」", self.stack as f64 * 0.2);
    }
}

impl BuffMeta for BuffAetherPyroC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AetherPyroC2,
        name_locale: locale!(
            zh_cn: "空-火-「长明的烛火」",
            en: "Aether-Pyro-Ever-Lit Candle"
        ),
        image: BuffImage::Avatar(CharacterName::AetherPyro),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "对抗「古斯托特」化形的蚀灭的源焰之主时，处于夜魂加持状态下时，旅行者的元素类型为火元素的攻击命中敌人后，该敌人的火元素抗性降低20%，持续6秒，此效果至多叠加2层，每层独立计算持续时间。",
            en: "When opposing the Lord of Eroded Primal Fire incarnated by Gosoythoth, while in the Nightsoul's Blessing state, after the Traveler hits an opponent with Pyro attacks, that opponent's Pyro RES is decreased by 20% for 6s. Max 2 stacks. Each stack's duration is counted independently."
        )),
        from: BuffFrom::Character(CharacterName::AetherPyro),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "效果层数",
                en: "Effect Stacks",
            ),
            config: ItemConfigType::Int { min: 0, max: 2, default: 2 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let stack = match *b {
            BuffConfig::AetherPyroC2 { stack } => stack,
            _ => 0
        };
        Box::new(BuffAetherPyroC2 {
            stack
        })
    }
}