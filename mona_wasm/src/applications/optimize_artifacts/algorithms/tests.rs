use super::common::ValueFunction;
use super::cutoff_a_star::AStarCutoff;
use super::cutoff_algo2::CutoffAlgo2;
use super::cutoff_heuristic::CutoffAlgorithmHeuristic;
use crate::applications::optimize_artifacts::algorithm::{
    SingleOptimizeAlgorithm, SingleOptimizeAlgorithmName,
};
use crate::applications::optimize_artifacts::inter::{
    ConstraintConfig, ConstraintSetMode, OptimizationResult,
};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName};
use mona::attribute::*;
use mona::buffs::Buff;
use mona::character::character_config::CharacterConfig;
use mona::character::team_status::CharacterStatus;
use mona::character::{Character, CharacterName};
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::target_functions::target_functions::common::max_atk::MaxATKTargetFunction;
use mona::target_functions::{TargetFunction, TargetFunctionOptConfig};
use mona::team::TeamQuantization;
use mona::weapon::{Weapon, WeaponConfig, WeaponName};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

type Row = ([Option<u64>; 5], f64);

struct Context {
    character: Character<SimpleAttribute>,
    weapon: Weapon<SimpleAttribute>,
    attribute: SimpleAttribute,
}
impl Context {
    fn new() -> Self {
        let character = Character::new(
            CharacterName::Amber,
            90,
            false,
            0,
            8,
            8,
            8,
            &CharacterConfig::NoConfig,
            &HashSet::new(),
            false,
        );
        let weapon = Weapon::new(
            WeaponName::FavoniusWarbow,
            90,
            false,
            1,
            &WeaponConfig::NoConfig,
            &character,
        );
        let attribute = SimpleAttribute::new(
            SimpleAttribute::new_with_base_edge(vec![CharacterStatus::new_single(
                0,
                CharacterName::Amber,
            )]),
            0,
        );
        Self {
            character,
            weapon,
            attribute,
        }
    }
    fn run(
        &self,
        algo: &dyn SingleOptimizeAlgorithm,
        arts: &[&Artifact],
        target: &Box<dyn TargetFunction>,
        constraint: &ConstraintConfig,
        k: usize,
        buffs: &[Box<dyn Buff<SimpleAttribute>>],
    ) -> Vec<OptimizationResult> {
        algo.optimize(
            arts,
            None,
            &self.character,
            &self.weapon,
            target,
            &Enemy::default(),
            buffs,
            &self.attribute,
            constraint,
            k,
        )
    }
    // Independent full Cartesian product + full sort: no production enumeration,
    // pattern logic, candidate heap, or bound checks are reused here.
    fn oracle(
        &self,
        arts: &[&Artifact],
        target: &Box<dyn TargetFunction>,
        constraint: &ConstraintConfig,
        k: usize,
    ) -> Vec<Row> {
        let slots: [Vec<Option<&Artifact>>; 5] = std::array::from_fn(|slot| {
            let mut group: Vec<_> = arts
                .iter()
                .copied()
                .filter(|a| a.slot as usize == slot)
                .map(Some)
                .collect();
            if group.is_empty() {
                group.push(None);
            }
            group
        });
        let config = target.get_default_artifact_config(&Default::default());
        let enemy = Enemy::default();
        let value = ValueFunction {
            artifact_effect_config: &config,
            character: &self.character,
            weapon: &self.weapon,
            target_function: target,
            buffs: &[],
            enemy: &enemy,
            attribute: &self.attribute,
            constraint,
        };
        let mut results = Vec::new();
        let mut seen = HashSet::new();
        for a in &slots[0] {
            for b in &slots[1] {
                for c in &slots[2] {
                    for d in &slots[3] {
                        for e in &slots[4] {
                            let selected: Vec<_> =
                                [a, b, c, d, e].into_iter().filter_map(|a| *a).collect();
                            let count = |set| selected.iter().filter(|a| a.set_name == set).count();
                            let valid = match constraint.set_mode {
                                None | Some(ConstraintSetMode::Any) => true,
                                Some(ConstraintSetMode::Set2(s)) => count(s) >= 2,
                                Some(ConstraintSetMode::Set4(s)) => count(s) >= 4,
                                Some(ConstraintSetMode::Set22(s, t)) => {
                                    if s == t {
                                        count(s) >= 4
                                    } else {
                                        count(s) >= 2 && count(t) >= 2
                                    }
                                }
                            };
                            if !valid {
                                continue;
                            }
                            let graph = value.get_attribute(&selected);
                            if !value.check_attribute_attribute(&graph.solve()) {
                                continue;
                            }
                            let score = value.score_attribute(&graph, &selected);
                            let ids = [a, b, c, d, e].map(|a| a.map(|a| a.id));
                            if score.is_finite() && seen.insert(ids) {
                                results.push((ids, if score == 0.0 { 0.0 } else { score }));
                            }
                        }
                    }
                }
            }
        }
        results.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        results.truncate(k);
        results
    }
}

fn rows(results: &[OptimizationResult]) -> Vec<Row> {
    results
        .iter()
        .map(|r| ([r.flower, r.feather, r.sand, r.goblet, r.head], r.value))
        .collect()
}

// Artificial data deliberately varies enhancement levels and duplicates numerical
// stats. It is a regression fixture, not a claim about game item distributions.
fn fixture(n: usize, sets: &[ArtifactSetName]) -> Vec<Artifact> {
    let slots = [
        ArtifactSlotName::Flower,
        ArtifactSlotName::Feather,
        ArtifactSlotName::Sand,
        ArtifactSlotName::Goblet,
        ArtifactSlotName::Head,
    ];
    let stats = [
        StatName::HPFixed,
        StatName::ATKFixed,
        StatName::ATKPercentage,
        StatName::ATKPercentage,
        StatName::CriticalRate,
    ];
    let mut arts = Vec::new();
    for (slot_i, slot) in slots.into_iter().enumerate() {
        for i in 0..n {
            let mut art = Artifact::new(
                sets[i % sets.len()],
                slot,
                i as i32,
                5,
                vec![
                    (StatName::ATKFixed, (i % 2) as f64),
                    (StatName::Recharge, i as f64 / 10.0),
                ],
                (
                    stats[slot_i],
                    if slot_i == 4 { 0.1 } else { i as f64 + 1.0 },
                ),
            );
            art.id = (slot_i * n + i) as u64;
            arts.push(art);
        }
    }
    arts
}

struct ScoreTarget {
    offset: f64,
    constant: bool,
    nonmonotone: bool,
    visits: Rc<RefCell<HashMap<[u64; 5], usize>>>,
}
impl ScoreTarget {
    fn new(offset: f64, constant: bool, nonmonotone: bool) -> Self {
        Self {
            offset,
            constant,
            nonmonotone,
            visits: Rc::default(),
        }
    }
}
impl TargetFunction for ScoreTarget {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        panic!("exact / grouped search must not require optional prefilter metadata")
    }
    fn get_default_artifact_config(&self, _: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }
    fn target(
        &self,
        _: &SimpleAttribute,
        _: &Character<SimpleAttribute>,
        _: &Weapon<SimpleAttribute>,
        arts: &[&Artifact],
        enemy: &Enemy,
    ) -> f64 {
        if arts.len() == 5 && arts.iter().all(|a| a.level < 20) {
            let ids: [u64; 5] = std::array::from_fn(|i| arts[i].id);
            *self.visits.borrow_mut().entry(ids).or_default() += 1;
        }
        let sum = arts.iter().map(|a| a.main_stat.1).sum::<f64>();
        let score = if self.constant {
            0.0
        } else if self.nonmonotone {
            -(sum - 7.1).powi(2)
        } else {
            sum
        };
        score + self.offset + (enemy.pyro_res - 0.1)
    }
}

#[test]
fn grouped_and_exact_match_independent_oracle_for_top_k_and_constraints() {
    let context = Context::new();
    let arts = fixture(
        3,
        &[
            ArtifactSetName::Adventurer,
            ArtifactSetName::LuckyDog,
            ArtifactSetName::TravelingDoctor,
        ],
    );
    let refs: Vec<_> = arts.iter().rev().collect();
    let target: Box<dyn TargetFunction> = Box::new(ScoreTarget::new(-100.0, false, false));
    let a = ArtifactSetName::Adventurer;
    let b = ArtifactSetName::LuckyDog;
    for mode in [
        None,
        Some(ConstraintSetMode::Any),
        Some(ConstraintSetMode::Set2(a)),
        Some(ConstraintSetMode::Set4(a)),
        Some(ConstraintSetMode::Set22(a, b)),
        Some(ConstraintSetMode::Set22(b, a)),
        Some(ConstraintSetMode::Set22(a, a)),
    ] {
        let constraint = ConstraintConfig {
            set_mode: mode,
            recharge_min: Some(1.2),
            ..Default::default()
        };
        for k in [0, 1, 17, 100, 300] {
            let expected = context.oracle(&refs, &target, &constraint, k);
            for algo in [
                &AStarCutoff as &dyn SingleOptimizeAlgorithm,
                &CutoffAlgo2 {
                    accuracy_factor: 1.0,
                },
            ] {
                assert_eq!(
                    rows(&context.run(algo, &refs, &target, &constraint, k, &[])),
                    expected
                );
            }
        }
    }
}

#[test]
fn constant_scores_fill_top_100_deterministically_without_repeated_leaf_scoring() {
    let context = Context::new();
    let arts = fixture(4, &[ArtifactSetName::Adventurer]);
    let target = ScoreTarget::new(0.0, true, false);
    let visits = target.visits.clone();
    let target: Box<dyn TargetFunction> = Box::new(target);
    let mut refs: Vec<_> = arts.iter().rev().collect();
    let expected = context.oracle(&refs, &target, &Default::default(), 100);
    visits.borrow_mut().clear();
    let result = context.run(
        &CutoffAlgo2 {
            accuracy_factor: 1.0,
        },
        &refs,
        &target,
        &Default::default(),
        100,
        &[],
    );
    assert_eq!(rows(&result), expected);
    assert_eq!(result.len(), 100);
    assert!(result.iter().all(|r| r.ratio == 1.0));
    // Score-and-ID bounds skip equal-score branches once all their IDs are worse.
    assert_eq!(visits.borrow().len(), 100);
    assert!(visits.borrow().values().all(|&n| n == 1));
    refs.reverse();
    assert_eq!(
        rows(&context.run(
            &CutoffAlgo2 {
                accuracy_factor: 1.0
            },
            &refs,
            &target,
            &Default::default(),
            100,
            &[]
        )),
        expected
    );
}

#[test]
fn max_atk_mixed_levels_and_equal_heads_return_all_top_100() {
    let context = Context::new();
    let arts = fixture(4, &[ArtifactSetName::Adventurer]);
    let refs: Vec<_> = arts.iter().collect();
    let target: Box<dyn TargetFunction> = Box::new(MaxATKTargetFunction);
    let expected = context.oracle(&refs, &target, &Default::default(), 100);
    let actual = context.run(
        &CutoffAlgo2 {
            accuracy_factor: 1.0,
        },
        &refs,
        &target,
        &Default::default(),
        100,
        &[],
    );
    assert_eq!(rows(&actual), expected);
    assert_eq!(actual.len(), 100);
}

#[test]
fn exact_handles_nonmonotone_targets_and_duplicate_input_ids() {
    let context = Context::new();
    let arts = fixture(3, &[ArtifactSetName::Adventurer]);
    let mut refs: Vec<_> = arts.iter().collect();
    refs.extend(arts.iter());
    let target: Box<dyn TargetFunction> = Box::new(ScoreTarget::new(0.0, false, true));
    let expected = context.oracle(&refs, &target, &Default::default(), 100);
    let algo = SingleOptimizeAlgorithmName::Naive.get_algorithm();
    assert_eq!(
        rows(&context.run(algo.as_ref(), &refs, &target, &Default::default(), 100, &[])),
        expected
    );
}

#[test]
fn absent_slots_and_impossible_set_constraints_are_consistent() {
    let context = Context::new();
    let arts = fixture(2, &[ArtifactSetName::Adventurer]);
    let refs: Vec<_> = arts.iter().filter(|a| (a.slot as usize) < 2).collect();
    let target: Box<dyn TargetFunction> = Box::new(ScoreTarget::new(-100.0, false, false));
    for mode in [
        None,
        Some(ConstraintSetMode::Set2(ArtifactSetName::Adventurer)),
        Some(ConstraintSetMode::Set4(ArtifactSetName::Adventurer)),
        Some(ConstraintSetMode::Set22(
            ArtifactSetName::Adventurer,
            ArtifactSetName::Adventurer,
        )),
    ] {
        let constraint = ConstraintConfig {
            set_mode: mode,
            ..Default::default()
        };
        let expected = context.oracle(&refs, &target, &constraint, 100);
        for algo in [
            &AStarCutoff as &dyn SingleOptimizeAlgorithm,
            &CutoffAlgo2 {
                accuracy_factor: 1.0,
            },
            &CutoffAlgorithmHeuristic {
                use_heuristic: false,
            },
        ] {
            assert_eq!(
                rows(&context.run(algo, &refs, &target, &constraint, 100, &[])),
                expected
            );
        }
    }
    assert_eq!(
        context
            .run(&AStarCutoff, &[], &target, &Default::default(), 1, &[])
            .len(),
        1
    );
}

struct EnemyBuff;
impl Buff<SimpleAttribute> for EnemyBuff {
    fn change_enemy(&self, enemy: &mut Enemy) {
        enemy.pyro_res += 2.0;
    }
}
#[test]
fn enemy_buffs_apply_once_in_both_paths() {
    let context = Context::new();
    let arts = fixture(1, &[ArtifactSetName::Adventurer]);
    let refs: Vec<_> = arts.iter().collect();
    let target: Box<dyn TargetFunction> = Box::new(ScoreTarget::new(0.0, true, false));
    for algo in [
        &AStarCutoff as &dyn SingleOptimizeAlgorithm,
        &CutoffAlgo2 {
            accuracy_factor: 1.0,
        },
    ] {
        let result = context.run(
            algo,
            &refs,
            &target,
            &Default::default(),
            1,
            &[Box::new(EnemyBuff)],
        );
        assert_eq!(result[0].value, 2.0);
    }
}

#[test]
fn real_damage_targets_match_enumeration_on_small_domains() {
    use mona::target_functions::target_functions::{
        AmberDefaultTargetFunction, NeuvilletteDefaultTargetFunction,
    };
    for neuvillette in [false, true] {
        let mut context = Context::new();
        let target: Box<dyn TargetFunction> = if neuvillette {
            context.character = Character::new(
                CharacterName::Neuvillette,
                90,
                false,
                0,
                8,
                8,
                8,
                &CharacterConfig::Neuvillette { current_hp: 100 },
                &HashSet::new(),
                false,
            );
            context.weapon = Weapon::new(
                WeaponName::PrototypeAmber,
                90,
                false,
                1,
                &WeaponConfig::NoConfig,
                &context.character,
            );
            context.attribute = SimpleAttribute::new(
                SimpleAttribute::new_with_base_edge(vec![CharacterStatus::new_single(
                    0,
                    CharacterName::Neuvillette,
                )]),
                0,
            );
            Box::new(NeuvilletteDefaultTargetFunction)
        } else {
            Box::new(AmberDefaultTargetFunction)
        };
        let arts = fixture(
            3,
            &[
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::EmblemOfSeveredFate,
            ],
        );
        let refs: Vec<_> = arts.iter().collect();
        let expected = context.oracle(&refs, &target, &Default::default(), 100);
        for name in [
            SingleOptimizeAlgorithmName::AStar,
            SingleOptimizeAlgorithmName::Naive,
            SingleOptimizeAlgorithmName::Heuristic,
        ] {
            let algo = name.get_algorithm();
            let actual = context.run(algo.as_ref(), &refs, &target, &Default::default(), 100, &[]);
            assert_eq!(rows(&actual), expected);
        }
    }
}

struct SolvedProbe {
    enable: bool,
    originals: Rc<std::cell::Cell<usize>>,
    reused: Rc<std::cell::Cell<usize>>,
}
impl TargetFunction for SolvedProbe {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unreachable!()
    }
    fn get_default_artifact_config(&self, _: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }
    fn target(
        &self,
        attribute: &SimpleAttribute,
        _: &Character<SimpleAttribute>,
        _: &Weapon<SimpleAttribute>,
        arts: &[&Artifact],
        enemy: &Enemy,
    ) -> f64 {
        self.originals.set(self.originals.get() + 1);
        attribute.solve().get_atk()
            + arts.iter().map(|art| art.id as f64).sum::<f64>()
            + enemy.pyro_res
    }
    fn target_from_solved(
        &self,
        attribute: &SimpleAttributeResult,
        _: &Character<SimpleAttribute>,
        _: &Weapon<SimpleAttribute>,
        arts: &[&Artifact],
        enemy: &Enemy,
    ) -> Option<f64> {
        if !self.enable {
            return None;
        }
        self.reused.set(self.reused.get() + 1);
        Some(
            attribute.get_atk()
                + arts.iter().map(|art| art.id as f64).sum::<f64>()
                + enemy.pyro_res,
        )
    }
}

#[test]
fn solved_fast_path_is_opt_in_and_preserves_artifacts_and_enemy_context() {
    let context = Context::new();
    let arts = fixture(2, &[ArtifactSetName::Adventurer]);
    let refs: Vec<_> = arts.iter().collect();
    let mut expected = None;
    for enable in [false, true] {
        let originals = Rc::new(std::cell::Cell::new(0));
        let reused = Rc::new(std::cell::Cell::new(0));
        let target: Box<dyn TargetFunction> = Box::new(SolvedProbe {
            enable,
            originals: originals.clone(),
            reused: reused.clone(),
        });
        let result = rows(&context.run(
            &AStarCutoff,
            &refs,
            &target,
            &Default::default(),
            100,
            &[Box::new(EnemyBuff)],
        ));
        assert_eq!(originals.get(), if enable { 0 } else { 32 });
        assert_eq!(reused.get(), if enable { 32 } else { 0 });
        if let Some(ref expected) = expected {
            assert_eq!(&result, expected);
        } else {
            expected = Some(result);
        }
        originals.set(0);
        reused.set(0);
        let constraint = ConstraintConfig {
            atk_min: Some(f64::INFINITY),
            ..Default::default()
        };
        assert!(context
            .run(&AStarCutoff, &refs, &target, &constraint, 100, &[])
            .is_empty());
        assert_eq!(originals.get() + reused.get(), 0);
    }
}

#[cfg(feature = "optimizer-legacy")]
#[test]
fn legacy_snapshot_remains_runnable_and_reproduces_top_k_shortfall() {
    let context = Context::new();
    let arts = fixture(4, &[ArtifactSetName::Adventurer]);
    let refs: Vec<_> = arts.iter().collect();
    let target: Box<dyn TargetFunction> = Box::new(ScoreTarget::new(1.0, true, false));
    let old = super::legacy::cutoff_algo2::CutoffAlgo2 {
        accuracy_factor: 1.0,
    };
    let original = context.run(&old, &refs, &target, &Default::default(), 100, &[]);
    let current = context.run(
        &CutoffAlgo2 {
            accuracy_factor: 1.0,
        },
        &refs,
        &target,
        &Default::default(),
        100,
        &[],
    );
    assert!(original.len() < 100);
    assert_eq!(current.len(), 100);
    assert_eq!(original[0].value, current[0].value);
}

struct MemoProbe {
    enabled: bool,
    calls: Rc<std::cell::Cell<usize>>,
}
impl TargetFunction for MemoProbe {
    fn can_memoize_solved_score(&self) -> bool {
        self.enabled
    }
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unreachable!()
    }
    fn get_default_artifact_config(&self, _: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }
    fn target(
        &self,
        attribute: &SimpleAttribute,
        character: &Character<SimpleAttribute>,
        weapon: &Weapon<SimpleAttribute>,
        arts: &[&Artifact],
        enemy: &Enemy,
    ) -> f64 {
        self.target_from_solved(&attribute.solve(), character, weapon, arts, enemy)
            .unwrap()
    }
    fn target_from_solved(
        &self,
        attribute: &SimpleAttributeResult,
        _: &Character<SimpleAttribute>,
        _: &Weapon<SimpleAttribute>,
        _: &[&Artifact],
        enemy: &Enemy,
    ) -> Option<f64> {
        self.calls.set(self.calls.get() + 1);
        Some(attribute.get_atk() + enemy.pyro_res)
    }
}

fn duplicate_profiles(n: usize) -> Vec<Artifact> {
    let mut arts = fixture(n, &[ArtifactSetName::Adventurer]);
    for slot in 0..5 {
        let first = arts[slot * n].clone();
        for i in 1..n {
            let id = arts[slot * n + i].id;
            arts[slot * n + i] = first.clone();
            arts[slot * n + i].id = id;
        }
    }
    arts
}

#[test]
fn score_memoization_preserves_all_ids_top_k_and_request_isolation() {
    let context = Context::new();
    let arts = duplicate_profiles(4);
    let refs: Vec<_> = arts.iter().rev().collect();
    let calls = Rc::new(std::cell::Cell::new(0));
    let cached: Box<dyn TargetFunction> = Box::new(MemoProbe {
        enabled: true,
        calls: calls.clone(),
    });
    let reference: Box<dyn TargetFunction> = Box::new(MemoProbe {
        enabled: false,
        calls: Rc::default(),
    });
    for k in [0, 1, 100, 2000] {
        let expected =
            rows(&context.run(&AStarCutoff, &refs, &reference, &Default::default(), k, &[]));
        calls.set(0);
        let actual = rows(&context.run(&AStarCutoff, &refs, &cached, &Default::default(), k, &[]));
        assert_eq!(actual, expected);
        assert_eq!(calls.get(), usize::from(k != 0));
    }
    let without_buff =
        rows(&context.run(&AStarCutoff, &refs, &cached, &Default::default(), 100, &[]));
    let with_buff = rows(&context.run(
        &AStarCutoff,
        &refs,
        &cached,
        &Default::default(),
        100,
        &[Box::new(EnemyBuff)],
    ));
    assert_eq!(with_buff[0].1, without_buff[0].1 + 2.0);
    let impossible = ConstraintConfig {
        atk_min: Some(f64::INFINITY),
        ..Default::default()
    };
    calls.set(0);
    assert!(context
        .run(&AStarCutoff, &refs, &cached, &impossible, 100, &[])
        .is_empty());
    assert_eq!(calls.get(), 0);
}

#[test]
fn raw_id_dependent_targets_keep_distinct_scores_with_equal_attributes() {
    let context = Context::new();
    let arts = duplicate_profiles(3);
    let refs: Vec<_> = arts.iter().collect();
    let calls = Rc::new(std::cell::Cell::new(0));
    let target: Box<dyn TargetFunction> = Box::new(SolvedProbe {
        enable: true,
        originals: Rc::default(),
        reused: calls.clone(),
    });
    let expected = context.oracle(&refs, &target, &Default::default(), 100);
    calls.set(0);
    assert_eq!(
        rows(&context.run(&AStarCutoff, &refs, &target, &Default::default(), 100, &[])),
        expected
    );
    assert_eq!(calls.get(), 243);
    assert!(!target.can_memoize_solved_score());
}

#[test]
fn grouped_memoization_preserves_results_with_duplicate_profiles() {
    let context = Context::new();
    let arts = duplicate_profiles(4);
    let refs: Vec<_> = arts.iter().rev().collect();
    let reference_calls = Rc::new(std::cell::Cell::new(0));
    let cached_calls = Rc::new(std::cell::Cell::new(0));
    let reference: Box<dyn TargetFunction> = Box::new(MemoProbe {
        enabled: false,
        calls: reference_calls.clone(),
    });
    let cached: Box<dyn TargetFunction> = Box::new(MemoProbe {
        enabled: true,
        calls: cached_calls.clone(),
    });
    let grouped = CutoffAlgo2 {
        accuracy_factor: 1.0,
    };
    for k in [1, 100, 2000] {
        let expected = context.oracle(&refs, &reference, &Default::default(), k);
        reference_calls.set(0);
        cached_calls.set(0);
        assert_eq!(
            rows(&context.run(&grouped, &refs, &reference, &Default::default(), k, &[])),
            expected
        );
        assert_eq!(
            rows(&context.run(&grouped, &refs, &cached, &Default::default(), k, &[])),
            expected
        );
        assert!(cached_calls.get() < reference_calls.get());
    }
}

#[test]
fn declared_memoization_without_a_solved_hook_falls_back_to_original_target() {
    struct MissingHook(SolvedProbe);
    impl TargetFunction for MissingHook {
        fn can_memoize_solved_score(&self) -> bool {
            true
        }
        fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
            unreachable!()
        }
        fn get_default_artifact_config(&self, _: &TeamQuantization) -> ArtifactEffectConfig {
            Default::default()
        }
        fn target(
            &self,
            attribute: &SimpleAttribute,
            character: &Character<SimpleAttribute>,
            weapon: &Weapon<SimpleAttribute>,
            arts: &[&Artifact],
            enemy: &Enemy,
        ) -> f64 {
            self.0.target(attribute, character, weapon, arts, enemy)
        }
    }
    let context = Context::new();
    let arts = duplicate_profiles(3);
    let refs: Vec<_> = arts.iter().collect();
    let originals = Rc::new(std::cell::Cell::new(0));
    let target: Box<dyn TargetFunction> = Box::new(MissingHook(SolvedProbe {
        enable: false,
        originals: originals.clone(),
        reused: Rc::default(),
    }));
    let expected = context.oracle(&refs, &target, &Default::default(), 100);
    originals.set(0);
    assert_eq!(
        rows(&context.run(&AStarCutoff, &refs, &target, &Default::default(), 100, &[])),
        expected
    );
    assert_eq!(originals.get(), 243);
}
