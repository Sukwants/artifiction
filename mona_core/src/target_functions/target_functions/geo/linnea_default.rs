use crate::target_functions::target_functions::prelude::*;
use crate::character::characters::Linnea;

pub struct LinneaDefaultTargetFunction {
    pub lunar_crystallize_coefficient: f64,
    pub def_demand: f64,
}

impl TargetFunctionMetaTrait for LinneaDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::LinneaDefault,
        name_locale: locale!(
            zh_cn: "莉奈娅-博闻异旅",
            en: "Linnea-White Horse's Fleeting Spring"
        ),
        description: locale!(
            zh_cn: "莉奈娅一轮后台总伤害",
            en: "Linnea Total Damage Output per Round"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Linnea),
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
            name: "def_demand",
            title: locale!(
                zh_cn: "防御力需求",
                en: "DEF Demand",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
        ItemConfig {
            name: "[aubade_of_morningstar_and_moon]set4_rate",
            title: locale!(
                zh_cn: "「晨星与月的晓歌」四件套被动比例",
                en: "[Aubade of Morningstar and Moon] 4-Set Ratio",
            ),
            config: ItemConfigType::GlobalLinkFloat { min: 0.0, max: 1.0, default: 1.0, 
                global_link: GlobalLinkConfig { key: "[aubade_of_morningstar_and_moon]set4_rate", priority: ItemConfig::PRIORITY_TARGETFUNCTION, team_shared: false } 
            }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (lunar_crystallize_coefficient, def_demand) = match *config {
            TargetFunctionConfig::LinneaDefault { lunar_crystallize_coefficient, def_demand } => (lunar_crystallize_coefficient, def_demand),
            _ => (0.0, 0.0),
        };
        Box::new(LinneaDefaultTargetFunction {
            lunar_crystallize_coefficient,
            def_demand,
        })
    }
}

impl TargetFunction for LinneaDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .aubade_of_morningstar_and_moon(1.0)
            .build()
    }

    fn target(&self, attribute: &TargetFunctionAttributeType, character: &Character<TargetFunctionAttributeType>, weapon: &Weapon<TargetFunctionAttributeType>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let config = CharacterSkillConfig::Linnea { field_catalog: 18 };
        
        let context = {
            let mut attribute_temp = (*attribute).clone();
            Linnea::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config);
            DamageContext { character_common_data: &character.common_data, attribute: &attribute_temp.solve(), enemy }
        };

        if context.attribute.get_def() < self.def_demand {
            return 0.0;
        }

        let moonsign = match &character.common_data.config {
            CharacterConfig::Linnea { moonsign, .. } => *moonsign,
            _ => Moonsign::None,
        };

        type Ty = <Linnea as CharacterTrait>::DamageEnumType;

        let dmg_e =
            Linnea::damage::<SimpleDamageBuilder>(&context, Ty::E1, &config, None).normal.expectation * 2.0
            + Linnea::damage::<SimpleDamageBuilder>(&context, Ty::E2, &config, None).normal.expectation;

        let dmg_lunar_crystallize = Linnea::elevative_damage::<SimpleDamageBuilder>(&context, ElevativeReaction::LunarCrystallizeReaction).normal.expectation;

        dmg_e * 6.0
        + dmg_lunar_crystallize * self.lunar_crystallize_coefficient * (
            6.0 + if moonsign.is_ascendant() && character.common_data.constellation >= 2 { 6.0 } else { 0.0 }
        )
    }
}
