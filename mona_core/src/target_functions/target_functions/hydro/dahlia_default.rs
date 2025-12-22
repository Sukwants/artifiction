use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::*;
use crate::character::characters::dahlia::DahliaDamageEnum;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Dahlia;
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

pub struct DahliaDefaultTargetFunction;

impl TargetFunctionMetaTrait for DahliaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::DahliaDefault,
        name_locale: locale!(
            zh_cn: "塔利雅-颂礼祝祭",
            en: "Dahlia-Ode and Oblation"
        ),
        description: locale!(
            zh_cn: "「QE」单轮期望伤害",
            en: "[QE] Expected DMG Per Round"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Dahlia),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(DahliaDefaultTargetFunction)
    }
}

impl TargetFunction for DahliaDefaultTargetFunction {
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

        type S = <Dahlia as CharacterTrait>::DamageEnumType;
        let e_dmg = Dahlia::damage::<SimpleDamageBuilder>(&context, S::E, &CharacterSkillConfig::NoConfig, None);
        let q_dmg = Dahlia::damage::<SimpleDamageBuilder>(&context, S::Q, &CharacterSkillConfig::NoConfig, None);

        e_dmg.normal.expectation + q_dmg.normal.expectation
    }
}
