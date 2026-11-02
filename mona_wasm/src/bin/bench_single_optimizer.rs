//! Reproducible native comparison. See docs/single-optimizer.md for CLI usage.
use mona::artifacts::eff::ARTIFACT_EFF5;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::artifacts::{Artifact, ArtifactSetName, ArtifactSlotName};
use mona::attribute::*;
use mona::character::character_config::CharacterConfig;
use mona::character::team_status::CharacterStatus;
use mona::character::{Character, CharacterName};
use mona::common::StatName;
use mona::enemies::Enemy;
use mona::target_functions::target_functions::{
    AmberDefaultTargetFunction, MaxATKTargetFunction, NeuvilletteDefaultTargetFunction,
};
use mona::target_functions::{TargetFunction, TargetFunctionOptConfig};
use mona::team::TeamQuantization;
use mona::weapon::{Weapon, WeaponConfig, WeaponName};
use mona_wasm::applications::optimize_artifacts::algorithm::SingleOptimizeAlgorithm;
use mona_wasm::applications::optimize_artifacts::algorithms::{
    cutoff_algo2::CutoffAlgo2, cutoff_heuristic::CutoffAlgorithmHeuristic, legacy,
};
use mona_wasm::applications::optimize_artifacts::inter::OptimizationResult;
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::time::Instant;

struct CountedTarget {
    inner: Box<dyn TargetFunction>,
    reuse: bool,
    memoize: bool,
    original_calls: Rc<Cell<usize>>,
    reused_calls: Rc<Cell<usize>>,
}

impl TargetFunction for CountedTarget {
    fn can_memoize_solved_score(&self) -> bool {
        self.reuse && self.memoize && self.inner.can_memoize_solved_score()
    }
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        self.inner.get_target_function_opt_config()
    }
    fn get_default_artifact_config(&self, team: &TeamQuantization) -> ArtifactEffectConfig {
        self.inner.get_default_artifact_config(team)
    }
    fn target(
        &self,
        attribute: &SimpleAttribute,
        character: &Character<SimpleAttribute>,
        weapon: &Weapon<SimpleAttribute>,
        arts: &[&Artifact],
        enemy: &Enemy,
    ) -> f64 {
        self.original_calls.set(self.original_calls.get() + 1);
        self.inner.target(attribute, character, weapon, arts, enemy)
    }
    fn target_from_solved(
        &self,
        attribute: &SimpleAttributeResult,
        character: &Character<SimpleAttribute>,
        weapon: &Weapon<SimpleAttribute>,
        arts: &[&Artifact],
        enemy: &Enemy,
    ) -> Option<f64> {
        if !self.reuse {
            return None;
        }
        let score = self
            .inner
            .target_from_solved(attribute, character, weapon, arts, enemy);
        if score.is_some() {
            self.reused_calls.set(self.reused_calls.get() + 1);
        }
        score
    }
}

fn synthetic(n: usize, distinct: bool) -> Vec<Artifact> {
    let mut artifacts = Vec::new();
    let slots = [
        ArtifactSlotName::Flower,
        ArtifactSlotName::Feather,
        ArtifactSlotName::Sand,
        ArtifactSlotName::Goblet,
        ArtifactSlotName::Head,
    ];
    let main_stats = [
        StatName::HPFixed,
        StatName::ATKFixed,
        StatName::ATKPercentage,
        StatName::HydroBonus,
        StatName::CriticalRate,
    ];
    let sets = [
        ArtifactSetName::GladiatorsFinale,
        ArtifactSetName::ShimenawasReminiscence,
        ArtifactSetName::EmblemOfSeveredFate,
    ];
    for (slot_i, slot) in slots.into_iter().enumerate() {
        for i in 0..n {
            let stats = [
                StatName::ATKPercentage,
                StatName::HPPercentage,
                StatName::CriticalDamage,
                StatName::Recharge,
            ];
            let sub = stats
                .into_iter()
                .filter(|&stat| stat != main_stats[slot_i])
                .enumerate()
                .map(|(j, stat)| {
                    let mut value = ARTIFACT_EFF5.get_value(stat, (i + j + slot_i) % 4)
                        * (1 + (i * 3 + j) % 3) as f64;
                    if distinct && stat == StatName::Recharge {
                        // Deliberately artificial low-order signature: stress
                        // cache misses without changing the search objective.
                        value += 1e-12 * i as f64 * (n + 1).pow(slot_i as u32) as f64;
                    }
                    (stat, value)
                })
                .collect();
            let mut art = Artifact::new(
                sets[i % sets.len()],
                slot,
                20,
                5,
                sub,
                (
                    main_stats[slot_i],
                    StatName::artifact_main_stat_max_value(main_stats[slot_i]),
                ),
            );
            art.id = (slot_i * n + i + 1) as u64;
            artifacts.push(art);
        }
    }
    artifacts
}

fn ids(result: &OptimizationResult) -> [Option<u64>; 5] {
    [
        result.flower,
        result.feather,
        result.sand,
        result.goblet,
        result.head,
    ]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut n = 6usize;
    let mut k = 100usize;
    let mut repeats = 5usize;
    let mut target_name = String::from("max_atk");
    let mut input = None;
    let mut copies = 1usize;
    let mut profile = String::from("rolls");
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args.next().ok_or("every option requires a value")?;
        match flag.as_str() {
            "--per-slot" => n = value.parse()?,
            "--k" => k = value.parse()?,
            "--repeats" => repeats = value.parse()?,
            "--target" => target_name = value,
            "--input" => input = Some(value),
            "--copies" => copies = value.parse()?,
            "--profile" => profile = value,
            _ => return Err(format!("unknown option: {flag}").into()),
        }
    }
    if repeats == 0 || repeats > 100 || n == 0 || n > 15 {
        return Err("use 1..100 repeats and 1..15 items per slot".into());
    }
    if copies == 0 || copies > 8 {
        return Err("copies must be in 1..8".into());
    }
    if profile != "rolls" && profile != "distinct" {
        return Err("profile must be rolls or distinct".into());
    }
    if input.is_some() && profile != "rolls" {
        return Err("profile only controls generated inputs".into());
    }
    let mut artifacts: Vec<Artifact> = if let Some(path) = input.as_ref() {
        serde_json::from_str(&std::fs::read_to_string(path)?)?
    } else {
        synthetic(n, profile == "distinct")
    };
    if copies > 1 {
        let original_len = artifacts.len();
        let mut next_id = artifacts.iter().map(|a| a.id).max().unwrap_or(0);
        for i in 0..original_len {
            for _ in 1..copies {
                next_id = next_id.checked_add(1).ok_or("artifact ID overflow")?;
                let mut copy = artifacts[i].clone();
                copy.id = next_id;
                artifacts.push(copy);
            }
        }
    }
    let refs: Vec<_> = artifacts.iter().collect();
    let slot_counts: [usize; 5] =
        std::array::from_fn(|i| artifacts.iter().filter(|a| a.slot as usize == i).count());
    let combinations = slot_counts
        .iter()
        .fold(1usize, |n, &slot| n.saturating_mul(slot.max(1)));
    if combinations > 1_000_000 {
        return Err("comparison includes exhaustive enumeration; limit the input to <= 1,000,000 combinations".into());
    }
    if k == 0 || k > combinations {
        return Err("use 1 <= k <= combinations for comparison".into());
    }
    let mut unique = HashSet::new();
    if artifacts.iter().any(|a| !unique.insert(a.id)) {
        return Err("benchmark input must have unique artifact IDs".into());
    }
    let (name, config, weapon_name) = match target_name.as_str() {
        "max_atk" | "amber" => (
            CharacterName::Amber,
            CharacterConfig::NoConfig,
            WeaponName::FavoniusWarbow,
        ),
        "neuvillette" => (
            CharacterName::Neuvillette,
            CharacterConfig::Neuvillette { current_hp: 100 },
            WeaponName::PrototypeAmber,
        ),
        _ => return Err("target must be max_atk, amber or neuvillette".into()),
    };
    let character = Character::new(name, 90, false, 0, 8, 8, 8, &config, &HashSet::new(), false);
    let weapon = Weapon::new(
        weapon_name,
        90,
        false,
        1,
        &WeaponConfig::NoConfig,
        &character,
    );
    let attribute = SimpleAttribute::new(
        SimpleAttribute::new_with_base_edge(vec![CharacterStatus::new_single(0, name)]),
        0,
    );
    let strategies: Vec<(&str, Box<dyn SingleOptimizeAlgorithm>, bool, bool)> = vec![
        (
            "exact_without_reuse",
            Box::new(CutoffAlgorithmHeuristic {
                use_heuristic: false,
            }),
            false,
            false,
        ),
        (
            "exact_with_reuse",
            Box::new(CutoffAlgorithmHeuristic {
                use_heuristic: false,
            }),
            true,
            false,
        ),
        (
            "exact_with_score_cache",
            Box::new(CutoffAlgorithmHeuristic {
                use_heuristic: false,
            }),
            true,
            true,
        ),
        (
            "grouped_without_reuse",
            Box::new(CutoffAlgo2 {
                accuracy_factor: 1.0,
            }),
            false,
            false,
        ),
        (
            "grouped_with_reuse",
            Box::new(CutoffAlgo2 {
                accuracy_factor: 1.0,
            }),
            true,
            false,
        ),
        (
            "grouped_with_score_cache",
            Box::new(CutoffAlgo2 {
                accuracy_factor: 1.0,
            }),
            true,
            true,
        ),
        (
            "legacy_grouped",
            Box::new(legacy::cutoff_algo2::CutoffAlgo2 {
                accuracy_factor: 1.0,
            }),
            false,
            false,
        ),
        (
            "legacy_a_star",
            Box::new(legacy::cutoff_a_star::AStarCutoff),
            false,
            false,
        ),
        (
            "legacy_enumeration",
            Box::new(legacy::cutoff_heuristic::CutoffAlgorithmHeuristic {
                use_heuristic: false,
            }),
            false,
            false,
        ),
    ];
    let mut reference: Vec<([Option<u64>; 5], f64)> = Vec::new();
    let mut reports = Vec::new();
    for (label, algo, reuse, memoize) in strategies {
        let inner: Box<dyn TargetFunction> = match target_name.as_str() {
            "amber" => Box::new(AmberDefaultTargetFunction),
            "neuvillette" => Box::new(NeuvilletteDefaultTargetFunction),
            _ => Box::new(MaxATKTargetFunction),
        };
        let original_calls = Rc::new(Cell::new(0));
        let reused_calls = Rc::new(Cell::new(0));
        let target: Box<dyn TargetFunction> = Box::new(CountedTarget {
            inner,
            reuse,
            memoize,
            original_calls: original_calls.clone(),
            reused_calls: reused_calls.clone(),
        });
        let run = || {
            algo.optimize(
                &refs,
                None,
                &character,
                &weapon,
                &target,
                &Enemy::default(),
                &[],
                &attribute,
                &Default::default(),
                k,
            )
        };
        let _ = run(); // warm code / allocator before measuring
        let mut trials = Vec::new();
        let mut times = Vec::new();
        let mut all_match = true;
        for trial in 0..repeats {
            original_calls.set(0);
            reused_calls.set(0);
            let start = Instant::now();
            let result = run();
            let millis = start.elapsed().as_secs_f64() * 1000.0;
            if label == "exact_without_reuse" && trial == 0 {
                reference = result.iter().map(|r| (ids(r), r.value)).collect();
            }
            let reference_ids: Vec<_> = reference.iter().map(|row| row.0).collect();
            let actual_ids: Vec<_> = result.iter().map(ids).collect();
            let scores: HashMap<_, _> = reference.iter().copied().collect();
            let matching_ids = actual_ids
                .iter()
                .filter(|id| scores.contains_key(*id))
                .count();
            let exact = actual_ids == reference_ids
                && result
                    .iter()
                    .zip(&reference)
                    .all(|(a, b)| a.value.to_bits() == b.1.to_bits());
            let same_ranked_scores = result.len() == reference.len()
                && result
                    .iter()
                    .zip(&reference)
                    .all(|(a, b)| a.value.to_bits() == b.1.to_bits());
            all_match &= exact;
            times.push(millis);
            trials.push(serde_json::json!({"ms": millis, "returned": result.len(), "matching_top_k_ids": matching_ids, "exact_top_k_match": exact,
                "same_ranked_scores": same_ranked_scores,
                "difference": if exact { "none" } else if same_ranked_scores { "tie_ids" } else { "scores_or_count" },
                "best_score": result.first().map(|r| r.value), "original_target_calls": original_calls.get(), "reused_target_calls": reused_calls.get()}));
        }
        times.sort_by(f64::total_cmp);
        reports.push(serde_json::json!({"algorithm": label, "median_ms": times[times.len()/2], "all_trials_match_reference": all_match, "trials": trials}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &serde_json::json!({"target": target_name, "synthetic": input.is_none(), "slot_counts": slot_counts,
        "combinations": combinations, "k": k, "repeats": repeats, "copies": copies, "profile": profile, "reference": "exact_without_reuse", "reports": reports,
        "note": "Native synthetic timings are not browser or real-account speedups; legacy rank ties/bugs are preserved."})
        )?
    );
    Ok(())
}
