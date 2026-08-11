use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use crate::applications::common::{CharacterFullInterface, CharacterInterface, TargetFunctionInterface, WeaponInterface};
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, ConstraintSetMode, OptimizationResult, OptimizeArtifactInterface};
use mona::artifacts::{Artifact, ArtifactList, ArtifactSlotName};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::*;
use mona::buffs::Buff;
use mona::character::Character;
use mona::common::CharacterFullInfo;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::utils;
use mona::weapon::Weapon;

pub fn optimize_single_interface_wasm(input: &OptimizeArtifactInterface, artifacts: &[&Artifact], algo: &Box<dyn SingleOptimizeAlgorithm>, count: usize) -> Vec<OptimizationResult> {

    let characters = CharacterFullInterface::get_characters(&input.characters);

    let attribute = AttributeUtils::create_attribute_from_list_except_active_character(&characters, input.active_character_id);
    let active_character = CharacterFullInfo::get_character(&characters, input.active_character_id);

    let target_function = input.target_function.to_target_function(&active_character.character, &active_character.weapon);
    // let constraint_ref = input.constraint.as_ref();

    let default_constraint = ConstraintConfig::default();
    let constraint = input.constraint.as_ref().unwrap_or(&default_constraint);

    let filtered_artifacts = input.filter.as_ref().map(|x| x.filter_artifact(artifacts));
    let artifacts = match filtered_artifacts {
        Some(ref a) => a.as_slice(),
        None => artifacts
    };
    let artifact_config = input.artifact_config.clone().map(|x| x.to_config());

    let result = algo.optimize(
        &artifacts,
        artifact_config,
        &active_character.character,
        &active_character.weapon,
        &target_function,
        &Default::default(),
        &active_character.buffs,
        &attribute,
        &constraint,
        count
    );

    result
}
