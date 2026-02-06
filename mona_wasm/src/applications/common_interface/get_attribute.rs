use mona::common::CharacterFullInfo;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::applications::common::{BuffInterface, CharacterFullInterface, CharactersInterface, CharacterInterface, SkillInterface, TargetFunctionInterface, WeaponInterface};
use mona::artifacts::{Artifact, ArtifactList};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::*;
use mona::buffs::{Buff, BuffConfig};
use mona::character::Character;
use mona::weapon::Weapon;

#[derive(Serialize, Deserialize)]
pub struct GetAttributeInterface {
    pub characters: CharactersInterface,

    pub active_character_id: usize,
}

pub fn get_attribute(val: JsValue) -> JsValue {
    let input: GetAttributeInterface = serde_wasm_bindgen::from_value(val).unwrap();

    let characters = CharacterFullInterface::get_characters(&input.characters);

    let attribute: ComplicatedAttribute = AttributeUtils::create_attribute_from_list(&characters, input.active_character_id);
    let active_character = CharacterFullInfo::get_character(&characters, input.active_character_id);

    let result = AttributeNoReactive::from(&attribute.solve());
    let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
    result.serialize(&s).unwrap()
}
