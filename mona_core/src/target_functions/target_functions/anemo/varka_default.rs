use crate::target_functions::target_functions::prelude::*;
use crate::character::characters::Varka;

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

    fn target(&self, attribute: &TargetFunctionAttributeType, character: &Character<TargetFunctionAttributeType>, _weapon: &Weapon<TargetFunctionAttributeType>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let hexerei_secret_rite = match &character.common_data.config {
            CharacterConfig::Varka { hexerei_secret_rite, .. } => *hexerei_secret_rite,
            _ => false,
        };

        let config = CharacterSkillConfig::Varka { azure_fang_oath: 4, c1_bonus: false };
        let config_with_c1 = CharacterSkillConfig::Varka { azure_fang_oath: 4, c1_bonus: true };

        let context = {
            let mut attribute_temp = (*attribute).clone();
            Varka::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config);
            DamageContext { character_common_data: &character.common_data, attribute: &attribute_temp.solve(), enemy }
        };
        let context_with_c1 = {
            let mut attribute_temp = (*attribute).clone();
            Varka::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config_with_c1);
            DamageContext { character_common_data: &character.common_data, attribute: &attribute_temp.solve(), enemy }
        };

        type Ty = <Varka as CharacterTrait>::DamageEnumType;

        let get_damage = |context: &DamageContext<'_, TargetFunctionAttributeResultType>, s: Ty, config: &CharacterSkillConfig| -> f64 {
            Varka::damage::<SimpleDamageBuilder>(context, s, config, None).normal.expectation
        };

        let dmg_a = get_damage(&context, Ty::EA1, &config)
            + get_damage(&context, Ty::EA21, &config) + get_damage(&context, Ty::EA22, &config)
            + get_damage(&context, Ty::EA31, &config) + get_damage(&context, Ty::EA32, &config)
            + get_damage(&context, Ty::EA41, &config) + get_damage(&context, Ty::EA42, &config)
            + get_damage(&context, Ty::EA51, &config) + get_damage(&context, Ty::EA52, &config);

        let dmg_e = get_damage(&context, Ty::E, &config);

        let dmg_e1 = get_damage(&context_with_c1, Ty::EE1, &config_with_c1) + get_damage(&context_with_c1, Ty::EE2, &config_with_c1) + get_damage(&context_with_c1, Ty::C2E, &config_with_c1)
            + if character.common_data.constellation >= 6 { get_damage(&context_with_c1, Ty::EEZ1, &config_with_c1) + get_damage(&context_with_c1, Ty::EEZ2, &config_with_c1) } else { 0.0 };
        let dmg_e2 = get_damage(&context, Ty::EE1, &config) + get_damage(&context, Ty::EE2, &config) + get_damage(&context_with_c1, Ty::C2E, &config_with_c1)
            + if character.common_data.constellation >= 6 { get_damage(&context, Ty::EEZ1, &config) + get_damage(&context, Ty::EEZ2, &config) } else { 0.0 };

        dmg_e + if character.common_data.constellation >= 1 { dmg_e1 } else { 0.0 }
            + dmg_a * 2.0 + dmg_e1 + if hexerei_secret_rite { dmg_e2 } else { 0.0 }
    }
}
