use mona::artifacts::effect_config::ArtifactConfigInterface;
use crate::applications::common::{BuffInterface, CharacterInterface, EnemyInterface, TargetFunctionInterface, WeaponInterface, CharactersInterface};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CalcArtifactBestSetInterface {
    pub characters: CharactersInterface,
    pub enemy: Option<EnemyInterface>,

    pub active_character_id: usize,
    pub target_function: TargetFunctionInterface,
}
