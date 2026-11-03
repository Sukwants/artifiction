// Genertaed by GitHub Copilot (DeepSeek V4.1 Flash 1017)
use crate::character::characters::prelude::*;
use crate::target_functions::target_functions::ValeriyDefaultTargetFunction;

// 冲击势能、攻斥指令的层数与持续时间等只限制可被强化的伤害次数或持续时间的数值不参与单次伤害计算；
// 攻斥指令单次提升值中由天赋 2 决定的部分由技能配置 `impact_momentum_consumed` 给出。
// 天赋 1（与徐进弹幕为伴）与命座 1、命座 4 只影响冲击势能的获取，不影响伤害，故不在此实现；
// 命座 3、命座 5 提升技能等级，按照约定不自动处理，由用户输入实际技能等级计算。

pub struct ValeriySkillType {
    // Normal Attack: 近卫战法·盾剑攻斥 / Guard Tactics: Sword and Shield
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub z_dmg: [f64; 15],        // 重击伤害（物理） / Charged Attack DMG
    pub z_cannon_dmg: [f64; 15], // 臼炮直射·抵近轰击伤害 / Direct-Fire Mortar: Point-Blank Bombardment DMG
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    // 攻斥指令 / Offensive Directive（随普通攻击技能等级提升）
    pub directive_electro_ratio: [f64; 15], // 雷元素伤害增加 / Electro DMG Bonus
    pub directive_sc_ratio: [f64; 15],      // 星超导反应伤害增加 / Stellar-Conduct DMG Bonus

    // 天赋 2：向防线缺口突击 / Into the Breach
    pub p2_electro_per_momentum: f64, // 不处于辉映·星超导状态时每点冲击势能的提升值
    pub p2_sc_per_momentum: f64,      // 处于辉映·星超导状态时每点冲击势能的提升值

    // Elemental Skill: 攻势防御·堡垒骤进 / Active Defense: Bastion Charge
    pub e_dmg: [f64; 15],
    pub e_shield_ratio: [f64; 15],
    pub e_shield_base: [f64; 15],

    // Elemental Burst: 火力焦点·雷霰急袭 / Focused Fire: Lightning Salvo
    pub q_dmg: [f64; 15],
    pub q_shell_dmg: [f64; 15], // 雷霰重弹伤害 / Thunderburst Shell DMG

    // 命座 2：伴随冲阵的队型 / Ranks That Charge as One
    pub c2_em: f64,
    pub c2_shield_per_momentum: f64,
    pub c2_shield_max_momentum: f64,

    // 命座 6：穿透敌阵的威能 / The Strength to Breach Enemy Lines
    pub c6_dmg_bonus: f64,
    pub c6_electro_crit_rate: f64,
    pub c6_sc_crit_damage: f64,
}

pub const VALERIY_SKILL: ValeriySkillType = ValeriySkillType {
    // Normal Attack: 近卫战法·盾剑攻斥
    a_dmg1: [0.324839, 0.35128, 0.37772, 0.415492, 0.441932, 0.47215, 0.513699, 0.555248, 0.596798, 0.642124, 0.68745, 0.732777, 0.778103, 0.82343, 0.868756],
    a_dmg2: [0.29326, 0.31713, 0.341, 0.3751, 0.39897, 0.42625, 0.46376, 0.50127, 0.53878, 0.5797, 0.62062, 0.66154, 0.70246, 0.74338, 0.7843],
    a_dmg3: [0.454175, 0.491142, 0.52811, 0.580921, 0.617889, 0.660137, 0.71823, 0.776322, 0.834414, 0.897787, 0.96116, 1.024533, 1.087907, 1.15128, 1.214653],
    z_dmg: [1.00276, 1.08438, 1.166, 1.2826, 1.36422, 1.4575, 1.58576, 1.71402, 1.84228, 1.9822, 2.12212, 2.26204, 2.40196, 2.54188, 2.6818],
    z_cannon_dmg: [2.292, 2.4639, 2.6358, 2.865, 3.0369, 3.2088, 3.438, 3.6672, 3.8964, 4.1256, 4.3548, 4.584, 4.8705, 5.157, 5.4435],
    x_dmg1: [0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572, 1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982],
    x_dmg2: [1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646, 2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915],
    x_dmg3: [1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586, 3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041],

    // 攻斥指令
    directive_electro_ratio: [0.2432, 0.26144, 0.27968, 0.304, 0.32224, 0.34048, 0.3648, 0.38912, 0.41344, 0.43776, 0.46208, 0.4864, 0.5168, 0.5472, 0.5776],
    directive_sc_ratio: [0.7296, 0.78432, 0.83904, 0.912, 0.96672, 1.02144, 1.0944, 1.16736, 1.24032, 1.31328, 1.38624, 1.4592, 1.5504, 1.6416, 1.7328],

    // 天赋 2：向防线缺口突击
    p2_electro_per_momentum: 0.003,
    p2_sc_per_momentum: 0.009,

    // Elemental Skill: 攻势防御·堡垒骤进
    e_dmg: [1.66, 1.7845, 1.909, 2.075, 2.1995, 2.324, 2.49, 2.656, 2.822, 2.988, 3.154, 3.32, 3.5275, 3.735, 3.9425],
    e_shield_ratio: [2.08896, 2.245632, 2.402304, 2.6112, 2.767872, 2.924544, 3.13344, 3.342336, 3.551232, 3.760128, 3.969024, 4.17792, 4.43904, 4.70016, 4.96128],
    e_shield_base: [1309.6384, 1440.6204, 1582.5176, 1735.33, 1899.0575, 2073.7002, 2259.258, 2455.7312, 2663.1194, 2881.4229, 3110.6414, 3350.7751, 3601.824, 3863.788, 4136.6675],

    // Elemental Burst: 火力焦点·雷霰急袭
    q_dmg: [1.3512, 1.45254, 1.55388, 1.689, 1.79034, 1.89168, 2.0268, 2.16192, 2.29704, 2.43216, 2.56728, 2.7024, 2.8713, 3.0402, 3.2091],
    q_shell_dmg: [0.3152, 0.33884, 0.36248, 0.394, 0.41764, 0.44128, 0.4728, 0.50432, 0.53584, 0.56736, 0.59888, 0.6304, 0.6698, 0.7092, 0.7486],

    // 命座 2：伴随冲阵的队型
    c2_em: 100.0,
    c2_shield_per_momentum: 0.08,
    c2_shield_max_momentum: 20.0,

    // 命座 6：穿透敌阵的威能
    c6_dmg_bonus: 1.0,
    c6_electro_crit_rate: 0.1,
    c6_sc_crit_damage: 0.4,
};

pub const VALERIY_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Valeriy,
    internal_name: "Valeriy",
    element: Element::Electro,
    hp: [839, 2155, 2782, 4167, 4612, 5305, 5898, 6591, 7037, 7729, 8174, 8867, 9312, 10005, 10697],
    atk: [21, 55, 71, 106, 117, 135, 150, 168, 179, 197, 208, 226, 237, 255, 320],
    def: [59, 151, 195, 293, 324, 373, 414, 463, 494, 543, 574, 623, 654, 703, 752],
    sub_stat: CharacterSubStatFamily::ATK240,
    weapon_type: WeaponType::Sword,
    star: 4,
    skill_name1: locale!(
        zh_cn: "近卫战法·盾剑攻斥",
        en: "Guard Tactics: Sword and Shield",
    ),
    skill_name2: locale!(
        zh_cn: "攻势防御·堡垒骤进",
        en: "Active Defense: Bastion Charge",
    ),
    skill_name3: locale!(
        zh_cn: "火力焦点·雷霰急袭",
        en: "Focused Fire: Lightning Salvo",
    ),
    name_locale: locale!(
        zh_cn: "瓦列里",
        en: "Valeriy",
    ),
};

pub struct ValeriyEffect {
    pub stellar_glimmer_state: StellarGlimmerState,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for ValeriyEffect {
    fn change_attribute(&self, _attribute: &mut A) {
        // 瓦列里的战斗效果均依赖攻斥指令、冲击势能等实时状态，
        // 统一在 CharacterTrait::change_attribute（CONFIG_SKILL）中根据技能配置施加。
        let _ = self.stellar_glimmer_state;
    }
}

damage_enum!(
    ValeriyDamageEnum
    A1
    A2
    A3
    Z
    ZC
    X1
    X2
    X3
    E
    ESHIELD
    Q
    QSHELL
);

impl ValeriyDamageEnum {
    pub fn get_element(&self) -> Element {
        use ValeriyDamageEnum::*;
        match *self {
            A1 | A2 | A3 | Z | X1 | X2 | X3 => Element::Physical,
            ZC | E | ESHIELD | Q | QSHELL => Element::Electro,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use ValeriyDamageEnum::*;
        match *self {
            A1 | A2 | A3 => SkillType::NormalAttack,
            Z | ZC => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E | ESHIELD => SkillType::ElementalSkill,
            Q | QSHELL => SkillType::ElementalBurst,
        }
    }
}

pub struct Valeriy;

impl CharacterTrait for Valeriy {
    const STATIC_DATA: CharacterStaticData = VALERIY_STATIC_DATA;
    type SkillType = ValeriySkillType;
    const SKILL: Self::SkillType = VALERIY_SKILL;
    type DamageEnumType = ValeriyDamageEnum;
    type RoleEnum = ();

    const DEFAULT_TAGS: Option<&'static [CharacterTag]> = None;

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            ValeriyDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            Z charged_dmg!()
            ZC locale!(zh_cn: "臼炮直射·抵近轰击伤害", en: "Direct-Fire Mortar: Point-Blank Bombardment DMG")
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            ValeriyDamageEnum
            E locale!(zh_cn: "技能伤害", en: "Skill DMG")
            ESHIELD locale!(zh_cn: "护盾吸收量", en: "Shield DMG Absorption")
        ),
        skill3: skill_map!(
            ValeriyDamageEnum
            Q locale!(zh_cn: "技能伤害", en: "Skill DMG")
            QSHELL locale!(zh_cn: "雷霰重弹伤害", en: "Thunderburst Shell DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STELLAR_GLIMMER_STATE(StellarGlimmerState::None, ItemConfig::PRIORITY_CHARACTER),
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "offensive_directive",
            title: locale!(
                zh_cn: "拥有攻斥指令（特殊重击后获得）",
                en: "Has Offensive Directive (gained after special Charged Attack)"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "impact_momentum_consumed",
            title: locale!(
                zh_cn: "特殊重击消耗的冲击势能（上限100，命座4为130）",
                en: "Impact Momentum Consumed by Special Charged Attack (cap 100, 130 with C4)"
            ),
            config: ItemConfigType::Int { min: 40, max: 130, default: 40 }
        },
        ItemConfig {
            name: "c2_overflow_momentum",
            title: locale!(
                zh_cn: "溢出的冲击势能（命座2，转化为蓄雷护盾吸收量）",
                en: "Excess Impact Momentum (C2, converted to Supercharged Shield DMG Absorption)"
            ),
            config: ItemConfigType::Int { min: 0, max: 20, default: 0 }
        },
    ]);

    fn change_attribute<A: Attribute>(attribute: &mut A, common_data: &CharacterCommonData, skill_config: &CharacterSkillConfig) {
        let stellar_glimmer_state = match &common_data.config {
            CharacterConfig::Valeriy { stellar_glimmer_state } => *stellar_glimmer_state,
            _ => StellarGlimmerState::None,
        };

        let (offensive_directive, impact_momentum_consumed) = match *skill_config {
            CharacterSkillConfig::Valeriy { offensive_directive, impact_momentum_consumed, .. } => {
                (offensive_directive, impact_momentum_consumed)
            }
            _ => (false, 0),
        };

        // 冲击势能的携带上限初始为 100 点，命座 4 提升 30 点
        let max_momentum = if common_data.constellation >= 4 { 130 } else { 100 };
        let impact_momentum_consumed = impact_momentum_consumed.min(max_momentum);

        // 攻斥指令：进行特殊重击后获得，每次雷元素伤害消耗一层，
        // 依据瓦列里是否处于辉映·星超导状态提升不同类型的雷元素伤害，提升值基于瓦列里的攻击力。
        // 提升值随普通攻击的技能等级提升；天赋 2 依据进行特殊重击时消耗的冲击势能，为每层追加额外提升值。
        // 层数只限制可被提升的伤害次数上限，不改变单次伤害的提升值，故不额外设置层数配置项。
        if offensive_directive {
            let s1 = common_data.skill1;
            let is_stellar_conduct = stellar_glimmer_state.is_stellar_conduct();
            let momentum = if common_data.has_talent2 {
                impact_momentum_consumed as f64
            } else {
                0.0
            };

            let (ratio, ty) = if is_stellar_conduct {
                (
                    VALERIY_SKILL.directive_sc_ratio[s1] + momentum * VALERIY_SKILL.p2_sc_per_momentum,
                    AttributeType::Invisible(InvisibleAttributeType::new(
                        AttributeVariableType::ReactionExtra,
                        Some(Element::Electro),
                        None,
                        Some(ReactionType::StellarConduct),
                    )),
                )
            } else {
                (
                    VALERIY_SKILL.directive_electro_ratio[s1] + momentum * VALERIY_SKILL.p2_electro_per_momentum,
                    AttributeType::Invisible(InvisibleAttributeType::new_element(
                        AttributeVariableType::BaseDamage,
                        Element::Electro,
                    )),
                )
            };

            attribute.add_edge_s1to1(
                CharacterSelector::select_onfield(attribute),
                AttributeType::Panel(AttributeName::ATK),
                ty,
                Arc::new(move |atk: f64, _| atk * ratio),
                "瓦列里技能A",
                EdgePriority::Invisible,
            );
        }

        // 命座 2：施放元素战技后的 18 秒内，队伍中附近的所有角色的元素精通提升 100 点。
        // 施放元素战技后该效果在输出循环中基本保持覆盖，默认按生效处理。
        if common_data.constellation >= 2 {
            attribute.set_value_by_s(
                CharacterSelector::select_all(attribute),
                AttributeType::Panel(AttributeName::ElementalMastery),
                "瓦列里命座2",
                VALERIY_SKILL.c2_em,
            );
        }

        // 命座 6：特殊重击与雷霰重弹命中敌人后的 7 秒内，敌人受到的雷元素伤害暴击率提升 10%，
        // 其中星超导反应伤害的暴击伤害额外提升 40%。
        // 该效果由瓦列里的主要输出手段持续刷新，默认按生效处理。
        if common_data.constellation >= 6 {
            attribute.set_value_by_s(
                CharacterSelector::select_all(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_element(
                    AttributeVariableType::CriticalRate,
                    Element::Electro,
                )),
                "瓦列里命座6",
                VALERIY_SKILL.c6_electro_crit_rate,
            );
            attribute.set_value_by_s(
                CharacterSelector::select_all(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalDamage,
                    Some(Element::Electro),
                    None,
                    Some(ReactionType::StellarConduct),
                )),
                "瓦列里命座6",
                VALERIY_SKILL.c6_sc_crit_damage,
            );
        }
    }

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: ValeriyDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (_, impact_momentum_consumed, c2_overflow_momentum) = match *config {
            CharacterSkillConfig::Valeriy { offensive_directive, impact_momentum_consumed, c2_overflow_momentum } => {
                (offensive_directive, impact_momentum_consumed, c2_overflow_momentum)
            }
            _ => (false, 0, 0),
        };

        let max_momentum = if context.character_common_data.constellation >= 4 { 130 } else { 100 };
        let impact_momentum_consumed = impact_momentum_consumed.min(max_momentum);

        use ValeriyDamageEnum::*;
        let mut builder = D::new();

        // 特殊重击需要至少 40 点冲击势能才能进行
        if s == ZC && impact_momentum_consumed < 40 {
            return builder.none();
        }

        // 命座 6：特殊重击与雷霰重弹造成的伤害提升 100%
        if context.character_common_data.constellation >= 6 && matches!(s, ZC | QSHELL) {
            builder.add_extra_bonus("瓦列里命座6", VALERIY_SKILL.c6_dmg_bonus);
        }

        match s {
            ESHIELD => {
                builder.add_atk_ratio("技能倍率", VALERIY_SKILL.e_shield_ratio[s2]);
                builder.add_base("护盾吸收量", VALERIY_SKILL.e_shield_base[s2]);

                // 命座 2：溢出的冲击势能转化为蓄雷护盾的伤害吸收量，至多转化 20 点
                if context.character_common_data.constellation >= 2 && c2_overflow_momentum > 0 {
                    let overflow = (c2_overflow_momentum as f64).min(VALERIY_SKILL.c2_shield_max_momentum);
                    builder.add_atk_ratio("瓦列里命座2", VALERIY_SKILL.c2_shield_per_momentum * overflow);
                }

                return builder.shield(&context.attribute, Element::Electro);
            }
            ZC => builder.add_atk_ratio("技能倍率", VALERIY_SKILL.z_cannon_dmg[s1]),
            QSHELL => builder.add_atk_ratio("技能倍率", VALERIY_SKILL.q_shell_dmg[s3]),
            _ => {
                let ratio = match s {
                    A1 => VALERIY_SKILL.a_dmg1[s1],
                    A2 => VALERIY_SKILL.a_dmg2[s1],
                    A3 => VALERIY_SKILL.a_dmg3[s1],
                    Z => VALERIY_SKILL.z_dmg[s1],
                    X1 => VALERIY_SKILL.x_dmg1[s1],
                    X2 => VALERIY_SKILL.x_dmg2[s1],
                    X3 => VALERIY_SKILL.x_dmg3[s1],
                    E => VALERIY_SKILL.e_dmg[s2],
                    Q => VALERIY_SKILL.q_dmg[s3],
                    _ => unreachable!(),
                };
                builder.add_atk_ratio("技能倍率", ratio);
            }
        }

        // 特殊重击本身即为雷元素伤害，不受附魔影响
        let fumo = if s == ZC { None } else { fumo };

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let stellar_glimmer_state = match *config {
            CharacterConfig::Valeriy { stellar_glimmer_state } => stellar_glimmer_state,
            _ => StellarGlimmerState::None,
        };
        Some(Box::new(ValeriyEffect {
            stellar_glimmer_state,
            common_data: common_data.clone(),
        }))
    }

    fn get_target_function_by_role(_role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        // 瓦列里以攻击力驱动攻斥指令与蓄雷护盾，仅有一个目标函数。
        Box::new(ValeriyDefaultTargetFunction {
            recharge_demand: 1.6,
        })
    }
}
