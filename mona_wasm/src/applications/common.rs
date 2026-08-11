use mona::artifacts::effect_config::ArtifactEffectConfig;
use mona::artifacts::{Artifact, ArtifactList, ArtifactSlotName};
use mona::attribute::{Attribute, SimpleAttribute};
use mona::buffs::buff_name::BuffName;
use mona::buffs::{Buff, BuffConfig};
use mona::character::skill_config::CharacterSkillConfig;
use mona::character::team_status::{CharacterStatus, CharacterTag, CharacterTags};
use mona::character::{Character, CharacterConfig, CharacterName};
use mona::common::{CharacterFullInfo, StatName};
use mona::enemies::Enemy;
use mona::potential_function::potential_function::PotentialFunction;
use mona::potential_function::potential_function_config::PotentialFunctionConfig;
use mona::potential_function::potential_function_name::PotentialFunctionName;
use mona::target_functions::{
    TargetFunction, TargetFunctionConfig, TargetFunctionName, TargetFunctionUtils,
};
use mona::weapon::{Weapon, WeaponConfig, WeaponName};
use serde::de::{DeserializeOwned, Error as DeError};
use serde::{Deserialize, Deserializer, Serialize};

fn empty_object_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    match value {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::Object(map)) if map.is_empty() => Ok(None),
        Some(serde_json::Value::Object(map))
            if map.len() == 1
                && map
                    .values()
                    .any(|value| value.as_object().is_some_and(|object| object.is_empty())) =>
        {
            Ok(None)
        }
        Some(value) => T::deserialize(value).map(Some).map_err(DeError::custom),
    }
}

#[derive(Serialize, Deserialize)]
pub struct SkillInterface {
    pub index: usize,
    #[serde(default, deserialize_with = "empty_object_as_none")]
    pub config: Option<CharacterSkillConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterInterface {
    pub name: CharacterName,
    pub level: usize,
    pub ascend: bool,
    pub constellation: i32,
    pub skill1: usize,
    pub skill2: usize,
    pub skill3: usize,
    #[serde(default, deserialize_with = "empty_object_as_none")]
    pub params: Option<CharacterConfig>,
    pub tags: Vec<CharacterTag>,
}

fn default_false() -> bool {
    false
}

impl CharacterInterface {
    pub fn to_character<T: Attribute>(&self, on_field: bool) -> Character<T> {
        let no_config = CharacterConfig::NoConfig;
        let params = self.params.as_ref().unwrap_or(&no_config);

        Character::new(
            self.name,
            self.level,
            self.ascend,
            self.constellation,
            self.skill1,
            self.skill2,
            self.skill3,
            params,
            &self.tags.iter().cloned().collect(),
            on_field,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct WeaponInterface {
    pub name: WeaponName,
    pub level: i32,
    pub ascend: bool,
    pub refine: i32,
    #[serde(default, deserialize_with = "empty_object_as_none")]
    pub params: Option<WeaponConfig>,
}

impl WeaponInterface {
    pub fn to_weapon<T: Attribute>(&self, character: &Character<T>) -> Weapon<T> {
        let no_config = WeaponConfig::NoConfig;
        let params = self.params.as_ref().unwrap_or(&no_config);

        Weapon::new(
            self.name,
            self.level,
            self.ascend,
            self.refine,
            params,
            character,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct TargetFunctionInterface {
    pub name: TargetFunctionName,
    #[serde(default, deserialize_with = "empty_object_as_none")]
    pub params: Option<TargetFunctionConfig>,
    #[serde(default = "default_false")]
    pub use_dsl: bool,
    pub dsl_source: Option<String>,
}

impl TargetFunctionInterface {
    pub fn to_target_function(
        &self,
        character: &Character<SimpleAttribute>,
        weapon: &Weapon<SimpleAttribute>,
    ) -> Box<dyn TargetFunction> {
        let no_config = TargetFunctionConfig::NoConfig;
        let params = self.params.as_ref().unwrap_or(&no_config);

        TargetFunctionUtils::new_target_function(self.name, character, weapon, params)
    }
}

#[derive(Serialize, Deserialize)]
pub struct BuffInterface {
    pub name: BuffName,
    #[serde(default, deserialize_with = "empty_object_as_none")]
    pub config: Option<BuffConfig>,
}

impl BuffInterface {
    pub fn to_buff<A: Attribute>(&self) -> Box<dyn Buff<A>> {
        let no_config = BuffConfig::NoConfig;
        let config = self.config.as_ref().unwrap_or(&no_config);

        self.name.create(config)
    }
}

#[derive(Serialize, Deserialize)]
pub struct EnemyInterface {
    pub level: usize,
    pub electro_res: f64,
    pub pyro_res: f64,
    pub hydro_res: f64,
    pub cryo_res: f64,
    pub geo_res: f64,
    pub anemo_res: f64,
    pub dendro_res: f64,
    pub physical_res: f64,
}

impl EnemyInterface {
    pub fn to_enemy(&self) -> Enemy {
        Enemy {
            level: self.level as i32,
            electro_res: self.electro_res,
            pyro_res: self.pyro_res,
            hydro_res: self.hydro_res,
            cryo_res: self.cryo_res,
            anemo_res: self.anemo_res,
            geo_res: self.geo_res,
            dendro_res: self.dendro_res,
            physical_res: self.physical_res,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ArtifactFilterConfig {
    pub sand_main_stat: Option<Vec<StatName>>,
    pub goblet_main_stat: Option<Vec<StatName>>,
    pub head_main_stat: Option<Vec<StatName>>,
}

impl ArtifactFilterConfig {
    pub fn filter_artifact<'a>(&self, artifacts: &[&'a Artifact]) -> Vec<&'a Artifact> {
        let mut results: Vec<&Artifact> = Vec::new();

        use ArtifactSlotName::*;
        for artifact in artifacts.iter() {
            match artifact.slot {
                Flower | Feather => results.push(artifact),
                Sand => match self.sand_main_stat {
                    None => results.push(artifact),
                    Some(ref li) => {
                        if li.contains(&artifact.main_stat.0) || li.len() == 0 {
                            results.push(artifact);
                        }
                    }
                },
                Goblet => match self.goblet_main_stat {
                    None => results.push(artifact),
                    Some(ref li) => {
                        if li.contains(&artifact.main_stat.0) || li.len() == 0 {
                            results.push(artifact);
                        }
                    }
                },
                Head => match self.head_main_stat {
                    None => results.push(artifact),
                    Some(ref li) => {
                        if li.contains(&artifact.main_stat.0) || li.len() == 0 {
                            results.push(artifact);
                        }
                    }
                },
            }
        }

        results
    }
}

#[derive(Serialize, Deserialize)]
pub struct PotentialFunctionInterface {
    pub name: PotentialFunctionName,
    #[serde(default, deserialize_with = "empty_object_as_none")]
    pub config: Option<PotentialFunctionConfig>,
}

impl PotentialFunctionInterface {
    pub fn to_pf(&self) -> Box<dyn PotentialFunction> {
        let no_config = PotentialFunctionConfig::NoConfig;
        let config = self.config.as_ref().unwrap_or(&no_config);
        self.name.create(config)
    }
}

#[derive(Serialize, Deserialize)]
pub struct CharacterFullInterface {
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub buffs: Vec<BuffInterface>,
    pub artifacts: Vec<Artifact>,
    pub artifact_config: Option<ArtifactEffectConfig>,
    pub skill: Option<SkillInterface>,

    pub character_id: usize,
    pub team_id: usize,
    pub on_field: bool,
}

pub type CharactersInterface = Vec<Option<CharacterFullInterface>>;

impl CharacterFullInterface {
    pub fn get_characters<'a, A: Attribute>(
        input: &'a Vec<Option<CharacterFullInterface>>,
    ) -> Vec<CharacterFullInfo<'a, A>> {
        let mut characters: Vec<CharacterFullInfo<A>> = Vec::new();

        for c in input.iter() {
            if let Some(c) = c {
                characters.push(CharacterFullInfo {
                    character: c.character.to_character(c.on_field),
                    weapon: c.weapon.to_weapon(&c.character.to_character(c.on_field)),
                    buffs: c.buffs.iter().map(|x| x.to_buff()).collect(),
                    artifacts: c.artifacts.iter().collect(),
                    artifact_config: match &c.artifact_config {
                        Some(x) => x.clone(),
                        None => Default::default(),
                    },
                    skill_config: if let Some(skill) = &c.skill {
                        skill
                            .config
                            .clone()
                            .unwrap_or(CharacterSkillConfig::NoConfig)
                    } else {
                        CharacterSkillConfig::NoConfig
                    },
                    skill_index: if let Some(skill) = &c.skill {
                        skill.index
                    } else {
                        usize::MAX
                    },
                    character_status: CharacterStatus::new(
                        c.character_id,
                        c.team_id,
                        c.on_field,
                        c.character.name.get_static_data(),
                        c.character.tags.iter().cloned().collect(),
                    ),
                });
            }
        }

        characters
    }
}
