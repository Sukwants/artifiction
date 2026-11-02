use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::*;
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::TargetFunctionMeta;
use crate::target_functions::TargetFunctionConfig;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;
use super::target_function_opt_config::TargetFunctionOptConfig;

pub trait TargetFunctionMetaTrait: TargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta;

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = None;

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction>;
}

pub type TargetFunctionAttributeType = SimpleAttribute;
pub type TargetFunctionAttributeResultType = SimpleAttributeResult;

pub trait TargetFunction {

    /// Opt in only if `target_from_solved` is pure and depends exclusively on
    /// the numeric node map + active character ID and the fixed character,
    /// weapon, enemy and target configuration of one optimization request.
    /// It must ignore raw artifacts, source provenance and character-status
    /// metadata. This is stronger than supporting `target_from_solved` alone.
    /// Optimizers may then memoize bitwise-identical solved inputs locally.
    fn can_memoize_solved_score(&self) -> bool {
        false
    }

    /// Optional fast path after the optimizer has solved this exact attribute
    /// graph to check constraints. Return Some only when this gives precisely
    /// the same score as `target`, including all context and artifact handling.
    /// Targets that modify the graph (e.g. apply skill effects before solving)
    /// must keep None unless they can preserve those semantics here.
    fn target_from_solved(
        &self,
        _attribute: &SimpleAttributeResult,
        _character: &Character<SimpleAttribute>,
        _weapon: &Weapon<SimpleAttribute>,
        _artifacts: &[&Artifact],
        _enemy: &Enemy,
    ) -> Option<f64> {
        None
    }
    
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig;

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig;

    fn target(
        &self,
        attribute: &SimpleAttribute,
        character: &Character<SimpleAttribute>,
        weapon: &Weapon<SimpleAttribute>,
        artifacts: &[&Artifact],
        enemy: &Enemy
    ) -> f64;
}
