use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::*;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Aino;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::prelude::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::{moonsign, Moonsign};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::damage::transformative_damage::transformative_damage;
use crate::enemies::Enemy;
use crate::target_functions::*;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct AinoDefaultTargetFunction {
}

impl TargetFunctionMetaTrait for AinoDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::AinoDefault,
        name_locale: locale!(
            zh_cn: "爱诺-妙械咣咣铛",
            en: "Aino-Clinky-Clank Gadgets-A-Gogo"
        ),
        description: locale!(
            zh_cn: "元素爆发最大伤害",
            en: "Max Elemental Burst DMG"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Aino),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(AinoDefaultTargetFunction {})
    }
}

impl TargetFunction for AinoDefaultTargetFunction {
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

        let config = CharacterSkillConfig::NoConfig;

        let dmg_q = Aino::damage::<SimpleDamageBuilder>(&context, <Aino as CharacterTrait>::DamageEnumType::Q, &config, None).normal.expectation;
        let dmg_c2 = Aino::damage::<SimpleDamageBuilder>(&context, <Aino as CharacterTrait>::DamageEnumType::C2, &config, None).normal.expectation;

        let interval = context.attribute.get_value(AttributeName::USER1);

        dmg_q / interval + dmg_c2 / 5.0
    }
}
