use crate::applications::optimize_artifacts::algorithms::cutoff_a_star::AStarCutoff;
use crate::applications::optimize_artifacts::algorithms::cutoff_algo2::CutoffAlgo2;
use crate::applications::optimize_artifacts::algorithms::cutoff_heu_plus_a_star::CutoffHeuristicPlusAStar;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::artifacts::Artifact;
use mona::attribute::*;
use mona::buffs::Buff;
use mona::character::Character;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;
use serde::{Deserialize, Serialize};

pub trait SingleOptimizeAlgorithm {
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
    ) -> Vec<OptimizationResult>;
}

#[derive(Serialize, Deserialize)]
pub enum SingleOptimizeAlgorithmName {
    // Serialized names are retained for existing presets. AStar is grouped DFS
    // with heuristic estimates; Naive is exact enumeration of the input domain.
    AStar,
    Naive,
    Heuristic,
}

impl Default for SingleOptimizeAlgorithmName {
    fn default() -> Self {
        SingleOptimizeAlgorithmName::AStar
    }
}

impl SingleOptimizeAlgorithmName {
    pub fn get_algorithm(&self) -> Box<dyn SingleOptimizeAlgorithm> {
        match *self {
            SingleOptimizeAlgorithmName::AStar => Box::new(CutoffAlgo2 {
                accuracy_factor: 1.0,
            }),
            SingleOptimizeAlgorithmName::Naive => Box::new(AStarCutoff),
            SingleOptimizeAlgorithmName::Heuristic => Box::new(CutoffHeuristicPlusAStar),
        }
    }
}
