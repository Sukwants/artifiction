//! Compatibility entry point for the persisted `Naive` algorithm name.
//! The old implementation also pruned synthetic scores, despite the UI promising
//! enumeration. Delegate to the exact enumerator so arbitrary targets have an oracle.
use super::cutoff_heuristic::CutoffAlgorithmHeuristic;
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

pub struct AStarCutoff;

impl SingleOptimizeAlgorithm for AStarCutoff {
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
        CutoffAlgorithmHeuristic {
            use_heuristic: false,
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
