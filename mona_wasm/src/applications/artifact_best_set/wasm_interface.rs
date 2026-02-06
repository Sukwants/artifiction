use mona::artifacts::ArtifactList;
use mona::attribute::*;
use mona::character::team_status::CharacterStatus;
use mona::common::CharacterFullInfo;
use mona::target_functions::TargetFunction;
use serde::Serialize;
use wasm_bindgen::JsValue;
use crate::applications::artifact_best_set::artifact_best_set::calc_artifact_best_set;
use crate::applications::artifact_best_set::type_interface::CalcArtifactBestSetInterface;
use crate::applications::common::CharacterFullInterface;
use crate::target_function::dsl_tf::TargetFunctionDSL;
use crate::utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use crate::utils;

pub struct CalcArtifactBestSet;

#[wasm_bindgen]
impl CalcArtifactBestSet {
    pub fn calc_artifact_best_set(args: JsValue) -> JsValue {
        set_panic_hook();

        let calc_best_set_interface: CalcArtifactBestSetInterface = serde_wasm_bindgen::from_value(args).unwrap();

        let characters = CharacterFullInterface::get_characters(&calc_best_set_interface.characters);

        let attribute = AttributeUtils::create_attribute_from_list_except_active_character(&characters, calc_best_set_interface.active_character_id);
        let active_character = CharacterFullInfo::get_character(&characters, calc_best_set_interface.active_character_id);

        let target_function: Box<dyn TargetFunction> = if calc_best_set_interface.target_function.use_dsl {
            Box::new(TargetFunctionDSL::new(&calc_best_set_interface.target_function.dsl_source.unwrap()))
        } else {
            calc_best_set_interface.target_function.to_target_function(&active_character.character, &active_character.weapon)
        };

        let enemy = if let Some(x) = &calc_best_set_interface.enemy {
            x.to_enemy()
        } else {
            Default::default()
        };

        let mut result = calc_artifact_best_set(
            &active_character.character,
            &active_character.weapon,
            &target_function,
            Some(&active_character.artifact_config),
            &active_character.buffs,
            &enemy,
            attribute,
        );
        // utils::log!("{:?}", result);

        let mut arr = Vec::new();

        while !result.is_empty() {
            arr.push(result.pop().unwrap());
        }
        // utils::log!("{:?}", arr);

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        arr.serialize(&s).unwrap()
    }
}
