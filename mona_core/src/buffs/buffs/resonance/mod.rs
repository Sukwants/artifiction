use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::team_status::CharacterSelector;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;
use std::sync::Arc;

pub struct BuffResonancePyro2 {
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffResonancePyro2 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.global {
            attribute.add_edge_s1ton(
                CharacterSelector::select_all(attribute),
                AttributeType::Panel(AttributeName::ATKBase),
                AttributeType::Panel(AttributeName::ATKPercentage),
                Arc::new(|x, _| x * 0.25),
                "元素共鸣-热诚之火",
                EdgePriority::Base,
            );
        } else {
            attribute.add_atk_percentage("元素共鸣-热诚之火", 0.25);
        }
    }
}

impl BuffMeta for BuffResonancePyro2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonancePyro2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素共鸣-热诚之火",
            en: "Resonance-Fervent Flames",
        ),
        image: BuffImage::Misc("pyro2"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "冰元素附着的持续时间下降40%。攻击力提高25%。",
            en: "Affected by Cryo for 40% less time. Increases ATK by 25%.",
        )),
        from: BuffFrom::Resonance,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "global",
            title: crate::common::i18n::locale!(
                zh_cn: "全局生效",
                en: "Global",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let global = match *b {
            BuffConfig::ResonancePyro2 { global } => global,
            _ => false
        };
        Box::new(BuffResonancePyro2 { global })
    }
}


pub struct BuffResonanceCryo2 {
    pub rate: f64,
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffResonanceCryo2 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = self.rate * 0.15;
        let ty = AttributeType::Invisible(InvisibleAttributeType::new_any(AttributeVariableType::CriticalRate));
        if self.global {
            attribute.set_value_to_s(CharacterSelector::select_all(attribute), ty, "元素共鸣-粉碎之冰", value);
        } else {
            attribute.set_value_to_t(ty, "元素共鸣-粉碎之冰", value);
        }
    }
}

impl BuffMeta for BuffResonanceCryo2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonanceCryo2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素共鸣-粉碎之冰",
            en: "Resonance-Shattering Ice",
        ),
        image: BuffImage::Misc("cryo2"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "雷元素附着持续时间下降40%。攻击冰元素附着或冻结状态下的敌人时，暴击率提高15%。",
            en: "Affected by Electro for 40% less time. Increases CRIT Rate against opponents that are Frozen or affected by Cryo by 15%.",
        )),
        from: BuffFrom::Resonance,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: crate::common::i18n::locale!(
                zh_cn: "应用比例",
                en: "Apply Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "global",
            title: crate::common::i18n::locale!(
                zh_cn: "全局生效",
                en: "Global",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (rate, global) = match *b {
            BuffConfig::ResonanceCryo2 { rate, global } => (rate, global),
            _ => (0.0, false)
        };

        Box::new(BuffResonanceCryo2 {
            rate, global
        })
    }
}


pub struct BuffResonanceGeo2 {
    pub rate1: f64,
    pub rate2: f64,
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffResonanceGeo2 {
    fn change_attribute(&self, attribute: &mut A) {
        let ty_bonus = AttributeType::Invisible(InvisibleAttributeType::new_any(AttributeVariableType::Bonus));
        let ty_res_minus = AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::ResMinus, crate::common::Element::Geo));

        if self.global {
            attribute.set_value_to_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::ShieldStrength), "元素共鸣-坚定之岩", 0.15);
            attribute.set_value_to_s(CharacterSelector::select_all(attribute), ty_bonus, "元素共鸣-坚定之岩", self.rate1 * 0.15);
            attribute.set_value_to_s(CharacterSelector::select_all(attribute), ty_res_minus, "元素共鸣-坚定之岩", self.rate2 * 0.2);
        } else {
            attribute.set_value_to_t(AttributeType::Panel(AttributeName::ShieldStrength), "元素共鸣-坚定之岩", 0.15);
            attribute.set_value_to_t(ty_bonus, "元素共鸣-坚定之岩", self.rate1 * 0.15);
            attribute.set_value_to_t(ty_res_minus, "元素共鸣-坚定之岩", self.rate2 * 0.2);
        }
    }
}

impl BuffMeta for BuffResonanceGeo2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonanceGeo2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素共鸣-坚定之岩",
            en: "Resonance-Enduring Rock",
        ),
        image: BuffImage::Misc("geo2"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "护盾强效提升15%。此外，角色处于护盾庇护下时，或附近存在月结晶反应产生的月笼时，具有如下特性：造成的伤害提升15%；角色对敌人造成伤害时，会使敌人的岩元素抗性降低20%，持续15秒。",
            en: "Increases shield strength by 15%. Additionally, when characters protected by a shield, or when Moondrifts formed by Lunar-Crystallize reactions are nearby, the following special characteristics will take effect: DMG dealt increased by 15%, dealing DMG to enemies will decrease their Geo RES by 20% for 15s.",
        )),
        from: BuffFrom::Common,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: crate::common::i18n::locale!(
                zh_cn: "效果①比例",
                en: "Effect① Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "rate2",
            title: crate::common::i18n::locale!(
                zh_cn: "效果②比例",
                en: "Effect② Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "global",
            title: crate::common::i18n::locale!(
                zh_cn: "全局生效",
                en: "Global",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (rate1, rate2, global) = match *b {
            BuffConfig::ResonanceGeo2 { rate1, rate2, global } => (rate1, rate2, global),
            _ => (0.0, 0.0, false)
        };

        Box::new(BuffResonanceGeo2 {
            rate1, rate2, global
        })
    }
}


pub struct BuffResonanceHydro2 {
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffResonanceHydro2 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.global {
            attribute.add_edge_s1ton(
                CharacterSelector::select_all(attribute),
                AttributeType::Panel(AttributeName::HPBase),
                AttributeType::Panel(AttributeName::HPPercentage),
                Arc::new(|x, _| x * 0.25),
                "元素共鸣-愈疗之水",
                EdgePriority::Base,
            );
        } else {
            attribute.add_hp_percentage("元素共鸣-愈疗之水", 0.25);
        }
    }
}

impl BuffMeta for BuffResonanceHydro2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonanceHydro2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素共鸣-愈疗之水",
            en: "Resonance-Soothing Water",
        ),
        image: BuffImage::Misc("hydro2"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "火元素附着的持续时间下降40%。生命值上限提升25%",
            en: "Affected by Pyro for 40% less time. Increases Max HP by 25%.",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "global",
            title: crate::common::i18n::locale!(
                zh_cn: "全局生效",
                en: "Global",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let global = match *b {
            BuffConfig::ResonanceHydro2 { global } => global,
            _ => false
        };
        Box::new(BuffResonanceHydro2 { global })
    }
}

pub struct BuffResonanceDendro2 {
    pub rate1: f64,
    pub rate2: f64,
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffResonanceDendro2 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = 50.0 + self.rate1 * 30.0 + self.rate2 * 20.0;
        if self.global {
            attribute.set_value_to_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::ElementalMastery), "元素共鸣-蔓生之草", value);
        } else {
            attribute.set_value_to_t(AttributeType::Panel(AttributeName::ElementalMastery), "元素共鸣-蔓生之草", value);
        }
    }
}

impl BuffMeta for BuffResonanceDendro2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ResonanceDendro2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "元素共鸣-蔓生之草",
            en: "Resonance-Sprawling Greenery",
        ),
        image: BuffImage::Misc("dendro2"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "元素精通提升50点。触发燃烧、原激化、绽放、月绽放反应后，队伍中附近的所有角色元素精通提升30点，持续6秒。触发超激化、蔓激化、超绽放、烈绽放反应后，队伍中附近的所有角色元素精通提升20点，持续6秒。以上效果的持续时间独立计算。",
            en: "Elemental Mastery increased by 50. After triggering Burning, Quicken, Bloom, or Lunar-Bloom reactions, all nearby party members gain 30 Elemental Mastery for 6s. After triggering Aggravate, Spread, Hyperbloom, or Burgeon reactions, all nearby party members gain 20 Elemental Mastery for 6s. The durations of the aforementioned effects will be counted independently.",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: crate::common::i18n::locale!(
                zh_cn: "效果①比例",
                en: "Effect① Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "rate2",
            title: crate::common::i18n::locale!(
                zh_cn: "效果②比例",
                en: "Effect② Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "global",
            title: crate::common::i18n::locale!(
                zh_cn: "全局生效",
                en: "Global",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (rate1, rate2, global) = match *b {
            BuffConfig::ResonanceDendro2 { rate1, rate2, global } => (rate1, rate2, global),
            _ => (0.0, 0.0, false)
        };

        Box::new(BuffResonanceDendro2 {
            rate1, rate2, global
        })
    }
}

pub mod moonsign;
pub use moonsign::BuffMoonsignPyro;
pub use moonsign::BuffMoonsignHydro;
pub use moonsign::BuffMoonsignAnemo;
pub use moonsign::BuffMoonsignElectro;
pub use moonsign::BuffMoonsignDendro;
pub use moonsign::BuffMoonsignCryo;
pub use moonsign::BuffMoonsignGeo;

mod fantastical_blessings;
pub use fantastical_blessings::BuffFantasticalBlessings;