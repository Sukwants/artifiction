use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeCommon, SimpleAttributeGraph2};
use crate::character::{Character, CharacterConfig, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Durin;
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

pub struct DurinDefaultTargetFunction {
    pub vaporize_rate: f64,
    pub melt_rate: f64,
    pub atk_demand: f64,
}

impl TargetFunctionMetaTrait for DurinDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::DurinDefault,
        name_locale: locale!(
            zh_cn: "杜林-「不熄灭的火」",
            en: "Durin-\"The Undying Fire\""
        ),
        description: locale!(
            zh_cn: "杜林一轮输出总伤害乘以攻击力满足需求比例",
            en: "Durin Total DMG per Round multiplied by ATK Demand Ratio"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Durin),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "vaporize_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "附着伤害蒸发占比",
                en: "Elemental Effected Damage's Vaporize Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "melt_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "附着伤害融化占比",
                en: "Elemental Effected Damage's Melt Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "atk_demand",
            title: locale!(
                zh_cn: "攻击力需求",
                en: "ATK Demand",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (vaporize_rate, melt_rate, atk_demand) = match *config {
            TargetFunctionConfig::DurinDefault { vaporize_rate, melt_rate, atk_demand } => (vaporize_rate, melt_rate, atk_demand),
            _ => (0.0, 0.0, 0.0)
        };
        Box::new(DurinDefaultTargetFunction {
            vaporize_rate,
            melt_rate,
            atk_demand,
        })
    }
}

impl TargetFunction for DurinDefaultTargetFunction {
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

        let (hexerei_secret_rite, essential_transmutation) = match &context.character_common_data.config {
            CharacterConfig::Durin { hexerei_secret_rite, essential_transmutation } => (*hexerei_secret_rite, *essential_transmutation),
            _ => (false, 0),
        };

        let config1 = CharacterSkillConfig::Durin {
            activated_res: true,
            primordial_fusion: true,
            cycle_of_enlightenment: true,
            activated_reaction: true,
        };
        let config2 = CharacterSkillConfig::Durin {
            activated_res: true,
            primordial_fusion: false,
            cycle_of_enlightenment: false,
            activated_reaction: true,
        };

        let demand_ratio = if attribute.get_atk() < self.atk_demand {
            attribute.get_atk() / self.atk_demand
        } else { 1.0 };

        type DmgType = <Durin as CharacterTrait>::DamageEnumType;

        let get_damage = |s: DmgType, config: &CharacterSkillConfig| -> f64 {
            Durin::damage::<SimpleDamageBuilder>(&context, s, config, None).normal.expectation
        };
        let get_effected_damage = |s: DmgType, config: &CharacterSkillConfig| -> f64 {
            let dmg = Durin::damage::<SimpleDamageBuilder>(&context, s, config, None);
            dmg.normal.expectation * (1.0 - self.vaporize_rate - self.melt_rate)
                + dmg.vaporize.unwrap_or(dmg.normal).expectation * self.vaporize_rate
                + dmg.melt.unwrap_or(dmg.normal).expectation * self.melt_rate
        };

        let dmg_e = if essential_transmutation == 0 {
            get_effected_damage(DmgType::EP, &config2)
        } else {
            get_effected_damage(DmgType::ED1, &config2) + get_damage(DmgType::ED2, &config2) + get_damage(DmgType::ED3, &config2)
        };

        let dmg_q = if essential_transmutation == 0 {
            get_effected_damage(DmgType::QP1, &config2) + get_damage(DmgType::QP2, &config2) + get_damage(DmgType::QP3, &config2)
        } else {
            get_effected_damage(DmgType::QD1, &config2) + get_damage(DmgType::QD2, &config2) + get_damage(DmgType::QD3, &config2)
        };

        let dmg_c = if essential_transmutation == 0 {
            (get_effected_damage(DmgType::QP, &config1) + get_damage(DmgType::QP, &config1)) * 5.0
            + (get_effected_damage(DmgType::QP, &config2) + get_damage(DmgType::QP, &config2)) * 5.0
        } else {
            (get_effected_damage(DmgType::QD, &config1) + get_damage(DmgType::QD, &config1)) * 5.0
            + (get_effected_damage(DmgType::QD, &config2) + get_damage(DmgType::QD, &config2)) * 3.0
        };

        (dmg_e + dmg_q + dmg_c) * demand_ratio
    }
}
