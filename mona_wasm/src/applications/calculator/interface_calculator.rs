use std::collections::HashMap;
use mona::attribute::attribute::ComplicatedAttribute;
use serde_json::json;

use mona::artifacts::{Artifact, ArtifactList};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::*;
use mona::buffs::{Buff, BuffConfig};
use mona::character::{Character, CharacterName, characters};
use mona::character::characters::damage;
use mona::character::skill_config::CharacterSkillConfig;
use mona::character::team_status::CharacterStatus;
use mona::character::traits::CharacterTrait;
use mona::common::{Element, MoonglareReaction, TransformativeType, CharacterFullInfo};
use mona::damage::{ComplicatedDamageBuilder, DamageAnalysis, DamageContext, DamageResult, SimpleDamageBuilder};
use mona::damage::damage_analysis::{EventAnalysis, TransformativeDamageAnalysisForAll, MoonglareDamageAnalysisForAll};
use mona::damage::damage_builder::DamageBuilder;
use mona::damage::damage_result::SimpleDamageResult;
use mona::enemies::Enemy;
use mona::target_functions::TargetFunction;
use mona::team::TeamQuantization;
use mona::utils;
use mona::weapon::Weapon;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

use crate::applications::common::{BuffInterface, CharacterFullInterface, CharacterInterface, CharactersInterface, EnemyInterface, SkillInterface, TargetFunctionInterface, WeaponInterface};

pub struct CalculatorInterface;

#[derive(Serialize, Deserialize)]
pub struct CalculatorConfigInterface {
    pub characters: CharactersInterface,
    pub enemy: Option<EnemyInterface>,

    pub active_character_id: usize,
}

// #[derive(Serialize, Deserialize)]
// pub struct DamageWithoutAttributeInterface {
//     pub atk: f64,
//     pub atk_ratio: f64,
//     pub def: f64,
//     pub def_ratio: f64,
//     pub hp: f64,
//     pub hp_ratio: f64,
//     pub extra_damage: f64,
//     pub bonus: f64,
//     pub critical: f64,
//     pub critical_damage: f64,
//     pub melt_enhance: f64,
//     pub vaporize_enhance: f64,
//
//     pub def_minus: f64,
//     pub def_penetration: f64,
//     pub res_minus: f64,
// }

// #[wasm_bindgen]
// pub struct TransformativeDamageOutput {
//     pub overload: f64,
//     pub electro_charged: f64,
//     pub swirl_pyro: f64,
//     pub swirl_electro: f64,
//     pub swirl_cryo: f64,
//     pub swirl_hydro: f64
// }

#[wasm_bindgen]
impl CalculatorInterface {
    pub fn get_damage_analysis(value: JsValue, fumo: JsValue) -> JsValue {
        utils::set_panic_hook();
        // utils::log!("start");

        let input: CalculatorConfigInterface = serde_wasm_bindgen::from_value(value).unwrap();
        let fumo: Option<Element> = serde_wasm_bindgen::from_value(fumo).unwrap();

        let characters = CharacterFullInterface::get_characters(&input.characters);

        let enemy = if let Some(x) = &input.enemy {
            x.to_enemy()
        } else {
            Default::default()
        };

        let result = CalculatorInterface::get_damage_analysis_internal(
            &characters,
            &enemy,
            fumo,
            input.active_character_id,
        );

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        result.serialize(&s).unwrap()
    }

    pub fn get_transformative_damage(value: JsValue) -> JsValue {
        utils::set_panic_hook();

        let input: CalculatorConfigInterface = serde_wasm_bindgen::from_value(value).unwrap();

        let characters = CharacterFullInterface::get_characters(&input.characters);

        let enemy = if let Some(x) = &input.enemy {
            x.to_enemy()
        } else {
            Default::default()
        };

        let result = CalculatorInterface::get_damage_transformative_internal(
            &characters,
            &enemy,
            input.active_character_id,
        );

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        result.serialize(&s).unwrap()
    }

    pub fn get_moonglare_damage(value: JsValue) -> JsValue {
        utils::set_panic_hook();

        let input: CalculatorConfigInterface = serde_wasm_bindgen::from_value(value).unwrap();

        let characters = CharacterFullInterface::get_characters(&input.characters);

        let enemy = if let Some(x) = &input.enemy {
            x.to_enemy()
        } else {
            Default::default()
        };

        let result = CalculatorInterface::get_damage_moonglare_internal(
            &characters,
            &enemy,
            input.active_character_id,
        );

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        result.serialize(&s).unwrap()
    }
}

impl CalculatorInterface {
    pub fn get_damage_analysis_internal(
        characters: &Vec<CharacterFullInfo<ComplicatedAttribute>>,
        enemy: &Enemy,
        fumo: Option<Element>,
        active_character_id: usize,
    ) -> EventAnalysis {

        let attribute = AttributeUtils::create_attribute_from_list(&characters, active_character_id);
        let active_character = CharacterFullInfo::get_character(&characters, active_character_id);

        let context = DamageContext {
            character_common_data: &active_character.character.common_data,
            attribute: &attribute.solve(),
            enemy: &enemy
        };

        let damage = damage::<ComplicatedDamageBuilder>(&context, active_character.skill_index, &active_character.skill_config, fumo);
        damage
    }

    pub fn get_damage_transformative_internal(
        characters: &Vec<CharacterFullInfo<ComplicatedAttribute>>,
        enemy: &Enemy,
        active_character_id: usize,
    ) -> TransformativeDamageAnalysisForAll {

        let attribute = AttributeUtils::create_attribute_from_list(&characters, active_character_id);
        let active_character = CharacterFullInfo::get_character(&characters, active_character_id);

        let context = DamageContext {
            character_common_data: &active_character.character.common_data,
            attribute: &attribute.solve(),
            enemy: &enemy
        };

        let builder = ComplicatedDamageBuilder::new();

        let get_damage = |transformative_type: TransformativeType| -> EventAnalysis {
            builder.transformative(
                &context.attribute,
                &context.enemy,
                transformative_type,
                context.character_common_data.level,
            )
        };

        let result = TransformativeDamageAnalysisForAll {
            swirl_cryo: get_damage(TransformativeType::SwirlCryo),
            swirl_pyro: get_damage(TransformativeType::SwirlPyro),
            swirl_hydro: get_damage(TransformativeType::SwirlHydro),
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
        };

        result
    }

    pub fn get_damage_moonglare_internal(
        characters: &Vec<CharacterFullInfo<ComplicatedAttribute>>,
        enemy: &Enemy,
        active_character_id: usize,
    ) -> MoonglareDamageAnalysisForAll {

        let attribute = AttributeUtils::create_attribute_from_list(&characters, active_character_id);
        let active_character = CharacterFullInfo::get_character(&characters, active_character_id);

        let context = DamageContext {
            character_common_data: &active_character.character.common_data,
            attribute: &attribute.solve(),
            enemy: &enemy
        };

        let builder = ComplicatedDamageBuilder::new();

        let get_damage = |lunar_type: MoonglareReaction| -> EventAnalysis {
            builder.moonglare(
                &context.attribute,
                &context.enemy,
                lunar_type.get_element().unwrap(),
                lunar_type,
                mona::common::SkillType::Moonglare,
                context.character_common_data.level,
                None,
            )
        };

        let result = MoonglareDamageAnalysisForAll {
            lunar_charged_reaction: get_damage(MoonglareReaction::LunarChargedReaction),
            lunar_crystallize_reaction: get_damage(MoonglareReaction::LunarCrystallizeReaction),
        };

        result
    }
}