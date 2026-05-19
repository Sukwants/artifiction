# Artifact

本文件包含圣遗物文件书写方法。

圣遗物文件应包含以下部分：

- 圣遗物配置项。
- 圣遗物效果效果实现。
- 圣遗物基本信息。
- 圣遗物配置项注册。
- 圣遗物效果配置实例化。

## 圣遗物配置项

圣遗物效果配置项的声明需要将圣遗物所需要的所有配置项都包含在内。

如果遇到难以确定的数据如某个层数较多或因素复杂的效果，可以通过在配置中设置平均层数的方式替代进行近似计算，请在注释中明确说明。请务必保证配置项的合理性，对于过程中变化不会太大的配置项不允许使用平均层数替代。

## 圣遗物效果实现

圣遗物效果实现部分需要实现 `effect{n}` 方法（其中 `n` 表示 $n$ 件套效果，如 `effect2`、`effect4`），应包含圣遗物该套装效果的修改逻辑。接口调用方法参见 `mona_docs/src/attribute.md`。

圣遗物效果实现部分的 `key` 命名一般命为“圣遗物名称+几件套效果”，如“风起之日4”。

请注意，对于应用到所有角色的辅助类圣遗物效果，往往会默认效果不可叠加，因此需要使用 `set_value_to` 方法来避免重复计算。

## 圣遗物基本信息

圣遗物基本信息部分包含了圣遗物的静态数据，包括圣遗物名称、各部件名称、可用稀有度、套装效果描述等。

圣遗物名称需要追加到 `mona::artifacts::artifact::ArtifactSetName` 中，无特殊说明直接追加到末尾。

## 圣遗物配置项注册

`CONFIG_DATA` 包含了需要用户给出的配置项，每一个配置项应包含以下字段：
- `name`：配置项的绑定名称，必须与 Effect 结构体中对应字段的名称一致。
- `title`：配置项的显示名称，需要中英文两种语言。
- `config`：配置项的类型定义，包含了配置项的类型、可选项、默认值等信息。可以使用的配置类型详见 `mona_docs/src/config.md`。

一般情况下，圣遗物的配置需要注册为全局联动配置，键名格式为“[圣遗物标识]配置名称”，如“[a_day_carved_from_rising_winds]set4_rate”，关于全局联动配置的说明详见 `mona_docs/src/config.md`。

## 圣遗物效果配置实例化

`create_effect` 函数用于将用户输入的配置项转换为 Effect 结构体，每一个配置项都应添加到 `mona::artifacts::effect_config::ArtifactEffectConfig` 中，并在 `create_effect` 函数中进行匹配解析。

此外，还需要在统一文件下的 `ArtifactConfigInterface`、`ArtifactEffectConfig` 与 `ArtifactEffectConfigBuilder` 等部分完成注册。

请注意，没有配置项的圣遗物请勿在以上四个位置进行注册。

## 示例

以下文件为可参考实现（越靠下的实现越新，实现时优先参考文档与较新的实现。请勿尝试参考没有列出的实现，几乎没有参考价值）：

`mona_core/src/artifacts/effects/silken_moons_serenade.rs`
`mona_core/src/artifacts/effects/night_of_the_skys_unveiling.rs`
`mona_core/src/artifacts/effects/aubade_of_morningstar_and_moon.rs`
`mona_core/src/artifacts/effects/a_day_carved_from_rising_winds.rs`
`mona_core/src/artifacts/effects/celestial_gift.rs`
`mona_core/src/artifacts/effects/disenchantment_in_deep_shadow.rs`

如果你是 AI，请仔细阅读以上示例与文档，严格按照以上说明，并尽量贴合示例的形式进行编写。
