use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::applications::common::{CharacterInterface, WeaponInterface};
use mona::artifacts::{Artifact, ArtifactList};
use mona::artifacts::eff::ARTIFACT_EFF5;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::*;
use mona::buffs::Buff;
use mona::character::Character;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::weapon::Weapon;

pub struct BonusPerStatInput<'a> {
    pub character: &'a Character<SimpleAttribute>,
    pub weapon: &'a Weapon<SimpleAttribute>,
    pub artifacts: &'a [&'a Artifact],
    pub enemy: &'a Enemy,
    pub tf: &'a Box<dyn TargetFunction>,
    pub buffs: &'a [Box<dyn Buff<SimpleAttribute>>],
    pub artifacts_config: Option<&'a ArtifactEffectConfig>,
}

const SIZE: usize = 10;
const MAX: usize = 10;
const INTERVAL: f64 = SIZE as f64 / MAX as f64;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct BonusPerStatOutput {
    pub atk: Vec<f64>,
    pub atk_percentage: Vec<f64>,
    pub def: Vec<f64>,
    pub def_percentage: Vec<f64>,
    pub hp: Vec<f64>,
    pub hp_percentage: Vec<f64>,
    pub critical_rate: Vec<f64>,
    pub critical_damage: Vec<f64>,
    pub recharge: Vec<f64>,
    pub elemental_mastery: Vec<f64>
}

fn get_atk(attribute: &mut SimpleAttribute, baseline: f64, artifacts: &Vec<&Artifact>, input: &BonusPerStatInput) -> Vec<f64> {
    attribute.set_value_by(AttributeName::ATKFixed, "temp", 10000.0);
    let test_value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
    attribute.set_value_by(AttributeName::ATKFixed, "temp", -10000.0);
    if (test_value - baseline).abs() < 1e-6 {
        return Vec::new();
    }

    let mut data = Vec::new();

    for i in 1..=SIZE {
        let x = i as f64 / INTERVAL;
        let add_value = ARTIFACT_EFF5.atk[3] * x;
        attribute.set_value_by(AttributeName::ATKFixed, "temp", add_value);
        let value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
        attribute.set_value_by(AttributeName::ATKFixed, "temp", -add_value);

        data.push((value - baseline) / baseline)
    }

    data
}

fn get_atk_p(attribute: &mut SimpleAttribute, baseline: f64, artifacts: &Vec<&Artifact>, input: &BonusPerStatInput) -> Vec<f64> {
    attribute.set_value_by(AttributeName::ATKFixed, "temp", 10000.0);
    let test_value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
    attribute.set_value_by(AttributeName::ATKFixed, "temp", -10000.0);
    if (test_value - baseline).abs() < 1e-6 {
        return Vec::new();
    }

    let mut data = Vec::new();

    for i in 1..=SIZE {
        let x = i as f64 / INTERVAL;
        let p = ARTIFACT_EFF5.atk_percentage[3] * x;

        attribute.add_atk_percentage("temp", p);
        let value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
        attribute.add_atk_percentage("temp", -p);

        data.push((value - baseline) / baseline)
    }

    data
}

fn get_def(attribute: &mut SimpleAttribute, baseline: f64, artifacts: &Vec<&Artifact>, input: &BonusPerStatInput) -> Vec<f64> {
    attribute.set_value_by(AttributeName::DEFFixed, "temp", 10000.0);
    let test_value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
    attribute.set_value_by(AttributeName::DEFFixed, "temp", -10000.0);
    if (test_value - baseline).abs() < 1e-6 {
        return Vec::new();
    }

    let mut data = Vec::new();

    for i in 1..=SIZE {
        let x = i as f64 / INTERVAL;
        let add_value = ARTIFACT_EFF5.def[3] * x;
        attribute.set_value_by(AttributeName::DEFFixed, "temp", add_value);
        let value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
        attribute.set_value_by(AttributeName::DEFFixed, "temp", -add_value);

        data.push((value - baseline) / baseline)
    }

    data
}

fn get_def_p(attribute: &mut SimpleAttribute, baseline: f64, artifacts: &Vec<&Artifact>, input: &BonusPerStatInput) -> Vec<f64> {
    attribute.set_value_by(AttributeName::DEFFixed, "temp", 10000.0);
    let test_value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
    attribute.set_value_by(AttributeName::DEFFixed, "temp", -10000.0);
    if (test_value - baseline).abs() < 1e-6 {
        return Vec::new();
    }

    let mut data = Vec::new();

    for i in 1..=SIZE {
        let x = i as f64 / INTERVAL;
        let p = ARTIFACT_EFF5.def_percentage[3] * x;

        attribute.add_def_percentage("temp", p);
        let value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
        attribute.add_def_percentage("temp", -p);

        data.push((value - baseline) / baseline)
    }

    data
}

fn get_hp(attribute: &mut SimpleAttribute, baseline: f64, artifacts: &Vec<&Artifact>, input: &BonusPerStatInput) -> Vec<f64> {
    attribute.set_value_by(AttributeName::HPFixed, "temp", 10000.0);
    let test_value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
    attribute.set_value_by(AttributeName::HPFixed, "temp", -10000.0);
    if (test_value - baseline).abs() < 1e-6 {
        return Vec::new();
    }

    let mut data = Vec::new();

    for i in 1..=SIZE {
        let x = i as f64 / INTERVAL;
        let add_value = ARTIFACT_EFF5.hp[3] * x;
        attribute.set_value_by(AttributeName::HPFixed, "temp", add_value);
        let value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
        attribute.set_value_by(AttributeName::HPFixed, "temp", -add_value);

        data.push((value - baseline) / baseline)
    }

    data
}

fn get_hp_p(attribute: &mut SimpleAttribute, baseline: f64, artifacts: &Vec<&Artifact>, input: &BonusPerStatInput) -> Vec<f64> {
    attribute.set_value_by(AttributeName::HPFixed, "temp", 10000.0);
    let test_value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
    attribute.set_value_by(AttributeName::HPFixed, "temp", -10000.0);
    if (test_value - baseline).abs() < 1e-6 {
        return Vec::new();
    }

    let mut data = Vec::new();

    for i in 1..=SIZE {
        let x = i as f64 / INTERVAL;
        let p = ARTIFACT_EFF5.hp_percentage[3] * x;

        attribute.add_hp_percentage("temp", p);
        let value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
        attribute.add_hp_percentage("temp", -p);

        data.push((value - baseline) / baseline)
    }

    data
}

fn get_critical_rate(attribute: &mut SimpleAttribute, baseline: f64, artifacts: &Vec<&Artifact>, input: &BonusPerStatInput) -> Vec<f64> {
    attribute.set_value_by(AttributeName::CriticalBase, "temp", 1.0);
    let test_value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
    attribute.set_value_by(AttributeName::CriticalBase, "temp", -1.0);
    if (test_value - baseline).abs() < 1e-6 {
        return Vec::new();
    }

    let mut data = Vec::new();
    for i in 1..=SIZE {
        let x = i as f64 / INTERVAL;
        let p = ARTIFACT_EFF5.critical_rate[3] * x;
        attribute.set_value_by(AttributeName::CriticalBase, "temp", p);
        let value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
        attribute.set_value_by(AttributeName::CriticalBase, "temp", -p);
        data.push((value - baseline) / baseline)
    }

    data
}

fn get_critical_damage(attribute: &mut SimpleAttribute, baseline: f64, artifacts: &Vec<&Artifact>, input: &BonusPerStatInput) -> Vec<f64> {
    attribute.set_value_by(AttributeName::CriticalDamageBase, "temp", 1.0);
    let test_value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
    attribute.set_value_by(AttributeName::CriticalDamageBase, "temp", -1.0);
    if (test_value - baseline).abs() < 1e-6 {
        return Vec::new();
    }

    let mut data = Vec::new();
    for i in 1..=SIZE {
        let x = i as f64 / INTERVAL;
        let p = ARTIFACT_EFF5.critical_damage[3] * x;
        attribute.set_value_by(AttributeName::CriticalDamageBase, "temp", p);
        let value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
        attribute.set_value_by(AttributeName::CriticalDamageBase, "temp", -p);
        data.push((value - baseline) / baseline)
    }

    data
}

fn get_recharge(attribute: &mut SimpleAttribute, baseline: f64, artifacts: &Vec<&Artifact>, input: &BonusPerStatInput) -> Vec<f64> {
    attribute.set_value_by(AttributeName::Recharge, "temp", 1.0);
    let test_value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
    attribute.set_value_by(AttributeName::Recharge, "temp", -1.0);
    if (test_value - baseline).abs() < 1e-6 {
        return Vec::new();
    }

    let mut data = Vec::new();
    for i in 1..=SIZE {
        let x = i as f64 / INTERVAL;
        let p = ARTIFACT_EFF5.recharge[3] * x;
        attribute.set_value_by(AttributeName::Recharge, "temp", p);
        let value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
        attribute.set_value_by(AttributeName::Recharge, "temp", -p);
        data.push((value - baseline) / baseline)
    }

    data
}

fn get_em(attribute: &mut SimpleAttribute, baseline: f64, artifacts: &Vec<&Artifact>, input: &BonusPerStatInput) -> Vec<f64> {
    attribute.set_value_by(AttributeName::ElementalMastery, "temp", 1000.0);
    let test_value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
    attribute.set_value_by(AttributeName::ElementalMastery, "temp", -1000.0);
    if (test_value - baseline).abs() < 1e-6 {
        return Vec::new();
    }

    let mut data = Vec::new();
    for i in 1..=SIZE {
        let x = i as f64 / INTERVAL;
        let p = ARTIFACT_EFF5.elemental_mastery[3] * x;
        attribute.set_value_by(AttributeName::ElementalMastery, "temp", p);
        let value = input.tf.target(&attribute.solve(), &input.character, &input.weapon, artifacts, &input.enemy);
        attribute.set_value_by(AttributeName::ElementalMastery, "temp", -p);
        data.push((value - baseline) / baseline)
    }

    data
}

pub fn bonus_per_stat(input: BonusPerStatInput) -> BonusPerStatOutput {
    let artifact_vec: Vec<&Artifact> = input.artifacts.iter().cloned().collect();
    let artifact_list = ArtifactList {
        artifacts: &artifact_vec
    };

    let default_artifact_config = if input.artifacts_config.is_none() {
        Some(input.tf.get_default_artifact_config(&Default::default()))
    } else {
        None
    };
    let artifact_config = if let Some(x) = input.artifacts_config {
        x
    } else {
        default_artifact_config.as_ref().unwrap()
    };

    let mut attribute = AttributeUtils::create_attribute_from_big_config_mut(
        &artifact_list,
        &artifact_config,
        &input.character,
        &input.weapon,
        &input.buffs
    );

    let value_baseline = input.tf.target(&attribute.solve(), &input.character, &input.weapon, &artifact_vec, &input.enemy);

    BonusPerStatOutput {
        atk: get_atk(&mut attribute, value_baseline, &artifact_vec, &input),
        atk_percentage: get_atk_p(&mut attribute, value_baseline, &artifact_vec, &input),
        def: get_def(&mut attribute, value_baseline, &artifact_vec, &input),
        def_percentage: get_def_p(&mut attribute, value_baseline, &artifact_vec, &input),
        hp: get_hp(&mut attribute, value_baseline, &artifact_vec, &input),
        hp_percentage: get_hp_p(&mut attribute, value_baseline, &artifact_vec, &input),
        critical_rate: get_critical_rate(&mut attribute, value_baseline, &artifact_vec, &input),
        critical_damage: get_critical_damage(&mut attribute, value_baseline, &artifact_vec, &input),
        recharge: get_recharge(&mut attribute, value_baseline, &artifact_vec, &input),
        elemental_mastery: get_em(&mut attribute, value_baseline, &artifact_vec, &input)
    }
}
