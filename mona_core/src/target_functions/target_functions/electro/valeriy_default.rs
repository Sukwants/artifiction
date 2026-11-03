// Genertaed by GitHub Copilot (DeepSeek V4.1 Flash 1017)
use crate::character::characters::Valeriy;
use crate::target_functions::target_functions::prelude::*;

pub struct ValeriyDefaultTargetFunction {
    pub recharge_demand: f64,
}

impl TargetFunctionMetaTrait for ValeriyDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::ValeriyDefault,
        name_locale: locale!(
            zh_cn: "瓦列里-攻击力",
            en: "Valeriy-ATK"
        ),
        description: locale!(
            zh_cn: "瓦列里实战攻击力（攻斥指令与蓄雷护盾均基于攻击力）",
            en: "Valeriy Practical ATK (Offensive Directive and Supercharged Shield both scale with ATK)"
        ),
        tags: "辅助",
        four: TargetFunctionFor::SomeWho(CharacterName::Valeriy),
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
            TargetFunctionConfig::ValeriyDefault { recharge_demand } => recharge_demand,
            _ => 1.0,
        };
        Box::new(ValeriyDefaultTargetFunction {
            recharge_demand,
        })
    }
}

impl TargetFunction for ValeriyDefaultTargetFunction {
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
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::EchoesOfAnOffering,
                ArtifactSetName::VermillionHereafter,
                ArtifactSetName::NoblesseOblige,
            ]),
            very_critical_set_names: Some(vec![
                ArtifactSetName::NoblesseOblige,
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
        // 攻斥指令与蓄雷护盾的加成均基于瓦列里的攻击力，故以攻击力为目标值。
        // 默认拥有攻斥指令，且特殊重击消耗 40 点冲击势能（进行特殊重击的最低要求）。
        let config = CharacterSkillConfig::Valeriy {
            offensive_directive: true,
            impact_momentum_consumed: 40,
            c2_overflow_momentum: 0,
        };

        let context = {
            let mut attribute_temp = (*attribute).clone();
            Valeriy::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config);
            attribute_temp.solve()
        };

        let recharge = context.get_value(AttributeName::Recharge);
        if recharge < self.recharge_demand {
            return 0.0;
        }

        context.get_atk()
    }
}
