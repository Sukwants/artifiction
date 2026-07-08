use std::f64::NAN;
use crate::target_functions::target_functions::prelude::*;
use crate::character::characters::electro::yae_miko::YaeMiko;
use crate::damage::transformative_damage::transformative_damage;
use crate::target_functions::TargetFunctionOptConfig;

pub struct YaeMikoDefaultTargetFunction {
    // 充能需求
    pub recharge_requirement: f64,
    // 连招：0 => 仅依靠E和Q；1 => 在0的基础上一直A。
    pub combo: usize,
    // 超激化比例
    pub aggravate_rate: f64,
    // 超绽放比例
    pub hyperbloom_rate: f64,
    // 天赋3强化落雷触发率（每几次E触发一次，默认3）
    pub p3_trigger_rate: f64,
}

impl YaeMikoDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> Self {
        let (
            recharge_requirement,
            combo,
            aggravate_rate,
            hyperbloom_rate,
            p3_trigger_rate,
        ) = match *config {
            TargetFunctionConfig::YaeMikoDefault {
                recharge_requirement,
                combo,
                aggravate_rate,
                hyperbloom_rate,
                p3_trigger_rate,
            } =>
                (
                    recharge_requirement,
                    combo,
                    aggravate_rate,
                    hyperbloom_rate,
                    p3_trigger_rate,
                ),
            _ => (0.0, 0, 0.0, 0.0, 1.0 / 3.0)
        };
        Self {
            recharge_requirement,
            combo,
            aggravate_rate,
            hyperbloom_rate,
            p3_trigger_rate,
        }
    }
}

impl TargetFunction for YaeMikoDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .thundersoother(1.0)
            .gilded_dreams(1, 2, 1.0)
            .build()
    }

    fn target(&self, attribute: &TargetFunctionAttributeType, character: &Character<TargetFunctionAttributeType>, _weapon: &Weapon<TargetFunctionAttributeType>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let config_e_normal = CharacterSkillConfig::YaeMiko { sesshou_sakura_level: 3, sesshou_sakura_count: 3, p3_enhanced: false };
        let config_e_p3 = CharacterSkillConfig::YaeMiko { sesshou_sakura_level: 3, sesshou_sakura_count: 3, p3_enhanced: true };

        // 普通 E
        let mut attr_e_normal = (*attribute).clone();
        YaeMiko::change_attribute::<TargetFunctionAttributeType>(&mut attr_e_normal, &character.common_data, &config_e_normal);
        let solved_e_normal = attr_e_normal.solve();
        let ctx_e_normal = DamageContext { character_common_data: &character.common_data, attribute: &solved_e_normal, enemy };

        // 天赋3强化 E
        let mut attr_e_p3 = (*attribute).clone();
        YaeMiko::change_attribute::<TargetFunctionAttributeType>(&mut attr_e_p3, &character.common_data, &config_e_p3);
        let solved_e_p3 = attr_e_p3.solve();
        let ctx_e_p3 = DamageContext { character_common_data: &character.common_data, attribute: &solved_e_p3, enemy };

        // 从角色配置读取极星辉域状态
        let in_polestar_field = match &character.common_data.config {
            CharacterConfig::YaeMiko { in_polestar_field, .. } => *in_polestar_field,
            _ => false,
        };

        type S = <YaeMiko as CharacterTrait>::DamageEnumType;

        let dmg_e = YaeMiko::damage::<SimpleDamageBuilder>(&ctx_e_normal, S::E3, &config_e_normal, None);
        let dmg_e_p3 = YaeMiko::damage::<SimpleDamageBuilder>(&ctx_e_p3, S::E3, &config_e_p3, None);
        let dmg_a1 = YaeMiko::damage::<SimpleDamageBuilder>(&ctx_e_normal, S::A1, &config_e_normal, None);
        let dmg_a2 = YaeMiko::damage::<SimpleDamageBuilder>(&ctx_e_normal, S::A2, &config_e_normal, None);
        let dmg_a3 = YaeMiko::damage::<SimpleDamageBuilder>(&ctx_e_normal, S::A3, &config_e_normal, None);

        let dmg_e_norm = dmg_e.normal.expectation;
        let dmg_e_p3_norm = dmg_e_p3.normal.expectation;
        let dmg_a1_norm = dmg_a1.normal.expectation;
        let dmg_a2_norm = dmg_a2.normal.expectation;
        let dmg_a3_norm = dmg_a3.normal.expectation;

        let dmg_e_aggravate = dmg_e.aggravate.unwrap_or(dmg_e.normal).expectation;
        let dmg_a1_aggravate = dmg_a1.aggravate.unwrap_or(dmg_a1.normal).expectation;
        let dmg_a2_aggravate = dmg_a2.aggravate.unwrap_or(dmg_a2.normal).expectation;
        let dmg_a3_aggravate = dmg_a3.aggravate.unwrap_or(dmg_a3.normal).expectation;

        let dmg_e_aggravate_bonus = dmg_e_aggravate - dmg_e_norm;
        let dmg_a1_aggravate_bonus = dmg_a1_aggravate - dmg_a1_norm;
        let dmg_a2_aggravate_bonus = dmg_a2_aggravate - dmg_a2_norm;
        let dmg_a3_aggravate_bonus = dmg_a3_aggravate - dmg_a3_norm;

        let mut dmg_hyperbloom = 0.0;
        if self.hyperbloom_rate > 0.0 {
            let transformative = transformative_damage::<SimpleDamageBuilder>(character.common_data.level, &solved_e_normal, enemy);
            dmg_hyperbloom = transformative.hyperbloom.expectation;
        }

        // 天赋3强化落雷: 每 1/p3_trigger_rate 次E触发一次 (默认每3次1次)
        let p3_count = (12.0 * self.p3_trigger_rate).round();
        let normal_e_count = 12.0 - p3_count;

        // 一轮12s, 12下E伤害 (含天赋3强化), 6轮A伤害
        let dmg_sum_normal = match self.combo {
            0 => dmg_e_norm * normal_e_count + dmg_e_p3_norm * p3_count + (dmg_a1_norm + dmg_a2_norm + dmg_a3_norm) * 0.0,
            1 => dmg_e_norm * normal_e_count + dmg_e_p3_norm * p3_count + (dmg_a1_norm + dmg_a2_norm + dmg_a3_norm) * 6.0,
            _ => NAN
        };

        // E的激化率约为1/3 (对单), A的激化率约为1/2
        // 天赋3强化的E伤害激化收益单独计算
        let dmg_e_p3_aggravate = dmg_e_p3.aggravate.unwrap_or(dmg_e_p3.normal).expectation;
        let dmg_e_p3_aggravate_bonus = dmg_e_p3_aggravate - dmg_e_p3_norm;

        let dmg_sum_aggravate_bonus = match self.combo {
            0 => dmg_e_aggravate_bonus * normal_e_count / 3.0 + dmg_e_p3_aggravate_bonus * p3_count / 3.0 + (dmg_a1_aggravate_bonus + dmg_a2_aggravate_bonus + dmg_a3_aggravate_bonus) * 0.0 / 2.0,
            1 => dmg_e_aggravate_bonus * normal_e_count / 3.0 + dmg_e_p3_aggravate_bonus * p3_count / 3.0 + (dmg_a1_aggravate_bonus + dmg_a2_aggravate_bonus + dmg_a3_aggravate_bonus) * 6.0 / 2.0,
            _ => NAN
        };

        // 天赋3星超导额外伤害 (仅极星辉域下生效, 从角色配置读取)
        let mut dmg_sum_p3_sc = 0.0;
        if in_polestar_field {
            let dmg_p3_sc = YaeMiko::damage::<SimpleDamageBuilder>(&ctx_e_p3, S::P3_SC, &config_e_p3, None);
            dmg_sum_p3_sc = dmg_p3_sc.normal.expectation * p3_count;
        }

        // 超绽放伤害冷却为0.5s/2次
        let dmg_sum_hyperbloom = 12.0 / (0.5 / 2.0);

        let r = solved_e_normal.get_value(AttributeName::Recharge).min(self.recharge_requirement);
        r * (dmg_sum_normal +
            dmg_sum_aggravate_bonus * self.aggravate_rate +
            dmg_sum_hyperbloom * self.hyperbloom_rate +
            dmg_sum_p3_sc)
    }
}

impl TargetFunctionMetaTrait for YaeMikoDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::YaeMikoDefault,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "八重神子-浮世笑百姿",
            en: "Yae-Astute Amusement"
        ),
        description: crate::common::i18n::locale!(
            zh_cn: "按照一轮12s：三阶杀生樱12下、普通攻击6×3下计算。由于杀生樱的激化率为1/3、普通攻击的激化率为1/2，在激元素充足的情况下（超激化比例=1），所以一轮杀生樱最大激化4下、普通攻击期望最大9下。超激化比例是根据激元素的充足与否决定实际激化数占最大激化数的比例。超绽放比例是根据草种子的重组与否决定实际绽放的种子数占最大绽放的种子数（0.5s/2个）的比例。",
            en: "DPS Yae Miko"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::YaeMiko),
        image: TargetFunctionMetaImage::Avatar,
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_requirement",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.0 },
        },
        ItemConfig {
            name: "combo",
            title: crate::common::i18n::locale!(
                zh_cn: "连招选择",
                en: "Combo",
            ), //连招选择
            config: ItemConfigType::Option { options: "不站场平A,站场平A", default: 0 },
        },
        ItemConfig {
            name: "aggravate_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "超激化比例",
                en: "Aggravate Ratio",
            ), //超激化比例
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "hyperbloom_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "超绽放比例",
                en: "Hyperbloom Ratio",
            ), //超绽放比例
            config: ItemConfigType::Float { min: 0.0, max: 4.0, default: 0.0 },
        },
        ItemConfig {
            name: "p3_trigger_rate",
            title: crate::common::i18n::locale!(
                zh_cn: "天赋3强化触发率 (1/N)",
                en: "P3 Enhanced Trigger Rate (1/N)",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 / 3.0 },
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(YaeMikoDefaultTargetFunction::new(config))
    }
}
