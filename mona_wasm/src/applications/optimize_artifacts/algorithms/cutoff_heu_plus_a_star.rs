use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::algorithms::cutoff_a_star::AStarCutoff;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult};
use mona::artifacts::Artifact;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::*;
use mona::buffs::Buff;
use mona::character::Character;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;
use crate::applications::optimize_artifacts::algorithms::cutoff_algo2::CutoffAlgo2;

pub struct CutoffHeuristicPlusAStar;


impl SingleOptimizeAlgorithm for CutoffHeuristicPlusAStar {
    fn optimize(&self, artifacts: &[&Artifact], artifact_config: Option<ArtifactEffectConfig>, character: &Character<SimpleAttribute>, weapon: &Weapon<SimpleAttribute>, target_function: &Box<dyn TargetFunction>, enemy: &Enemy, buffs: &[Box<dyn Buff<SimpleAttribute>>], constraint: &ConstraintConfig, count: usize) -> Vec<OptimizationResult> {
        let target_function_opt_config = target_function.get_target_function_opt_config();

        let artifacts_vec: Vec<&Artifact> = artifacts.iter().cloned().collect();
        let filtered_artifacts = target_function_opt_config.filter(artifacts_vec);

        let a_star_algo = CutoffAlgo2 { accuracy_factor: 1.0 };

        a_star_algo.optimize(
            &filtered_artifacts,
            artifact_config,
            character,
            weapon,
            target_function,
            enemy,
            buffs,
            constraint,
            count
        )
    }
}