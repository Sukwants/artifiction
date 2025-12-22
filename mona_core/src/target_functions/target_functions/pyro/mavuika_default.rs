use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::*;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Mavuika;
use crate::character::prelude::CharacterTrait;
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::i18n::locale;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::*;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct MavuikaDefaultTargetFunction;

impl TargetFunction for MavuikaDefaultTargetFunction {
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
            enemy
        };

        type S = <Mavuika as CharacterTrait>::DamageEnumType;
        let damage = Mavuika::damage::<SimpleDamageBuilder>(
            &context,
            S::ECharged1,
            &CharacterSkillConfig::Mavuika { after_q: true },
            None
        );

        damage.normal.expectation
    }
}

impl TargetFunctionMetaTrait for MavuikaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::MavuikaDefault,
        name_locale: locale!(
            zh_cn: "玛薇卡-焚夜以炎",
            en: "Mavuika - 「Night-Igniting Flame」"
        ),
        description: locale!(
            zh_cn: "最大化开大后E重伤害",
            en: "Maximize E Charged DMG after Q"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::Mavuika),
        image: TargetFunctionMetaImage::Avatar,
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(MavuikaDefaultTargetFunction)
    }
}
