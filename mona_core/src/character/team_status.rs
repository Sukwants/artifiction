use crate::attribute::Attribute;
use crate::character::{CharacterName, CharacterStaticData};
use crate::common::Element;
use crate::common::i18n::{locale, I18nLocale};
use mona_derive::EnumLen;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};
use strum_macros::{Display, EnumIter};

#[derive(Clone)]
pub struct CharacterStatus {
    pub character_id: usize,
    pub team_id: usize,
    pub on_field: bool,
    pub character_static_data: CharacterStaticData,
    pub tags: CharacterTags,
}

#[derive(Clone, Display, Hash, Eq, PartialEq, Serialize, Deserialize, EnumLen, EnumIter, FromPrimitive)]
pub enum CharacterTag {
    Moonsign,
    Hexerei,
}

pub type CharacterTags = HashSet<CharacterTag>;

impl CharacterTag {
    pub fn get_locale(&self) -> I18nLocale {
        match self {
            CharacterTag::Moonsign => locale!(zh_cn: "月兆", en: "Moonsign"),
            CharacterTag::Hexerei => locale!(zh_cn: "魔导", en: "Hexerei"),
        }
    }
}

impl CharacterStatus {
    pub fn new(
        character_id: usize,
        team_id: usize,
        on_field: bool,
        character_static_data: CharacterStaticData,
        tags: CharacterTags,
    ) -> Self {
        CharacterStatus {
            character_id,
            team_id,
            on_field,
            character_static_data,
            tags,
        }
    }

    pub fn new_single(character_id: usize, character_name: CharacterName) -> Self {
        CharacterStatus {
            character_id,
            team_id: 0,
            on_field: true,
            character_static_data: character_name.get_static_data(),
            tags: HashSet::new(),
        }
    }
}

pub struct CharacterSelector {
    pub selector: Arc<dyn Fn(&CharacterStatus) -> bool>,
}

impl CharacterSelector {
    pub fn new<F>(f: F) -> Self
    where
        F: 'static + Fn(&CharacterStatus) -> bool,
    {
        CharacterSelector {
            selector: Arc::new(f),
        }
    }

    pub fn get_matched_list(&self, team: &Vec<CharacterStatus>) -> Vec<usize> {
        let mut list: Vec<usize> = Vec::new();
        for status in team.iter() {
            if (self.selector)(status) {
                list.push(status.character_id);
            }
        }
        list
    }

    pub fn select_self<A: Attribute>(attribute: &A) -> Self {
        let character_id = attribute.get_character().character_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.character_id == character_id ),
        }
    }

    pub fn select_self_onfield<A: Attribute>(attribute: &A) -> Self {
        let character_id = attribute.get_character().character_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.character_id == character_id && status.on_field ),
        }
    }

    pub fn select_self_offfield<A: Attribute>(attribute: &A) -> Self {
        let character_id = attribute.get_character().character_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.character_id == character_id && !status.on_field ),
        }
    }

    pub fn select_onfield<A: Attribute>(attribute: &A) -> Self {
        let team_id = attribute.get_character().team_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.on_field && status.team_id == team_id ),
        }
    }

    pub fn select_offfield<A: Attribute>(attribute: &A) -> Self {
        let team_id = attribute.get_character().team_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| !status.on_field && status.team_id == team_id ),
        }
    }

    pub fn select_onfield_except_self<A: Attribute>(attribute: &A) -> Self {
        let character_id = attribute.get_character().character_id;
        let team_id = attribute.get_character().team_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.on_field && status.team_id == team_id && status.character_id != character_id ),
        }
    }

    pub fn select_team<A: Attribute>(attribute: &A) -> Self {
        let team_id = attribute.get_character().team_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.team_id == team_id ),
        }
    }

    pub fn select_team_except_self<A: Attribute>(attribute: &A) -> Self {
        let character_id = attribute.get_character().character_id;
        let team_id = attribute.get_character().team_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.character_id != character_id && status.team_id == team_id ),
        }
    }

    pub fn select_all<A: Attribute>(attribute: &A) -> Self {
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| true ),
        }
    }

    pub fn select_all_onfield<A: Attribute>(attribute: &A) -> Self {
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.on_field ),
        }
    }

    pub fn select_all_onfield_except_self<A: Attribute>(attribute: &A) -> Self {
        let character_id = attribute.get_character().character_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.on_field && status.character_id != character_id ),
        }
    }

    pub fn select_all_except_self<A: Attribute>(attribute: &A) -> Self {
        let character_id = attribute.get_character().character_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.character_id != character_id ),
        }
    }

    pub fn select_element<A: Attribute>(attribute: &A, element: Element) -> Self {
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.character_static_data.element == element ),
        }
    }

    pub fn select_by_tag<A: Attribute>(attribute: &A, tag: CharacterTag) -> Self {
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.tags.contains(&tag) ),
        }
    }
}
