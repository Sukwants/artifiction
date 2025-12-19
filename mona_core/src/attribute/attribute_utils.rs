use crate::{artifacts::ArtifactList, character::team_status::CharacterStatus};
use crate::character::Character;
use crate::character::skill_config::CharacterSkillConfig;
use crate::weapon::weapon::Weapon;
use crate::buffs::Buff;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::attribute::*;
use crate::common::{ChangeAttribute, change_attribute};

pub struct AttributeUtils {}

impl AttributeUtils {
    pub fn create_attribute_from_c_w_bs<T: Attribute>(
        character: &Character<T>,
        weapon: &Weapon<T>,
        buffs: &Vec<Box<dyn Buff<T>>>
    ) -> T::ResultType {
        let mut attribute = T::new(T::new_with_base_edge(vec![CharacterStatus::new_single(0)]), 0);

        character.change_attribute(&mut attribute);
        weapon.change_attribute(&mut attribute);
        for buff in buffs.iter() {
            buff.change_attribute(&mut attribute);
        }

        attribute.solve()
    }

    pub fn create_attribute_from_big_config<T: Attribute>(
        artifacts: &ArtifactList,
        artifact_config: &ArtifactEffectConfig,
        character: &Character<T>,
        weapon: &Weapon<T>,
        buffs: &[Box<dyn Buff<T>>],
    ) -> T::ResultType {
        let mut attribute = T::new(T::new_with_base_edge(vec![CharacterStatus::new_single(0)]), 0);

        character.change_attribute(&mut attribute);
        weapon.change_attribute(&mut attribute);
        artifacts.apply(&mut attribute, character, artifact_config);

        for buff in buffs.iter() {
            buff.change_attribute(&mut attribute);
        }

        attribute.solve()
    }

    pub fn create_attribute_from_big_config_mut<T: Attribute>(
        artifacts: &ArtifactList,
        artifact_config: &ArtifactEffectConfig,
        character: &Character<T>,
        weapon: &Weapon<T>,
        buffs: &[Box<dyn Buff<T>>],
    ) -> T {
        let mut attribute = T::new(T::new_with_base_edge(vec![CharacterStatus::new_single(0)]), 0);

        character.change_attribute(&mut attribute);
        weapon.change_attribute(&mut attribute);
        artifacts.apply(&mut attribute, character, artifact_config);

        for buff in buffs.iter() {
            buff.change_attribute(&mut attribute);
        }

        attribute
    }

    pub fn create_attribute_from_big_config_with_skill_config<T: Attribute>(
        artifacts: &ArtifactList,
        artifact_config: &ArtifactEffectConfig,
        character: &Character<T>,
        character_skill_config: &CharacterSkillConfig,
        weapon: &Weapon<T>,
        buffs: &[Box<dyn Buff<T>>],
    ) -> T::ResultType {
        let mut attribute = T::new(T::new_with_base_edge(vec![CharacterStatus::new_single(0)]), 0);

        character.change_attribute(&mut attribute);
        weapon.change_attribute(&mut attribute);
        artifacts.apply(&mut attribute, character, artifact_config);

        for buff in buffs.iter() {
            buff.change_attribute(&mut attribute);
        }

        character.common_data.name.change_attribute(&mut attribute, &character.common_data, character_skill_config);

        attribute.solve()
    }
}