//! Compatibility entry point for old `Heuristic` presets. The former item
//! prefilter called unimplemented metadata on many targets and could delete an
//! entire slot or necessary top-k alternatives. Use grouped search directly.
use super::cutoff_algo2::CutoffAlgo2;
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::artifacts::Artifact;
use mona::attribute::SimpleAttribute;
use mona::buffs::Buff;
use mona::character::Character;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;

pub struct CutoffHeuristicPlusAStar;

impl SingleOptimizeAlgorithm for CutoffHeuristicPlusAStar {
    fn optimize(
        &self,
        artifacts: &[&Artifact],
        artifact_config: Option<ArtifactEffectConfig>,
        character: &Character<SimpleAttribute>,
        weapon: &Weapon<SimpleAttribute>,
        target_function: &Box<dyn TargetFunction>,
        enemy: &Enemy,
        buffs: &[Box<dyn Buff<SimpleAttribute>>],
        attribute: &SimpleAttribute,
        constraint: &ConstraintConfig,
        count: usize,
    ) -> Vec<OptimizationResult> {
        CutoffAlgo2 {
            accuracy_factor: 1.0,
        }
        .optimize(
            artifacts,
            artifact_config,
            character,
            weapon,
            target_function,
            enemy,
            buffs,
            attribute,
            constraint,
            count,
        )
    }
}
