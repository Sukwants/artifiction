use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Flins;
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

pub struct FlinsDefaultTargetFunction {
    pub lunar_charged_coefficient: f64,
    pub if_thunderclouds: bool,
}

impl TargetFunctionMetaTrait for FlinsDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::FlinsDefault,
        name_locale: locale!(
            zh_cn: "菲林斯-诡灯陌影",
            en: "Flins-Shadowy Lights, Stranger Wights"
        ),
        description: locale!(
            zh_cn: "菲林斯一轮输出总伤害",
            en: "Flins Total DMG per Round"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Flins),
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
        ItemConfig {
            name: "if_thunderclouds",
            title: locale!(
                zh_cn: "存在雷暴云",
                en: "If Thunderclouds Exist",
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (rate, if_thunderclouds) = match *config {
            TargetFunctionConfig::FlinsDefault { lunar_charged_coefficient, if_thunderclouds } => (lunar_charged_coefficient, if_thunderclouds),
            _ => (0.0, false)
        };
        Box::new(FlinsDefaultTargetFunction {
            lunar_charged_coefficient: rate.clamp(0.0, 1.0),
            if_thunderclouds,
        })
    }
}

impl TargetFunction for FlinsDefaultTargetFunction {
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

        let dmg_e = Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::ENS, &CharacterSkillConfig::NoConfig, None).normal.expectation
            + if context.character_common_data.constellation >= 2 { Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::C2, &CharacterSkillConfig::NoConfig, None).normal.expectation } else { 0.0 };

        let dmg_ea = Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::E1, &CharacterSkillConfig::NoConfig, None).normal.expectation
            + Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::E2, &CharacterSkillConfig::NoConfig, None).normal.expectation
            + Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::E3, &CharacterSkillConfig::NoConfig, None).normal.expectation
            + Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::E4, &CharacterSkillConfig::NoConfig, None).normal.expectation
            + Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::E5, &CharacterSkillConfig::NoConfig, None).normal.expectation;

        let dmg_eq = Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::QTS, &CharacterSkillConfig::NoConfig, None).normal.expectation
            + if self.if_thunderclouds { Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::QTSA, &CharacterSkillConfig::NoConfig, None).normal.expectation } else { 0.0 };

        let dmg_q = Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::Q1, &CharacterSkillConfig::NoConfig, None).normal.expectation
            + Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::Q2, &CharacterSkillConfig::NoConfig, None).normal.expectation * if self.if_thunderclouds { 4.0 } else { 2.0 }
            + Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::Q3, &CharacterSkillConfig::NoConfig, None).normal.expectation;

        let dmg_lunar_charged = Flins::damage::<SimpleDamageBuilder>(&context, <Flins as CharacterTrait>::DamageEnumType::LunarCharged, &CharacterSkillConfig::NoConfig, None).normal.expectation;

        dmg_q + if context.character_common_data.constellation >= 1 { (dmg_e + dmg_eq + dmg_ea) * 2.0 + (dmg_e + dmg_eq) } else { (dmg_e + dmg_eq + dmg_ea * 2.0) + (dmg_e + dmg_eq + dmg_ea) } + dmg_lunar_charged * (20.0 / 2.0) * self.lunar_charged_coefficient // 一轮输出时间以元素爆发 cd 20s 为准，一套输出时间以特殊元素战技北国枪阵 cd 为准
    }
}
