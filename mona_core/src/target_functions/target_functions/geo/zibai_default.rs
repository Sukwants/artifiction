use crate::target_functions::target_functions::prelude::*;
use crate::character::characters::Zibai;

pub struct ZibaiDefaultTargetFunction {
    pub lunar_crystallize_coefficient: f64,
}

impl TargetFunctionMetaTrait for ZibaiDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::ZibaiDefault,
        name_locale: locale!(
            zh_cn: "兹白-驹隙隐泉",
            en: "Zibai-White Horse's Fleeting Spring"
        ),
        description: locale!(
            zh_cn: "兹白一轮输出总伤害",
            en: "Zibai Total Damage Output per Round"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Zibai),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "lunar_crystallize_coefficient",
            title: locale!(
                zh_cn: "月结晶伤害系数",
                en: "Lunar Crystallize DMG Coefficient",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
        ItemConfig {
            name: "[night_of_the_skys_unveiling]on_field",
            title: locale!(
                zh_cn: "「穹境示现之夜」位于场上",
                en: "[Night of the Sky's Unveiling] On Field",
            ),
            config: ItemConfigType::GlobalLinkBool { default: true, 
                global_link: GlobalLinkConfig { key: "[night_of_the_skys_unveiling]on_field", priority: ItemConfig::PRIORITY_TARGETFUNCTION, team_shared: false } 
            }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let lunar_crystallize_coefficient = match *config {
            TargetFunctionConfig::ZibaiDefault { lunar_crystallize_coefficient } => lunar_crystallize_coefficient,
            _ => 0.0,
        };
        Box::new(ZibaiDefaultTargetFunction {
            lunar_crystallize_coefficient,
        })
    }
}

impl TargetFunction for ZibaiDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            // .night_of_the_skys_unveiling(Moonsign::Ascendant, true)
            .build()
    }

    fn target(&self, attribute: &TargetFunctionAttributeType, character: &Character<TargetFunctionAttributeType>, weapon: &Weapon<TargetFunctionAttributeType>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let config = CharacterSkillConfig::Zibai { lunar_phase_shift: true, activated_c1: false, activated_c4: true, stack_c6: 30 };
        let config1 = CharacterSkillConfig::Zibai { lunar_phase_shift: true, activated_c1: true, activated_c4: true, stack_c6: 30 };
        
        let context = {
            let mut attribute_temp = (*attribute).clone();
            Zibai::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config);
            DamageContext { character_common_data: &character.common_data, attribute: &attribute_temp.solve(), enemy }
        };
        let context1 = {
            let mut attribute_temp = (*attribute).clone();
            Zibai::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config1);
            DamageContext { character_common_data: &character.common_data, attribute: &attribute_temp.solve(), enemy }
        };

        let moonsign = match &character.common_data.config {
            CharacterConfig::Zibai { moonsign, geo_count, hydro_count } => *moonsign,
            _ => Moonsign::None,
        };

        type Ty = <Zibai as CharacterTrait>::DamageEnumType;

        let dmg_ea =
            Zibai::damage::<SimpleDamageBuilder>(&context, Ty::EA1, &config, None).normal.expectation
            + Zibai::damage::<SimpleDamageBuilder>(&context, Ty::EA2, &config, None).normal.expectation
            + Zibai::damage::<SimpleDamageBuilder>(&context, Ty::EA31, &config, None).normal.expectation
            + Zibai::damage::<SimpleDamageBuilder>(&context, Ty::EA32, &config, None).normal.expectation
            + Zibai::damage::<SimpleDamageBuilder>(&context, Ty::EA4, &config, None).normal.expectation
            + Zibai::damage::<SimpleDamageBuilder>(&context, Ty::EA4E, &config, None).normal.expectation;
        let dmg_e =
            Zibai::damage::<SimpleDamageBuilder>(&context, Ty::E1, &config, None).normal.expectation
            + Zibai::damage::<SimpleDamageBuilder>(&context, Ty::E2, &config, None).normal.expectation;
        let dmg_e1 =
            Zibai::damage::<SimpleDamageBuilder>(&context, Ty::E1, &config, None).normal.expectation
            + Zibai::damage::<SimpleDamageBuilder>(&context1, Ty::E2, &config1, None).normal.expectation;
        let dmg_q = 
            Zibai::damage::<SimpleDamageBuilder>(&context, Ty::Q1, &config, None).normal.expectation
            + Zibai::damage::<SimpleDamageBuilder>(&context, Ty::Q2, &config, None).normal.expectation;

        let dmg_lunar_crystallize = Zibai::moonglare_damage::<SimpleDamageBuilder>(&context, MoonglareReaction::LunarCrystallizeReaction).normal.expectation;

        dmg_ea * 4.0
        + dmg_e * if moonsign.is_ascendant() { 4.0 } else { 2.0 } + if character.common_data.constellation >= 1 { dmg_e1 } else { 0.0 }
        + dmg_q
        + dmg_lunar_crystallize * self.lunar_crystallize_coefficient * 6.0
    }
}
