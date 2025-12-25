use serde::{Serialize, Deserialize};
use strum_macros::Display;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use crate::common::SkillType;

#[derive(Serialize, Deserialize, Display, FromPrimitive)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Element {
    Pyro,
    Hydro,
    Anemo,
    Electro,
    Dendro,
    Cryo,
    Geo,
    Physical,
}

impl Element {
    pub fn from_number(i: usize) -> Element {
        FromPrimitive::from_usize(i).unwrap_or(Element::Physical)
    }
}
