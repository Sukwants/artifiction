use super::damage_analysis::EventAnalysis;
use super::damage_builder::DamageBuilder;
use super::*;
use crate::attribute::*;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::electro::mitya::{Mitya, MityaDamageEnum};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::team_status::{CharacterStatus, CharacterTags};
use crate::character::traits::CharacterTrait;
use crate::character::{CharacterConfig, CharacterName};
use crate::common::{Element, ElevativeReaction, SkillType, TransformativeType};
use crate::enemies::Enemy;

fn attribute<A: Attribute>() -> A {
    let mut a = A::new(
        A::new_with_base_edge(vec![CharacterStatus::new_single(0, CharacterName::Mitya)]),
        0,
    );
    a.set_value_to(AttributeName::ATKBase, "test", 1000.0);
    a.set_value_to(AttributeName::ElementalMastery, "test", 500.0);
    a
}

fn builder<D: DamageBuilder>(count: Option<f64>) -> D {
    let mut builder = D::new();
    builder.add_atk_ratio("test", 1.2);
    builder.add_em_ratio("test", 0.9);
    builder.add_extra_reaction_extra("test", 1000.0);
    if let Some(count) = count {
        builder.set_damage_count(count);
    }
    builder
}

fn calculate<D: DamageBuilder>(
    builder: &D,
    attribute: &D::AttributeType,
    kind: usize,
) -> D::Result {
    let enemy = Enemy::default();
    match kind {
        0..=2 => builder.damage(
            attribute,
            &enemy,
            [Element::Pyro, Element::Dendro, Element::Electro][kind],
            SkillType::ChargedAttack,
            90,
            None,
        ),
        3 => builder.transformative(attribute, &enemy, TransformativeType::ElectroCharged, 90),
        4 => builder.elevative(
            attribute,
            &enemy,
            Element::Electro,
            ElevativeReaction::StellarConductElectro,
            SkillType::Elevative,
            90,
            None,
        ),
        5 => builder.heal(attribute),
        6 => builder.shield(attribute, Element::Electro),
        7 => builder.number(attribute),
        _ => builder.none(),
    }
}

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9 * expected.abs().max(1.0),
        "{actual} != {expected}"
    );
}

fn close_result(actual: DamageResult, expected: DamageResult) {
    close(actual.critical, expected.critical);
    close(actual.non_critical, expected.non_critical);
    close(actual.expectation, expected.expectation);
}

fn check_analysis(single: &serde_json::Value, total: &serde_json::Value, count: f64) {
    if single.get("result").is_some() {
        close(total["damage_count"].as_f64().unwrap(), count);
        for key in ["critical", "non_critical", "expectation"] {
            close(
                total["result"][key].as_f64().unwrap(),
                single["result"][key].as_f64().unwrap() * count,
            );
        }
        for (key, value) in single.as_object().unwrap() {
            if key != "result" && key != "damage_count" {
                assert_eq!(value, &total[key], "single-hit source changed: {key}");
            }
        }
    } else if let Some(object) = single.as_object() {
        for (key, value) in object {
            check_analysis(value, &total[key], count);
        }
    }
}

#[test]
fn count_multiplies_all_damage_variants_but_keeps_single_hit_sources() {
    let simple_attribute = attribute::<SimpleAttribute>().solve();
    let detailed_attribute = attribute::<ComplicatedAttribute>().solve();
    for kind in 0..=4 {
        let single = calculate(
            &builder::<SimpleDamageBuilder>(None),
            &simple_attribute,
            kind,
        );
        let detailed = calculate(
            &ComplicatedDamageBuilder::default(),
            &detailed_attribute,
            kind,
        );
        // Default 构造器的次数也必须为 1，不能因为数值字段默认值为 0 而丢失伤害。
        let detailed_json = serde_json::to_value(&detailed).unwrap();
        check_analysis(&detailed_json, &detailed_json, 1.0);
        let detailed = calculate(
            &builder::<ComplicatedDamageBuilder>(None),
            &detailed_attribute,
            kind,
        );
        for count in [0.0, 0.5, 1.0, 2.5, 6.0 / 0.66] {
            let total = calculate(
                &builder::<SimpleDamageBuilder>(Some(count)),
                &simple_attribute,
                kind,
            );
            close(total.damage_count, count);
            close_result(total.normal, single.normal * count);
            for (single, total) in [
                (single.melt, total.melt),
                (single.vaporize, total.vaporize),
                (single.spread, total.spread),
                (single.aggravate, total.aggravate),
            ] {
                match (single, total) {
                    (Some(single), Some(total)) => close_result(total, single * count),
                    (None, None) => {}
                    _ => panic!("reaction variant changed"),
                }
            }
            let analysis = calculate(
                &builder::<ComplicatedDamageBuilder>(Some(count)),
                &detailed_attribute,
                kind,
            );
            close_result(analysis.get_result(), total.normal);
            check_analysis(
                &serde_json::to_value(&detailed).unwrap(),
                &serde_json::to_value(&analysis).unwrap(),
                count,
            );
        }
    }
}

#[test]
fn count_is_a_setter_and_does_not_change_non_damage_outputs() {
    let simple_attribute = attribute::<SimpleAttribute>().solve();
    let detailed_attribute = attribute::<ComplicatedAttribute>().solve();
    let mut b = builder::<SimpleDamageBuilder>(Some(2.0));
    b.set_damage_count(3.0);
    close(calculate(&b, &simple_attribute, 0).damage_count, 3.0);
    for kind in 5..=8 {
        close_result(
            calculate(
                &builder::<SimpleDamageBuilder>(None),
                &simple_attribute,
                kind,
            )
            .normal,
            calculate(
                &builder::<SimpleDamageBuilder>(Some(3.0)),
                &simple_attribute,
                kind,
            )
            .normal,
        );
        close_result(
            calculate(
                &builder::<ComplicatedDamageBuilder>(None),
                &detailed_attribute,
                kind,
            )
            .get_result(),
            calculate(
                &builder::<ComplicatedDamageBuilder>(Some(3.0)),
                &detailed_attribute,
                kind,
            )
            .get_result(),
        );
    }
}

#[test]
fn mitya_total_exposes_fractional_count_without_scaling_skill_ratio() {
    let simple = attribute::<SimpleAttribute>().solve();
    let detailed = attribute::<ComplicatedAttribute>().solve();
    let enemy = Enemy::default();
    for (constellation, carried, extra, count) in [
        (0, 4, false, 4.0),
        (0, 99, true, 5.0),
        (6, 5, false, 5.0 / 0.66),
        (6, 99, true, 6.0 / 0.66),
    ] {
        let common = CharacterCommonData::new(
            CharacterName::Mitya,
            90,
            false,
            constellation,
            0,
            0,
            0,
            &CharacterConfig::NoConfig,
            &CharacterTags::new(),
            true,
        );
        let config = CharacterSkillConfig::Mitya {
            core_state: 2,
            beacon_count: carried,
            talent2_crit_stack: 4,
            beacon_generated_during_charge: extra,
        };
        let context = DamageContext {
            character_common_data: &common,
            attribute: &detailed,
            enemy: &enemy,
        };
        let single = Mitya::damage::<ComplicatedDamageBuilder>(
            &context,
            MityaDamageEnum::ZSC,
            &config,
            None,
        );
        let total = Mitya::damage::<ComplicatedDamageBuilder>(
            &context,
            MityaDamageEnum::ZSC_TOTAL,
            &config,
            None,
        );
        check_analysis(
            &serde_json::to_value(single).unwrap(),
            &serde_json::to_value(&total).unwrap(),
            count,
        );
        let total_simple = Mitya::damage::<SimpleDamageBuilder>(
            &DamageContext {
                character_common_data: &common,
                attribute: &simple,
                enemy: &enemy,
            },
            MityaDamageEnum::ZSC_TOTAL,
            &config,
            None,
        );
        close(total_simple.damage_count, count);
        close_result(total_simple.normal, total.get_result());
    }
}

#[test]
fn legacy_analysis_without_count_deserializes_as_one() {
    let attribute = attribute::<ComplicatedAttribute>().solve();
    let mut json = serde_json::to_value(calculate(
        &builder::<ComplicatedDamageBuilder>(None),
        &attribute,
        4,
    ))
    .unwrap();
    json["ElevativeDamage"]
        .as_object_mut()
        .unwrap()
        .remove("damage_count");
    let analysis: EventAnalysis = serde_json::from_value(json).unwrap();
    match analysis {
        EventAnalysis::ElevativeDamage(d) => close(d.damage_count, 1.0),
        _ => panic!("wrong analysis variant"),
    }
}
