// Genertaed by GitHub Copilot (DeepSeek V4.1 Flash 1017)
use crate::character::characters::prelude::*;
use crate::target_functions::target_functions::{MityaOffFieldTargetFunction, MityaOnFieldTargetFunction};

pub const CORE_NONE: usize = 0;
pub const CORE_STEADY: usize = 1;
pub const CORE_OVERLOAD: usize = 2;

// 当前携带数量由技能配置 `beacon_count` 给出；聚爆总伤害计算六命返还的期望，
// 过程中的额外生成由技能开关控制。持续时间与能量恢复等不直接改变伤害的效果不单独建模。

// 递变信标的初始可携带上限，命座1提升至 5 枚
pub const MITYA_BEACON_LIMIT: usize = 4;
pub const MITYA_BEACON_LIMIT_C1: usize = 5;

pub(crate) fn detonation_beacon_count(constellation: i32, carried: usize, generated_during_charge: bool) -> usize {
    let limit = if constellation >= 1 { MITYA_BEACON_LIMIT_C1 } else { MITYA_BEACON_LIMIT };
    let carried = carried.min(limit);
    if carried == 0 {
        return 0;
    }
    // 约定：额外信标在消耗过程中出现空位时生成，保证计入一枚。
    // 此值是整次聚爆可消耗的信标总数，并非同一时刻的携带数量，不再按携带上限截断。
    carried + usize::from(generated_during_charge)
}

pub(crate) fn expected_detonation_beacons(constellation: i32, carried: usize, generated_during_charge: bool) -> f64 {
    let count = detonation_beacon_count(constellation, carried, generated_during_charge) as f64;
    // 六命每次消耗均有概率返还，返还的信标继续参与后续消耗。
    let refund_chance = if constellation >= 6 { MITYA_SKILL.c6_beacon_create_chance } else { 0.0 };
    count / (1.0 - refund_chance)
}

pub struct MityaSkillType {
    // Normal Attack: 链式反应 / Chain Reaction
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub z_dmg: [f64; 15],
    pub z_detonation_dmg: [f64; 15],    // 递变聚爆伤害 / Gradient Detonation DMG
    pub z_detonation_sc_dmg: [f64; 15], // 递变聚爆星超导伤害 / Gradient Detonation Stellar-Conduct DMG
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    // Elemental Skill: 律定·变量侦测 / Principle: Variable Detection
    pub e_tap_dmg: [f64; 15],
    pub e_hold_dmg: [f64; 15],
    pub e_core_dmg: [f64; 15],          // 稳态炉心伤害 / Steady State Core DMG
    pub e_core_consume_dmg: [f64; 15],  // 稳态炉心消耗递变信标伤害 / Steady State Core Gradient Beacon Consumption DMG
    pub e_core_detonate_dmg: [f64; 15], // 稳态炉心引爆伤害 / Steady State Core Detonation DMG

    // Elemental Burst: 解析·递变之理 / Analysis: Gradient Theory
    pub q_dmg_atk: [f64; 15],
    pub q_dmg_em: [f64; 15],

    // 天赋1：基准承压测验 / Baseline Stress Test
    pub p1_em_per_beacon: f64,

    // 天赋2：超限易爆试剂 / Explosive Reagent: Safe Limits Exceeded
    pub p2_crit_rate_per_stack: f64,

    // 天赋3：星耀祝礼·极密协议 / Stellar Jubilee: Top-Secret Protocol
    pub p3_stellar_conduct_base_per_em: f64,
    pub p3_stellar_conduct_base_max: f64,

    // 命座1：缺位寻踪的观察者 / The Observer Seeking What Is Missing
    pub c1_crit_damage: f64,
    pub c1_extra_beacon: f64, // 施放元素战技时额外创造的递变信标数量

    // 命座2：惰性之外的沉寂物 / The Matter That Slumbers Beyond Inertness
    pub c2_res_minus: f64,
    pub c2_steady_em: f64,
    pub c2_overload_em: f64,

    // 命座4：衰变相生的同位素 / The Isotopes Born of Decay
    pub c4_dmg: f64,

    // 命座6：元素阶跃的理之门 / Elemental Transition at the Gate of Reason
    pub c6_steady_core_multiplier: f64,
    pub c6_beacon_create_chance: f64, // 过载炉心消耗递变信标时创造新递变信标的概率
    pub c6_stellar_conduct_elevate: f64,
}

pub const MITYA_SKILL: MityaSkillType = MityaSkillType {
    // Normal Attack: 链式反应 / Chain Reaction
    a_dmg1: [0.587984, 0.632083, 0.676182, 0.73498, 0.779079, 0.823178, 0.881976, 0.940774, 0.999573, 1.058371, 1.11717, 1.175968, 1.249466, 1.322964, 1.396462],
    a_dmg2: [0.483008, 0.519234, 0.555459, 0.60376, 0.639986, 0.676211, 0.724512, 0.772813, 0.821114, 0.869414, 0.917715, 0.966016, 1.026392, 1.086768, 1.147144],
    a_dmg3: [0.690224, 0.741991, 0.793758, 0.86278, 0.914547, 0.966314, 1.035336, 1.104358, 1.173381, 1.242403, 1.311426, 1.380448, 1.466726, 1.553004, 1.639282],
    z_dmg: [1.3688, 1.47146, 1.57412, 1.711, 1.81366, 1.91632, 2.0532, 2.19008, 2.32696, 2.46384, 2.60072, 2.7376, 2.9087, 3.0798, 3.2509],
    z_detonation_dmg: [1.68, 1.806, 1.932, 2.1, 2.226, 2.352, 2.52, 2.688, 2.856, 3.024, 3.192, 3.36, 3.57, 3.78, 3.99],
    z_detonation_sc_dmg: [2.24, 2.408, 2.576, 2.8, 2.968, 3.136, 3.36, 3.584, 3.808, 4.032, 4.256, 4.48, 4.76, 5.04, 5.32],
    x_dmg1: [0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064, 1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984],
    x_dmg2: [1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686, 2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036],
    x_dmg3: [1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632, 2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592],

    // Elemental Skill: 律定·变量侦测 / Principle: Variable Detection
    e_tap_dmg: [0.9072, 0.97524, 1.04328, 1.134, 1.20204, 1.27008, 1.3608, 1.45152, 1.54224, 1.63296, 1.72368, 1.8144, 1.9278, 2.0412, 2.1546],
    e_hold_dmg: [1.296, 1.3932, 1.4904, 1.62, 1.7172, 1.8144, 1.944, 2.0736, 2.2032, 2.3328, 2.4624, 2.592, 2.754, 2.916, 3.078],
    e_core_dmg: [0.72, 0.774, 0.828, 0.9, 0.954, 1.008, 1.08, 1.152, 1.224, 1.296, 1.368, 1.44, 1.53, 1.62, 1.71],
    e_core_consume_dmg: [0.8, 0.86, 0.92, 1.0, 1.06, 1.12, 1.2, 1.28, 1.36, 1.44, 1.52, 1.6, 1.7, 1.8, 1.9],
    e_core_detonate_dmg: [0.92, 0.989, 1.058, 1.15, 1.219, 1.288, 1.38, 1.472, 1.564, 1.656, 1.748, 1.84, 1.955, 2.07, 2.185],

    // Elemental Burst: 解析·递变之理 / Analysis: Gradient Theory
    q_dmg_atk: [3.336, 3.5862, 3.8364, 4.17, 4.4202, 4.6704, 5.004, 5.3376, 5.6712, 6.0048, 6.3384, 6.672, 7.089, 7.506, 7.923],
    q_dmg_em: [6.672, 7.1724, 7.6728, 8.34, 8.8404, 9.3408, 10.008, 10.6752, 11.3424, 12.0096, 12.6768, 13.344, 14.178, 15.012, 15.846],

    // 天赋1：基准承压测验 / Baseline Stress Test
    p1_em_per_beacon: 30.0,

    // 天赋2：超限易爆试剂 / Explosive Reagent: Safe Limits Exceeded
    p2_crit_rate_per_stack: 0.05,

    // 天赋3：星耀祝礼·极密协议 / Stellar Jubilee: Top-Secret Protocol
    p3_stellar_conduct_base_per_em: 0.00028,
    p3_stellar_conduct_base_max: 0.14,

    // 命座1：缺位寻踪的观察者 / The Observer Seeking What Is Missing
    c1_crit_damage: 0.4,
    c1_extra_beacon: 1.0,

    // 命座2：惰性之外的沉寂物 / The Matter That Slumbers Beyond Inertness
    c2_res_minus: 0.2,
    c2_steady_em: 123.0,
    c2_overload_em: 234.0,

    // 命座4：衰变相生的同位素 / The Isotopes Born of Decay
    c4_dmg: 3.0,

    // 命座6：元素阶跃的理之门 / Elemental Transition at the Gate of Reason
    c6_steady_core_multiplier: 1.15,
    c6_beacon_create_chance: 0.34,
    c6_stellar_conduct_elevate: 0.2,
};

pub const MITYA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Mitya,
    internal_name: "Mitya",
    element: Element::Electro,
    hp: [906, 2349, 3126, 4677, 5229, 6016, 6752, 7547, 8098, 8901, 9453, 10264, 10816, 11634, 12460],
    atk: [20, 53, 71, 106, 118, 136, 153, 171, 183, 201, 214, 232, 245, 263, 322],
    def: [62, 160, 213, 318, 356, 409, 459, 514, 551, 606, 643, 698, 736, 792, 848],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Catalyst,
    star: 5,
    skill_name1: locale!(
        zh_cn: "链式反应",
        en: "Chain Reaction",
    ),
    skill_name2: locale!(
        zh_cn: "律定·变量侦测",
        en: "Principle: Variable Detection",
    ),
    skill_name3: locale!(
        zh_cn: "解析·递变之理",
        en: "Analysis: Gradient Theory",
    ),
    name_locale: locale!(
        zh_cn: "米提亚",
        en: "Mitya",
    ),
};

pub struct MityaEffect {
    pub stellar_glimmer_state: StellarGlimmerState,
    pub stellar_conduct_application_count: usize,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for MityaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        // 角色基础元素精通
        attribute.set_value_by(AttributeName::ElementalMastery, "米提亚初始精通", 100.0);

        // 天赋3：队伍中附近的所有角色造成的星超导反应基础伤害提升，
        // 每点元素精通提升 0.028%，至多提升 14%。
        attribute.add_edge_s1to1(
            CharacterSelector::select_all(attribute),
            AttributeType::Panel(AttributeName::ElementalMastery),
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::ElevativeBase,
                ReactionType::StellarConduct,
            )),
            Arc::new(move |em: f64, _| {
                (em * MITYA_SKILL.p3_stellar_conduct_base_per_em).min(MITYA_SKILL.p3_stellar_conduct_base_max)
            }),
            "米提亚天赋3",
            EdgePriority::Invisible,
        );

        // 命座6：队伍中附近的所有角色造成的星超导反应伤害擢升 20%
        if self.common_data.constellation >= 6 {
            attribute.set_value_to_s(
                CharacterSelector::select_all(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                    AttributeVariableType::ElevativeElevate,
                    ReactionType::StellarConduct,
                )),
                "米提亚命座6",
                MITYA_SKILL.c6_stellar_conduct_elevate,
            );
        }

        ElevativeReaction::apply_stellar_conduct_extra_coefficient(attribute, self.stellar_conduct_application_count);
    }
}

damage_enum!(
    MityaDamageEnum
    A1
    A2
    A3
    Z
    ZSC
    ZSC_TOTAL
    X1
    X2
    X3
    ETap
    EHold
    ECore
    EConsume
    EDetonate
    EDetonate_TOTAL
    C4
    Q
);

impl MityaDamageEnum {
    pub fn get_element(&self) -> Element {
        Element::Electro
    }

    pub fn get_skill_type(&self) -> SkillType {
        use MityaDamageEnum::*;
        match *self {
            A1 | A2 | A3 => SkillType::NormalAttack,
            Z => SkillType::ChargedAttack,
            ZSC | ZSC_TOTAL => SkillType::Elevative,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            ETap | EHold | ECore => SkillType::ElementalSkill,
            EConsume | EDetonate | EDetonate_TOTAL => SkillType::Elevative,
            C4 => SkillType::Elevative,
            Q => SkillType::ElementalBurst,
        }
    }

    pub fn get_elevative_type(&self) -> Option<ElevativeReaction> {
        use MityaDamageEnum::*;
        match *self {
            ZSC | ZSC_TOTAL | EConsume | EDetonate | EDetonate_TOTAL | C4 => {
                Some(ElevativeReaction::StellarConductElectro)
            }
            _ => None,
        }
    }
}

pub struct Mitya;

#[derive(Copy, Clone, FromPrimitive)]
pub enum MityaRoleEnum {
    OnField,
    OffField,
}

impl CharacterTrait for Mitya {
    const STATIC_DATA: CharacterStaticData = MITYA_STATIC_DATA;
    type SkillType = MityaSkillType;
    const SKILL: Self::SkillType = MITYA_SKILL;
    type DamageEnumType = MityaDamageEnum;
    type RoleEnum = MityaRoleEnum;

    const DEFAULT_TAGS: Option<&'static [CharacterTag]> = None;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            MityaDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            Z locale!(zh_cn: "重击/递变聚爆伤害", en: "Charged Attack / Gradient Detonation DMG")
            ZSC locale!(zh_cn: "递变聚爆星超导伤害（单枚递变信标）", en: "Gradient Detonation Stellar-Conduct DMG (per Beacon)")
            ZSC_TOTAL locale!(zh_cn: "递变聚爆星超导总伤害（期望）", en: "Gradient Detonation Stellar-Conduct Total DMG (Expected)")
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            MityaDamageEnum
            ETap locale!(zh_cn: "短按技能伤害", en: "Tap Skill DMG")
            EHold locale!(zh_cn: "长按技能伤害", en: "Hold Skill DMG")
            ECore locale!(zh_cn: "稳态炉心伤害", en: "Steady State Core DMG")
            EConsume locale!(zh_cn: "稳态炉心消耗递变信标伤害", en: "Steady State Core Gradient Beacon Consumption DMG")
            EDetonate locale!(zh_cn: "稳态炉心引爆伤害（单枚递变信标）", en: "Steady State Core Detonation DMG (per Beacon)")
            EDetonate_TOTAL locale!(zh_cn: "稳态炉心引爆总伤害", en: "Steady State Core Detonation Total DMG")
            C4 locale!(zh_cn: "命座4伤害", en: "C4 DMG")
        ),
        skill3: skill_map!(
            MityaDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STELLAR_CONDUCT_APPLICATION_COUNT(0, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig::STELLAR_GLIMMER_STATE(StellarGlimmerState::StellarConduct, ItemConfig::PRIORITY_CHARACTER),
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "core_state",
            title: locale!(
                zh_cn: "侦测炉心状态",
                en: "Detection Core State"
            ),
            config: ItemConfigType::Option2 {
                options_zh: "无,稳态,过载",
                options_en: "None,Steady,Overload",
                default: 1,
            },
        },
        ItemConfig {
            name: "beacon_count",
            title: locale!(
                zh_cn: "携带的递变信标数量",
                en: "Gradient Beacons Carried"
            ),
            config: ItemConfigType::Int { min: 0, max: 5, default: 4 },
        },
        ItemConfig {
            name: "beacon_generated_during_charge",
            title: locale!(
                zh_cn: "聚爆释放过程中新生成信标",
                en: "Generate a Beacon During Gradient Detonation"
            ),
            config: ItemConfigType::Bool { default: false },
        },
        ItemConfig {
            name: "talent2_crit_stack",
            title: locale!(
                zh_cn: "「超限易爆试剂」暴击率层数（天赋2）",
                en: "Explosive Reagent CRIT Rate Stacks (Talent 2)"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 4 },
        },
    ]);

    fn change_attribute<A: Attribute>(attribute: &mut A, common_data: &CharacterCommonData, skill_config: &CharacterSkillConfig) {
        let stellar_glimmer_state = match &common_data.config {
            CharacterConfig::Mitya { stellar_glimmer_state, .. } => *stellar_glimmer_state,
            _ => StellarGlimmerState::None,
        };

        let (core_state, beacon_count, talent2_crit_stack) = match *skill_config {
            CharacterSkillConfig::Mitya { core_state, beacon_count, talent2_crit_stack, .. } => {
                (core_state, beacon_count, talent2_crit_stack)
            }
            _ => (CORE_NONE, 0, 0),
        };

        // 命座1 提升递变信标可携带上限至 5 枚，否则上限为 4 枚
        let max_beacon_count = if common_data.constellation >= 1 { MITYA_BEACON_LIMIT_C1 } else { MITYA_BEACON_LIMIT };
        let beacon_count = beacon_count.min(max_beacon_count);
        let core_exists = core_state != CORE_NONE;

        // 天赋1：稳态炉心的每枚递变信标使队伍中附近的所有角色元素精通提升 30 点
        if common_data.has_talent1 && core_state == CORE_STEADY && beacon_count > 0 {
            attribute.set_value_by_s(
                CharacterSelector::select_all(attribute),
                AttributeType::Panel(AttributeName::ElementalMastery),
                "米提亚天赋1",
                MITYA_SKILL.p1_em_per_beacon * beacon_count as f64,
            );
        }

        // 天赋2：创造递变信标后，米提亚的暴击率提升 5%，至多叠加 4 层
        if common_data.has_talent2 && talent2_crit_stack > 0 {
            attribute.set_value_by_t(
                AttributeType::Panel(AttributeName::CriticalBase),
                "米提亚天赋2",
                MITYA_SKILL.p2_crit_rate_per_stack * talent2_crit_stack.min(4) as f64,
            );
        }

        // 命座1：侦测炉心存在期间，队伍中自己的当前场上角色暴击伤害提升 40%
        if common_data.constellation >= 1 && core_exists {
            attribute.set_value_to_s(
                CharacterSelector::select_onfield(attribute),
                AttributeType::Panel(AttributeName::CriticalDamageBase),
                "米提亚命座1",
                MITYA_SKILL.c1_crit_damage,
            );
        }

        if common_data.constellation >= 2 && core_exists {
            // 命座2：侦测炉心存在期间，处于极星辉域内的所有敌人冰元素抗性与雷元素抗性降低 20%
            // 按当前约定，处于极星辉域与辉映·星超导等价，统一使用角色的辉映状态。
            if stellar_glimmer_state.is_stellar_conduct() {
                for element in [Element::Cryo, Element::Electro] {
                    attribute.set_value_to_s(
                        CharacterSelector::select_all(attribute),
                        AttributeType::Invisible(InvisibleAttributeType::new_element(
                            AttributeVariableType::ResMinus,
                            element,
                        )),
                        "米提亚命座2",
                        MITYA_SKILL.c2_res_minus,
                    );
                }
            }

            // 命座2：稳态炉心使队伍中附近的所有角色元素精通提升 123 点；
            // 过载炉心使队伍中自己的当前场上角色元素精通提升 234 点
            match core_state {
                CORE_STEADY => attribute.set_value_by_s(
                    CharacterSelector::select_all(attribute),
                    AttributeType::Panel(AttributeName::ElementalMastery),
                    "米提亚命座2",
                    MITYA_SKILL.c2_steady_em,
                ),
                CORE_OVERLOAD => attribute.set_value_by_s(
                    CharacterSelector::select_onfield(attribute),
                    AttributeType::Panel(AttributeName::ElementalMastery),
                    "米提亚命座2",
                    MITYA_SKILL.c2_overload_em,
                ),
                _ => {}
            }
        }
    }

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: MityaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (core_state, beacon_count, beacon_generated_during_charge) = match *config {
            CharacterSkillConfig::Mitya { core_state, beacon_count, beacon_generated_during_charge, .. } => {
                (core_state, beacon_count, beacon_generated_during_charge)
            }
            _ => (CORE_NONE, 0, false),
        };

        let max_beacon_count = if context.character_common_data.constellation >= 1 { MITYA_BEACON_LIMIT_C1 } else { MITYA_BEACON_LIMIT };
        let beacon_count = beacon_count.min(max_beacon_count);
        let steady_core = core_state == CORE_STEADY;
        let overload_core = core_state == CORE_OVERLOAD;

        use MityaDamageEnum::*;
        let mut builder = D::new();

        // 递变聚爆及其星超导伤害需要过载炉心且至少持有一枚递变信标
        if matches!(s, ZSC | ZSC_TOTAL) && (!overload_core || beacon_count == 0) {
            return builder.none();
        }

        // 稳态炉心的伤害需要稳态炉心存在
        if matches!(s, ECore | EConsume | EDetonate | EDetonate_TOTAL) && !steady_core {
            return builder.none();
        }
        if s == EDetonate_TOTAL && beacon_count == 0 {
            return builder.none();
        }

        // 命座4 需要解锁命座且场上存在自己创造的侦测炉心
        if s == C4 && (context.character_common_data.constellation < 4 || !(steady_core || overload_core)) {
            return builder.none();
        }

        // 命座6（稳态）：侦测炉心造成的伤害提升至原本的 115%，对炉心的所有伤害生效
        let c6_steady_multiplier = if context.character_common_data.constellation >= 6 && steady_core {
            MITYA_SKILL.c6_steady_core_multiplier
        } else {
            1.0
        };

        if s == Q {
            builder.add_atk_ratio("技能倍率", MITYA_SKILL.q_dmg_atk[s3]);
            builder.add_em_ratio("技能倍率", MITYA_SKILL.q_dmg_em[s3]);
        } else if matches!(s, ZSC | ZSC_TOTAL | EConsume | EDetonate | EDetonate_TOTAL | C4) {
            let ratio = match s {
                ZSC | ZSC_TOTAL => MITYA_SKILL.z_detonation_sc_dmg[s1],
                EConsume => MITYA_SKILL.e_core_consume_dmg[s2] * c6_steady_multiplier,
                EDetonate => MITYA_SKILL.e_core_detonate_dmg[s2] * c6_steady_multiplier,
                EDetonate_TOTAL => MITYA_SKILL.e_core_detonate_dmg[s2] * beacon_count as f64 * c6_steady_multiplier,
                C4 => MITYA_SKILL.c4_dmg * c6_steady_multiplier,
                _ => 0.0,
            };
            builder.add_em_ratio("技能倍率", ratio);
        } else {
            let ratio = match s {
                A1 => MITYA_SKILL.a_dmg1[s1],
                A2 => MITYA_SKILL.a_dmg2[s1],
                A3 => MITYA_SKILL.a_dmg3[s1],
                Z => {
                    if overload_core && beacon_count > 0 {
                        MITYA_SKILL.z_detonation_dmg[s1]
                    } else {
                        MITYA_SKILL.z_dmg[s1]
                    }
                }
                X1 => MITYA_SKILL.x_dmg1[s1],
                X2 => MITYA_SKILL.x_dmg2[s1],
                X3 => MITYA_SKILL.x_dmg3[s1],
                ETap => MITYA_SKILL.e_tap_dmg[s2],
                EHold => MITYA_SKILL.e_hold_dmg[s2],
                ECore => MITYA_SKILL.e_core_dmg[s2] * c6_steady_multiplier,
                _ => 0.0,
            };
            builder.add_atk_ratio("技能倍率", ratio);
        }

        if let Some(elevative_type) = s.get_elevative_type() {
            if s == ZSC_TOTAL {
                // 倍率与固定加成保留单枚数值，独立的期望伤害次数参与总伤害结算。
                builder.set_damage_count(expected_detonation_beacons(
                    context.character_common_data.constellation,
                    beacon_count,
                    beacon_generated_during_charge,
                ));
            }
            builder.elevative(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                elevative_type,
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        } else {
            builder.damage(
                &context.attribute,
                &context.enemy,
                s.get_element(),
                s.get_skill_type(),
                context.character_common_data.level,
                fumo,
            )
        }
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (stellar_glimmer_state, stellar_conduct_application_count) = match *config {
            CharacterConfig::Mitya { stellar_glimmer_state, stellar_conduct_application_count } => {
                (stellar_glimmer_state, stellar_conduct_application_count)
            }
            _ => (StellarGlimmerState::None, 0),
        };
        Some(Box::new(MityaEffect {
            stellar_glimmer_state,
            stellar_conduct_application_count,
            common_data: common_data.clone(),
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        let role: MityaRoleEnum = num::FromPrimitive::from_usize(role_index).unwrap_or(MityaRoleEnum::OnField);
        match role {
            MityaRoleEnum::OnField => Box::new(MityaOnFieldTargetFunction {}),
            MityaRoleEnum::OffField => Box::new(MityaOffFieldTargetFunction {
                recharge_demand: 1.6,
            }),
        }
    }
}
