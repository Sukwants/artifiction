# TargetFunction

本文件包含 target function 文件书写方法。

如果你是 AI，请注意，target_function 与角色实际情况关系极大，因此需要与用户进行充分沟通并按照用户的指示进行编写，请勿擅自猜测，如果用户未指明请直接询问。

target_function 除了需要引用 `prelude.rs`，还需要引用对应角色文件。

target_function 文件应包含以下部分：

- target_function 配置项。
- target_function 基本信息。
- target_function 配置项注册。
- target_function 效果配置实例化。
- 词条权重设置。
- 默认圣遗物配置设置。
- target_function 计算实现。

## target_function 配置项

target_function 效果配置项的声明需要将 target_function 所需要的所有配置项都包含在内。如果你是 AI，请根据用户指示的配置项进行填写。

## target_function 基本信息

target_function 基本信息部分包含了 target_function 的静态数据。如果你是 AI，请根据用户指示填写 target_function 名称、描述、标签。

target_function 名称需要追加到 `mona::target_functions::target_function_name::TargetFunctionName` 中，无特殊说明直接追加到末尾。

## target_function 配置项注册

`CONFIG` 包含了需要用户给出的配置项，每一个配置项应包含以下字段：
- `name`：配置项的绑定名称，必须与 Effect 结构体中对应字段的名称一致。
- `title`：配置项的显示名称，需要中英文两种语言。
- `config`：配置项的类型定义，包含了配置项的类型、可选项、默认值等信息。可以使用的配置类型详见 `mona_docs/src/config.md`。

如果你是 AI，用户可能会指示在此添加未出现在 Effect 结构体中的配置项（如用于覆写全局联动圣遗物配置），请不要擅自添加到 Effect 结构体中。

## target_function 效果配置实例化

`create` 函数用于将用户输入的配置项转换为 Effect 结构体，每一个配置项都应添加到 `mona::target_functions::target_function_config::TargetFunctionConfig` 中，并在 `create` 函数中进行匹配解析。

## 词条权重设置

通过完成 `get_target_function_opt_config` 来设置当前角色各词条权重。如果你是 AI，生成默认模板即可。

## 默认圣遗物配置设置

通过完成 `get_default_artifact_config` 方法来将部分圣遗物的默认配置项修改为更适合当前角色的值。如果你是 AI，生成默认模板即可。

## target_function 效果实现

target_function 效果实现部分需要实现 `target` 方法，有如下参数：

```rust
fn target(
    &self,
    attribute: &TargetFunctionAttributeType,
    character: &Character<TargetFunctionAttributeType>,
    weapon: &Weapon<TargetFunctionAttributeType>,
    artifacts: &[&Artifact],
    enemy: &Enemy
) -> f64
```

其中 `attribute` 为经过角色、武器、圣遗物、buff 效果添加后的属性，尚未进行计算，仍然可进行修改。`character`、`weapon`、`artifacts` 分别为角色、武器、圣遗物的结构体，多数情况只会用到 `character.common_data`（如果你是 AI，请勿在没有明确指示的情况下通过其他方法使用这些参数）。`enemy` 包含了敌人的相关信息，主要用于构建 `DamageContext`。

所有角色技能配置都应当由 target_function 决定，一个角色可能在不同情况下有不同技能配置。每一套技能配置都需要先将 `attribute` 传入 `change_attribute` 方法得到配置后的属性，然后计算属性并构建 `DamageContext` 实例。每一个技能配置应当对应一个独立的 `DamageContext` 实例。

此后按照角色的输出方式，依次调用伤害计算方法并传入相应技能配置的 `DamageContext` 实例和角色技能配置，通过一定方式加和得到一套技能配置的总伤害，有以下可用方法：

- `pub fn damage<D>(context: &DamageContext<'_, D::AttributeType>, s: Self::DamageEnumType, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result`：计算角色技能列表（`DamageEnum`）内已列出技能的伤害、护盾、治疗量，需要额外传入角色 `DamageEnum` 实例以确定具体技能。
- `pub fn transformative_damage<D>(context: &DamageContext<'_, D::AttributeType>, transformative_type: TransformativeType) -> D::Result`：计算当前角色触发的聚变反应伤害，需要额外传入 `TransformativeType` 实例以确定具体反应类型。
- `pub fn moonglare_damage<D>(context: &DamageContext<'_, D::AttributeType>, lunar_type: MoonglareReaction) -> D::Result`：计算当前角色触发的月曜反应伤害，需要额外传入 `MoonglareReaction` 实例以确定具体反应类型。请注意这里仅用于计算狭义的月曜反应伤害（`MoonglareReaction::LunarChargedReaction` 或 `MoonglareReaction::LunarCrystallizeReaction`），不包括角色技能中可能存在的月曜反应伤害。该伤害结果没有乘以角色在反应中伤害系数，如果你是 AI，请根据用户指示确定系数。

某些 target_function 可能存在特殊要求，如最低攻击力、最低充能效率，此时需要通过查询 `attribute` 中的相关属性来判断是否满足要求，如果不满足则直接返回 0。`attribute` 的查询方法参见 `mona_docs/src/attribute.md`。

某些 target_function 的结果可能不是伤害、护盾或治疗量，而是其他数值（如攻击力、某个增益的数值等），此时需要根据用户指示进行计算并返回。

## 示例

以下文件为可参考实现（越靠下的实现越新，实现时优先参考文档与较新的实现。请勿尝试参考没有列出的实现，几乎没有参考价值）：

`mona_core/src/target_functions/target_functions/geo/zibai_default.rs`
`mona_core/src/target_functions/target_functions/geo/linnea_default.rs`
`mona_core/src/target_functions/target_functions/anemo/prune_default.rs`
`mona_core/src/target_functions/target_functions/pyro/nicole_default.rs`
`mona_core/src/target_functions/target_functions/cryo/lohen_default.rs`

如果你是 AI，请仔细阅读以上示例与文档，严格按照以上说明，并尽量贴合示例的形式进行编写。
