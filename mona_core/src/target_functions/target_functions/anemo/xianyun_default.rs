use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::*;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Xianyun;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::i18n::locale;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::*;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct XianyunDefaultTargetFunction;

impl TargetFunction for XianyunDefaultTargetFunction {
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

        type S = <Xianyun as CharacterTrait>::DamageEnumType;
        let damage = Xianyun::damage::<SimpleDamageBuilder>(
            &context, S::E4, &CharacterSkillConfig::NoConfig, None
        );
        damage.normal.expectation
    }
}

impl TargetFunctionMetaTrait for XianyunDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::XianyunDefault,
        name_locale: locale!(
            zh_cn: "闲云-鸾音鹤信",
            en: "Xianyun-Passerine Herald"
        ),
        description: locale!(
            zh_cn: "最大化闲云冲击波",
            en: "Maximize Driftcloud Wave DMG"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Xianyun),
        image: TargetFunctionMetaImage::Avatar
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(XianyunDefaultTargetFunction)
    }
}
