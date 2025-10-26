use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Nefer;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::prelude::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::{moonsign, Moonsign};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::damage::transformative_damage::transformative_damage;
use crate::enemies::Enemy;
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct NeferDefaultTargetFunction {

}

impl TargetFunctionMetaTrait for NeferDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::NeferDefault,
        name_locale: locale!(
            zh_cn: "菈乌玛-湮沙的秘闻",
            en: "Nefer-Secret Beneath the Sands"
        ),
        description: locale!(
            zh_cn: "奈芙尔一轮输出总伤害",
            en: "Nefer Total DMG per Round"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Nefer),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(NeferDefaultTargetFunction {

        })
    }
}

impl TargetFunction for NeferDefaultTargetFunction {
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

        let config = CharacterSkillConfig::Nefer { veil_of_falsehood: (
            if context.character_common_data.constellation >= 2 { 5 } else { 3 } ), 
            shadow_dance: ( true ) };

        type DmgType = <Nefer as CharacterTrait>::DamageEnumType;
        let c_count = context.character_common_data.constellation;

        let dmg_e = Nefer::damage::<SimpleDamageBuilder>(&context, DmgType::E, &config, None).normal.expectation;
        let dmg_z = 
            Nefer::damage::<SimpleDamageBuilder>(&context, DmgType::E1, &config, None).normal.expectation
            + Nefer::damage::<SimpleDamageBuilder>(&context, DmgType::E2, &config, None).normal.expectation
            + Nefer::damage::<SimpleDamageBuilder>(&context, DmgType::ES1, &config, None).normal.expectation
            + Nefer::damage::<SimpleDamageBuilder>(&context, if c_count >= 6 { DmgType::ES2 } else { DmgType::C61 }, &config, None).normal.expectation
            + Nefer::damage::<SimpleDamageBuilder>(&context, DmgType::ES3, &config, None).normal.expectation
            + if c_count >= 6 { Nefer::damage::<SimpleDamageBuilder>(&context, DmgType::C62, &config, None).normal.expectation } else { 0.0 };
        let dmg_q = Nefer::damage::<SimpleDamageBuilder>(&context, DmgType::Q1, &config, None).normal.expectation
            + Nefer::damage::<SimpleDamageBuilder>(&context, DmgType::Q2, &config, None).normal.expectation;

        (dmg_e + dmg_z * 3.0) * 2.0 + dmg_q
    }
}
