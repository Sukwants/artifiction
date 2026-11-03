use super::common::{ResultRecorder, ValueFunction};
use super::score_cache::SolvedScoreCache;
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::inter::{
    ConstraintConfig, ConstraintSetMode, OptimizationResult,
};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::artifacts::{Artifact, ArtifactSetName};
use mona::attribute::*;
use mona::buffs::Buff;
use mona::character::Character;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;

pub(super) fn set_requirements(constraint: &ConstraintConfig) -> Vec<(ArtifactSetName, usize)> {
    match constraint.set_mode {
        Some(ConstraintSetMode::Set2(set)) => vec![(set, 2)],
        Some(ConstraintSetMode::Set4(set)) => vec![(set, 4)],
        Some(ConstraintSetMode::Set22(a, b)) if a == b => vec![(a, 4)],
        Some(ConstraintSetMode::Set22(a, b)) => vec![(a, 2), (b, 2)],
        _ => Vec::new(),
    }
}

fn enumerate<'a>(
    slots: &[Vec<&'a Artifact>; 5],
    slot: usize,
    selected: &mut Vec<&'a Artifact>,
    requirements: &[(ArtifactSetName, usize)],
    suffix_available: &[[usize; 6]],
    value: &ValueFunction,
    recorder: &mut ResultRecorder,
    cache: &mut SolvedScoreCache,
) {
    // Only prune provably infeasible set constraints. No synthetic score bounds,
    // stat dominance or target-dependent filtering on the exact path.
    for (i, &(set, required)) in requirements.iter().enumerate() {
        let have = selected.iter().filter(|art| art.set_name == set).count();
        if have + suffix_available[i][slot] < required {
            return;
        }
    }
    if slot == 5 {
        if let Some(score) = value.evaluate_cached(selected, cache) {
            let mut ids = [None; 5];
            for art in selected.iter() {
                ids[art.slot as usize] = Some(art.id);
            }
            recorder.push_optional_result(ids, score);
        }
    } else if slots[slot].is_empty() {
        // Preserve the existing partial-outfit behavior for absent slots.
        enumerate(
            slots,
            slot + 1,
            selected,
            requirements,
            suffix_available,
            value,
            recorder,
            cache,
        );
    } else {
        for &art in &slots[slot] {
            selected.push(art);
            enumerate(
                slots,
                slot + 1,
                selected,
                requirements,
                suffix_available,
                value,
                recorder,
                cache,
            );
            selected.pop();
        }
    }
}

/// Exact enumerator; the legacy heuristic flag delegates to grouped search.
pub struct CutoffAlgorithmHeuristic {
    pub use_heuristic: bool,
}

impl SingleOptimizeAlgorithm for CutoffAlgorithmHeuristic {
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
        if count == 0 {
            return Vec::new();
        }
        if self.use_heuristic {
            return super::cutoff_algo2::CutoffAlgo2 {
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
            );
        }
        let mut slots: [Vec<&Artifact>; 5] = std::array::from_fn(|_| Vec::new());
        for &art in artifacts {
            slots[art.slot as usize].push(art);
        }
        for slot in &mut slots {
            slot.sort_by_key(|art| art.id);
            slot.dedup_by_key(|art| art.id);
        }
        let requirements = set_requirements(constraint);
        let suffix_available: Vec<[usize; 6]> = requirements
            .iter()
            .map(|&(set, _)| {
                let mut suffix = [0; 6];
                for i in (0..5).rev() {
                    suffix[i] =
                        suffix[i + 1] + usize::from(slots[i].iter().any(|art| art.set_name == set));
                }
                suffix
            })
            .collect();
        let mut enemy = enemy.clone();
        for buff in buffs {
            buff.change_enemy(&mut enemy);
        }
        let config = artifact_config
            .unwrap_or_else(|| target_function.get_default_artifact_config(&Default::default()));
        let value = ValueFunction {
            artifact_effect_config: &config,
            character,
            weapon,
            target_function,
            buffs,
            enemy: &enemy,
            attribute,
            constraint,
        };
        let mut recorder = ResultRecorder::new(count);
        enumerate(
            &slots,
            0,
            &mut Vec::with_capacity(5),
            &requirements,
            &suffix_available,
            &value,
            &mut recorder,
            &mut SolvedScoreCache::default(),
        );
        recorder.into_results()
    }
}
