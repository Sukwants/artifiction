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

以下文件为可参考实现（越靠下的实现越新，实现时优先参考文档与较新的实现。请勿尝试参考没有列出的实现，几乎没有参考价值）：

`mona_core/src/character/characters/pyro/durin.rs`
`mona_core/src/character/characters/anemo/venti.rs`
`mona_core/src/character/characters/hydro/columbina.rs`
`mona_core/src/character/characters/geo/zibai.rs`
`mona_core/src/character/characters/geo/illuga.rs`
`mona_core/src/character/characters/anemo/varka.rs`
`mona_core/src/character/characters/geo/linnea.rs`
`mona_core/src/character/characters/anemo/prune.rs`
`mona_core/src/character/characters/pyro/nicole.rs`
`mona_core/src/character/characters/cryo/lohen.rs`
`mona_core/src/character/characters/cryo/sandrone.rs`

如果你是 AI，请仔细阅读以上示例与文档，严格按照以上说明，并尽量贴合示例的形式进行编写。

由于角色文件主要用于效果应用、属性计算、伤害计算以及护盾、治疗等其他计算，因此有关攻击次数、攻击频率、攻击范围、元素能量等数据的处理不应放在角色文件中。

如果遇到难以确定的数据如某个层数较多或因素复杂的效果，可以通过平均层数的方式替代进行近似计算，请在注释中明确说明。请务必保证配置项的合理性，对于过程中变化不会太大的配置项不允许使用平均层数替代。

另，如出现影响角色技能等级的效果，不进行自动处理，按照规范应由用户输入实际技能等级进行计算。
