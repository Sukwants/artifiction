# Weapon

本文件包含武器文件书写方法。

武器文件应包含以下部分：

- 武器配置项。
- 武器效果实现。
- 武器基本信息。
- 武器配置项注册。
- 武器效果配置实例化。

## 武器配置项

武器效果配置项的声明需要将武器所需要的所有配置项都包含在内。

如果遇到难以确定的数据如某个层数较多或因素复杂的效果，可以通过在配置中设置平均层数的方式替代进行近似计算，请在注释中明确说明。请务必保证配置项的合理性，对于过程中变化不会太大的配置项不允许使用平均层数替代。

## 武器效果实现

武器效果实现部分需要实现 `apply` 方法，应包含武器所有效果的修改逻辑，除了通过调用自身的配置项，还需要通过参数中的 `data.refine` 获得武器精炼等级。接口调用方法参见 `mona_docs/src/attribute.md`。

武器效果实现部分的 `key` 命名一般命为“武器名称+‘被动’”，如“霜结的誓金枝被动”。

## 武器基本信息

武器基本信息部分包含了武器的静态数据与属性值数据，需要填写的属性值有 `weapon_base` 和 `weapon_sub_stat`，分别表示武器的基础攻击力和副属性类型与数值。

武器名称需要追加到 `mona::weapon::weapon_name::WeaponName` 中，无特殊说明直接追加到末尾。

武器特效说明部分直接填写武器特效的原文说明，并将与精炼等级有关的数值按照 `<span style=\"color: #409EFF;\">{1}-{2}-{3}-{4}-{5}</span>` 的格式进行填写（`{n}` 即表示精炼等级为 $n$ 时的数值，注意包含百分号）。

武器的基础攻击力有以下选项，`ATK` 后的数值均表示武器满级时的基础攻击力：

```rust
mona::weapon::weapon_base_atk
pub enum WeaponBaseATKFamily {
    ATK185,
    ATK243,
    ATK354,
    ATK401,
    ATK440,
    ATK448,
    ATK454,
    ATK510,
    ATK542,
    ATK565,
    ATK620,
    ATK608,
    ATK674,
    ATK741,
}
```

武器副属性的类型与数值参见 `mona_core/src/weapon/weapon_sub_stat.rs`，其中有如下前缀：

- `ATK`：攻击力百分比。
- `CriticalDamage`：暴击伤害百分比。
- `CriticalRate`：暴击率百分比。
- `DEF`：防御力百分比。
- `EM`：元素精通。
- `HP`：生命值百分比。
- `PhysicalBonus`：物理伤害加成百分比。
- `Recharge`：元素充能效率百分比。

对于所有百分比数值，后缀的数字表示小数点后一到三位的数值，对于元素精通数值，后缀的数字表示元素精通四舍五入的数值。所有数值均为武器等级一级时的数值，其他等级的数值可参见源文件。

## 武器配置项注册

`CONFIG_DATA` 包含了需要用户给出的配置项，每一个配置项应包含以下字段：
- `name`：配置项的绑定名称，必须与 Effect 结构体中对应字段的名称一致。
- `title`：配置项的显示名称，需要中英文两种语言。
- `config`：配置项的类型定义，包含了配置项的类型、可选项、默认值等信息。可以使用的配置类型详见 `mona_docs/src/config.md`。

## 武器效果配置实例化

`get_effect` 函数用于将用户输入的配置项转换为 Effect 结构体，每一个配置项都应添加到 `mona::weapon::weapon_config::WeaponConfig` 中，并在 `get_effect` 函数中进行匹配解析。

## 示例

以下文件为可参考实现（越靠下的实现越新，实现时优先参考文档与较新的实现。请勿尝试参考没有列出的实现，几乎没有参考价值）：

`mona_core/src/weapon/weapons/catalysts/nocturnes_curtain_call.rs`
`mona_core/src/weapon/weapons/swords/lightbearing_moonshard.rs`
`mona_core/src/weapon/weapons/claymores/gest_of_the_mighty_wolf.rs`
`mona_core/src/weapon/weapons/bows/golden_frostbound_oath.rs`

如果你是 AI，请仔细阅读以上示例与文档，严格按照以上说明，并尽量贴合示例的形式进行编写。
