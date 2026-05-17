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
        unimplemented!()
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
