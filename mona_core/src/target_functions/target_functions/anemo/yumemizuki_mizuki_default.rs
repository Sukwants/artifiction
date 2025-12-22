use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::*;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::damage::DamageContext;
use crate::enemies::Enemy;
use crate::target_functions::*;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct YumemizukiMizukiDefaultTargetFunction;

impl TargetFunction for YumemizukiMizukiDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &TargetFunctionAttributeResultType, character: &Character<TargetFunctionAttributeType>, weapon: &Weapon<TargetFunctionAttributeType>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        attribute.get_em_all()
    }
}

impl TargetFunctionMetaTrait for YumemizukiMizukiDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::YumemizukiMizukiDefault,
        name_locale: locale!(
            zh_cn: "梦见月瑞希-「绮梦缱绻」",
            en: "Yumemizuki Mizuki: Embrace of Enchanting Dreams"
        ),
        description: locale!(
            zh_cn: "最大化元素精通",
            en: "Maximize EM"
        ),
        tags: "",
        four: TargetFunctionFor::SomeWho(CharacterName::YumemizukiMizuki),
        image: TargetFunctionMetaImage::Avatar,
    };

    fn create(character: &CharacterCommonData, weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(YumemizukiMizukiDefaultTargetFunction)
    }
}
