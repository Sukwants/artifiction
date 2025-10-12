use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffEscoffierTalent2 {
    pub hydro_cryo_count: usize,
}

impl<A: Attribute> Buff<A> for BuffEscoffierTalent2 {
    fn change_attribute(&self, attribute: &mut A) {

        attribute.set_value_by(AttributeName::ResMinusHydro, "爱可菲「灵感浸入调味」", [0.05, 0.10, 0.15, 0.55][self.hydro_cryo_count - 1]);
        attribute.set_value_by(AttributeName::ResMinusCryo, "爱可菲「灵感浸入调味」", [0.05, 0.10, 0.15, 0.55][self.hydro_cryo_count - 1]);
    }
}

impl BuffMeta for BuffEscoffierTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::EscoffierTalent2,
        name_locale: locale!(
            zh_cn: "爱可菲-「灵感浸入调味」",
            en: "Escoffier-Inspiration-Immersed Seasoning"
        ),
        image: BuffImage::Avatar(CharacterName::Escoffier),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "当队伍中存在1/2/3/4名水元素或冰元素角色时，爱可菲的元素战技低温烹饪或元素爆发花刀技法命中敌人时，将使该敌人的水元素抗性与冰元素抗性降低5%/10%/15%/55%，持续12秒。",
            en: "When there are 1/2/3/4 Hydro or Cryo characters in the party, Escoffier will decrease the Hydro RES and Cryo RES of any opponents hit by her Elemental Skill, Low-Temperature Cooking, or her Elemental Burst, Scoring Cuts, by 5%/10%/15%/55% for 12s."
        )),
        from: BuffFrom::Character(CharacterName::Escoffier),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hydro_cryo_count",
            title: locale!(
                zh_cn: "冰水角色数量",
                en: "Hydro or Cryo Characters Count"
            ),
            config: ItemConfigType::Int { min: 1, max: 4, default: 4 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let hydro_cryo_count = match *b {
            BuffConfig::EscoffierTalent2 { hydro_cryo_count } => hydro_cryo_count,
            _ => 4
        };
        Box::new(BuffEscoffierTalent2 {
            hydro_cryo_count,
        })
    }
}
pub struct BuffEscoffierC1 {}

impl<A: Attribute> Buff<A> for BuffEscoffierC1 {
    fn change_attribute(&self, attribute: &mut A) {

        attribute.set_value_by(AttributeName::CriticalDamageCryo, "爱可菲「味蕾绽放的餐前旋舞」", 0.6);
    }
}

impl BuffMeta for BuffEscoffierC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::EscoffierC1,
        name_locale: locale!(
            zh_cn: "爱可菲-「味蕾绽放的餐前旋舞」",
            en: "Escoffier-Pre-Dinner Dance for Your Taste Buds"
        ),
        image: BuffImage::Avatar(CharacterName::Escoffier),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "队伍中4名角色的元素类型均为水元素或冰元素时，爱可菲施放元素战技低温烹饪或元素爆发花刀技法后的15秒内，队伍中附近的所有角色造成冰元素伤害时的暴击伤害提升60%。",
            en: "When 4 party members are Hydro or Cryo, all nearby party members will have their Cryo DMG CRIT DMG increased by 60% for 15s after Escoffier uses her Elemental Skill Low-Temperature Cooking or Elemental Burst Scoring Cuts."
        )),
        from: BuffFrom::Character(CharacterName::Escoffier),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffEscoffierC1 {})
    }
}
pub struct BuffEscoffierC2 {
    atk: f64,
}

impl<A: Attribute> Buff<A> for BuffEscoffierC2 {
    fn change_attribute(&self, attribute: &mut A) {

        attribute.set_value_by(AttributeName::ExtraDmgCryo, "爱可菲「鲜香味腴的炖煮艺术」", self.atk * 2.4);
    }
}

impl BuffMeta for BuffEscoffierC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::EscoffierC2,
        name_locale: locale!(
            zh_cn: "爱可菲-「鲜香味腴的炖煮艺术」",
            en: "Escoffier-Fresh, Fragrant Stew Is an Art"
        ),
        image: BuffImage::Avatar(CharacterName::Escoffier),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "爱可菲以低温冷藏模式启动厨艺机关时，将获得「现制名肴」效果，持续15秒：持续期间，爱可菲获得5层「冷煮」，除爱可菲外的附近的当前场上角色普通攻击、重击、下落攻击、元素战技和元素爆发对敌人造成冰元素伤害时，将消耗1层「冷煮」，提升造成的伤害，提升值相当于爱可菲攻击力的240%。",
            en: "When 4 party members are Hydro or Cryo, all nearby party members will have their Cryo DMG CRIT DMG increased by 60% for 15s after Escoffier uses her Elemental Skill Low-Temperature Cooking or Elemental Burst Scoring Cuts."
        )),
        from: BuffFrom::Character(CharacterName::Escoffier),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "atk",
            title: locale!(
                zh_cn: "攻击力",
                en: "Attack"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let atk = match *b {
            BuffConfig::EscoffierC2 { atk } => atk,
            _ => 0.0
        };
        Box::new(BuffEscoffierC2 {
            atk,
        })
    }
}