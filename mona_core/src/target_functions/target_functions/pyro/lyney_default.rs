use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::*;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{Character, CharacterName};
use crate::character::characters::Lyney;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::*;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct LyneyDefaultTargetFunction;

impl TargetFunction for LyneyDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &TargetFunctionAttributeResultType, character: &Character<TargetFunctionAttributeType>, weapon: &Weapon<TargetFunctionAttributeType>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, TargetFunctionAttributeResultType> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy,
        };

        type S = <Lyney as CharacterTrait>::DamageEnumType;
        let damage = Lyney::damage::<SimpleDamageBuilder>(&context, S::A1, &CharacterSkillConfig::Lyney {
            prop_stack: 5.0,
            under_pyro: true,
            pyro_count: 1
        }, None);

        damage.normal.expectation
    }
}

impl TargetFunctionMetaTrait for LyneyDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::LyneyDefault,
        name_locale: locale!(
            zh_cn: "林尼-惑光幻戏",
            en: "Lyney-	Spectacle of Phantasmagoria"
        ),
        description: locale!(
            zh_cn: "普通输出林尼",
            en: "DPS Lyney"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Lyney),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(LyneyDefaultTargetFunction)
    }
}