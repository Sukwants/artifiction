use super::score_cache::SolvedScoreCache;
use crate::applications::optimize_artifacts::inter::{ConstraintConfig, OptimizationResult};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::artifacts::{Artifact, ArtifactList, ArtifactSetName};
use mona::attribute::*;
use mona::buffs::Buff;
use mona::character::Character;
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Debug)]
pub struct OptimizationIntermediateResult {
    // arts must have same slot order
    pub arts: [Option<u64>; 5],
    pub score: f64,
}

impl OptimizationIntermediateResult {
    pub fn to_result(&self, max_score: f64) -> OptimizationResult {
        OptimizationResult {
            flower: self.arts[0],
            feather: self.arts[1],
            sand: self.arts[2],
            goblet: self.arts[3],
            head: self.arts[4],
            value: self.score,
            ratio: if max_score == 0.0 {
                if self.score == 0.0 {
                    1.0
                } else {
                    0.0
                }
            } else {
                let ratio = self.score / max_score;
                if ratio.is_finite() {
                    ratio
                } else {
                    0.0
                }
            },
        }
    }
}

impl Eq for OptimizationIntermediateResult {}

impl PartialEq<Self> for OptimizationIntermediateResult {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PartialOrd<Self> for OptimizationIntermediateResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OptimizationIntermediateResult {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher score wins; equal scores prefer the lexicographically smaller
        // slot-ordered ID tuple, independently of traversal / HashMap order.
        self.score
            .total_cmp(&other.score)
            .then_with(|| other.arts.cmp(&self.arts))
    }
}

pub struct ValueFunction<'a> {
    pub artifact_effect_config: &'a ArtifactEffectConfig,
    pub character: &'a Character<SimpleAttribute>,
    pub weapon: &'a Weapon<SimpleAttribute>,
    pub target_function: &'a Box<dyn TargetFunction>,
    pub buffs: &'a [Box<dyn Buff<SimpleAttribute>>],
    pub enemy: &'a Enemy,
    pub attribute: &'a SimpleAttribute,
    pub constraint: &'a ConstraintConfig,
}

impl<'a> ValueFunction<'a> {
    pub fn get_attribute(&self, arts: &[&Artifact]) -> SimpleAttribute {
        let art_list = ArtifactList { artifacts: arts };
        let attribute = AttributeUtils::change_attribute_without_skill(
            self.attribute.clone(),
            &art_list,
            &self.artifact_effect_config,
            &self.character,
            &self.weapon,
            &self.buffs,
        );
        attribute
    }

    pub fn score_attribute(&self, attribute: &SimpleAttribute, arts: &[&Artifact]) -> f64 {
        let score = self.target_function.target(
            &attribute,
            &self.character,
            &self.weapon,
            &arts,
            &self.enemy,
        );
        score
    }

    pub fn score(&self, arts: &[&Artifact]) -> f64 {
        let attribute = self.get_attribute(arts);

        self.score_attribute(&attribute, arts)
    }

    pub fn check_attribute_attribute(&self, attribute: &SimpleAttributeResult) -> bool {
        if attribute.get_atk() < self.constraint.atk_min.unwrap_or(0.0) {
            return false;
        }
        if attribute.get_def() < self.constraint.def_min.unwrap_or(0.0) {
            return false;
        }
        if attribute.get_hp() < self.constraint.hp_min.unwrap_or(0.0) {
            return false;
        }
        if attribute.get_value(AttributeName::ElementalMastery)
            < self.constraint.em_min.unwrap_or(0.0)
        {
            return false;
        }
        if attribute.get_value(AttributeName::Recharge)
            < self.constraint.recharge_min.unwrap_or(0.0)
        {
            return false;
        }
        let critical = attribute
            .get_value(AttributeName::CriticalBase)
            .clamp(0.0, 1.0);
        if critical < self.constraint.crit_min.unwrap_or(0.0) {
            return false;
        }
        if attribute.get_value(AttributeName::CriticalDamageBase)
            < self.constraint.crit_dmg_min.unwrap_or(0.0)
        {
            return false;
        }

        true
    }

    pub fn check_attribute(&self, arts: &[&Artifact]) -> bool {
        let attribute = self.get_attribute(arts);

        self.check_attribute_attribute(&attribute.solve())
    }

    pub fn evaluate(&self, arts: &[&Artifact]) -> Option<f64> {
        let attribute = self.get_attribute(arts);
        let solved = attribute.solve();
        if !self.check_attribute_attribute(&solved) {
            return None;
        }
        let score = self.score_with_solved_attribute(&attribute, &solved, arts);
        score.is_finite().then_some(score)
    }

    pub(super) fn evaluate_cached(
        &self,
        arts: &[&Artifact],
        cache: &mut SolvedScoreCache,
    ) -> Option<f64> {
        let attribute = self.get_attribute(arts);
        let solved = attribute.solve();
        if !self.check_attribute_attribute(&solved) {
            return None;
        }
        let score = self.score_cached(&attribute, &solved, arts, cache);
        score.is_finite().then_some(score)
    }

    pub(super) fn score_cached(
        &self,
        attribute: &SimpleAttribute,
        solved: &SimpleAttributeResult,
        arts: &[&Artifact],
        cache: &mut SolvedScoreCache,
    ) -> f64 {
        if !self.target_function.can_memoize_solved_score() {
            return self.score_with_solved_attribute(attribute, solved, arts);
        }
        cache
            .score(solved, || {
                self.target_function.target_from_solved(
                    solved,
                    self.character,
                    self.weapon,
                    arts,
                    self.enemy,
                )
            })
            .unwrap_or_else(|| self.score_attribute(attribute, arts))
    }

    pub fn score_with_solved_attribute(
        &self,
        attribute: &SimpleAttribute,
        solved: &SimpleAttributeResult,
        arts: &[&Artifact],
    ) -> f64 {
        self.target_function
            .target_from_solved(solved, self.character, self.weapon, arts, self.enemy)
            .unwrap_or_else(|| self.score_attribute(attribute, arts))
    }
}

pub struct ResultRecorder {
    size: usize,
    results: BinaryHeap<Reverse<OptimizationIntermediateResult>>,
    result_hash: HashSet<[Option<u64>; 5]>,
}

impl ResultRecorder {
    pub fn new(size: usize) -> ResultRecorder {
        ResultRecorder {
            size,
            results: BinaryHeap::new(),
            result_hash: HashSet::new(),
        }
    }

    pub fn get_results_descend(&self) -> Vec<OptimizationIntermediateResult> {
        let mut result = Vec::new();

        for item in self.results.iter() {
            result.push(item.0.clone());
        }

        result.sort_by(|x, y| y.cmp(x));
        result
    }

    pub fn push_result(&mut self, arts: [u64; 5], score: f64) {
        self.push_optional_result(arts.map(Some), score);
    }

    pub fn push_optional_result(&mut self, arts: [Option<u64>; 5], score: f64) {
        if self.size == 0 || !score.is_finite() || self.result_hash.contains(&arts) {
            return;
        }
        // Normalize signed zero so it is a score tie, too.
        let candidate = OptimizationIntermediateResult {
            arts,
            score: if score == 0.0 { 0.0 } else { score },
        };
        if self.results.len() == self.size {
            if candidate <= self.results.peek().unwrap().0 {
                return;
            }
            let mut worst = self.results.peek_mut().unwrap();
            self.result_hash.remove(&worst.0.arts);
            *worst = Reverse(candidate);
        } else {
            self.results.push(Reverse(candidate));
        }
        self.result_hash.insert(arts);
    }

    pub fn current_least(&self) -> f64 {
        if self.size == 0 {
            f64::INFINITY
        } else if self.results.len() < self.size {
            f64::NEG_INFINITY
        } else {
            self.results.peek().unwrap().0.score
        }
    }

    pub fn may_improve(&self, upper_score: f64) -> bool {
        // Unknown bounds cannot reject a branch. Equality can still improve IDs.
        self.size != 0 && (!upper_score.is_finite() || upper_score >= self.current_least())
    }

    pub fn may_improve_with_ids(&self, upper_score: f64, min_ids: [u64; 5]) -> bool {
        if !self.may_improve(upper_score) {
            return false;
        }
        if self.results.len() < self.size || !upper_score.is_finite() {
            return true;
        }
        let worst = &self.results.peek().unwrap().0;
        // The relaxed per-slot minimum ID tuple bounds every descendant's tie
        // rank even when that tuple itself violates a set constraint.
        upper_score > worst.score || min_ids.map(Some) < worst.arts
    }

    pub fn into_results(self) -> Vec<OptimizationResult> {
        let results = self.get_results_descend();
        let max_score = results.first().map_or(0.0, |result| result.score);
        results
            .iter()
            .map(|result| result.to_result(max_score))
            .collect()
    }
}

pub fn get_super_artifacts(
    arts: &[&Artifact],
) -> HashMap<(ArtifactSetName, usize, StatName), Artifact> {
    let mut result = HashMap::new();

    for art in arts.iter() {
        let set_name = art.set_name;
        let slot = art.slot;
        let main_stat = art.main_stat.0;

        let entry = result
            .entry((set_name, slot as usize, main_stat))
            .or_insert(Artifact::new(
                set_name,
                slot,
                20,
                5,
                vec![],
                (main_stat, art.main_stat.1),
            ));
        entry.main_stat.1 = entry.main_stat.1.max(art.main_stat.1);
        for sub_slot in art.sub_stats.iter() {
            let mut flag = false;
            for i in entry.sub_stats.iter_mut() {
                if i.0 == sub_slot.0 {
                    i.1 = i.1.max(sub_slot.1);
                    flag = true;
                    break;
                }
            }
            if !flag {
                entry.sub_stats.push((sub_slot.0, sub_slot.1));
            }
        }
    }

    result
}

pub fn get_super_artifacts_without_set(arts: &[&Artifact]) -> HashMap<(usize, StatName), Artifact> {
    let mut result = HashMap::new();

    for art in arts.iter() {
        let slot = art.slot;
        let main_stat = art.main_stat.0;

        let entry = result
            .entry((slot as usize, main_stat))
            .or_insert(Artifact::new(
                ArtifactSetName::Empty,
                slot,
                20,
                5,
                vec![],
                (main_stat, art.main_stat.1),
            ));
        entry.main_stat.1 = entry.main_stat.1.max(art.main_stat.1);
        for sub_slot in art.sub_stats.iter() {
            let mut flag = false;
            for i in entry.sub_stats.iter_mut() {
                if i.0 == sub_slot.0 {
                    i.1 = i.1.max(sub_slot.1);
                    flag = true;
                    break;
                }
            }
            if !flag {
                entry.sub_stats.push((sub_slot.0, sub_slot.1));
            }
        }
    }

    result
}

pub fn get_artifacts_group<'a>(
    arts: &[&'a Artifact],
) -> HashMap<(ArtifactSetName, usize, StatName), Vec<&'a Artifact>> {
    let mut result = HashMap::new();

    for art in arts.iter() {
        let set_name = art.set_name;
        let slot = art.slot;
        let main_stat = art.main_stat.0;

        let entry = result
            .entry((set_name, slot as usize, main_stat))
            .or_insert(Vec::new());
        entry.push(*art);
    }

    result
}

pub fn get_artifacts_group_without_set<'a>(
    arts: &[&'a Artifact],
) -> HashMap<(usize, StatName), Vec<&'a Artifact>> {
    let mut result = HashMap::new();

    for art in arts.iter() {
        let slot = art.slot;
        let main_stat = art.main_stat.0;

        let entry = result
            .entry((slot as usize, main_stat))
            .or_insert(Vec::new());
        entry.push(*art);
    }

    result
}

pub fn get_set_names(arts: &[&Artifact]) -> Vec<ArtifactSetName> {
    let mut set = HashSet::new();
    for art in arts.iter() {
        set.insert(art.set_name);
    }

    set.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mona::artifacts::ArtifactSlotName;

    #[test]
    fn top_k_matches_sort_for_ties_negatives_duplicates_and_evictions() {
        for k in [0, 1, 2, 17, 100, 200] {
            let mut expected: Vec<_> = (0..137)
                .map(|id| OptimizationIntermediateResult {
                    arts: [Some(id); 5],
                    score: ((id * 31) % 19) as f64 - 19.0,
                })
                .collect();
            expected.sort_by(|a, b| {
                b.score
                    .partial_cmp(&a.score)
                    .unwrap()
                    .then_with(|| a.arts.cmp(&b.arts))
            });
            expected.truncate(k);
            let mut recorder = ResultRecorder::new(k);
            // Visit each identity twice, in a non-sorted order, including evicted IDs.
            for i in 0..274 {
                let id = (i * 67 % 137) as u64;
                recorder.push_result([id; 5], ((id * 31) % 19) as f64 - 19.0);
            }
            assert_eq!(recorder.get_results_descend(), expected);
        }
    }

    #[test]
    fn threshold_waits_for_k_and_does_not_prune_equal_or_unknown_bounds() {
        let mut recorder = ResultRecorder::new(2);
        recorder.push_result([9; 5], -2.0);
        recorder.push_result([9; 5], -2.0);
        assert_eq!(recorder.current_least(), f64::NEG_INFINITY);
        recorder.push_result([8; 5], -3.0);
        assert_eq!(recorder.current_least(), -3.0);
        assert!(recorder.may_improve(-3.0));
        assert!(recorder.may_improve(f64::NAN));
        assert!(!recorder.may_improve(-4.0));
        assert!(!recorder.may_improve_with_ids(-3.0, [9; 5]));
        assert!(recorder.may_improve_with_ids(-3.0, [7; 5]));
        recorder.push_result([7; 5], -3.0);
        assert_eq!(recorder.get_results_descend()[1].arts, [Some(7); 5]);
    }

    #[test]
    fn zero_and_nonfinite_scores_do_not_break_results() {
        let mut recorder = ResultRecorder::new(4);
        for (id, score) in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -0.0, 0.0]
            .into_iter()
            .enumerate()
        {
            recorder.push_result([id as u64; 5], score);
        }
        let results = recorder.into_results();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].flower, Some(3));
        assert!(results.iter().all(|r| r.ratio == 1.0));
    }

    #[test]
    fn super_artifact_uses_largest_main_stat_independent_of_input_order() {
        let mut low = Artifact::new(
            ArtifactSetName::Adventurer,
            ArtifactSlotName::Flower,
            0,
            5,
            vec![(StatName::ATKFixed, 20.0)],
            (StatName::HPFixed, 10.0),
        );
        low.id = 1;
        let mut high = low.clone();
        high.id = 2;
        high.level = 20;
        high.main_stat.1 = 100.0;
        high.sub_stats[0].1 = 10.0;
        for arts in [[&low, &high], [&high, &low]] {
            let grouped = get_super_artifacts(&arts);
            let ungrouped = get_super_artifacts_without_set(&arts);
            for art in grouped.values().chain(ungrouped.values()) {
                assert_eq!(art.main_stat.1, 100.0);
                assert_eq!(art.sub_stats, vec![(StatName::ATKFixed, 20.0)]);
            }
        }
    }
}
