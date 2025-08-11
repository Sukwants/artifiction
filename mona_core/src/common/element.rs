use serde::{Serialize, Deserialize};
use strum_macros::Display;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Serialize, Deserialize, Display, FromPrimitive)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Element {
    Electro,
    Pyro,
    Cryo,
    Dendro,
    Geo,
    Anemo,
    Hydro,
    Physical,
}

#[derive(Serialize, Deserialize, Display, FromPrimitive)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum MoonglareReaction {
    LunarChargedReaction,   // 月感电
    LunarCharged,           // 广义月感电
    LunarBloom,
    None,
}

impl Element {
    pub fn from_number(i: usize) -> Element {
        FromPrimitive::from_usize(i).unwrap_or(Element::Physical)
    }
}
