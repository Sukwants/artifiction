use mona::artifacts::ArtifactList;
use mona::attribute::*;
use mona::character::team_status::CharacterStatus;
use mona::common::CharacterFullInfo;
use serde::{Serialize, Deserialize};
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use crate::applications::bonus_per_stat::bonus_per_stat::{BonusPerStatInput, BonusPerStatOutput};
use crate::applications::common::{BuffInterface, CharacterInterface, TargetFunctionInterface, WeaponInterface, EnemyInterface, CharacterFullInterface, CharactersInterface};
use mona::artifacts::Artifact;
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::target_functions::TargetFunction;
use mona::utils;
use crate::target_function::dsl_tf::TargetFunctionDSL;
use super::bonus_per_stat::bonus_per_stat;

#[derive(Serialize, Deserialize)]
pub struct WasmInput {
    pub characters: CharactersInterface,
    pub enemy: Option<EnemyInterface>,

    pub active_character_id: usize,
    pub tf: TargetFunctionInterface,
}

// #[wasm_bindgen]
// pub struct WasmOutput {
//     pub atk_ptr: *const f64,
//     pub atk_len: usize,
//     pub atk_percentage_ptr: *const f64,
//     pub atk_percentage_len: usize,
//     pub def_ptr: *const f64,
//     pub def_len: usize,
//     pub def_percentage_ptr: *const f64,
//     pub def_percentage_len: usize,
//     pub hp_ptr: *const f64,
//     pub hp_len: usize,
//     pub hp_percentage_ptr: *const f64,
//     pub hp_percentage_len: usize,
//     pub critical_ptr: *const f64,
//     pub critical_len: usize,
//     pub critical_damage_ptr: *const f64,
//     pub critical_damage_len: usize,
//     pub recharge_ptr: *const f64,
//     pub recharge_len: usize,
//     pub elemental_mastery_ptr: *const f64,
//     pub elemental_mastery_len: usize,
// }

pub struct BonusPerStat;

#[wasm_bindgen]
impl BonusPerStat {
    pub fn bonus_per_stat(val: JsValue) -> JsValue {
        utils::set_panic_hook();

        let input: WasmInput = serde_wasm_bindgen::from_value(val).unwrap();

        let characters = CharacterFullInterface::get_characters(&input.characters);

        let attribute = AttributeUtils::create_attribute_from_list_except_active_character(&characters, input.active_character_id);
        let active_character = CharacterFullInfo::get_character(&characters, input.active_character_id);

        let tf: Box<dyn TargetFunction> = if input.tf.use_dsl {
            Box::new(TargetFunctionDSL::new(&input.tf.dsl_source.unwrap()))
        } else {
            input.tf.to_target_function(&active_character.character, &active_character.weapon)
        };

        let enemy = if let Some(x) = &input.enemy {
            x.to_enemy()
        } else {
            Default::default()
        };

        let result = bonus_per_stat(BonusPerStatInput {
            character: &active_character.character,
            weapon: &active_character.weapon,
            artifacts: &active_character.artifacts,
            enemy: &enemy,
            tf: &tf,
            buffs: &active_character.buffs,
            artifacts_config: Some(&active_character.artifact_config),
            attribute: &attribute,
        });

        // utils::log!("{:?}", result.atk);
        // utils::log!("{:?}", result.atk.as_ptr());

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        result.serialize(&s).unwrap()

        // WasmOutput {
        //     atk_ptr: result.atk.as_ptr(),
        //     atk_len: result.atk.len(),
        //     atk_percentage_ptr: result.atk_percentage.as_ptr(),
        //     atk_percentage_len: result.atk_percentage.len(),
        //     def_ptr: result.def.as_ptr(),
        //     def_len: result.def.len(),
        //     def_percentage_ptr: result.def_percentage.as_ptr(),
        //     def_percentage_len: result.def_percentage.len(),
        //     hp_ptr: result.hp.as_ptr(),
        //     hp_len: result.hp.len(),
        //     hp_percentage_ptr: result.hp_percentage.as_ptr(),
        //     hp_percentage_len: result.hp_percentage.len(),
        //     critical_ptr: result.critical_rate.as_ptr(),
        //     critical_len: result.critical_rate.len(),
        //     critical_damage_ptr: result.critical_damage.as_ptr(),
        //     critical_damage_len: result.critical_damage.len(),
        //     recharge_ptr: result.recharge.as_ptr(),
        //     recharge_len: result.recharge.len(),
        //     elemental_mastery_ptr: result.elemental_mastery.as_ptr(),
        //     elemental_mastery_len: result.elemental_mastery.len()
        // }
    }
}
