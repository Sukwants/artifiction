use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{AttributeCommon, SimpleAttributeGraph2};
use crate::character::{Character, CharacterConfig, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Venti;
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

pub struct VentiDefaultTargetFunction {
}

impl TargetFunctionMetaTrait for VentiDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::VentiDefault,
        name_locale: locale!(
            zh_cn: "温迪-风色诗人",
            en: "Venti-Windborne Bard"
        ),
        description: locale!(
            zh_cn: "温迪前台一轮输出总伤害",
            en: "Venti main DPS total damage output"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Venti),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(VentiDefaultTargetFunction {
        })
    }
}

impl TargetFunction for VentiDefaultTargetFunction {
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

        let config1 = CharacterSkillConfig::Venti { activated_q: false, active: true, breeze_blow: false };
        let config2 = CharacterSkillConfig::Venti { activated_q: true, active: true, breeze_blow: true };
        let config3 = CharacterSkillConfig::Venti { activated_q: false, active: true, breeze_blow: true };

        type Ty = <Venti as CharacterTrait>::DamageEnumType;

        let get_damage = |s: Ty, config: &CharacterSkillConfig| -> f64 {
            Venti::damage::<SimpleDamageBuilder>(&context, s, config, None).normal.expectation
        };

        let dmg_a = (
                get_damage(Ty::A11, &config2)
                + get_damage(Ty::A12, &config2)
                + get_damage(Ty::A2, &config2)
                + get_damage(Ty::A3, &config2)
            ) * 6.0;

        let dmg_e = get_damage(Ty::E1, &config1)
            + get_damage(Ty::E1, &config2)
            + get_damage(Ty::E1, &config2)
            + get_damage(Ty::E1, &config3);
        
        let dmg_q = (get_damage(Ty::Q, &config2) + get_damage(Ty::QA, &config2)) * 10.0;

        dmg_a + dmg_e + dmg_q
    }
}
