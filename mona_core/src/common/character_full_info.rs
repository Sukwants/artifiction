use crate::attribute::*;
use crate::character::Character;
use crate::weapon::Weapon;
use crate::buffs::Buff;
use crate::artifacts::{Artifact, ArtifactList};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::team_status::CharacterStatus;


pub struct CharacterFullInfo<'a, A: Attribute> {
    pub character: Character<A>,
    pub weapon: Weapon<A>,
    pub buffs: Vec<Box<dyn Buff<A>>>,
    pub artifacts: Vec<&'a Artifact>,
    pub artifact_config: ArtifactEffectConfig,
    pub skill_config: CharacterSkillConfig,
    pub skill_index: usize,

    pub character_status: CharacterStatus,
}

impl<'a, A: Attribute> CharacterFullInfo<'a, A> {
    pub fn get_character(characters: &'a Vec<CharacterFullInfo<'a, A>>, character_id: usize) -> &'a CharacterFullInfo<'a, A> {
        for c in characters.iter() {
            if c.character_status.character_id == character_id {
                return &c;
            }
        }

        panic!("Character with id {} not found", character_id);
    }
}