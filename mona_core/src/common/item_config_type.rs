use std::fmt::Formatter;
use std::marker::PhantomData;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde_json::json;
use smallvec::{SmallVec};
use crate::common::{Element, Moonsign, SkillType};
use crate::common::i18n::{I18nLocale, locale};

#[derive(Default, Debug, Clone, Copy)]
pub struct ConfigElements8Multi {
    pub pyro: bool,
    pub electro: bool,
    pub dendro: bool,
    pub cryo: bool,
    pub anemo: bool,
    pub geo: bool,
    pub hydro: bool,
    pub physical: bool,
}

impl ConfigElements8Multi {
    pub fn collect_elements(&self) -> Vec<Element> {
        let mut ret = Vec::new();
        if self.pyro {
            ret.push(Element::Pyro);
        }
        if self.electro {
            ret.push(Element::Electro);
        }
        if self.dendro {
            ret.push(Element::Dendro);
        }
        if self.cryo {
            ret.push(Element::Cryo);
        }
        if self.anemo {
            ret.push(Element::Anemo);
        }
        if self.geo {
            ret.push(Element::Geo);
        }
        if self.hydro {
            ret.push(Element::Hydro);
        }
        if self.physical {
            ret.push(Element::Physical);
        }
        ret
    }
}

impl Serialize for ConfigElements8Multi {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut root = serializer.serialize_seq(None)?;
        if self.pyro {
            root.serialize_element(&Element::Pyro)?;
        }
        if self.electro {
            root.serialize_element(&Element::Electro)?;
        }
        if self.dendro {
            root.serialize_element(&Element::Dendro)?;
        }
        if self.cryo {
            root.serialize_element(&Element::Cryo)?;
        }
        if self.anemo {
            root.serialize_element(&Element::Anemo)?;
        }
        if self.geo {
            root.serialize_element(&Element::Geo)?;
        }
        if self.hydro {
            root.serialize_element(&Element::Hydro)?;
        }
        if self.physical {
            root.serialize_element(&Element::Physical)?;
        }

        root.end()
    }
}

struct ConfigElements8MultiVisitor;

impl<'de> Visitor<'de> for ConfigElements8MultiVisitor {
    type Value = ConfigElements8Multi;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("expecting an array of elements")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut ret = ConfigElements8Multi::default();
        while let Some(item) = seq.next_element::<Element>()? {
            if item == Element::Pyro {
                ret.pyro = true;
            } else if item == Element::Electro {
                ret.electro = true;
            } else if item == Element::Anemo {
                ret.anemo = true;
            } else if item == Element::Cryo {
                ret.cryo = true;
            } else if item == Element::Geo {
                ret.geo = true;
            } else if item == Element::Hydro {
                ret.hydro = true;
            } else if item == Element::Dendro {
                ret.dendro = true;
            } else if item == Element::Physical {
                ret.physical = true;
            }
        }
        Ok(ret)
    }
}



impl<'de> Deserialize<'de> for ConfigElements8Multi {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(ConfigElements8MultiVisitor)
    }
}

pub struct GlobalLinkConfig {
    pub key: &'static str,
    pub priority: usize,
    pub team_shared: bool,
}

pub enum ItemConfigType {
    Float {
        min: f64,
        max: f64,
        default: f64,
    },
    Int {
        min: i32,
        max: i32,
        default: i32
    },
    IntInput {
        min: i32,
        max: i32,
        default: i32
    },
    Bool {
        default: bool
    },
    Option {
        options: &'static str, // comma separated
        default: usize
    },
    Option2 {
        options_zh: &'static str,
        options_en: &'static str,
        default: usize
    },
    // NullOrValueInput {
    //     min: f64,
    //     max: f64,
    //     default: f64,
    // },
    FloatPercentageInput {
        default: f64,
    },
    FloatInput {
        default: f64,
    },
    Element {
        elements: &'static [Element],
        default: Element
    },
    Element4 {      // cryo, pyro, electro, hydro
        default: Element
    },
    Element8 {
        default: Element
    },
    Element8Multi {
        default: ConfigElements8Multi
    },
    Skill4 {
        default: SkillType
    },
    Moonsign2 {
        default: Moonsign
    },
    Moonsign3 {
        default: Moonsign
    },
    GlobalLinkBool {
        default: bool,
        global_link: GlobalLinkConfig,
    },
    GlobalLinkFloat {
        min: f64,
        max: f64,
        default: f64,
        global_link: GlobalLinkConfig,
    },
    GlobalLinkMoonsign3 {
        default: Moonsign,
        global_link: GlobalLinkConfig,
    }
}

pub struct ItemConfig {
    pub title: I18nLocale,
    pub name: &'static str,
    pub config: ItemConfigType,
}

impl ItemConfigType {
}

impl ItemConfig {
    pub const DEFAULT_RATE_TITLE: I18nLocale = locale!(zh_cn: "被动应用比例", en: "Avg Effect Ratio");
    pub const DEFAULT_STACK_TITLE: I18nLocale = locale!(zh_cn: "被动等效层数", en: "Avg Effect Stack");
    pub const DEFAULT_RECHARGE_TITLE: I18nLocale = locale!(zh_cn: "充能需求", en: "Recharge demand");
    pub const DEFAULT_BUFF_TITLE: I18nLocale = locale!(zh_cn: "数值", en: "Number");

    pub const RATE01_TYPE: ItemConfigType = ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 };
    pub const RATE01: ItemConfig = ItemConfig { name: "rate", title: Self::DEFAULT_RATE_TITLE, config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 } };
    pub const STACK02: ItemConfig = ItemConfig { name: "stack", title: Self::DEFAULT_STACK_TITLE, config: ItemConfigType::Float { min: 0.0, max: 2.0, default: 0.0 } };
    pub const STACK03: ItemConfig = ItemConfig { name: "stack", title: Self::DEFAULT_STACK_TITLE, config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 3.0 } };
    pub const STACK04: ItemConfig = ItemConfig { name: "stack", title: Self::DEFAULT_STACK_TITLE, config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 } };
    pub const STACK05: ItemConfig = ItemConfig { name: "stack", title: Self::DEFAULT_STACK_TITLE, config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 0.0 } };
    pub const STACK06: ItemConfig = ItemConfig { name: "stack", title: Self::DEFAULT_STACK_TITLE, config: ItemConfigType::Float { min: 0.0, max: 6.0, default: 0.0 } };
    pub const BUFFV1P: ItemConfig = ItemConfig { name: "p", title: Self::DEFAULT_BUFF_TITLE, config: ItemConfigType::FloatPercentageInput { default: 0.0 } };
    pub const BUFFV1: ItemConfig = ItemConfig { name: "value", title: Self::DEFAULT_BUFF_TITLE, config: ItemConfigType::FloatInput { default: 0.0 } };
    pub const REFINE: ItemConfig = ItemConfig { name: "refine", title: locale!(zh_cn: "精炼", en: "Refine"), config: ItemConfigType::IntInput { min: 1, max: 5, default: 1 } };

    pub const MOONSIGN2: ItemConfig = ItemConfig { name: "moonsign", title: locale!(zh_cn: "月兆", en: "Moonsign"), config: ItemConfigType::Moonsign2 { default: Moonsign::Nascent } };
    pub const MOONSIGN3: ItemConfig = ItemConfig { name: "moonsign", title: locale!(zh_cn: "月兆", en: "Moonsign"), config: ItemConfigType::Moonsign3 { default: Moonsign::None } };

    pub const PRIORITY_DEFAULT: usize = 0;
    pub const PRIORITY_CHARACTERSKILL: usize = 1;
    pub const PRIORITY_ARTIFACT: usize = 2;
    pub const PRIORITY_TARGETFUNCTION: usize = 3;
    pub const PRIORITY_BUFF: usize = 4;
    pub const PRIORITY_WEAPON: usize = 5;
    pub const PRIORITY_CHARACTER: usize = 6;

    pub const fn MOONSIGN_GLOBAL(default: Moonsign, priority: usize, team_shared: bool) -> ItemConfig {
        return ItemConfig { name: "moonsign", title: locale!(zh_cn: "月兆", en: "Moonsign"), config: ItemConfigType::GlobalLinkMoonsign3 { default, global_link: GlobalLinkConfig { key: "moonsign", priority, team_shared } } };
    }
}
