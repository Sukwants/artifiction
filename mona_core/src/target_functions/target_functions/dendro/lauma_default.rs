use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Lauma;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::prelude::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::{moonsign, Moonsign};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::damage::transformative_damage::transformative_damage;
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct LaumaDefaultTargetFunction {
    pub bloom_count: usize,
    pub hyperbloom_count: usize,
    pub burgeon_count: usize,
}

impl TargetFunctionMetaTrait for LaumaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::LaumaDefault,
        name_locale: locale!(
            zh_cn: "菈乌玛-永月的祀歌",
            en: "Lauma-Evermoon's Sacrament Song"
        ),
        description: locale!(
            zh_cn: "一轮元素战技总伤害加反应伤害",
            en: "Birgitta Discharge DMG"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Lauma),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "bloom_count",
            title: locale!(
                zh_cn: "绽放加成次数",
                en: "Bloom Enhanced Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 48, default: 0 }
        },
        ItemConfig {
            name: "hyperbloom_count",
            title: locale!(
                zh_cn: "超绽放加成次数",
                en: "Hyperbloom Enhanced Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 48, default: 0 }
        },
        ItemConfig {
            name: "burgeon_count",
            title: locale!(
                zh_cn: "烈绽放加成次数",
                en: "Burgeon Enhanced Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 48, default: 0 }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (bloom_count, hyperbloom_count, burgeon_count) = match *config {
            TargetFunctionConfig::LaumaDefault { bloom_count, hyperbloom_count, burgeon_count } => (
                bloom_count, hyperbloom_count, burgeon_count),
            _ => (0, 0, 0)
        };
        Box::new(LaumaDefaultTargetFunction {
            bloom_count,
            hyperbloom_count,
            burgeon_count,
        })
    }
}

impl TargetFunction for LaumaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        let config = CharacterSkillConfig::NoConfig;

        let dmg_e = 
            Lauma::damage::<SimpleDamageBuilder>(&context, <Lauma as CharacterTrait>::DamageEnumType::EHold1, &config, None).normal.expectation
            + Lauma::damage::<SimpleDamageBuilder>(&context, <Lauma as CharacterTrait>::DamageEnumType::EHold2, &config, None).normal.expectation;
        let dmg_frostgrove_sanctuary = (
            Lauma::damage::<SimpleDamageBuilder>(&context, <Lauma as CharacterTrait>::DamageEnumType::EFrostgroveSanctuary, &config, None).normal.expectation
            + (if context.character_common_data.constellation >= 6 {
                Lauma::damage::<SimpleDamageBuilder>(&context, <Lauma as CharacterTrait>::DamageEnumType::C6E, &config, None).normal.expectation
            } else { 0.0 })
        ) * 6.0; // 2s 一次伤害
        let dmg_c6a = if context.character_common_data.constellation >= 6 {
            Lauma::damage::<SimpleDamageBuilder>(&context, <Lauma as CharacterTrait>::DamageEnumType::C6A, &config, None).normal.expectation
        } else { 0.0 } * 18.0; // 12s 内共计六轮普攻
        let dmg_bloom = 
            transformative_damage(character.common_data.level, attribute, enemy).bloom * self.bloom_count as f64
            + transformative_damage(character.common_data.level, attribute, enemy).hyperbloom * self.hyperbloom_count as f64
            + transformative_damage(character.common_data.level, attribute, enemy).burgeon * self.burgeon_count as f64;

        dmg_e + dmg_frostgrove_sanctuary + dmg_c6a + dmg_bloom
    }
}
