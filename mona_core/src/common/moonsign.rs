use serde::{Serialize, Deserialize};
use strum_macros::Display;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Serialize, Deserialize, Display, FromPrimitive)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Moonsign {
    None,
    Nascent,
    Ascendant,
}