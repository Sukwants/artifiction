use crate::damage::transformative_damage::TransformativeDamage;
use serde::{Deserialize, Serialize};
use std::ops::Mul;
use wasm_bindgen::prelude::*;

pub trait DamageBuilderResult {
    fn get_result(&self) -> DamageResult;
}

pub(crate) fn default_damage_count() -> f64 {
    1.0
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
#[derive(Deserialize, Serialize)]
pub struct DamageResult {
    pub critical: f64,
    pub non_critical: f64,
    pub expectation: f64,
}

impl Mul<f64> for DamageResult {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        DamageResult {
            critical: self.critical * rhs,
            non_critical: self.non_critical * rhs,
            expectation: self.expectation * rhs,
        }
    }
}

impl Default for DamageResult {
    fn default() -> Self {
        DamageResult {
            critical: 0.0,
            non_critical: 0.0,
            expectation: 0.0,
        }
    }
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SimpleDamageResult {
    #[serde(default = "default_damage_count")]
    pub damage_count: f64,
    pub normal: DamageResult,
    pub melt: Option<DamageResult>,
    pub vaporize: Option<DamageResult>,
    pub spread: Option<DamageResult>,
    pub aggravate: Option<DamageResult>,
}

impl SimpleDamageResult {
    pub fn new_normal(normal: DamageResult) -> Self {
        SimpleDamageResult {
            damage_count: 1.0,
            normal,
            melt: None,
            vaporize: None,
            spread: None,
            aggravate: None,
        }
    }

    pub fn new(normal: DamageResult,
        melt: Option<DamageResult>,
        vaporize: Option<DamageResult>,
        spread: Option<DamageResult>,
        aggravate: Option<DamageResult>
    ) -> Self {
        SimpleDamageResult { damage_count: 1.0, normal, melt, vaporize, spread, aggravate }
    }

    // 由 builder 在单次伤害计算完成后调用一次，结果数值保持为总伤害。
    pub(crate) fn with_damage_count(mut self, count: f64) -> Self {
        self.damage_count = count;
        self.normal = self.normal * count;
        for result in [&mut self.melt, &mut self.vaporize, &mut self.spread, &mut self.aggravate] {
            if let Some(damage) = result {
                *damage = *damage * count;
            }
        }
        self
    }
}

impl DamageBuilderResult for SimpleDamageResult {
    fn get_result(&self) -> DamageResult {
        self.normal
    }
}
