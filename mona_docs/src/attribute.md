## Attribute

本文件包含 Attribute 系统的接口说明。

### 如何使用

对于 character 和 buff 界面的 `change_attribute`，weapon 的 `apply`，artifact 的 `effect2`、`effect4` 等接口实现中，参数中会包含 `attribute: &mut A`，在接口实现中可以通过 `attribute.<property>` 来调用相关接口。

### 概念说明

#### AttributeName

定义位于 `mona_core/src/attribute/attribute_name.rs`，是一个枚举类型，包含了所有属性的名称，如攻击力、元素精通等。

由于历史原因，AttributeName 中包含了面板属性和大量非面板属性，按照最新规范，任何时候都只应使用 AttributeName 中的面板属性，非面板属性不应被使用。面板属性判断接口如下：

```rust
mona::attribute::attribute_name::AttributeName
pub fn is_panel(&self) -> bool {  
    match *self {
        // 基础属性
        
        AttributeName::ATKBase |
        AttributeName::ATKFixed |
        AttributeName::ATKPercentage |
        AttributeName::ATK |
        
        AttributeName::HPBase |
        AttributeName::HPFixed |
        AttributeName::HPPercentage |
        AttributeName::HP |

        AttributeName::DEFBase |
        AttributeName::DEFFixed |
        AttributeName::DEFPercentage |
        AttributeName::DEF |

        AttributeName::ElementalMastery |
        AttributeName::ElementalMasteryExtra |

        // 进阶属性

        AttributeName::CriticalBase |

        AttributeName::CriticalDamageBase |

        AttributeName::HealingBonus |

        AttributeName::IncomingHealingBonus |

        AttributeName::Recharge |
        AttributeName::RechargeExtra |

        AttributeName::ShieldStrength |

        // 元素属性

        AttributeName::BonusPyro |
        AttributeName::BonusHydro |
        AttributeName::BonusAnemo |
        AttributeName::BonusElectro |
        AttributeName::BonusDendro |
        AttributeName::BonusCryo |
        AttributeName::BonusGeo |
        AttributeName::BonusPhysical
        => true,  
        _ => false,  
    }
}
```

#### InvisibleAttributeType

定义位于 `mona_core/src/attribute/attribute_name.rs`，用来指示一个非面板属性，定义如下：
```rust
mona::attribute::attribute_name
pub struct InvisibleAttributeType {
    pub attribute_variable_type: AttributeVariableType,
    pub element: Option<Element>,
    pub skill: Option<SkillType>,
    pub reaction: Option<ReactionType>,
}
```

其中 `attribute_variable_type` 指示了当前非面板属性在伤害计算中作用部分，具体类型如下：

```rust
mona::attribute::attribute_name
pub enum AttributeVariableType {
    BaseDamage, // 基础提升
    Bonus, // 伤害加成
    ReactionEnhance, // 反应系数提升
    CriticalRate, // 暴击率
    CriticalDamage, // 暴击伤害
    ResMinus, // 减抗
    DefMinus, // 减防
    DefPenetration, // 穿防

    ReactionExtra, // 反应额外提升

    ElevativeBase, // 月曜反应基础提升
    ElevativeElevate, // 月曜反应擢升

    HealingBonus, // 治疗加成
    IncomingHealingBonus, // 受治疗加成
    HealingCriticalRate, // 治疗暴击率
    HealingCriticalDamage, // 治疗暴击伤害

    ShieldStrength, // 护盾强效
}
```

对于一般伤害（包含蒸发、融化、超激化、蔓激化反应伤害）的叙述中：

- “基于特定数值，提升造成的伤害”指 `BaseDamage` 部分。
- “按照一定百分比，提升造成的伤害”指 `Bonus` 部分。请注意，若描述为“按照一定百分比，提升火元素伤害加成”等完整包含“*元素伤害加成”的描述，则应指 `AttributeName::BonusPyro` 等的面板属性。
- “按照一定百分比，提升特定反应的伤害”、“按照一定百分比，提升特定反应的加成系数”指 `ReactionEnhance` 部分。
- “按照一定百分比，提升某种伤害的暴击率”指 `CriticalRate` 部分。请注意，若描述为“按照一定百分比，提升角色的暴击率”等不指示特定伤害类型的描述，则应指 `AttributeName::CriticalBase` 的面板属性。
- “按照一定百分比，提升某种伤害的暴击伤害”指 `CriticalDamage` 部分。请注意，若描述为“按照一定百分比，提升角色的暴击伤害”等不指示特定伤害类型的描述，则应指 `AttributeName::CriticalDamageBase` 的面板属性。
- “无视敌人一定比例的防御力”指 `DefPenetration` 部分。

对于聚变反应（扩散、超载、感电、碎冰、超导、绽放、烈绽放、超绽放、燃烧、结晶）伤害：

- “按照一定百分比，提升特定反应的伤害”指 `ReactionEnhance` 部分。
- “基于特定数值，提升特定反应的伤害”指 `ReactionExtra` 部分。
- “某种反应可以造成暴击，暴击率为”指 `CriticalRate` 部分。
- “某种反应可以造成暴击，暴击伤害为”指 `CriticalDamage` 部分。

对于月曜反应（月感电、月绽放、月结晶）伤害：

- “按照一定百分比，提升特定反应的伤害”指 `ReactionEnhance` 部分。
- “基于特定数值，提升特定反应的伤害”指 `ReactionExtra` 部分。
- “按照一定百分比，提升某种伤害的暴击率”指 `CriticalRate` 部分。请注意，若描述为“按照一定百分比，提升角色的暴击率”等不指示特定伤害类型的描述，则应指 `AttributeName::CriticalBase` 的面板属性。
- “按照一定百分比，提升某种伤害的暴击伤害”指 `CriticalDamage` 部分。请注意，若描述为“按照一定百分比，提升角色的暴击伤害”等不指示特定伤害类型的描述，则应指 `AttributeName::CriticalDamageBase` 的面板属性。
- “按照一定百分比，提升队伍中角色造成的特定反应的基础伤害”指 `ElevativeBase` 部分。
- “造成的特定反应伤害擢升一定百分比”指 `ElevativeElevate` 部分。
- 特别的，对于作用于所有月曜反应的效果，应当使用 `ReactionType::get_elevative_reaction()` 来获取所有月曜反应的列表，以保证良好的可扩展性。

对于治疗效果：

- “角色造成的治疗效果提升一定百分比”、“角色治疗加成提升一定百分比”指 `HealingBonus` 部分。
- “角色受到的治疗效果提升一定百分比”、“角色受治疗加成提升一定百分比”指 `IncomingHealingBonus` 部分。
- “角色造成的治疗效果可以暴击，暴击率为”、“角色造成的治疗有几率提升一定百分比，几率为”指 `HealingCriticalRate` 部分。
- “角色造成的治疗效果可以暴击，暴击伤害为”、“角色造成的治疗有几率提升一定百分比”指 `HealingCriticalDamage` 部分。

对于护盾效果：

- “角色护盾强效提升一定百分比”指 `ShieldStrength` 部分。

对于施加在敌人身上的效果（减抗、减防）：

- “降低敌人的某种元素抗性”、“降低敌人所有元素抗性与物理抗性”指 `ResMinus` 部分。
- “降低敌人的防御力”指 `DefMinus` 部分。

`InvisibleAttributeType` 中的 `element`、`skill`、`reaction` 字段用来指示当前非面板属性作用的特定元素、技能或反应类型，如没有指示则为 `None`，将会匹配任意值，具体类型如下：

```rust
mona::common::element
pub enum Element {
    Pyro,           // 火元素
    Hydro,          // 水元素
    Anemo,          // 风元素
    Electro,        // 雷元素
    Dendro,         // 草元素
    Cryo,           // 冰元素
    Geo,            // 岩元素
    Physical,       // 物理
}
```

```rust
mona::common::skill_type
pub enum SkillType {
    NoneType,                   // 无
    NormalAttack,               // 普通攻击
    ChargedAttack,              // 重击
    PlungingAttackInAction,     // 下落攻击下坠期间
    PlungingAttackOnGround,     // 下落攻击坠地冲击
    ElementalSkill,             // 元素战技
    ElementalBurst,             // 元素爆发
    Elevative,                  // 月曜反应
}
```

请注意，任何技能触发的月曜反应伤害（月感电、月绽放、月结晶）的技能类型都必须是 `SkillType::Elevative`。

```rust
mona::common::reaction_type
pub enum ReactionType {
    Melt,                   // 融化
    Vaporize,               // 蒸发
    Spread,                 // 蔓激化
    Aggravate,              // 超激化
    CryoSwirl,              // 冰元素扩散
    PyroSwirl,              // 火元素扩散
    HydroSwirl,             // 水元素扩散
    ElectroSwirl,           // 雷元素扩散
    Superconduct,           // 超导
    Overload,               // 超载
    Burning,                // 燃烧
    ElectroCharged,         // 感电
    Shatter,                // 碎冰
    Bloom,                  // 绽放
    Burgeon,                // 烈绽放
    Hyperbloom,             // 超绽放
    Crystallize,            // 结晶
    LunarCharged,           // 月感电
    LunarBloom,             // 月绽放
    LunarCrystallize,       // 月结晶
}
```

要声明一个 `InvisibleAttributeType` 实例，应当通过 `InvisibleAttributeType::new` 函数，并传入 `attribute_variable_type` 以及指示特定元素、技能或反应类型的参数（如有）。如不指示特定元素、技能或反应类型，则传入 `None`。特别的，可以通过以下接口声明特定情况的 `InvisibleAttributeType`：
- `pub fn new_any(attribute_variable_type: AttributeVariableType) -> Self`：不指示特定元素、技能或反应类型。  
- `pub fn new_element(attribute_variable_type: AttributeVariableType, element: Element) -> Self`：指示特定元素。  
- `pub fn new_skill(attribute_variable_type: AttributeVariableType, skill: SkillType) -> Self`：指示特定技能。  
- `pub fn new_reaction(attribute_variable_type: AttributeVariableType, reaction: ReactionType) -> Self`：指示特定反应。  

#### AttributeType

定义位于 `mona_core/src/attribute/attribute.rs`，是用来指示任意一个属性的新标准实现，包含面板属性和非面板属性，定义如下：

```rust
mona::attribute::attribute
pub enum AttributeType {
    Panel(AttributeName),
    Invisible(InvisibleAttributeType),
}
```

通过 `AttributeType::Panel` 和 `AttributeType::Invisible` 来声明一个 `AttributeType` 实例。

#### CharacterSelector

定义位于 `mona_core/src/character/team_status.rs`，用来指示一个效果的应用角色，定义如下：

```rust
mona::character::team_status
pub struct CharacterSelector {
    pub selector: Arc<dyn Fn(&CharacterStatus) -> bool + 'static>,  
}
```

一个 `CharacterSelector` 实例即指示选择队伍中满足 `selector` 函数条件的角色。

`CharacterStatus` 定义位于 `mona_core/src/character/team_status.rs`，用来指示一个角色在队伍中对外展示的状态，定义如下：

```rust
mona::character::team_status
pub struct CharacterStatus {
    pub character_id: usize,
    pub team_id: usize,
    pub on_field: bool,
    pub character_static_data: CharacterStaticData,
    pub tags: HashSet<CharacterTag>,
}
```

其中 `character_id` 指示当前为该角色分配的 ID，`team_id` 指示当前角色所在队伍的 ID，这两个变量的值来自 `Attribute` 实例，保证 `character_id` 相同当且仅当为同一角色，`team_id` 相同当且仅当在同一队伍。`on_field` 指示当前角色是否在场上，`character_static_data` 包含了当前角色的静态数据，`tags` 包含了当前角色的标签。

`CharacterStaticData` 定义如下：
```rust
mona::character::character_static_data
pub struct CharacterStaticData {
    pub name: CharacterName,
    pub internal_name: &'static str,
    pub name_locale: I18nLocale,
    pub element: Element,
    pub hp: [i32; 15],
    pub atk: [i32; 15],
    pub def: [i32; 15],
    pub sub_stat: CharacterSubStatFamily,
    pub weapon_type: WeaponType,
    pub star: i32,

    pub skill_name1: I18nLocale,
    pub skill_name2: I18nLocale,
    pub skill_name3: I18nLocale,
}
```

其中可能会用于选择角色的字段包括 `element`、`weapon_type`。

`CharacterTag` 定义如下：
```rust
mona::character::team_status
pub type CharacterTags = HashSet<CharacterTag>
```

其中 `CharacterTag` 是一个枚举类型，包含角色可能具有的标签，目前情况下包含“月兆”、“魔导”。一个角色为月兆角色当且仅当其具有“月兆”标签，一个角色为魔导角色当且仅当其具有“魔导”标签，可以通过判断角色是否具有特定标签来得出当前角色是否为月兆角色或魔导角色，也可以通过在选择器中判断来选取具有特定标签的角色。

一般情况下使用 `CharacterSelector` 可直接调用 `CharacterSelector` 中已封装好的选择器，遇到较为复杂的情况再手动实现一个 `CharacterSelector` 实例。已有的选择器包括：

- `pub fn select_self<A>(attribute: &A) -> Self`：选择自己。  
- `pub fn select_self_onfield<A>(attribute: &A) -> Self`：仅当自己在场上时选择自己，否则不选择任何角色。  
- `pub fn select_self_offfield<A>(attribute: &A) -> Self`：仅当自己不在场上时选择自己，否则不选择任何角色。  
- `pub fn select_onfield<A>(attribute: &A) -> Self`：选择自己队伍中在场上的角色。  
- `pub fn select_offfield<A>(attribute: &A) -> Self`：选择自己队伍中不在场上的角色。  
- `pub fn select_onfield_except_self<A>(attribute: &A) -> Self`：选择自己队伍中在场上的角色，但不包括自己。  
- `pub fn select_team<A>(attribute: &A) -> Self`：选择自己队伍中的所有角色。  
- `pub fn select_team_except_self<A>(attribute: &A) -> Self`：选择自己队伍中除自己之外的所有角色。  
- `pub fn select_all<A>(attribute: &A) -> Self`：选择附近所有角色。  
- `pub fn select_all_onfield<A>(attribute: &A) -> Self`：选择附近所有在场上的角色。  
- `pub fn select_all_onfield_except_self<A>(attribute: &A) -> Self`：选择附近所有在场上的角色，但不包括自己。  
- `pub fn select_all_except_self<A>(attribute: &A) -> Self`：选择附近所有角色，但不包括自己。  
- `pub fn select_element<A>(attribute: &A, element: Element) -> Self`：选择附近特定元素的角色。  
- `pub fn select_by_tag<A>(attribute: &A, tag: CharacterTag) -> Self`：选择附近具有特定标签的角色。  
- `pub fn select_self_by_tag<A>(attribute: &A, tag: CharacterTag) -> Self`：当自己具有特定标签时选择自己，否则不选择任何角色。  

以上接口均需要通过传入 `attribute` 来获取当前角色的 `character_id` 和 `team_id`。如自行实现一个选择器，则可以通过 `attribute.get_character()` 接口获取当前角色的 `CharacterStatus` 来实现选择逻辑。注意 `Attribute` 与 `AttributeResult` 类型均是合法的可传入 `attribute` 的类型。

具体实现见 `mona_core/src/character/team_status.rs`。

#### GetCharacterMethod

`Attribute` 和 `AttributeResult` 中都打包有标记当前角色的 `character_id`，所有查询与修改接口都会默认作用于该 `character_id` 指示的角色，称为当前角色。

`Attribute` 和 `AttributeResult` 均实现了 `GetCharacterMethod` 接口，该接口有如下方法：

- `pub fn get_character_id(&self) -> &usize`：获取当前角色的 `character_id`。
- `pub fn get_character(&self) -> &CharacterStatus`：获取当前角色的 `CharacterStatus`。
- `pub fn get_characters(&self) -> &Vec<CharacterStatus>`：获取当前角色所在队伍的所有角色的 `CharacterStatus` 的列表。
- `pub fn get_characters_by_selector(&self, selector: CharacterSelector) -> Vec<&CharacterStatus>`：获取 `selector` 选择的角色的 `CharacterStatus` 的列表。
- `pub fn get_change_active_character(&self, character_id: usize) -> Self`：获取更换当前角色后的 `Attribute` 或 `AttributeResult` 的拷贝，切换后所有倍率均作用于切换后的角色面板属性，在当前角色会触发“视为其他某个角色造成的伤害”效果的场景中可能会用到。

#### EdgePriority

定义位于 `mona_core/src/attribute/attribute.rs`，用来指示一个属性边的优先级，定义如下：

```rust
mona::attribute::attribute
pub enum EdgePriority {
    Static,
    Base,
    Common,
    Invisible,
    Last,
}
```

Attribute 系统中允许创建属性边来指示一个属性对另一个属性的提升关系，如“基于某属性的一定百分比，提升某属性”。`EdgePriority` 用来指示属性边的计算优先级，计算时会按照顺序依次对当前优先级的所有属性边计算贡献。也就是优先级更高的属性边计算结果会影响当前属性边的计算结果，但优先级相等或更低的属性边计算结果不会影响当前属性边的计算结果。

- `Static`：静态优先级，并非一个普通的优先级，应用了该优先级的属性边会的计算结果会实时更新，因此需要保证应用了该优先级的属性边不成环。该优先级仅用于将 `ATKBase`、`ATKFixed`、`ATKPercentage` 等基础属性组成部分与 `ATK` 等基础属性连接起来的属性边。原则上不允许在角色、武器、圣遗物等接口实现中使用该优先级。
- `Base`：基础优先级，指示当前属性边为从基础属性（`ATKBase`、`DEFBase`、`HPBase`）引出的属性边，应用于“某角色攻击力提升一定百分比”的场景。由于基础属性不允许存在入边，因此不会出现成环问题。
- `Common`：面板属性优先级，指示当前属性边为从一个面板属性到另一面板属性的属性边。因避免成环问题，贡献到面板属性的计算结果无法二次贡献到其他面板属性。
- `Invisible`：非面板属性优先级，指示当前属性边为从一个面板属性到一个非面板属性的属性边。由于非面板属性不允许存在出边，因此不会出现成环问题。
- `Last`：末尾优先级，由于历史原因存在的一个特殊优先级，原则上不允许在角色、武器、圣遗物等接口实现中使用该优先级。

#### EdgeFunction

定义位于 `mona_core/src/attribute/attribute.rs`，用来指示一个属性边的函数关系，定义如下：

```rust
mona::attribute::attribute
pub type EdgeFunction = Arc<dyn Fn(f64, f64) -> f64>;  
```

`EdgeFunction` 的函数签名为 `Fn(f64, f64) -> f64`，传入的参数分别为属性边的两个输入值，返回值为属性边的输出值。一般情况下，属性边只有一个输入值，此时不应使用另一个输入值。  

#### AttributeResult

定义位于 `mona_core/src/attribute/attribute_result.rs`，是由 `Attribute` 类型进行 `solve()` 方法后得到的计算结果，无法再进行修改操作，并可以进行以下查询操作：

- `pub fn get_value(&self, name: AttributeName) -> f64`：查询当前角色面板属性 `name` 的值。原则上仅允许查询面板属性。
- `pub fn get_result(&self, name: AttributeName) -> Self::ResultType`：查询当前角色面板属性 `name` 的计算结果，包含了贡献到该属性的来源信息。原则上仅允许查询面板属性。
- `pub fn get_result_t(&self, ty: AttributeType) -> Self::ResultType`：查询当前角色属性 `ty` 的计算结果，包含了贡献到该属性的来源信息。
- `pub fn get_result_merge(&self, names: &[AttributeName]) -> Self::ResultType`：查询当前角色面板属性 `names` 的计算结果，包含了贡献到这些属性的来源信息的合并结果。原则上不应在角色、武器、圣遗物等接口实现中使用。
- `pub fn get_value_by_selector(&self, selector: CharacterSelector, ty: AttributeType) -> f64`：查询 `selector` 选择的角色的属性 `ty` 的值的和。
- `pub fn get_result_by_selector(&self, selector: CharacterSelector, ty: AttributeType) -> Self::ResultType`：查询 `selector` 选择的角色的属性 `ty` 的计算结果的和，包含了贡献到这些属性的来源信息的合并结果。
- `pub fn get_characters_by_selector(&self, selector: CharacterSelector) -> Vec<&CharacterStatus>`：查询 `selector` 选择的角色的 `CharacterStatus` 的列表。
- `pub fn get_em_all(&self) -> f64`：查询当前角色的面板元素精通。
- `pub fn get_atk(&self) -> f64`：查询当前角色的面板攻击力。
- `pub fn get_hp(&self) -> f64`：查询当前角色的面板生命值上限。
- `pub fn get_def(&self) -> f64`：查询当前角色的面板防御力。

其余能在 `AttributeResult` 中发现的接口均为历史原因存在的接口，不允许使用。

### 接口说明

Attribute 系统入口文件为 `mona_core/src/attribute/attribute.rs`，可以参阅。

对于所有接口传递的参数中，`key: &str` 均标识了当前属性的来源，包含发起角色与技能、发起武器或发起圣遗物及套装效果。

如无特别说明，接口传递的参数中，`name: AttributeName` 均指面板属性或因历史原因存在的其他情况，`ty: AttributeType` 为新的标准实现，包含面板属性和非面板属性。对于非历史原因存在的调用，不允许使用 `name: AttributeName` 来指示非面板属性，面板属性可以自行选择使用 `name: AttributeName` 或 `ty: AttributeType` 来指示。

如无特别说明，接口传递的参数中，`func: EdgeFunction` 指示了属性边的函数关系。

Attribute 对外提供的接口有：

- `fn set_value_by(&mut self, name: AttributeName, key: &str, value: f64)`：将当前角色的面板属性 `name` 提高 `value`。
- `fn set_value_to(&mut self, name: AttributeName, key: &str, value: f64)`：将当前角色的面板属性 `name` 中键名为 `key` 的贡献的值设定为 `value`。原则上不得在同一键名上同时使用 `set_value_by` 和 `set_value_to` 或使用多次 `set_value_to`。
- `fn set_value_by_t(&mut self, ty: AttributeType, key: &str, value: f64)`：将当前角色的属性 `ty` 提高 `value`。
- `fn set_value_to_t(&mut self, ty: AttributeType, key: &str, value: f64)`：将当前角色的属性 `ty` 中键名为 `key` 的贡献的值设定为 `value`。原则上不得在同一键名上同时使用 `set_value_by_t` 和 `set_value_to_t` 或使用多次 `set_value_to_t`。
- `fn set_value_by_s(&mut self, character_selector: CharacterSelector, ty: AttributeType, key: &str, value: f64)`：将 `character_selector` 选择的角色的属性 `ty` 提高 `value`。
- `fn set_value_to_s(&mut self, character_selector: CharacterSelector, ty: AttributeType, key: &str, value: f64)`：将 `character_selector` 选择的角色的属性 `ty` 中键名为 `key` 的贡献设定为 `value`。原则上不得在同一键名上同时使用 `set_value_by_s` 和 `set_value_to_s` 或使用多次 `set_value_to_s`。
- `fn add_edge_n1(&mut self, from: AttributeName, to: AttributeName, func: EdgeFunction, key: &str, priority: EdgePriority)`：添加从面板属性 `from` 到面板属性 `to` 的属性边，函数关系为 `func`，优先级为 `priority`。
- `fn add_edge_n2(&mut self, from1: AttributeName, from2: AttributeName, to: AttributeName, func: EdgeFunction, key: &str, priority: EdgePriority)`：添加从面板属性 `from1` 和 `from2` 到面板属性 `to` 的属性边，函数关系为 `func`，优先级为 `priority`。
- `fn add_edge_t1(&mut self, from: AttributeType, to: AttributeType, func: EdgeFunction, key: &str, priority: EdgePriority)`：添加从属性 `from` 到属性 `to` 的属性边，函数关系为 `func`，优先级为 `priority`。
- `fn add_edge_t2(&mut self, from1: AttributeType, from2: AttributeType, to: AttributeType, func: EdgeFunction, key: &str, priority: EdgePriority)`：添加从属性 `from1` 和 `from2` 到属性 `to` 的属性边，函数关系为 `func`，优先级为 `priority`。
- `fn add_edge_s1to1(&mut self, character_selector: CharacterSelector, from: AttributeType, to: AttributeType, func: EdgeFunction, key: &str, priority: EdgePriority)`：添加从当前角色属性 `from` 到被 `character_selector` 选择的角色属性 `to` 的属性边，函数关系为 `func`，优先级为 `priority`。
- `fn add_edge_s2to1(&mut self, character_selector: CharacterSelector, from1: AttributeType, from2: AttributeType, to: AttributeType, func: EdgeFunction, key: &str, priority: EdgePriority)`：添加从当前角色属性 `from1` 和 `from2` 到被 `character_selector` 选择的角色属性 `to` 的属性边，函数关系为 `func`，优先级为 `priority`。
- `fn add_edge_s1ton(&mut self, character_selector: CharacterSelector, from: AttributeType, to: AttributeType, func: EdgeFunction, key: &str, priority: EdgePriority)`：添加从被 `character_selector` 选择的角色属性 `from` 到该角色属性 `to` 的属性边，函数关系为 `func`，优先级为 `priority`。
- `fn add_edge_s2ton(&mut self, character_selector: CharacterSelector, from1: AttributeType, from2: AttributeType, to: AttributeType, func: EdgeFunction, key: &str, priority: EdgePriority)`：添加从被 `character_selector` 选择的角色属性 `from1` 和 `from2` 到该角色属性 `to` 的属性边，函数关系为 `func`，优先级为 `priority`。
- `fn add_atk_percentage(&mut self, key: &str, value: f64)`：将当前角色的攻击力提升 `value` 百分比。
- `fn add_atk_percentage_s(&mut self, character_selector: CharacterSelector, key: &str, value: f64)`：将 `character_selector` 选择的角色的攻击力提升 `value` 百分比。
- `fn add_def_percentage(&mut self, key: &str, value: f64)`：将当前角色的防御力提升 `value` 百分比。
- `fn add_def_percentage_s(&mut self, character_selector: CharacterSelector, key: &str, value: f64)`：将 `character_selector` 选择的角色的防御力提升 `value` 百分比。
- `fn add_hp_percentage(&mut self, key: &str, value: f64)`：将当前角色的生命值上限提升 `value` 百分比。
- `fn add_hp_percentage_s(&mut self, character_selector: CharacterSelector, key: &str, value: f64)`：将 `character_selector` 选择的角色的生命值上限提升 `value` 百分比。
- `fn add_elemental_bonus(&mut self, key: &str, value: f64)`：将当前角色的所有元素伤害加成的面板属性提升 `value` 百分比。

请注意，由于默认情况下同一来源的增益无法叠加（包括同名武器的同名效果、同名圣遗物的同名效果），因此除非明确可以叠加的增益（如“多件同名武器产生的此效果可以叠加”），其余均建议使用 `set_value_to`、`set_value_to_t`、`set_value_to_s` 等而非 `set_value_by`、`set_value_by_t`、`set_value_by_s` 等接口来实现，以避免错误叠加与重复计算。

以下为历史接口，不应使用：

- `fn add_edge1(&mut self, from: AttributeName, to: AttributeName, fwd: EdgeFunctionFwd, bwd: EdgeFunctionBwd, key: &str)`：历史接口，不应使用。
- `fn add_edge2(&mut self, from1: AttributeName, from2: AttributeName, to: AttributeName, fwd: EdgeFunctionFwd, bwd: EdgeFunctionBwd, key: &str)`：历史接口，不应使用。
- `fn add_atk_percentage_base(&mut self, key: &str, value: f64)`：历史接口，不应使用。
- `fn add_def_percentage_base(&mut self, key: &str, value: f64)`：历史接口，不应使用。
- `fn add_hp_percentage_base(&mut self, key: &str, value: f64)`：历史接口，不应使用。

### 来源命名

`key` 的命名规范为“效果来源对象+效果来源位置”，如“杜林天赋1”、“杜林命座6”、“黑蚀被动”、“风起之日4”等，一般不需要对同一来源的不同效果进行区分，方便在 Attribute 系统中对效果进行合并。

对于角色来源，应命名为“角色名+效果来源”。普通攻击、元素战技、元素爆发应命名为“技能A”、“技能E”、“技能Q”，天赋按照顺序依次命名为“天赋1”、“天赋2”、“天赋3”（不计算生活天赋），命座依次命名为“命座1”到“命座6”。  
对于武器来源，一般直接命名为“武器名+‘被动’”即可，如“黑蚀被动”。  
对于圣遗物来源，应命名为“套装名+n”，其中 n 为套装效果为几件套，如“风起之日4”。  
对于 buff 来源，应与其真实来源的命名规范保持一致，如 “哥伦比娅Q技能”、“哥伦比娅天赋3”、“哥伦比娅命座2”等。

### 注意事项

#### 攻击力 / 防御力 / 生命值上限

> 本条极易出错，请在实现前后仔细确认。

对于攻击力、防御力、生命值上限，共有四个有关属性：`ATKBase`、`ATKFixed`、`ATKPercentage`、`ATK`（防御力和生命值上限同理）。其中 `ATKBase` 指基础攻击力，除非明确提升基础攻击力，否则不得修改该属性；`ATKFixed` 指攻击力固定数值部分提升；`ATKPercentage` 指攻击力百分比部分提升，但使用方法不是直接将提升百分比加到该属性上，而是需要通过从 `ATKBase` 到 `ATKPercentage` 的相应比例的属性边来实现或者通过封装的便捷方法；`ATK` 是 `ATKBase`、`ATKFixed`、`ATKPercentage` 三者的实时和，基于攻击力的加成应当从 `ATK` 引出属性边来实现，不允许有指向 `ATK` 的属性边。

具体的：

| 属性 | 含义 | 如何修改 |
|------|------|---------|
| `ATKBase` | 基础攻击力 | 除非明确说"提升基础攻击力"，否则**不得修改** |
| `ATKFixed` | 攻击力固定数值 | 可直接 `set_value_to` 或 `set_value_by` |
| `ATKPercentage` | 攻击力百分比 | **不可直接 `set_value_to` 或 `set_value_by`**，推荐使用 `add_atk_percentage` 或 `add_atk_percentage_s` 等方法，或者通过 `ATKBase -> ATKPercentage` 的属性边实现 |
| `ATK` | 最终攻击力 | 不允许有指向 `ATK` 的属性边，所有攻击力加成应按照固定数值或百分比应用到 `ATKFixed` 或 `ATKPercentage`。基于攻击力的加成应从 `ATK` 引出边 |

防御力（`DEF`）和生命值上限（`HP`）同理。

**常见错误 vs 正确做法：**

```rust
// 错误：直接给 ATKPercentage / HPPercentage 设值
attribute.set_value_to(AttributeName::ATKPercentage, "key", 0.25);
attribute.set_value_to_t(AttributeType::Panel(AttributeName::HPPercentage), "key", 0.25);
attribute.set_value_by_s(selector, AttributeType::Panel(AttributeName::ATKPercentage), "key", 0.25);

// 正确：对当前角色，使用便捷方法
attribute.add_atk_percentage("key", 0.25);
attribute.add_hp_percentage("key", 0.25);
attribute.add_def_percentage("key", 0.25);

// 正确：对其他角色（场上/全队等），使用 add_edge_s1ton
attribute.add_edge_s1ton(
    CharacterSelector::select_onfield(attribute),
    AttributeType::Panel(AttributeName::ATKBase),
    AttributeType::Panel(AttributeName::ATKPercentage),
    Arc::new(move |base: f64, _| base * 0.3),
    "效果名称",
    EdgePriority::Base,
);

// 正确：对其他角色（场上/全队等），使用便捷方法
attribute.add_atk_percentage_s(CharacterSelector::select_onfield(attribute), "效果名称", 0.3);
attribute.add_def_percentage_s(CharacterSelector::select_onfield(attribute), "效果名称", 0.3);
attribute.add_hp_percentage_s(CharacterSelector::select_onfield(attribute), "效果名称", 0.3);
```

> `add_atk_percentage` 仅对当前角色生效。为其他角色添加时，必须用 `add_edge_s1ton`（在**被选中角色自身**的 Base → Percentage 间建边），**不可**用 `add_edge_s1to1`（那会从当前角色建边）。

#### 减抗 / 减防

对于施加在敌人身上的减抗（`ResMinus`）、减防（`DefMinus`）效果，应**总是**使用 `CharacterSelector::select_all(attribute)`，以保证所有伤害都受到该效果的影响：

```rust
// 正确
attribute.set_value_to_s(
    CharacterSelector::select_all(attribute),
    AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::ResMinus, Element::Geo)),
    "效果名称",
    value,
);
```

### 调用示例

> 队伍中附近的角色对敌人触发燃烧、超载、火元素扩散、火元素结晶反应后，或对处于燃烧状态下的敌人造成火元素伤害或草元素伤害时，该敌人的火元素抗性与参与反应的对应元素抗性降低20%，持续6秒。  
```rust
if self.elements.pyro {
    attribute.set_value_by_s(CharacterSelector::select_all(attribute),
        AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::ResMinus, Element::Pyro)),
        "杜林天赋1", ratio,
    );
}
```

> 杜林触发的蒸发反应造成的伤害提升40%，融化反应造成的伤害提升40%。  
```rust
attribute.set_value_by_t(
    AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, ReactionType::Vaporize)),
    "杜林天赋1", if self.hexerei_secret_rite { 0.7 } else { 0.4 }
);
```

> 杜林造成的元素爆发伤害提升40%。此外，触发命之座「红土之逆」中消耗「轮变启迪」的效果时，有30%几率不会消耗「轮变启迪」。  
```rust
attribute.set_value_by_t(
    AttributeType::Invisible(InvisibleAttributeType::new_skill(AttributeVariableType::Bonus, SkillType::ElementalBurst)),
    "杜林命座4",
    0.4
);
```

> 队伍中附近的角色造成绽放、超绽放、烈绽放、月绽放反应伤害时，将消耗一层「苍色祷歌」，提升造成的伤害，提升值基于菈乌玛的元素精通。上述伤害同时命中多名敌人时，会依据命中敌人的数量消耗「苍色祷歌」层数。  
```rust
attribute.add_edge_s1to1(
    CharacterSelector::select_all(attribute),
    AttributeType::Panel(AttributeName::ElementalMastery),
    AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionExtra, ReactionType::Bloom)),
    Arc::new(move |em, _| {
        em * q_bloom_increase
    }),
    "菈乌玛Q技能",
    EdgePriority::Invisible,
);
```

> 队伍中附近的所有角色触发的绽放、超绽放、烈绽放反应造成的伤害能够造成暴击，暴击率固定为15%，暴击伤害固定为100%。该效果提供的暴击率可以与使对应元素反应能够造成暴击的同类效果提供的暴击率叠加。  
```rust
attribute.set_value_to_s(
    CharacterSelector::select_all(attribute),
    AttributeType::Invisible(InvisibleAttributeType::new_reaction(
        AttributeVariableType::CriticalDamage,
        ReactionType::Bloom
    )),
    "菈乌玛天赋1",
    1.0
);
```

> 队伍中的角色触发绽放反应时，将转为触发月绽放反应，且基于菈乌玛的元素精通，提升队伍中角色造成的月绽放反应的基础伤害：每点元素精通都将提升0.0175%月绽放反应的基础伤害，至多通过这种方式提升14%伤害。  
```rust
attribute.add_edge_s1to1(
    CharacterSelector::select_all(attribute),
    AttributeType::Panel(AttributeName::ElementalMastery),
    AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ElevativeBase, ReactionType::LunarBloom)),
    Arc::new(move |em: f64, _| (em * 0.000175).min(0.14) ),
    "菈乌玛天赋3",
    EdgePriority::Invisible,
);
```