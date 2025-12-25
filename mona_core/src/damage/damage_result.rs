use crate::common::MoonglareReaction;
use crate::damage::transformative_damage::TransformativeDamage;
use serde::{Deserialize, Serialize};
use std::ops::Mul;
use wasm_bindgen::prelude::*;

pub trait DamageBuilderResult {
    fn get_result(&self) -> DamageResult;
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
    pub normal: DamageResult,
    pub melt: Option<DamageResult>,
    pub vaporize: Option<DamageResult>,
    pub spread: Option<DamageResult>,
    pub aggravate: Option<DamageResult>,
}

impl SimpleDamageResult {
    pub fn new_normal(normal: DamageResult) -> Self {
        SimpleDamageResult {
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
        SimpleDamageResult { normal, melt, vaporize, spread, aggravate }
    }
}

impl DamageBuilderResult for SimpleDamageResult {
    fn get_result(&self) -> DamageResult {
        self.normal
    }
}