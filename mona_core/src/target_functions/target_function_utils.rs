use super::target_function::{TargetFunction, TargetFunctionAttributeType};
use super::target_function_name::TargetFunctionName;
use super::target_function_config::TargetFunctionConfig;

use crate::weapon::Weapon;
use crate::character::Character;
use crate::target_functions::target_functions::get_target_function;

pub struct TargetFunctionUtils {}

impl TargetFunctionUtils {
    pub fn new_target_function(
        name: TargetFunctionName,
        character: &Character<TargetFunctionAttributeType>,
        weapon: &Weapon<TargetFunctionAttributeType>,
        config: &TargetFunctionConfig
    ) -> Box<dyn TargetFunction> {
        get_target_function(
            name, character, weapon, config
        )
    }
}