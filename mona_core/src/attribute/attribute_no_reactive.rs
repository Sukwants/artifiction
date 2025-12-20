use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::attribute::*;


type AttributeEntryType = HashMap<String, f64>;

#[derive(Serialize, Deserialize)]
#[derive(Default, Debug)]
pub struct AttributeNoReactive {
    pub atk: AttributeEntryType,
    pub def: AttributeEntryType,
    pub hp: AttributeEntryType,

    pub healing_bonus: AttributeEntryType,
    pub elemental_mastery: AttributeEntryType,
    pub recharge: AttributeEntryType,
    pub shield_strength: AttributeEntryType,

    pub critical: AttributeEntryType,
    pub critical_damage: AttributeEntryType,

    pub bonus_electro: AttributeEntryType,
    pub bonus_pyro: AttributeEntryType,
    pub bonus_anemo: AttributeEntryType,
    pub bonus_cryo: AttributeEntryType,
    pub bonus_geo: AttributeEntryType,
    pub bonus_hydro: AttributeEntryType,
    pub bonus_dendro: AttributeEntryType,
    pub bonus_physical: AttributeEntryType,
}

// fn merge(x: &mut AttributeEntryType, y: &AttributeEntryType) {
//     for (key, value) in y.iter() {
//         *x.entry(key.clone()).or_insert(0.0) += value;
//     }
// }

impl AttributeNoReactive {
    pub fn new() -> AttributeNoReactive {
        Default::default()
    }
}

impl From<&AttributeResultWithCharacter<ComplicatedAttributeGraphResult>> for AttributeNoReactive {
    fn from(attribute_result: &AttributeResultWithCharacter<ComplicatedAttributeGraphResult>) -> Self {
        let mut attribute = AttributeNoReactive::new();

        attribute.atk = attribute_result.get_result_merge(&vec![
            AttributeName::ATK,
        ]).0;

        attribute.def = attribute_result.get_result_merge(&vec![
            AttributeName::DEF,
        ]).0;

        attribute.hp = attribute_result.get_result_merge(&vec![
            AttributeName::HP,
        ]).0;

        attribute.healing_bonus = attribute_result.get_result(AttributeName::HealingBonus).0;
        attribute.elemental_mastery = attribute_result.get_result_merge(&vec![
            AttributeName::ElementalMastery,
            AttributeName::ElementalMasteryExtra
        ]).0;
        attribute.recharge = attribute_result.get_result_merge(&vec![
            AttributeName::Recharge,
            AttributeName::RechargeExtra
        ]).0;
        attribute.shield_strength = attribute_result.get_result(AttributeName::ShieldStrength).0;
        attribute.critical = attribute_result.get_result(AttributeName::CriticalBase).0;
        attribute.critical_damage = attribute_result.get_result(AttributeName::CriticalDamageBase).0;

        attribute.bonus_electro = attribute_result.get_result(AttributeName::BonusElectro).0;
        attribute.bonus_pyro = attribute_result.get_result(AttributeName::BonusPyro).0;
        attribute.bonus_anemo = attribute_result.get_result(AttributeName::BonusAnemo).0;
        attribute.bonus_cryo = attribute_result.get_result(AttributeName::BonusCryo).0;
        attribute.bonus_hydro = attribute_result.get_result(AttributeName::BonusHydro).0;
        attribute.bonus_geo = attribute_result.get_result(AttributeName::BonusGeo).0;
        attribute.bonus_dendro = attribute_result.get_result(AttributeName::BonusDendro).0;
        attribute.bonus_physical = attribute_result.get_result(AttributeName::BonusPhysical).0;

        // todo other attributes

        attribute
    }
}