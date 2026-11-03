// Genertaed by GitHub Copilot (DeepSeek V4.1 Flash 1017)
use crate::character::characters::electro::mitya::{
    detonation_beacon_count, CORE_OVERLOAD, MITYA_BEACON_LIMIT, MITYA_BEACON_LIMIT_C1, MITYA_SKILL,
};
use crate::character::characters::Mitya;
use crate::target_functions::target_functions::prelude::*;

pub struct MityaOnFieldTargetFunction {}

// 一轮从零累计消耗开始，连续两次满信标聚爆，每次在空位中额外生成一枚。
// 六命的触发次数只依赖固定循环，缓存以避免每套圣遗物重复计算概率分布。
lazy_static::lazy_static! {
    static ref C6_C4_COUNT: f64 = expected_c4_count(
        2 * detonation_beacon_count(6, MITYA_BEACON_LIMIT_C1, true),
        MITYA_SKILL.c6_beacon_create_chance,
    );
}

fn expected_c4_count(beacon_count: usize, refund_chance: f64) -> f64 {
    // 每枚初始/额外信标对应一个几何分布 G（包含最终不返还的一次消耗）。
    // 将 G mod 5 的分布卷积，精确计算 E[floor(N/5)] = (E[N] - E[N mod 5]) / 5。
    let mut remainder = [0.0; 5];
    remainder[0] = 1.0;
    let mut single = [0.0; 5];
    for count in 1..=5 {
        single[count % 5] = (1.0 - refund_chance) * refund_chance.powi(count as i32 - 1)
            / (1.0 - refund_chance.powi(5));
    }
    for _ in 0..beacon_count {
        let mut next = [0.0; 5];
        for i in 0..5 {
            for j in 0..5 {
                next[(i + j) % 5] += remainder[i] * single[j];
            }
        }
        remainder = next;
    }
    let expected_remainder: f64 = remainder.iter().enumerate().map(|(i, p)| i as f64 * p).sum();
    (beacon_count as f64 / (1.0 - refund_chance) - expected_remainder) / 5.0
}

impl TargetFunctionMetaTrait for MityaOnFieldTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::MityaOnField,
        name_locale: locale!(
            zh_cn: "米提亚-「前台输出」",
            en: "Mitya-On-Field"
        ),
        description: locale!(
            zh_cn: "米提亚前台一轮输出总伤害",
            en: "Mitya On-Field Total DMG per Round"
        ),
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Mitya),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(MityaOnFieldTargetFunction {})
    }
}

impl TargetFunction for MityaOnFieldTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 0.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.2,
            elemental_mastery: 1.0,
            critical: 1.0,
            critical_damage: 1.0,
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
                StatName::ElementalMastery,
            ],
            goblet_main_stats: vec![
                StatName::ElementalMastery,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::ElementalMastery,
            ],
            set_names: Some(vec![
                ArtifactSetName::DisenchantmentInDeepShadow,
                ArtifactSetName::GildedDreams,
            ]),
            very_critical_set_names: Some(vec![
                ArtifactSetName::DisenchantmentInDeepShadow,
                ArtifactSetName::ThunderingFury,
            ]),
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD,
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &TargetFunctionAttributeType, character: &Character<TargetFunctionAttributeType>, _weapon: &Weapon<TargetFunctionAttributeType>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let constellation = character.common_data.constellation;
        let beacon_limit = if constellation >= 1 { MITYA_BEACON_LIMIT_C1 } else { MITYA_BEACON_LIMIT };

        let config = CharacterSkillConfig::Mitya {
            core_state: CORE_OVERLOAD,
            beacon_count: beacon_limit,
            talent2_crit_stack: 4,
            // 维持既有循环约定：聚爆过程中在有空位时额外生成一枚。
            beacon_generated_during_charge: true,
        };

        let solved = {
            let mut attribute_temp = (*attribute).clone();
            Mitya::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config);
            attribute_temp.solve()
        };
        let context = DamageContext {
            character_common_data: &character.common_data,
            attribute: &solved,
            enemy,
        };

        type Ty = <Mitya as CharacterTrait>::DamageEnumType;
        let dmg = |s: Ty| -> f64 {
            Mitya::damage::<SimpleDamageBuilder>(&context, s, &config, None).normal.expectation
        };

        // 长按元素战技创造侦测炉心·过载
        let e = dmg(Ty::EHold);
        // 元素爆发，创造 3 枚递变信标
        let q = dmg(Ty::Q);
        // 特殊重击：1 次递变聚爆伤害 + 每枚递变信标 1 次星超导反应伤害
        let charged = dmg(Ty::Z) + dmg(Ty::ZSC_TOTAL);
        // 命座4：对随机消耗量逐个取整后求期望，不能对消耗量期望直接取整。
        let c4_count = if constellation >= 6 {
            *C6_C4_COUNT
        } else {
            (2 * detonation_beacon_count(constellation, beacon_limit, true) / 5) as f64
        };
        let c4 = dmg(Ty::C4) * c4_count;

        // 一轮输出：长按 E → 特殊重击 → Q → 特殊重击
        e + q + charged * 2.0 + c4
    }
}
