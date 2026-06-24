## DamageEnum

本文件包含角色文件中 DamageEnum 的部分书写说明。

DamageEnum 部分包含 `DamageEnum` 枚举类型的声明和部分接口实现，以及 `CharacterTrait` 中的静态数据。

一个 DamageEnum 部分示例如下：

```rust
damage_enum!(
    DurinDamageEnum
    A1
    A2
    A31
    A32
    A4
    Z
    X1
    X2
    X3
    EP
    ED1
    ED2
    ED3
    QP1
    QP2
    QP3
    QP
    QD1
    QD2
    QD3
    QD
);

impl DurinDamageEnum {
    pub fn get_element(&self) -> Element {
        use DurinDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 | A4 | Z | X1 | X2 | X3 => Element::Physical,
            EP | ED1 | ED2 | ED3 | QP1 | QP2 | QP3 | QP | QD1 | QD2 | QD3 | QD => Element::Pyro,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use DurinDamageEnum::*;
        match *self {
            A1 | A2 | A31 | A32 | A4 => SkillType::NormalAttack,
            Z => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            EP | ED1 | ED2 | ED3 => SkillType::ElementalSkill,
            QP1 | QP2 | QP3 | QP | QD1 | QD2 | QD3 | QD => SkillType::ElementalBurst,
        }
    }
}

impl CharacterTrait for Durin {
    const STATIC_DATA: CharacterStaticData = DURIN_STATIC_DATA;
    type SkillType = DurinSkillType;
    const SKILL: Self::SkillType = DURIN_SKILL;
    type DamageEnumType = DurinDamageEnum;
    type RoleEnum = ();

    const DEFAULT_TAGS: Option<&'static [CharacterTag]> = Some(
        &[CharacterTag::Hexerei]
    );

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            DurinDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A31 hit_n_dmg!(3, 1)
            A32 hit_n_dmg!(3, 2)
            A4 hit_n_dmg!(4)
            Z charged_dmg!()
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            DurinDamageEnum
            EP locale!(zh_cn: "转变·白化之是伤害", en: "Transmutation: Confirmation of Purity DMG")
            ED1 locale!(zh_cn: "转变·黑度之否一段伤害", en: "Transmutation: Denial of Darkness DMG 1")
            ED2 locale!(zh_cn: "转变·黑度之否二段伤害", en: "Transmutation: Denial of Darkness DMG 2")
            ED3 locale!(zh_cn: "转变·黑度之否三段伤害", en: "Transmutation: Denial of Darkness DMG 3")
        ),
        skill3: skill_map!(
            DurinDamageEnum
            QP1 locale!(zh_cn: "白化法·如光流变一段伤害", en: "Principle of Purity: As the Light Shifts DMG 1")
            QP2 locale!(zh_cn: "白化法·如光流变二段伤害", en: "Principle of Purity: As the Light Shifts DMG 2")
            QP3 locale!(zh_cn: "白化法·如光流变三段伤害", en: "Principle of Purity: As the Light Shifts DMG 3")
            QP locale!(zh_cn: "白焰之龙伤害", en: "Dragon of White Flame DMG")
            QD1 locale!(zh_cn: "黑度法·如星阴燃一段伤害", en: "Principle of Darkness: As the Stars Smolder DMG 1")
            QD2 locale!(zh_cn: "黑度法·如星阴燃二段伤害", en: "Principle of Darkness: As the Stars Smolder DMG 2")
            QD3 locale!(zh_cn: "黑度法·如星阴燃三段伤害", en: "Principle of Darkness: As the Stars Smolder DMG 3")
            QD locale!(zh_cn: "黑焰之龙伤害", en: "Dragon of Dark Decay DMG")
        )
    };

    // ...
}
```

其中，通过宏 `damage_enum!` 声明 `DamageEnum` 枚举类型，应包含所有需要被计算的技能（包括用户需要和 target_function 需要），应当主要分为三部分：普通攻击、元素战技、元素爆发，根据每一个技能所属的描述部分分别严格对应 `A`、`E`、`Q` 开头的技能，其余还存在天赋描述、命座描述中产生的技能，对应 `P1`、`C2` 等形式开头的技能。  
每个技能部分的命名根据技能产生影响的不同使用 `Heal`、`Shield` 表示治疗、护盾，一般情况伤害不作特别标识，同一技能的不同段数使用数字区分，有独立名称的技能可自行提取合适的表达（如首字母、缩写等）标识。在根据变量名无法明确区分的情况下，建议在注释中标明技能名称以供参考。

对于一个造成多次相同倍率伤害的技能，应拆分为单次伤害和总伤害两个技能。

对于 `DamageEnum`，需要实现 `get_element` 和 `get_skill_type` 等接口。  
`get_element` 接口返回技能的元素类型，如为护盾返回护盾元素类型，如为治疗一般返回 `Element::Physical`。
`get_skill_type` 接口返回技能的技能类型，一般根据通过普通攻击、重击、下落攻击、元素战技、元素爆发触发的技能类型即为对应类型，与该技能描述所在部分无必然关系，如元素战技描述中存在通过普通攻击触发的技能，则该技能类型应为 `SkillType::NormalAttack`。非当前角色操作触发的技能，如协同攻击，一般根据初始触发的技能类型进行判断。若存在“视为元素战技伤害”等描述，应严格按照该描述进行判断。  
若明确某个技能不存在元素类型或技能类型，也不应被查询元素类型或技能类型时，可以返回 `panic!()`，应用于如治疗量、纯数值等不存在明确元素类型、技能类型的情况。

请注意，任何技能触发的擢升反应伤害（月感电、月绽放、月结晶、星超导）的技能类型都必须是 `SkillType::Elevative`。

特别的，如果该角色存在伤害元素类型与配置相关的情况（如染色伤害），则 `get_element` 接口的实现需要新增参数、进行判断并返回相应元素类型。

特别的，如果该角色会触发擢升反应伤害，则需要额外实现 `get_elevative_type` 接口，示例如下：

```rust
pub fn get_elevative_type(&self) -> Option<ElevativeReaction> {
    use LaumaDamageEnum::*;
    match *self {
        EHold2 | C6E | C6A => Some(ElevativeReaction::LunarBloom),
        _ => None,
    }
}
```

其中，`ElevativeReaction` 定义位于 `mona_core/src/common/reaction_type.rs`，定义如下：

```rust
mona::common::reaction_type
pub enum ElevativeReaction {
    LunarChargedReaction,       // 月感电
    LunarCharged,               // 广义月感电
    LunarBloom,
    LunarCrystallizeReaction,   // 月结晶
    LunarCrystallize,           // 广义月结晶
}
```

其中 `LunarCharged`、`LunarBloom`、`LunarCrystallize` 分别为通过角色技能触发的月感电、月绽放、月结晶伤害，`LunarChargedReaction`、`LunarCrystallizeReaction` 分别为通过元素反应触发的月感电、月结晶伤害。在 `DamageEnum` 中，原则上不允许出现效果为 `LunarChargedReaction`、`LunarCrystallizeReaction` 的技能。

`CharacterTrait` 的实现中，前几个变量仿照上例即可。`DEFAULT_TAGS` 根据当前角色是否为月兆或魔导角色进行设置。`SKILL_MAP` 为所有需要对用户展示的技能以及相应名称，按照技能描述填写名称即可，对于部分天赋或命座中的技能，可通过“命座2伤害”等形式进行命名。

当一个命座产生的伤害可能由不同技能来源触发时（如命座4的弹射伤害既可能由元素战技触发也可能由元素爆发触发），应将该伤害拆分为多个 DamageEnum 变体，分别归入对应技能的 `skill_map` 中。拆分后在 `get_skill_type` 中也应分别返回对应的 `SkillType`：

```rust
damage_enum!(
    XxxDamageEnum
    C4E   // 命座4弹射-元素战技触发
    C4Q   // 命座4弹射-元素爆发触发
);

impl XxxDamageEnum {
    pub fn get_skill_type(&self) -> SkillType {
        match *self {
            C4E => SkillType::ElementalSkill,
            C4Q => SkillType::ElementalBurst,
        }
    }
}
```

在 `SKILL_MAP` 中，`C4E` 放入 `skill2`，`C4Q` 放入 `skill3`。两个变体可共用 `SkillType` 中的同一个 `f64` 字段。