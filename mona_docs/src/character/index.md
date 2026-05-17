# Character

本文件包含角色文件书写方法。

角色文件应包含以下部分：

- 头文件：`use crate::character::characters::prelude::*;`
- Stats 部分：包含角色技能声明、角色技能数值、角色静态数据等信息，详见 `mona_docs/src/character/stats.md`。
- Effect 部分：包含角色效果配置声明与角色效果接口实现，以及 `CharacterTrait` 中的 `CONFIG_DATA`，详见 `mona_docs/src/character/effect.md`。
- DamageEnum 部分：包含 `DamageEnum` 枚举类型的声明和部分接口实现，以及 `CharacterTrait` 中的静态数据，详见 `mona_docs/src/character/damage_enum.md`。
- EffectSkill 部分：包含角色技能效果配置声明与角色技能效果接口实现，以及 `CharacterTrait` 中的 `CONFIG_SKILL`，详见 `mona_docs/src/character/effect_skill.md`。
- Damage 部分：包含声明 `builder`，将与具体技能相关的配置应用进 `builder`，并调用 `builder` 中的方法完成计算，详见 `mona_docs/src/character/damage.md`。
- 以及其他与具体角色相关度不大的部分。

以下文件为可参考实现：

`mona_core/src/character/characters/pyro/durin.rs`
`mona_core/src/character/characters/anemo/venti.rs`
`mona_core/src/character/characters/hydro/columbina.rs`
`mona_core/src/character/characters/geo/zibai.rs`
`mona_core/src/character/characters/geo/illuga.rs`
`mona_core/src/character/characters/anemo/varka.rs`
`mona_core/src/character/characters/geo/linnea.rs`

以上文件越靠下的实现越新，实现时优先参考文档与较新的实现。请勿尝试参考没有列出的实现，几乎没有参考价值。

如果你是 AI，请仔细阅读以上示例与文档，严格按照以上说明，并尽量贴合示例的形式进行编写。
