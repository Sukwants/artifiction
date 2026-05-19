use crate::target_functions::target_functions::prelude::*;
use crate::character::characters::Nicole;

pub struct NicoleDefaultTargetFunction {
    pub recharge_demand: f64,
}

impl TargetFunctionMetaTrait for NicoleDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::NicoleDefault,
        name_locale: locale!(
            zh_cn: "尼可-「喧寂于心」",
            en: "Nicole-Clamor Within"
        ),
        description: locale!(
            zh_cn: "尼可实战攻击力",
            en: "Nicole Practical ATK"
        ),
        tags: "辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::Nicole),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: locale!(
                zh_cn: "充能需求",
                en: "Recharge Requirement",
            ),
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.6 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let recharge_demand = match *config {
            TargetFunctionConfig::NicoleDefault { recharge_demand } => recharge_demand,
            _ => 1.0,
        };
        Box::new(NicoleDefaultTargetFunction {
            recharge_demand,
        })
    }
}

impl TargetFunction for NicoleDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.6,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 1.0,
            elemental_mastery: 0.0,
            critical: 0.0,
            critical_damage: 0.0,
            healing_bonus: 0.0,
            bonus_electro: 0.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::Recharge,
                StatName::ATKPercentage,
            ],
            goblet_main_stats: vec![
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::NoblesseOblige,
                ArtifactSetName::ScrollOfTheHeroOfCinderCity,
            ]),
            very_critical_set_names: Some(vec![
                ArtifactSetName::CelestialGift,
            ]),
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD,
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &TargetFunctionAttributeType, character: &Character<TargetFunctionAttributeType>, _weapon: &Weapon<TargetFunctionAttributeType>, _artifacts: &[&Artifact], _enemy: &Enemy) -> f64 {
        let config = CharacterSkillConfig::Nicole { on_field_over_3s: true };

        let context = {
            let mut attribute_temp = (*attribute).clone();
            Nicole::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config);
            attribute_temp.solve()
        };

        let recharge = context.get_value(AttributeName::Recharge);
        if recharge < self.recharge_demand {
            return 0.0;
        }

        context.get_atk()
    }
}
