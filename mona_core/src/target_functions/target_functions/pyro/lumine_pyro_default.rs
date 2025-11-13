use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::LuminePyro;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::prelude::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType, GlobalLinkConfig};
use crate::damage::damage_result::SimpleDamageResult;
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

pub struct LuminePyroDefaultTargetFunction {
    pub vaporize_rate: f64,
    pub melt_rate: f64,
    pub gosoythoth: bool,
}

impl TargetFunctionMetaTrait for LuminePyroDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::LuminePyroDefault,
        name_locale: locale!(
            zh_cn: "荧-火-旅行者",
            en: "Lumine-Pyro-Traveller"
        ),
        description: locale!(
            zh_cn: "旅行者荧-火前台每轮总伤害",
            en: "Traveller Lumine-Pyro Total DMG per Round"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::LuminePyro),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "vaporize_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "蒸发占比",
                en: "Vaporize Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "melt_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "融化占比",
                en: "Melt Ratio",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "gosoythoth",
            title: locale!(
                zh_cn: "对抗「古斯托特」化形的蚀灭的源焰之主",
                en: "Opposing the Lord of Eroded Primal Fire incarnated by Gosoythoth",
            ),
            config: ItemConfigType::GlobalLinkBool { default: false, global_link:
                GlobalLinkConfig { key: "gosoythoth", priority: ItemConfig::PRIORITY_TARGETFUNCTION, team_shared: true }
            }
        },
        ItemConfig {
            name: "[obsidian_codex]set2_rate",
            title: locale!(
                zh_cn: "「黑曜秘典」二件套被动比例",
                en: "[Obsidian Codex] 2-Set Ratio"
            ),
            config: ItemConfigType::GlobalLinkFloat { min: 0.0, max: 1.0, default: 1.0, 
                global_link: GlobalLinkConfig { key: "[obsidian_codex]set2_rate", priority: ItemConfig::PRIORITY_TARGETFUNCTION, team_shared: false } 
            }
        },
        ItemConfig {
            name: "[obsidian_codex]set4_rate",
            title: locale!(
                zh_cn: "「黑曜秘典」四件套被动比例",
                en: "[Obsidian Codex] 4-Set Ratio"
            ),
            config: ItemConfigType::GlobalLinkFloat { min: 0.0, max: 1.0, default: 1.0, 
                global_link: GlobalLinkConfig { key: "[obsidian_codex]set4_rate", priority: ItemConfig::PRIORITY_TARGETFUNCTION, team_shared: false } 
            }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (vaporize_rate, melt_rate, gosoythoth) = match *config {
            TargetFunctionConfig::LuminePyroDefault { vaporize_rate, melt_rate, gosoythoth } => (vaporize_rate, melt_rate, gosoythoth),
            _ => (0.0, 0.0, false)
        };
        Box::new(LuminePyroDefaultTargetFunction {
            vaporize_rate,
            melt_rate,
            gosoythoth,
        })
    }
}

impl TargetFunction for LuminePyroDefaultTargetFunction {
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

        let config = CharacterSkillConfig::LuminePyro {
            gosoythoth: self.gosoythoth,
            nightsouls_blessing: true,
            active: true,
            activated_q: true,
        };

        let calc_damage = |dmg: SimpleDamageResult| -> f64 {
            dmg.normal.expectation * (1.0 - self.vaporize_rate - self.melt_rate)
                + dmg.vaporize.unwrap_or(dmg.normal).expectation * self.vaporize_rate
                + dmg.melt.unwrap_or(dmg.normal).expectation * self.melt_rate
        };

        type DmgType = <LuminePyro as CharacterTrait>::DamageEnumType;
        let c_count = context.character_common_data.constellation;

        let dmg_q = calc_damage(LuminePyro::damage::<SimpleDamageBuilder>(&context, DmgType::Q, &CharacterSkillConfig::LuminePyro {
            gosoythoth: self.gosoythoth,
            nightsouls_blessing: true,
            active: true,
            activated_q: false,
        }, None));

        let dmg_e = calc_damage(LuminePyro::damage::<SimpleDamageBuilder>(&context, DmgType::E1, &config, None));

        let dmg_a = calc_damage(LuminePyro::damage::<SimpleDamageBuilder>(&context, DmgType::A1, &config, None))
            + calc_damage(LuminePyro::damage::<SimpleDamageBuilder>(&context, DmgType::A2, &config, None))
            + calc_damage(LuminePyro::damage::<SimpleDamageBuilder>(&context, DmgType::A3, &config, None));
        
        let dmg_z = calc_damage(LuminePyro::damage::<SimpleDamageBuilder>(&context, DmgType::Z1, &config, None))
            + calc_damage(LuminePyro::damage::<SimpleDamageBuilder>(&context, DmgType::Z2, &config, None));

        dmg_e * 12.0 + dmg_q + (dmg_a + dmg_z) * 5.0
    }
}
