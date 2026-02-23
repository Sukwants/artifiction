use crate::artifacts::Artifact;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::*;
use crate::character::{Character, CharacterConfig, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::Varka;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::prelude::CharacterTrait;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::{moonsign, Moonsign};
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::*;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct VarkaDefaultTargetFunction {
}

impl TargetFunctionMetaTrait for VarkaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::VarkaDefault,
        name_locale: locale!(
            zh_cn: "法尔伽-北风骑士",
            en: "Varka-Knight of Boreas"
        ),
        description: locale!(
            zh_cn: "法尔伽前台一轮输出总伤害",
            en: "Varka main DPS total damage output"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Varka),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(VarkaDefaultTargetFunction {
        })
    }
}

impl TargetFunction for VarkaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &TargetFunctionAttributeType, character: &Character<TargetFunctionAttributeType>, weapon: &Weapon<TargetFunctionAttributeType>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let attribute = &attribute.solve();
        let context: DamageContext<'_, TargetFunctionAttributeResultType> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        let hexerei_secret_rite = match &context.character_common_data.config {
            CharacterConfig::Varka { hexerei_secret_rite, .. } => *hexerei_secret_rite,
            _ => false,
        };

        let config = CharacterSkillConfig::Varka { azure_fang_oath: 4, c1_bonus: false };
        let config_with_c1 = CharacterSkillConfig::Varka { azure_fang_oath: 4, c1_bonus: true };

        type Ty = <Varka as CharacterTrait>::DamageEnumType;

        let get_damage = |s: Ty, config: &CharacterSkillConfig| -> f64 {
            Varka::damage::<SimpleDamageBuilder>(&context, s, config, None).normal.expectation
        };

        let dmg_a = get_damage(Ty::EA1, &config)
            + get_damage(Ty::EA21, &config) + get_damage(Ty::EA22, &config)
            + get_damage(Ty::EA31, &config) + get_damage(Ty::EA32, &config) 
            + get_damage(Ty::EA31, &config) + get_damage(Ty::EA32, &config)
            + get_damage(Ty::EA41, &config) + get_damage(Ty::EA42, &config)
            + get_damage(Ty::EA51, &config) + get_damage(Ty::EA52, &config);

        let dmg_e = get_damage(Ty::E, &config);

        let dmg_e1 = get_damage(Ty::EE1, &config_with_c1) + get_damage(Ty::EE2, &config_with_c1);
        let dmg_e2 = get_damage(Ty::EE1, &config) + get_damage(Ty::EE2, &config);

        dmg_e + dmg_a * 2.0 + dmg_e1 + if hexerei_secret_rite { dmg_e2 } else { 0.0 }
    }
}
