use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use crate::attribute::*;
use crate::common::{DamageResult, Element, SkillType};
use crate::common::reaction_type::TransformativeType;
use crate::damage::{DamageBuilderResult, SimpleDamageBuilder};
use crate::damage::damage_builder::DamageBuilder;
use crate::enemies::Enemy;
use crate::damage::level_coefficient::{LEVEL_MULTIPLIER, CRYSTALLIZE_BASE};

#[derive(Serialize, Deserialize)]
pub struct TransformativeDamage {
    pub swirl_cryo: DamageResult,
    pub swirl_hydro: DamageResult,
    pub swirl_pyro: DamageResult,
    pub swirl_electro: DamageResult,
    pub overload: DamageResult,
    pub electro_charged: DamageResult,
    pub shatter: DamageResult,
    pub superconduct: DamageResult,
    pub bloom: DamageResult,
    pub hyperbloom: DamageResult,
    pub burgeon: DamageResult,
    pub burning: DamageResult,
    pub crystallize: DamageResult,
}

#[inline]
fn get_reaction_coefficient(t: TransformativeType) -> f64 {
    t.get_reaction_coefficient()
}

#[inline]
pub fn get_transformative_base(level: usize, t: TransformativeType) -> f64 {
    t.get_reaction_base(level) * get_reaction_coefficient(t)
}

#[inline]
pub fn get_em_bonus(em: f64) -> f64 {
    16.0 * em / (em + 2000.0)
}

pub fn transformative_damage<D: DamageBuilder>(level: usize, attribute: &D::AttributeType, enemy: &Enemy) -> TransformativeDamage {
    let get_damage = | transformative_type: TransformativeType | -> DamageResult {
        let builder = D::new();

        builder.transformative(attribute, enemy, transformative_type, level).get_result()
    };
    
    TransformativeDamage {
        swirl_cryo: get_damage(TransformativeType::SwirlCryo),
        swirl_hydro: get_damage(TransformativeType::SwirlHydro),
        swirl_pyro: get_damage(TransformativeType::SwirlPyro),
        swirl_electro: get_damage(TransformativeType::SwirlElectro),
        overload: get_damage(TransformativeType::Overload),
        electro_charged: get_damage(TransformativeType::ElectroCharged),
        shatter: get_damage(TransformativeType::Shatter),
        superconduct: get_damage(TransformativeType::Superconduct),
        bloom: get_damage(TransformativeType::Bloom),
        hyperbloom: get_damage(TransformativeType::Hyperbloom),
        burgeon: get_damage(TransformativeType::Burgeon),
        burning: get_damage(TransformativeType::Burning),
        crystallize: get_damage(TransformativeType::Crystallize),
    }
}