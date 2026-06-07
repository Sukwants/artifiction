use crate::buffs::buffs::prelude::*;

pub struct BuffMoonsignPyro {
    pub atk: f64,
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffMoonsignPyro {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (self.atk / 100.0 * 0.009).min(0.36);
        for reaction in ReactionType::get_moonglare_reaction() {
            let ty = AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, reaction));
            if self.global {
                attribute.set_value_to_s(CharacterSelector::select_all(attribute), ty, "月荫-火", value);
            } else {
                attribute.set_value_to_t(ty, "月荫-火", value);
            }
        }
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
        let (atk, global) = match *b {
            BuffConfig::MoonsignPyro { atk, global } => (atk, global),
            _ => (0.0, false)
        };

        Box::new(BuffMoonsignPyro {
            atk, global
        })
    }
}

pub struct BuffMoonsignHydro {
    pub hp: f64,
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffMoonsignHydro {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (self.hp / 1000.0 * 0.006).min(0.36);
        for reaction in ReactionType::get_moonglare_reaction() {
            let ty = AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, reaction));
            if self.global {
                attribute.set_value_to_s(CharacterSelector::select_all(attribute), ty, "月荫-水", value);
            } else {
                attribute.set_value_to_t(ty, "月荫-水", value);
            }
        }
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
        let (hp, global) = match *b {
            BuffConfig::MoonsignHydro { hp, global } => (hp, global),
            _ => (0.0, false)
        };

        Box::new(BuffMoonsignHydro {
            hp, global
        })
    }
}

pub struct BuffMoonsignAnemo {
    pub em: f64,
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffMoonsignAnemo {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (self.em / 100.0 * 0.0225).min(0.36);
        for reaction in ReactionType::get_moonglare_reaction() {
            let ty = AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, reaction));
            if self.global {
                attribute.set_value_to_s(CharacterSelector::select_all(attribute), ty, "月荫-风", value);
            } else {
                attribute.set_value_to_t(ty, "月荫-风", value);
            }
        }
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
        let (em, global) = match *b {
            BuffConfig::MoonsignAnemo { em, global } => (em, global),
            _ => (0.0, false)
        };

        Box::new(BuffMoonsignAnemo {
            em, global
        })
    }
}

pub struct BuffMoonsignElectro {
    pub atk: f64,
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffMoonsignElectro {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (self.atk / 100.0 * 0.009).min(0.36);
        for reaction in ReactionType::get_moonglare_reaction() {
            let ty = AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, reaction));
            if self.global {
                attribute.set_value_to_s(CharacterSelector::select_all(attribute), ty, "月荫-雷", value);
            } else {
                attribute.set_value_to_t(ty, "月荫-雷", value);
            }
        }
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
        let (atk, global) = match *b {
            BuffConfig::MoonsignElectro { atk, global } => (atk, global),
            _ => (0.0, false)
        };

        Box::new(BuffMoonsignElectro {
            atk, global
        })
    }
}

pub struct BuffMoonsignDendro {
    pub em: f64,
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffMoonsignDendro {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (self.em / 100.0 * 0.0225).min(0.36);
        for reaction in ReactionType::get_moonglare_reaction() {
            let ty = AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, reaction));
            if self.global {
                attribute.set_value_to_s(CharacterSelector::select_all(attribute), ty, "月荫-草", value);
            } else {
                attribute.set_value_to_t(ty, "月荫-草", value);
            }
        }
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
        let (em, global) = match *b {
            BuffConfig::MoonsignDendro { em, global } => (em, global),
            _ => (0.0, false)
        };

        Box::new(BuffMoonsignDendro {
            em, global
        })
    }
}

pub struct BuffMoonsignCryo {
    pub atk: f64,
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffMoonsignCryo {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (self.atk / 100.0 * 0.009).min(0.36);
        for reaction in ReactionType::get_moonglare_reaction() {
            let ty = AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, reaction));
            if self.global {
                attribute.set_value_to_s(CharacterSelector::select_all(attribute), ty, "月荫-冰", value);
            } else {
                attribute.set_value_to_t(ty, "月荫-冰", value);
            }
        }
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
        let (atk, global) = match *b {
            BuffConfig::MoonsignCryo { atk, global } => (atk, global),
            _ => (0.0, false)
        };

        Box::new(BuffMoonsignCryo {
            atk, global
        })
    }
}

pub struct BuffMoonsignGeo {
    pub def: f64,
    pub global: bool,
}

impl<A: Attribute> Buff<A> for BuffMoonsignGeo {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (self.def / 100.0 * 0.01).min(0.36);
        for reaction in ReactionType::get_moonglare_reaction() {
            let ty = AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, reaction));
            if self.global {
                attribute.set_value_to_s(CharacterSelector::select_all(attribute), ty, "月荫-岩", value);
            } else {
                attribute.set_value_to_t(ty, "月荫-岩", value);
            }
        }
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
        let (def, global) = match *b {
            BuffConfig::MoonsignGeo { def, global } => (def, global),
            _ => (0.0, false)
        };

        Box::new(BuffMoonsignGeo {
            def, global
        })
    }
}