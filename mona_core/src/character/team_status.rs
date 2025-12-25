use std::sync::Arc;

use crate::character::CharacterStaticData;

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

    pub fn new_single(character_id: usize) -> Self {
        CharacterStatus {
            character_id,
            team_id: 0,
            on_field: false,
            character_static_data: None,
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
}
