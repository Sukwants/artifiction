use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigArchaicPetra, ConfigLevel, ConfigRate};
use crate::attribute::*;
use crate::character::{Character, CharacterConfig, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::pyro::klee::Klee;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::{Element, StatName};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::*;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct KleeDefaultTargetFunction {
}

impl TargetFunctionMetaTrait for KleeDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::KleeDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "可莉-逃跑的太阳",
            en: "Klee-Fleeing Sunlight"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "可莉前台一轮输出总伤害",
            en: "Klee on-field total damage output"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Klee),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(KleeDefaultTargetFunction {
        })
    }
}

impl TargetFunction for KleeDefaultTargetFunction {
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

        let hexerei_secret_rite = match &context.character_common_data.config {
            CharacterConfig::Klee { hexerei_secret_rite } => *hexerei_secret_rite,
            _ => false
        };

        let config = CharacterSkillConfig::Klee { boom_badge: 3, active: true, activated_q: true, activated_c1: true };

        type Ty = <Klee as CharacterTrait>::DamageEnumType;

        let get_damage = |s: Ty, config: &CharacterSkillConfig| -> f64 {
            Klee::damage::<SimpleDamageBuilder>(&context, s, config, None).normal.expectation
        };

        let dmg_a = (get_damage(Ty::Normal1, &config) + get_damage(Ty::Normal2, &config) + get_damage(Ty::Normal3, &config))
            + get_damage(Ty::ChargedWithTalent, &config) * if context.character_common_data.constellation >= 6 && hexerei_secret_rite { 1.8 } else { 1.0 };

        let dmg_e = get_damage(Ty::E1, &config) * 3.0 + get_damage(Ty::E2, &config) * 5.0 + get_damage(Ty::C1, &config) * 0.5;
        
        let dmg_q = get_damage(Ty::Q1, &config) * 10.0 + get_damage(Ty::C4, &config);

        dmg_a * 6.0 + dmg_e + dmg_q
    }
}
