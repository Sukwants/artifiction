use std::sync::Arc;

use crate::{attribute::Attribute, character::{CharacterName, CharacterStaticData}};

#[derive(Clone)]
pub struct CharacterStatus {
    pub character_id: usize,
    pub team_id: usize,
    pub on_field: bool,
    pub character_static_data: Option<CharacterStaticData>,
}

impl CharacterStatus {
    pub fn new(
        character_id: usize,
        team_id: usize,
        on_field: bool,
        character_static_data: Option<CharacterStaticData>,
    ) -> Self {
        CharacterStatus {
            character_id,
            team_id,
            on_field,
            character_static_data,
        }
    }

    pub fn new_single(character_id: usize, character_name: CharacterName) -> Self {
        CharacterStatus {
            character_id,
            team_id: 0,
            on_field: true,
            character_static_data: Some(character_name.get_static_data()),
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

    pub fn select_all_except_self<A: Attribute>(attribute: &A) -> Self {
        let character_id = attribute.get_character().character_id;
        CharacterSelector {
            selector: Arc::new(move |status: &CharacterStatus| status.character_id != character_id ),
        }
    }
}
