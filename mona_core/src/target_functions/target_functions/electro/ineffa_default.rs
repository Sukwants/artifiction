use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Ineffa;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::prelude::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
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

pub struct IneffaDefaultTargetFunction {
    pub lunar_charged_coefficient: f64,
}

impl TargetFunctionMetaTrait for IneffaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::IneffaDefault,
        name_locale: locale!(
            zh_cn: "伊涅芙-轰隆雷鸣波",
            en: "Ineffa-Boom Boom Thunderwave"
        ),
        description: locale!(
            zh_cn: "薇尔琪塔放电伤害",
            en: "Birgitta Discharge DMG"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Ineffa),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "lunar_charged_coefficient",
            title: locale!(
                zh_cn: "月感电伤害系数",
                en: "Lunar Charged DMG Coefficient",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let rate = match *config {
            TargetFunctionConfig::IneffaDefault { lunar_charged_coefficient } => lunar_charged_coefficient,
            _ => 0.0
        };
        Box::new(IneffaDefaultTargetFunction {
            lunar_charged_coefficient: rate.clamp(0.0, 1.0)
        })
    }
}

impl TargetFunction for IneffaDefaultTargetFunction {
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

        let dmg_e = Ineffa::damage::<SimpleDamageBuilder>(&context, <Ineffa as CharacterTrait>::DamageEnumType::EContinued, &CharacterSkillConfig::NoConfig, None).normal.expectation;
        let dmg_c6 = if context.character_common_data.constellation >= 6 {
            Ineffa::damage::<SimpleDamageBuilder>(&context, <Ineffa as CharacterTrait>::DamageEnumType::C6, &CharacterSkillConfig::NoConfig, None).normal.expectation
            * 2.0 / 3.5
        } else { 0.0 };
        let dmg_lunar_charged = Ineffa::damage::<SimpleDamageBuilder>(&context, <Ineffa as CharacterTrait>::DamageEnumType::LunarCharged, &CharacterSkillConfig::NoConfig, None).lunar_charged.unwrap().expectation;

        dmg_e + dmg_c6 + dmg_lunar_charged * self.lunar_charged_coefficient
    }
}
