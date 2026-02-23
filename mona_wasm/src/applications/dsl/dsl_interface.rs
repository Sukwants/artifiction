use mona::artifacts::{Artifact, ArtifactList};
use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::attribute::*;
use mona::buffs::Buff;
use mona::common::CharacterFullInfo;
use mona_dsl::common::UnsafeDamageContext;
use mona_dsl::compile_source_to_code_object;
use mona_dsl::error::CompileError;
use mona_dsl::error::runtime_error::RuntimeError;
use mona_dsl::vm::env::MonaEnv;
use mona_dsl::vm::stream::StringOutputStream;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::applications::common::{BuffInterface, CharactersInterface, CharacterFullInterface, CharacterInterface, EnemyInterface, WeaponInterface};
use crate::utils;

pub struct DSLInterface;

#[derive(Serialize)]
pub struct RunResult {
    pub is_error: bool,
    pub error_msg: String,
    pub output: String,
}

#[derive(Deserialize)]
pub struct RunInput {
    pub characters: CharactersInterface,
    pub enemy: Option<EnemyInterface>,

    pub active_character_id: usize,
}

impl RunResult {
    pub fn from_compile_error(e: &CompileError) -> Self {
        Self {
            is_error: true,
            error_msg: e.to_string(),
            output: String::new()
        }
    }

    pub fn from_runtime_error(e: &RuntimeError) -> Self {
        Self {
            is_error: true,
            error_msg: e.to_string(),
            output: String::new()
        }
    }
}

#[wasm_bindgen]
impl DSLInterface {
    pub fn run(source: &str, damage_env: JsValue, artifacts: JsValue) -> JsValue {
        utils::set_panic_hook();
        let damage_env: RunInput = serde_wasm_bindgen::from_value(damage_env).unwrap();
        let artifacts: Vec<Artifact> = serde_wasm_bindgen::from_value(artifacts).unwrap();

        // get all items
        let characters = CharacterFullInterface::get_characters(&damage_env.characters);

        let attribute: SimpleAttribute = AttributeUtils::create_attribute_from_list_except_active_character(&characters, damage_env.active_character_id);
        let active_character = CharacterFullInfo::get_character(&characters, damage_env.active_character_id);

        let enemy = if let Some(x) = damage_env.enemy {
            x.to_enemy()
        } else {
            Default::default()
        };

        // compile
        let code_obj = match compile_source_to_code_object(source) {
            Err(e) => {
                let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
                return RunResult::from_compile_error(&e).serialize(&s).unwrap()
            }
            Ok(v) => v
        };

        // set output stream
        let mut env = MonaEnv::new(code_obj);
        let os = StringOutputStream::new();
        env.set_ostream(Box::new(os));

        // set damage context
        let unsafe_context = UnsafeDamageContext {
            character_common_data: &active_character.character.common_data,
            enemy: &enemy,
            attribute: &attribute.solve()
        };
        env.add_damage_context(unsafe_context);
        utils::log!("{:?}", env.damage_ctx.keys());

        // init
        if let Err(e) = env.init_prop() {
            let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
            return RunResult::from_runtime_error(&e).serialize(&s).unwrap();
        }
        if let Err(e) = env.init_damage() {
            let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
            return RunResult::from_runtime_error(&e).serialize(&s).unwrap();
        }
        utils::log!("{:?}", env.namespace.map.keys());

        // execute
        if let Err(e) = env.execute() {
            let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
            return RunResult::from_runtime_error(&e).serialize(&s).unwrap();
        }

        // get output
        let os = env.ostream.as_any().downcast_ref::<StringOutputStream>().unwrap();
        let output = os.get_string();
        let ret = RunResult {
            is_error: false,
            error_msg: String::new(),
            output
        };

        let s = serde_wasm_bindgen::Serializer::new().serialize_maps_as_objects(true);
        ret.serialize(&s).unwrap()
    }
}