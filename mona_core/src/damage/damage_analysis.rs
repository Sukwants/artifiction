use std::{collections::HashMap, hash::Hash};
use serde::{Serialize, Deserialize};
use crate::common::{DamageResult, Element, MoonglareReaction, ReactionType, TransformativeType};
use crate::damage::damage_result::DamageBuilderResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageAnalysis {
    pub element: Element,
    pub reaction_type: Option<ReactionType>,
    
    pub atk: HashMap<String, f64>,
    pub atk_ratio: HashMap<String, f64>,
    pub hp: HashMap<String, f64>,
    pub hp_ratio: HashMap<String, f64>,
    pub def: HashMap<String, f64>,
    pub def_ratio: HashMap<String, f64>,
    pub em: HashMap<String, f64>,
    pub em_ratio: HashMap<String, f64>,
    pub reaction_base: f64,
    pub reaction_coefficient: f64,

    pub base_damage: HashMap<String, f64>,
    pub bonus: HashMap<String, f64>,
    pub reaction_enhance: HashMap<String, f64>,
    pub critical_rate: HashMap<String, f64>,
    pub critical_damage: HashMap<String, f64>,
    pub res_minus: HashMap<String, f64>,
    pub def_minus: HashMap<String, f64>,
    pub def_penetration: HashMap<String, f64>,

    pub result: DamageResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransformativeDamageAnalysis {
    pub element: Option<Element>,
    pub transformative_type: TransformativeType,

    pub reaction_base: f64,
    pub reaction_coefficient: f64,

    pub reaction_enhance: HashMap<String, f64>,
    pub reaction_extra: HashMap<String, f64>,
    pub critical_rate: HashMap<String, f64>,
    pub critical_damage: HashMap<String, f64>,
    pub res_minus: HashMap<String, f64>,

    pub result: DamageResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoonglareDamageAnalysis {
    pub element: Element,
    pub lunar_type: MoonglareReaction,
    
    pub atk: HashMap<String, f64>,
    pub atk_ratio: HashMap<String, f64>,
    pub hp: HashMap<String, f64>,
    pub hp_ratio: HashMap<String, f64>,
    pub def: HashMap<String, f64>,
    pub def_ratio: HashMap<String, f64>,
    pub em: HashMap<String, f64>,
    pub em_ratio: HashMap<String, f64>,
    pub reaction_base: f64,
    pub reaction_coefficient: f64,

    pub reaction_enhance: HashMap<String, f64>,
    pub reaction_extra: HashMap<String, f64>,
    pub critical_rate: HashMap<String, f64>,
    pub critical_damage: HashMap<String, f64>,
    pub res_minus: HashMap<String, f64>,
    pub moonglare_base: HashMap<String, f64>,
    pub moonglare_elevate: HashMap<String, f64>,

    pub result: DamageResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealAnalysis {
    pub atk: HashMap<String, f64>,
    pub atk_ratio: HashMap<String, f64>,
    pub hp: HashMap<String, f64>,
    pub hp_ratio: HashMap<String, f64>,
    pub def: HashMap<String, f64>,
    pub def_ratio: HashMap<String, f64>,
    pub em: HashMap<String, f64>,
    pub em_ratio: HashMap<String, f64>,
    pub base: HashMap<String, f64>,

    pub healing_bonus: HashMap<String, f64>,
    pub incoming_healing_bonus: HashMap<String, f64>,
    pub critical_rate: HashMap<String, f64>,
    pub critical_damage: HashMap<String, f64>,

    pub result: DamageResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShieldAnalysis {
    pub element: Element,
    
    pub atk: HashMap<String, f64>,
    pub atk_ratio: HashMap<String, f64>,
    pub hp: HashMap<String, f64>,
    pub hp_ratio: HashMap<String, f64>,
    pub def: HashMap<String, f64>,
    pub def_ratio: HashMap<String, f64>,
    pub em: HashMap<String, f64>,
    pub em_ratio: HashMap<String, f64>,
    pub base: HashMap<String, f64>,

    pub shield_strength: HashMap<String, f64>,

    pub result: DamageResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DamageAnalysisWithPossibleReaction {
    pub normal: DamageAnalysis,
    pub melt: Option<DamageAnalysis>,
    pub vaporize: Option<DamageAnalysis>,
    pub spread: Option<DamageAnalysis>,
    pub aggravate: Option<DamageAnalysis>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventAnalysis {
    Damage(DamageAnalysisWithPossibleReaction),
    TransformativeDamage(TransformativeDamageAnalysis),
    MoonglareDamage(MoonglareDamageAnalysis),
    Heal(HealAnalysis),
    Shield(ShieldAnalysis),
    None,
}

impl DamageBuilderResult for EventAnalysis {
    fn get_result(&self) -> DamageResult {
        match self {
            EventAnalysis::Damage(d) => d.normal.result,
            EventAnalysis::TransformativeDamage(d) => d.result,
            EventAnalysis::MoonglareDamage(d) => d.result,
            EventAnalysis::Heal(d) => d.result,
            EventAnalysis::Shield(d) => d.result,
            EventAnalysis::None => DamageResult::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransformativeDamageAnalysisForAll {
    pub swirl_cryo: EventAnalysis,
    pub swirl_pyro: EventAnalysis,
    pub swirl_hydro: EventAnalysis,
    pub swirl_electro: EventAnalysis,
    pub overload: EventAnalysis,
    pub electro_charged: EventAnalysis,
    pub shatter: EventAnalysis,
    pub superconduct: EventAnalysis,
    pub bloom: EventAnalysis,
    pub hyperbloom: EventAnalysis,
    pub burgeon: EventAnalysis,
    pub burning: EventAnalysis,
    pub crystallize: EventAnalysis,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MoonglareDamageAnalysisForAll {
    pub lunar_charged_reaction: EventAnalysis,
}