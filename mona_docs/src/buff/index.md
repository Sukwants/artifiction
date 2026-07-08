# Buff

本文件包含 buff 文件书写方法。

buff 用于将其他角色、武器、圣遗物的增益效果应用到当前角色上，因此只有在不仅对自己生效的效果才可以在 buff 中实现。

buff 的作用为在通过新 Attribute 系统无法有效选中角色以应用效果、或用户期望通过旧的 buff 系统来配置的情况下提供添加效果的方法。原则上所有不止作用与当前角色自身的效果都需要在 buff 中体现，尽管该效果可能已经在角色、武器、圣遗物文件中体现。

buff 不同于角色、武器、圣遗物文件，buff 中不允许对非当前角色进行操作，包括但不限于使用角色选择器等，所有 buff 都应作用到当前角色。同时 buff 不应通过 Attribute 系统读取其他角色的属性如元素类型等来实现效果，所有 buff 的效果都应通过配置项来实现。

一个 buff 文件中可能包含多个 buff 的实现，对于每一个 buff 的实现需要包含以下几个部分：

- buff 配置项。
- buff 效果效果实现。
- buff 基本信息。
- buff 配置项注册。
- buff 效果配置实例化。

## buff 配置项

buff 效果配置项的声明需要将 buff 所需要的所有配置项都包含在内。

如果遇到难以确定的数据如某个层数较多或因素复杂的效果，可以通过在配置中设置平均层数的方式替代进行近似计算，请在注释中明确说明。请务必保证配置项的合理性，对于过程中变化不会太大的配置项不允许使用平均层数替代。

如果存在整个 buff 的效果都需要某个配置项为真才生效，则一般不添加该配置项，若用户应用该 buff 视为条件成立。

## buff 效果实现

buff 效果实现部分需要实现 `change_attribute` 方法，应包含 buff 所有效果的修改逻辑。接口调用方法参见 `mona_docs/src/attribute.md`。

buff 效果实现部分的 `key` 命名一般与其真实来源的命名规范保持一致，如 “哥伦比娅Q技能”、“哥伦比娅天赋3”、“哥伦比娅命座2”、“霜结的誓金枝被动”、“纺月的夜歌4”等。

请注意，由于 buff 可能与其他位置的效果存在重叠，因此在实现时尽量使用 `set_value_to` 等接口来实现，以避免错误叠加与重复计算。

如果有需要使用角色倍率等情况，可以直接通过 `use` 引入。

## buff 基本信息

buff 基本信息部分包含了 buff 的静态数据，包括 buff 名称、buff 效果描述等。

buff 名称需要追加到 `mona::buffs::buff_name::BuffName` 中，无特殊说明直接追加到末尾。buff 名称规范为“来源标识-「效果名称」”，如“哥伦比娅-「她的乡愁」”，其中“她的乡愁”是元素爆发的名称，又如“哥伦比娅-「月兆祝赐·借汝月光」”，其中“月兆祝赐·借汝月光”是天赋 3 的名称，“霜结的誓金枝-「霜妖精的恶戏」”中“霜妖精的恶戏”是武器效果描述中的效果名称。特别的，对于圣遗物使用形如“纺月的夜歌4”的名称，其中“4”是四件套的意思。

buff 效果描述应以来源描述开头，如“哥伦比娅Q技能：”、“哥伦比娅天赋3：”等，后续为效果描述，应尽量摘抄原文，并剔除与效果无关的部分，在合适位置应使用 `<br>` 换行。

## buff 配置项注册

`CONFIG` 包含了需要用户给出的配置项，每一个配置项应包含以下字段：
- `name`：配置项的绑定名称，必须与 Effect 结构体中对应字段的名称一致。
- `title`：配置项的显示名称，需要中英文两种语言。
- `config`：配置项的类型定义，包含了配置项的类型、可选项、默认值等信息。可以使用的配置类型详见 `mona_docs/src/config.md`。

## buff 效果配置实例化

`create` 函数用于将用户输入的配置项转换为 Effect 结构体，每一个配置项都应添加到 `mona::buffs::buff_config::BuffConfig` 中，并在 `create` 函数中进行匹配解析。

## 示例

以下文件为可参考实现（越靠下的实现越新，实现时优先参考文档与较新的实现。请勿尝试参考没有列出的实现，几乎没有参考价值）：

`mona_core/src/buffs/buffs/artifact/silken_moons_serenade.rs`
`mona_core/src/buffs/buffs/artifact/night_of_the_skys_unveiling.rs`
`mona_core/src/buffs/buffs/character/hydro/columbina.rs`
`mona_core/src/buffs/buffs/character/geo/zibai.rs`
`mona_core/src/buffs/buffs/character/geo/linnea.rs`
`mona_core/src/buffs/buffs/weapon/bow/golden_frostbound_oath.rs`

如果你是 AI，请仔细阅读以上示例与文档，严格按照以上说明，并尽量贴合示例的形式进行编写。
