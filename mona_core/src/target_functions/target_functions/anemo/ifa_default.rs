use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::*;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Ifa;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::prelude::CharacterTrait;
use crate::common::i18n::locale;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::damage::transformative_damage::transformative_damage;
use crate::enemies::Enemy;
use crate::target_functions::*;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct IfaDefaultTargetFunction;

impl TargetFunctionMetaTrait for IfaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::IfaDefault,
        name_locale: locale!(
            zh_cn: "伊法-蔚风引灵",
            en: "Ifa-In the Wake of Wandering Winds"
        ),
        description: locale!(
            zh_cn: "扩散 / 感电单次平均伤害",
            en: "Swirl / Electro-Charged Average DMG Per Hit"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Ifa),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(IfaDefaultTargetFunction)
    }
}

impl TargetFunction for IfaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &TargetFunctionAttributeResultType, character: &Character<TargetFunctionAttributeType>, weapon: &Weapon<TargetFunctionAttributeType>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, TargetFunctionAttributeResultType> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        let dmg = transformative_damage::<SimpleDamageBuilder>(character.common_data.level, attribute, enemy);

        ((dmg.swirl_cryo.expectation + dmg.swirl_hydro.expectation + dmg.swirl_pyro.expectation + dmg.swirl_electro.expectation) / 4.0 + dmg.electro_charged.expectation) / 2.0
    }
}
