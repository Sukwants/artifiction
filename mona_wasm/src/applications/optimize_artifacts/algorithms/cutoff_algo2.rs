use super::common::{
    get_artifacts_group, get_artifacts_group_without_set, get_set_names, get_super_artifacts,
    get_super_artifacts_without_set, ResultRecorder, ValueFunction,
};
use super::cutoff_heuristic::{set_requirements, CutoffAlgorithmHeuristic};
use super::score_cache::SolvedScoreCache;
use super::weight_heuristic::{NaiveWeightHeuristic, WeightHeuristicAlgorithm};
use crate::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult};
use mona::artifacts::eff::ARTIFACT_EFF5;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::artifacts::{Artifact, ArtifactSetName};
use mona::attribute::*;
use mona::buffs::Buff;
use mona::character::Character;
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;
use std::collections::{HashMap, HashSet};

/// Disjoint partition by active 4-piece / 2+2 / single 2-piece / no-pair sets.
/// The fixed positions are the FIRST 4 or FIRST 2 occurrences of each set.
/// Extra occurrences may only follow those positions, removing mask overlap.
#[derive(Clone, Debug)]
struct SetPattern {
    slots: [Option<ArtifactSetName>; 5],
    required: Vec<(ArtifactSetName, usize)>,
}

impl SetPattern {
    fn accepts_prefix(&self, arts: &[&Artifact]) -> bool {
        for (i, art) in arts.iter().enumerate() {
            if self.slots[i].is_some_and(|set| set != art.set_name) {
                return false;
            }
            let fixed_count = self
                .required
                .iter()
                .find(|&&(set, _)| set == art.set_name)
                .map_or(0, |&(_, n)| n);
            let count = arts[..=i]
                .iter()
                .filter(|a| a.set_name == art.set_name)
                .count();
            let max_count = match fixed_count {
                4 => 5,
                2 => 3,
                _ => 1,
            };
            if count > max_count {
                return false;
            }
            if self.slots[i].is_none()
                && fixed_count != 0
                && self.slots[i + 1..]
                    .iter()
                    .any(|&set| set == Some(art.set_name))
            {
                return false;
            }
        }
        true
    }
}

fn visit_patterns(
    sets: &[ArtifactSetName],
    constraint: &ConstraintConfig,
    mut visit: impl FnMut(SetPattern),
) {
    let constraints = set_requirements(constraint);
    let mut emit = |required: Vec<(ArtifactSetName, usize)>| {
        if constraints
            .iter()
            .any(|&(set, n)| !required.iter().any(|&(s, count)| s == set && count >= n))
        {
            return;
        }
        if required.is_empty() {
            visit(SetPattern {
                slots: [None; 5],
                required,
            });
            return;
        }
        for mask1 in 0u32..32 {
            if mask1.count_ones() as usize != required[0].1 {
                continue;
            }
            for mask2 in 0u32..32 {
                if required.len() == 1 && mask2 != 0 {
                    continue;
                }
                if required.len() == 2 && mask2.count_ones() as usize != required[1].1 {
                    continue;
                }
                if mask1 & mask2 != 0 {
                    continue;
                }
                let slots = std::array::from_fn(|i| {
                    if mask1 & (1 << i) != 0 {
                        Some(required[0].0)
                    } else if mask2 & (1 << i) != 0 {
                        Some(required[1].0)
                    } else {
                        None
                    }
                });
                visit(SetPattern {
                    slots,
                    required: required.clone(),
                });
            }
        }
    };
    for &set in sets {
        emit(vec![(set, 4)]);
    }
    for (i, &a) in sets.iter().enumerate() {
        for &b in &sets[i + 1..] {
            emit(vec![(a, 2), (b, 2)]);
        }
    }
    for &set in sets {
        emit(vec![(set, 2)]);
    }
    emit(Vec::new());
}

pub struct CutoffAlgo2Helper<'a> {
    artifacts: HashMap<(ArtifactSetName, usize, StatName), Vec<&'a Artifact>>,
    artifacts_without_set: HashMap<(usize, StatName), Vec<&'a Artifact>>,
    artifact_sets: Vec<ArtifactSetName>,
    super_artifacts: HashMap<(ArtifactSetName, usize, StatName), Artifact>,
    super_artifacts_without_set: HashMap<(usize, StatName), Artifact>,
    main_stats: [Vec<StatName>; 5],
    factor_a: f64,
}

fn finite_weight<K: Eq + std::hash::Hash>(weights: &HashMap<K, f64>, key: &K) -> f64 {
    weights
        .get(key)
        .copied()
        .filter(|v| v.is_finite())
        .unwrap_or(0.0)
}

impl<'a> CutoffAlgo2Helper<'a> {
    pub fn new(
        artifacts: &'a [&'a Artifact],
        weight_heuristic: Option<HashMap<StatName, f64>>,
        set_heuristics: Option<HashMap<ArtifactSetName, f64>>,
        factor_a: f64,
    ) -> Self {
        let weights = weight_heuristic.unwrap_or_default();
        let set_weights = set_heuristics.unwrap_or_default();
        let mut main_stats: [Vec<StatName>; 5] = std::array::from_fn(|_| Vec::new());
        for art in artifacts {
            let stats = &mut main_stats[art.slot as usize];
            if !stats.contains(&art.main_stat.0) {
                stats.push(art.main_stat.0);
            }
        }
        for stats in &mut main_stats {
            stats.sort_by(|a, b| {
                finite_weight(&weights, b)
                    .total_cmp(&finite_weight(&weights, a))
                    .then_with(|| (*a as usize).cmp(&(*b as usize)))
            });
        }
        let mut grouped = get_artifacts_group(artifacts);
        let mut without_set = get_artifacts_group_without_set(artifacts);
        let score = |art: &Artifact| {
            std::iter::once(&art.main_stat)
                .chain(art.sub_stats.iter())
                .map(|&(stat, value)| {
                    let roll = ARTIFACT_EFF5.get_value(stat, 3);
                    if roll > 0.0 {
                        finite_weight(&weights, &stat) * value / roll
                    } else {
                        0.0
                    }
                })
                .sum::<f64>()
        };
        // Cache scores once per artifact; no float-to-unsigned cast or quantization.
        let scores: HashMap<u64, f64> = artifacts.iter().map(|art| (art.id, score(art))).collect();
        for group in grouped.values_mut().chain(without_set.values_mut()) {
            group.sort_by(|a, b| {
                finite_weight(&scores, &b.id)
                    .total_cmp(&finite_weight(&scores, &a.id))
                    .then_with(|| a.id.cmp(&b.id))
            });
            group.dedup_by_key(|art| art.id);
        }
        let mut sets = get_set_names(artifacts);
        sets.sort_by(|a, b| {
            finite_weight(&set_weights, b)
                .total_cmp(&finite_weight(&set_weights, a))
                .then_with(|| (*a as usize).cmp(&(*b as usize)))
        });
        Self {
            artifacts: grouped,
            artifacts_without_set: without_set,
            artifact_sets: sets,
            super_artifacts: get_super_artifacts(artifacts),
            super_artifacts_without_set: get_super_artifacts_without_set(artifacts),
            main_stats,
            factor_a,
        }
    }

    fn search<'b>(
        &self,
        pattern: &SetPattern,
        groups: &[&[&'b Artifact]; 5],
        slot: usize,
        upper: &mut [&'b Artifact; 5],
        min_ids: &mut [u64; 5],
        value: &ValueFunction,
        recorder: &mut ResultRecorder,
        cache: &mut SolvedScoreCache,
    ) {
        if slot == 5 {
            if let Some(score) = value.evaluate_cached(upper, cache) {
                recorder.push_result(upper.map(|art| art.id), score);
            }
            return;
        }
        // A componentwise maximum is a heuristic estimate, not a certified
        // bound for arbitrary effects / DSL targets. Exact mode never uses it.
        let attribute = value.get_attribute(upper);
        let solved = attribute.solve();
        if !value.check_attribute_attribute(&solved) {
            return;
        }
        let score = value.score_cached(&attribute, &solved, upper, cache);
        // Multiplying negative scores by a factor reverses the intended margin.
        let estimate = score - score.abs() * (1.0 - self.factor_a);
        if !recorder.may_improve_with_ids(estimate, *min_ids) {
            return;
        }
        let saved = upper[slot];
        let saved_id = min_ids[slot];
        for &art in groups[slot] {
            upper[slot] = art;
            min_ids[slot] = art.id;
            if pattern.accepts_prefix(&upper[..=slot]) {
                self.search(
                    pattern,
                    groups,
                    slot + 1,
                    upper,
                    min_ids,
                    value,
                    recorder,
                    cache,
                );
            }
        }
        upper[slot] = saved;
        min_ids[slot] = saved_id;
    }

    fn search_pattern(
        &self,
        pattern: &SetPattern,
        value: &ValueFunction,
        recorder: &mut ResultRecorder,
        cache: &mut SolvedScoreCache,
    ) {
        // Precompute available domains per slot / main stat for this pattern.
        let domains: [Vec<(&[&Artifact], &Artifact)>; 5] = std::array::from_fn(|slot| {
            self.main_stats[slot]
                .iter()
                .filter_map(|&stat| match pattern.slots[slot] {
                    Some(set) => self
                        .artifacts
                        .get(&(set, slot, stat))
                        .map(|group| (group.as_slice(), &self.super_artifacts[&(set, slot, stat)])),
                    None => self.artifacts_without_set.get(&(slot, stat)).map(|group| {
                        (
                            group.as_slice(),
                            &self.super_artifacts_without_set[&(slot, stat)],
                        )
                    }),
                })
                .collect()
        });
        for a in &domains[0] {
            for b in &domains[1] {
                for c in &domains[2] {
                    for d in &domains[3] {
                        for e in &domains[4] {
                            let groups = [a.0, b.0, c.0, d.0, e.0];
                            let mut upper = [a.1, b.1, c.1, d.1, e.1];
                            let mut min_ids =
                                groups.map(|group| group.iter().map(|art| art.id).min().unwrap());
                            self.search(
                                pattern,
                                &groups,
                                0,
                                &mut upper,
                                &mut min_ids,
                                value,
                                recorder,
                                cache,
                            );
                        }
                    }
                }
            }
        }
    }

    pub fn do_calculation(&self, value: &ValueFunction, recorder: &mut ResultRecorder) {
        let mut cache = SolvedScoreCache::default();
        visit_patterns(&self.artifact_sets, value.constraint, |pattern| {
            self.search_pattern(&pattern, value, recorder, &mut cache)
        });
    }
}

pub struct CutoffAlgo2 {
    pub accuracy_factor: f64,
}

impl SingleOptimizeAlgorithm for CutoffAlgo2 {
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
        let present: HashSet<_> = artifacts.iter().map(|art| art.slot).collect();
        if present.len() != 5 {
            return CutoffAlgorithmHeuristic {
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
            );
        }
        let config = artifact_config
            .unwrap_or_else(|| target_function.get_default_artifact_config(&Default::default()));
        let mut enemy = enemy.clone();
        for buff in buffs {
            buff.change_enemy(&mut enemy);
        }
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
        let weights = NaiveWeightHeuristic {
            character,
            weapon,
            buffs,
            attribute,
        };
        let helper = CutoffAlgo2Helper::new(
            artifacts,
            Some(weights.generate_stat(target_function)),
            Some(weights.generate_set(target_function)),
            self.accuracy_factor,
        );
        let mut recorder = ResultRecorder::new(count);
        helper.do_calculation(&value, &mut recorder);
        recorder.into_results()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::applications::optimize_artifacts::inter::ConstraintSetMode;
    use mona::artifacts::ArtifactSlotName;

    #[test]
    fn partitions_cover_every_set_layout_exactly_once() {
        use ArtifactSetName::*;
        let sets = [
            Adventurer,
            LuckyDog,
            TravelingDoctor,
            GladiatorsFinale,
            Empty,
        ];
        for mode in [
            None,
            Some(ConstraintSetMode::Set2(LuckyDog)),
            Some(ConstraintSetMode::Set4(LuckyDog)),
            Some(ConstraintSetMode::Set22(LuckyDog, Adventurer)),
            Some(ConstraintSetMode::Set22(LuckyDog, LuckyDog)),
        ] {
            let constraint = ConstraintConfig {
                set_mode: mode,
                ..Default::default()
            };
            let mut patterns = Vec::new();
            visit_patterns(&sets, &constraint, |pattern| patterns.push(pattern));
            for code in 0usize..3125 {
                let arts: Vec<_> = (0..5)
                    .map(|i| {
                        Artifact::new(
                            sets[code / 5usize.pow(i) % 5],
                            ArtifactSlotName::Flower,
                            0,
                            5,
                            vec![],
                            (StatName::HPFixed, 0.0),
                        )
                    })
                    .collect();
                let refs: Vec<_> = arts.iter().collect();
                let count = |set| arts.iter().filter(|art| art.set_name == set).count();
                let valid = match constraint.set_mode {
                    Some(ConstraintSetMode::Set2(s)) => count(s) >= 2,
                    Some(ConstraintSetMode::Set4(s)) => count(s) >= 4,
                    Some(ConstraintSetMode::Set22(a, b)) if a == b => count(a) >= 4,
                    Some(ConstraintSetMode::Set22(a, b)) => count(a) >= 2 && count(b) >= 2,
                    _ => true,
                };
                let matches: Vec<_> = patterns
                    .iter()
                    .filter(|p| p.accepts_prefix(&refs))
                    .collect();
                assert_eq!(matches.len(), usize::from(valid), "layout {code}");
                for pattern in matches {
                    for end in 0..5 {
                        assert!(pattern.accepts_prefix(&refs[..end]));
                    }
                }
            }
        }
    }

    #[test]
    fn ordering_uses_float_weights_and_deterministic_ties() {
        let mut a = Artifact::new(
            ArtifactSetName::Adventurer,
            ArtifactSlotName::Sand,
            20,
            5,
            vec![(StatName::ATKFixed, 1.0)],
            (StatName::ATKPercentage, 0.0),
        );
        a.id = 10;
        let mut b = a.clone();
        b.id = 20;
        b.sub_stats[0].1 = 2.0;
        let mut c = b.clone();
        c.id = 15;
        let refs = [&a, &b, &c];
        let helper = CutoffAlgo2Helper::new(
            &refs,
            Some(HashMap::from([(StatName::ATKFixed, 0.0001)])),
            None,
            1.0,
        );
        let group = &helper.artifacts_without_set[&(2, StatName::ATKPercentage)];
        assert_eq!(
            group.iter().map(|a| a.id).collect::<Vec<_>>(),
            vec![15, 20, 10]
        );
    }
}
