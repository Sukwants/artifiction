use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffMoonsignPyro {
    pub atk: f64
}

impl<A: Attribute> Buff<A> for BuffMoonsignPyro {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceMoonglare, "月荫-火", (self.atk / 100.0 * 0.009).min(0.36));
    }
}

impl BuffMeta for BuffMoonsignPyro {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MoonsignPyro,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "月荫-火",
            en: "Moonsign-Benediction Pyro",
        ),
        image: BuffImage::Misc("moonsign_pyro"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "非月兆角色在释放元素战技或元素爆发时，能基于自身的属性，使附近所有角色提升至多 36% 的月曜反应伤害<br>火：每100点攻击力提升 0.9%",
            en: "When non-Moonsign characters use Elemental Skills or Elemental Bursts, they can increase Lunar Reaction DMG by up to 36% for all nearby characters based on their own attributes.<br>Pyro: DMG increases by 0.9% per 100 ATK.",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "atk",
            title: crate::common::i18n::locale!(
                zh_cn: "攻击力",
                en: "ATK",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let atk = match *b {
            BuffConfig::MoonsignPyro { atk } => atk,
            _ => 0.0
        };

        Box::new(BuffMoonsignPyro {
            atk
        })
    }
}

pub struct BuffMoonsignHydro {
    pub hp: f64
}

impl<A: Attribute> Buff<A> for BuffMoonsignHydro {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceMoonglare, "月荫-水", (self.hp / 1000.0 * 0.006).min(0.36));
    }
}

impl BuffMeta for BuffMoonsignHydro {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MoonsignHydro,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "月荫-水",
            en: "Moonsign-Benediction Hydro",
        ),
        image: BuffImage::Misc("moonsign_hydro"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "非月兆角色在释放元素战技或元素爆发时，能基于自身的属性，使附近所有角色提升至多 36% 的月曜反应伤害<br>水：每1000点最大生命值提升 0.6%",
            en: "When non-Moonsign characters use Elemental Skills or Elemental Bursts, they can increase Lunar Reaction DMG by up to 36% for all nearby characters based on their own attributes.<br>Hydro: DMG increases by 0.6% per 1000 HP.",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "hp",
            title: crate::common::i18n::locale!(
                zh_cn: "最大生命值",
                en: "HP",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let hp = match *b {
            BuffConfig::MoonsignHydro { hp } => hp,
            _ => 0.0
        };

        Box::new(BuffMoonsignHydro {
            hp
        })
    }
}

pub struct BuffMoonsignAnemo {
    pub em: f64
}

impl<A: Attribute> Buff<A> for BuffMoonsignAnemo {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceMoonglare, "月荫-风", (self.em / 100.0 * 0.0225).min(0.36));
    }
}

impl BuffMeta for BuffMoonsignAnemo {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MoonsignAnemo,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "月荫-风",
            en: "Moonsign-Benediction Anemo",
        ),
        image: BuffImage::Misc("moonsign_anemo"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "非月兆角色在释放元素战技或元素爆发时，能基于自身的属性，使附近所有角色提升至多 36% 的月曜反应伤害<br>风：每100点元素精通提升 2.25%",
            en: "When non-Moonsign characters use Elemental Skills or Elemental Bursts, they can increase Lunar Reaction DMG by up to 36% for all nearby characters based on their own attributes.<br>Anemo: DMG increases by 2.25% per 100 EM.",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: crate::common::i18n::locale!(
                zh_cn: "元素精通",
                en: "Elemental Mastery",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::MoonsignAnemo { em } => em,
            _ => 0.0
        };

        Box::new(BuffMoonsignAnemo {
            em
        })
    }
}

pub struct BuffMoonsignElectro {
    pub atk: f64
}

impl<A: Attribute> Buff<A> for BuffMoonsignElectro {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceMoonglare, "月荫-雷", (self.atk / 100.0 * 0.009).min(0.36));
    }
}

impl BuffMeta for BuffMoonsignElectro {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MoonsignElectro,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "月荫-雷",
            en: "Moonsign-Benediction Electro",
        ),
        image: BuffImage::Misc("moonsign_electro"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "非月兆角色在释放元素战技或元素爆发时，能基于自身的属性，使附近所有角色提升至多 36% 的月曜反应伤害<br>雷：每100点攻击力提升 0.9%",
            en: "When non-Moonsign characters use Elemental Skills or Elemental Bursts, they can increase Lunar Reaction DMG by up to 36% for all nearby characters based on their own attributes.<br>Electro: DMG increases by 0.9% per 100 ATK.",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "atk",
            title: crate::common::i18n::locale!(
                zh_cn: "攻击力",
                en: "ATK",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let atk = match *b {
            BuffConfig::MoonsignElectro { atk } => atk,
            _ => 0.0
        };

        Box::new(BuffMoonsignElectro {
            atk
        })
    }
}

pub struct BuffMoonsignDendro {
    pub em: f64
}

impl<A: Attribute> Buff<A> for BuffMoonsignDendro {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceMoonglare, "月荫-草", (self.em / 100.0 * 0.0225).min(0.36));
    }
}

impl BuffMeta for BuffMoonsignDendro {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MoonsignDendro,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "月荫-草",
            en: "Moonsign-Benediction Dendro",
        ),
        image: BuffImage::Misc("moonsign_dendro"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "非月兆角色在释放元素战技或元素爆发时，能基于自身的属性，使附近所有角色提升至多 36% 的月曜反应伤害<br>草：每100点元素精通提升 2.25%",
            en: "When non-Moonsign characters use Elemental Skills or Elemental Bursts, they can increase Lunar Reaction DMG by up to 36% for all nearby characters based on their own attributes.<br>Dendro: DMG increases by 2.25% per 100 EM.",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: crate::common::i18n::locale!(
                zh_cn: "元素精通",
                en: "Elemental Mastery",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let em = match *b {
            BuffConfig::MoonsignDendro { em } => em,
            _ => 0.0
        };

        Box::new(BuffMoonsignDendro {
            em
        })
    }
}

pub struct BuffMoonsignCryo {
    pub atk: f64
}

impl<A: Attribute> Buff<A> for BuffMoonsignCryo {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceMoonglare, "月荫-冰", (self.atk / 100.0 * 0.009).min(0.36));
    }
}

impl BuffMeta for BuffMoonsignCryo {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MoonsignCryo,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "月荫-冰",
            en: "Moonsign-Benediction Cryo",
        ),
        image: BuffImage::Misc("moonsign_cryo"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "非月兆角色在释放元素战技或元素爆发时，能基于自身的属性，使附近所有角色提升至多 36% 的月曜反应伤害<br>冰：每100点攻击力提升 0.9%",
            en: "When non-Moonsign characters use Elemental Skills or Elemental Bursts, they can increase Lunar Reaction DMG by up to 36% for all nearby characters based on their own attributes.<br>Cryo: DMG increases by 0.9% per 100 ATK.",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "atk",
            title: crate::common::i18n::locale!(
                zh_cn: "攻击力",
                en: "ATK",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let atk = match *b {
            BuffConfig::MoonsignCryo { atk } => atk,
            _ => 0.0
        };

        Box::new(BuffMoonsignCryo {
            atk
        })
    }
}

pub struct BuffMoonsignGeo {
    pub def: f64
}

impl<A: Attribute> Buff<A> for BuffMoonsignGeo {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceMoonglare, "月荫-岩", (self.def / 100.0 * 0.01).min(0.36));
    }
}

impl BuffMeta for BuffMoonsignGeo {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::MoonsignGeo,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "月荫-岩",
            en: "Moonsign-Benediction Geo",
        ),
        image: BuffImage::Misc("moonsign_geo"),
        genre: BuffGenre::Resonance,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "非月兆角色在释放元素战技或元素爆发时，能基于自身的属性，使附近所有角色提升至多 36% 的月曜反应伤害<br>岩：每100点防御力提升 1%",
            en: "When non-Moonsign characters use Elemental Skills or Elemental Bursts, they can increase Lunar Reaction DMG by up to 36% for all nearby characters based on their own attributes.<br>Geo: DMG increases by 1% per 100 DEF.",
        )),
        from: BuffFrom::Resonance
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "def",
            title: crate::common::i18n::locale!(
                zh_cn: "防御力",
                en: "DEF",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let def = match *b {
            BuffConfig::MoonsignGeo { def } => def,
            _ => 0.0
        };

        Box::new(BuffMoonsignGeo {
            def
        })
    }
}