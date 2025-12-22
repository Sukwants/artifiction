use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::artifacts::Artifact;
use crate::attribute::*;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Chasca;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::character::{Character, CharacterName};
use crate::common::i18n::locale;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::*;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::Weapon;

pub struct ChascaDefaultTargetFunction;

impl TargetFunction for ChascaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &TargetFunctionAttributeResultType, character: &Character<TargetFunctionAttributeType>, weapon: &Weapon<TargetFunctionAttributeType>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, TargetFunctionAttributeResultType> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Chasca as CharacterTrait>::DamageEnumType;
        let dmg = Chasca::damage::<SimpleDamageBuilder>(
            &context, S::E3, &CharacterSkillConfig::Chasca { element_count: 2, c6_rate: 0.0 }, None
        );

        dmg.normal.expectation
    }
}

impl TargetFunctionMetaTrait for ChascaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::ChascaDefault,
        name_locale: locale!(
            zh_cn: "恰斯卡-巡宇翦定",
            en: "Chasca-Skyborne Arbiter"
        ),
        description: locale!(
            zh_cn: "最大化追影弹伤害",
            en: "Maximize Shadowhunt Shell DMG"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Chasca),
        image: TargetFunctionMetaImage::Avatar,
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(ChascaDefaultTargetFunction)
    }
}
