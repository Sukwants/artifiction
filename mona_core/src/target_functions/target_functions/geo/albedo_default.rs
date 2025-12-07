use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigLevel, ConfigRate};
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::traits::{CharacterTrait};
use crate::character::characters::Albedo;
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::item_config_type::ItemConfig;
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct AlbedoDefaultTargetFunction;

impl TargetFunctionMetaTrait for AlbedoDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::AlbedoDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "阿贝多-白垩之子",
            en: "Albedo-Kreideprinz"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "阿贝多后台一轮输出总伤害",
            en: "Albedo off-field total damage output"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Albedo),
        image: TargetFunctionMetaImage::Avatar,
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(AlbedoDefaultTargetFunction)
    }
}

impl TargetFunction for AlbedoDefaultTargetFunction {
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

        let config1 = [
            CharacterSkillConfig::Albedo { lower50: false, activated_q: false, fatal_count: 4, crystallize_shield: false },
            CharacterSkillConfig::Albedo { lower50: true, activated_q: false, fatal_count: 4, crystallize_shield: false }
        ];
        let config2 = [
            CharacterSkillConfig::Albedo { lower50: false, activated_q: true, fatal_count: 4, crystallize_shield: false },
            CharacterSkillConfig::Albedo { lower50: true, activated_q: true, fatal_count: 4, crystallize_shield: false }
        ];

        type Ty = <Albedo as CharacterTrait>::DamageEnumType;

        let get_damage = |s: Ty, config: &[CharacterSkillConfig]| -> f64 {
            (
                Albedo::damage::<SimpleDamageBuilder>(&context, s, &config[0], None).normal.expectation
                + Albedo::damage::<SimpleDamageBuilder>(&context, s, &config[1], None).normal.expectation
            ) / 2.0
        };

        let dmg_e = get_damage(Ty::E1, &config1)
            + get_damage(Ty::ETransientBlossom, &config2) * 9.0;

        let dmg_q = get_damage(Ty::Q1, &config1)
            + get_damage(Ty::QFatalBlossom, &config2) * 7.0
            + get_damage(Ty::C2, &config2) * 4.0 * 2.0;

        dmg_e + dmg_q
    }
}
