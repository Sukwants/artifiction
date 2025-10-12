use serde::{Serialize, Deserialize};
use strum_macros::Display;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use crate::common::{i18n::locale, item_config_type::{ItemConfig, ItemConfigType}};

#[derive(Serialize, Deserialize, Display, FromPrimitive)]
#[derive(Default, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Moonsign {
    #[default]
    None,
    Nascent,
    Ascendant,
}

impl Moonsign {
    pub fn is_nascent(&self) -> bool {
        *self == Moonsign::Nascent || *self == Moonsign::Ascendant
    }

    pub fn is_ascendant(&self) -> bool {
        *self == Moonsign::Ascendant
    }
}