## Config

本文件包含 Config 类型使用说明。

Config 类型定义位于 `mona_core/src/common/item_config_type.rs`，有如下定义：

```rust
mona::common::item_config_type
pub enum ItemConfigType {
    Float {
        min: f64,
        max: f64,
        default: f64,
    },
    Int {
        min: i32,
        max: i32,
        default: i32
    },
    IntInput {
        min: i32,
        max: i32,
        default: i32
    },
    Bool {
        default: bool
    },
    Option {
        options: &'static str, // comma separated
        default: usize
    },
    Option2 {
        options_zh: &'static str,
        options_en: &'static str,
        default: usize
    },
    // NullOrValueInput {
    //     min: f64,
    //     max: f64,
    //     default: f64,
    // },
    FloatPercentageInput {
        default: f64,
    },
    FloatInput {
        default: f64,
    },
    Element {
        elements: &'static [Element],
        default: Element
    },
    ElementOptional {
        elements: &'static [Element],
        default: Option<Element>
    },
    Element4 {      // cryo, pyro, electro, hydro
        default: Element
    },
    Element8 {
        default: Element
    },
    ElementMulti {
        elements: &'static [Element],
        default: ConfigElements8Multi
    },
    Element8Multi {
        default: ConfigElements8Multi
    },
    Skill4 {
        default: SkillType
    },
    Moonsign2 {
        default: Moonsign
    },
    Moonsign3 {
        default: Moonsign
    },
    GlobalLinkBool {
        default: bool,
        global_link: GlobalLinkConfig,
    },
    GlobalLinkFloat {
        min: f64,
        max: f64,
        default: f64,
        global_link: GlobalLinkConfig,
    },
    GlobalLinkInt {
        min: i32,
        max: i32,
        default: i32,
        global_link: GlobalLinkConfig,
    },
    GlobalLinkMoonsign3 {
        default: Moonsign,
        global_link: GlobalLinkConfig,
    }
}
```

对于不带 `GlobalLink` 的配置项，有如下种类：

- `Float`：浮点数输入，表现为滑块与输入框。
- `Int`：整数输入滑块，范围较小的整数输入（如技能等级）建议使用该类型。
- `IntInput`：整数输入框。范围较大的整数输入（如攻击力）建议使用该类型。
- `Bool`：布尔值输入。
- `Option`：单选项输入。
- `Option2`：包含中英文两种语言的单选项输入。若选项内容包含有实际意义的表述，建议使用该类型而非 `Option`。
- `FloatPercentageInput`：百分比输入框，表现为输入框与一个 `%` 符号。该类型不建议使用。
- `FloatInput`：浮点数输入框。
- `Element`：从元素列表中选择一个元素。
- `ElementOptional`：从包含“无”选项的元素列表中选择一个元素。
- `Element4`：冰、火、水、雷四元素单选，仅在扩散、结晶反应等情况下使用。
- `Element8`：火、水、风、雷、草、冰、岩、物理八元素单选。
- `ElementMulti`：从元素列表中选择多个元素。默认值为一个结构体，包含每个元素的布尔值。
- `Element8Multi`：火、水、风、雷、草、冰、岩、物理八元素多选。默认值为一个结构体，包含每个元素的布尔值。
- `Skill4`：普通攻击、重击、元素战技、元素爆发四选一。
- `Moonsign2`：月兆选择，从“初辉”、“满辉”中选择一个。
- `Moonsign3`：月兆选择，从“无”、“初辉”、“满辉”中选择一个。

对于带 `GlobalLink` 的配置项，除上述输入方式外，还包含一个全局联动配置项 `global_link`，该配置项包含以下字段：

```rust
mona::common::item_config_type
pub struct GlobalLinkConfig {
    pub key: &'static str,
    pub priority: usize,
    pub team_shared: bool,
}
```

其中 `key` 字段表示全局联动的键值，唯一标识一个全局联动配置。`priority` 字段该配置项在全局联动中的优先级，数值越大优先级越高。`team_shared` 字段表示该配置项的值是否在整个队伍中联动。

`priority` 字段有如下可选优先级：
- `ItemConfig::PRIORITY_DEFAULT`：默认优先级，数值为 0。
- `ItemConfig::PRIORITY_CHARACTERSKILL`：角色技能配置应使用该优先级，数值为 1。
- `ItemConfig::PRIORITY_ARTIFACT`：圣遗物配置应使用该优先级，数值为 2。
- `ItemConfig::PRIORITY_TARGETFUNCTION`：target_function 配置应使用该优先级，数值为 3。
- `ItemConfig::PRIORITY_BUFF`：buff 配置应使用该优先级，数值为 4。
- `ItemConfig::PRIORITY_WEAPON`：武器配置应使用该优先级，数值为 5。
- `ItemConfig::PRIORITY_CHARACTER`：角色配置应使用该优先级，数值为 6。

若出现以下情况，应调用已配置的全局联动配置项：
- `ItemConfig::MOONSIGN_GLOBAL`：月兆配置项。在与月兆相关的角色、武器、圣遗物、buff 等配置中需要出现该配置项，需要传入相应的默认值和优先级。
- `ItemConfig::HEXEREI_SECRET_RITE_GLOBAL`：魔导配置项。在与魔导相关的角色、武器、圣遗物、buff 等配置中需要出现该配置项，需要传入相应的默认值和优先级。
- `ItemConfig::STELLAR_CONDUCT_APPLICATION_COUNT`： 星超导附着次数配置项。在与星超导相关的角色中需要出现该配置项，需要传入相应的默认值和优先级。请注意，该配置项的值应当决定星超导反应伤害的额外基础倍率，附着次数达到 $0 \sim 12$ 次时，会分别产生 $[0, 0.45, 0.5, 0.54, 0.6, 0.64, 0.71, 0.75, 0.79, 0.85, 0.89, 0.95, 1.0]$ 的额外基础倍率，该额外基础倍率应当通过 `ElevativeCoefficient` 部分进行加成。

此外，`ItemConfig` 中已给出的配置项均可自行在合适的位置使用。
