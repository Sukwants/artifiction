use crate::{artifacts::ArtifactList, character::team_status::CharacterStatus};
use crate::character::Character;
use crate::character::skill_config::CharacterSkillConfig;
use crate::weapon::weapon::Weapon;
use crate::buffs::Buff;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use super::attribute::*;
use crate::common::{ChangeAttribute, CharacterFullInfo, change_attribute};

pub struct AttributeUtils {}

impl AttributeUtils {
    // pub fn create_attribute_from_c_w_bs_result<T: Attribute>(
    //     character: &Character<T>,
    //     weapon: &Weapon<T>,
    //     buffs: &Vec<Box<dyn Buff<T>>>
    // ) -> T::ResultType {
    //     let mut attribute = T::new(T::new_with_base_edge(vec![CharacterStatus::new_single(0)]), 0);

    //     character.change_attribute(&mut attribute);
    //     weapon.change_attribute(&mut attribute);
    //     for buff in buffs.iter() {
    //         buff.change_attribute(&mut attribute);
    //     }

    //     attribute.solve()
    // }

    // pub fn create_attribute_from_c_w_bs<T: Attribute>(
    //     character: &Character<T>,
    //     weapon: &Weapon<T>,
    //     buffs: &Vec<Box<dyn Buff<T>>>
    // ) -> T {
    //     let mut attribute = T::new(T::new_with_base_edge(vec![CharacterStatus::new_single(0)]), 0);

    //     character.change_attribute(&mut attribute);
    //     weapon.change_attribute(&mut attribute);
    //     for buff in buffs.iter() {
    //         buff.change_attribute(&mut attribute);
    //     }

    //     attribute
    // }

    // pub fn create_attribute_from_big_config_result<T: Attribute>(
    //     artifacts: &ArtifactList,
    //     artifact_config: &ArtifactEffectConfig,
    //     character: &Character<T>,
    //     weapon: &Weapon<T>,
    //     buffs: &[Box<dyn Buff<T>>],
    // ) -> T::ResultType {
    //     let mut attribute = T::new(T::new_with_base_edge(vec![CharacterStatus::new_single(0)]), 0);

    //     character.change_attribute(&mut attribute);
    //     weapon.change_attribute(&mut attribute);
    //     artifacts.apply(&mut attribute, character, artifact_config);

    //     for buff in buffs.iter() {
    //         buff.change_attribute(&mut attribute);
    //     }

    //     attribute.solve()
    // }

    // pub fn create_attribute_from_big_config<T: Attribute>(
    //     artifacts: &ArtifactList,
    //     artifact_config: &ArtifactEffectConfig,
    //     character: &Character<T>,
    //     weapon: &Weapon<T>,
    //     buffs: &[Box<dyn Buff<T>>],
    // ) -> T {
    //     let mut attribute = T::new(T::new_with_base_edge(vec![CharacterStatus::new_single(0)]), 0);

    //     character.change_attribute(&mut attribute);
    //     weapon.change_attribute(&mut attribute);
    //     artifacts.apply(&mut attribute, character, artifact_config);

    //     for buff in buffs.iter() {
    //         buff.change_attribute(&mut attribute);
    //     }

    //     attribute
    // }

    // pub fn create_attribute_from_big_config_with_skill_config_result<T: Attribute>(
    //     artifacts: &ArtifactList,
    //     artifact_config: &ArtifactEffectConfig,
    //     character: &Character<T>,
    //     character_skill_config: &CharacterSkillConfig,
    //     weapon: &Weapon<T>,
    //     buffs: &[Box<dyn Buff<T>>],
    // ) -> T::ResultType {
    //     let mut attribute = T::new(T::new_with_base_edge(vec![CharacterStatus::new_single(0)]), 0);

    //     character.change_attribute(&mut attribute);
    //     weapon.change_attribute(&mut attribute);
    //     artifacts.apply(&mut attribute, character, artifact_config);

    //     for buff in buffs.iter() {
    //         buff.change_attribute(&mut attribute);
    //     }

    //     character.common_data.name.change_attribute(&mut attribute, &character.common_data, character_skill_config);

    //     attribute.solve()
    // }

    // pub fn create_attribute_from_big_config_with_skill_config<T: Attribute>(
    //     artifacts: &ArtifactList,
    //     artifact_config: &ArtifactEffectConfig,
    //     character: &Character<T>,
    //     character_skill_config: &CharacterSkillConfig,
    //     weapon: &Weapon<T>,
    //     buffs: &[Box<dyn Buff<T>>],
    // ) -> T {
    //     let mut attribute = T::new(T::new_with_base_edge(vec![CharacterStatus::new_single(0)]), 0);

    //     character.change_attribute(&mut attribute);
    //     weapon.change_attribute(&mut attribute);
    //     artifacts.apply(&mut attribute, character, artifact_config);

    //     for buff in buffs.iter() {
    //         buff.change_attribute(&mut attribute);
    //     }

    //     character.common_data.name.change_attribute(&mut attribute, &character.common_data, character_skill_config);

    //     attribute
    // }

    pub fn change_attribute_graph<T: Attribute>(
        attribute_graph: T::GraphTy,
        character_id: usize,

        artifacts: &ArtifactList,
        artifact_config: &ArtifactEffectConfig,
        character: &Character<T>,
        character_skill_config: &CharacterSkillConfig,
        weapon: &Weapon<T>,
        buffs: &[Box<dyn Buff<T>>],
    ) -> T::GraphTy {
        let mut attribute = T::new(attribute_graph, character_id);

        character.change_attribute(&mut attribute);
        weapon.change_attribute(&mut attribute);
        artifacts.apply(&mut attribute, character, artifact_config);

        for buff in buffs.iter() {
            buff.change_attribute(&mut attribute);
        }

        character.common_data.name.change_attribute(&mut attribute, &character.common_data, character_skill_config);

        attribute.get_attribute()
    }

    pub fn change_attribute<T: Attribute>(
        mut attribute: T,

        artifacts: &ArtifactList,
        artifact_config: &ArtifactEffectConfig,
        character: &Character<T>,
        character_skill_config: &CharacterSkillConfig,
        weapon: &Weapon<T>,
        buffs: &[Box<dyn Buff<T>>],
    ) -> T {
        character.change_attribute(&mut attribute);
        weapon.change_attribute(&mut attribute);
        artifacts.apply(&mut attribute, character, artifact_config);

        for buff in buffs.iter() {
            buff.change_attribute(&mut attribute);
        }

        character.common_data.name.change_attribute(&mut attribute, &character.common_data, character_skill_config);

        attribute
    }

    pub fn change_attribute_without_skill<T: Attribute>(
        mut attribute: T,

        artifacts: &ArtifactList,
        artifact_config: &ArtifactEffectConfig,
        character: &Character<T>,
        weapon: &Weapon<T>,
        buffs: &[Box<dyn Buff<T>>],
    ) -> T {
        character.change_attribute(&mut attribute);
        weapon.change_attribute(&mut attribute);
        artifacts.apply(&mut attribute, character, artifact_config);

        for buff in buffs.iter() {
            buff.change_attribute(&mut attribute);
        }

        attribute
    }

    pub fn change_attribute_from_c_w_bs<T: Attribute>(
        mut attribute: T,

        character: &Character<T>,
        weapon: &Weapon<T>,
        buffs: &[Box<dyn Buff<T>>],
    ) -> T {
        character.change_attribute(&mut attribute);
        weapon.change_attribute(&mut attribute);

        for buff in buffs.iter() {
            buff.change_attribute(&mut attribute);
        }

        attribute
    }

    pub fn create_attribute_from_list<T: Attribute>(
        characters: &Vec<CharacterFullInfo<T>>,
        active_character_id: usize,
    ) -> T {
        let character_list: Vec<CharacterStatus> = characters.iter().map(|c| c.character_status.clone()).collect();

        let mut attribute_graph = T::new_with_base_edge(character_list);

        for c in characters.iter() {
            attribute_graph = AttributeUtils::change_attribute_graph(
                attribute_graph,
                c.character_status.character_id,
                &ArtifactList::new(&c.artifacts),
                &c.artifact_config,
                &c.character,
                &c.skill_config,
                &c.weapon,
                &c.buffs
            );
        }

        T::new(attribute_graph, active_character_id)
    }

    pub fn create_attribute_from_list_except_active_character<T: Attribute>(
        characters: &Vec<CharacterFullInfo<T>>,
        active_character_id: usize,
    ) -> T {
        let character_list: Vec<CharacterStatus> = characters.iter().map(|c| c.character_status.clone()).collect();

        let mut attribute_graph = T::new_with_base_edge(character_list);

        for c in characters.iter() {
            if c.character_status.character_id == active_character_id {
                continue;
            }
            attribute_graph = AttributeUtils::change_attribute_graph(
                attribute_graph,
                c.character_status.character_id,
                &ArtifactList::new(&c.artifacts),
                &c.artifact_config,
                &c.character,
                &c.skill_config,
                &c.weapon,
                &c.buffs
            );
        }

        T::new(attribute_graph, active_character_id)
    }
}