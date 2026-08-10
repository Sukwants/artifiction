# Review 检查清单

> 本文档由 GitHub Copilot (DeepSeek V4 Pro) 于 2026-06-29 生成，用于代码 review 时系统性地检查可能存在的问题。

---

## 一、角色文件（Character）

### 1.1 Stats 部分

- [ ] **`SkillType` 结构体字段命名**：普通攻击以 `a_` 开头，重击以 `z_` 开头，下落攻击以 `x_` 开头，元素战技以 `e_` 开头，元素爆发以 `q_` 开头，天赋技能以 `p1_`、`p2_`、`p3_` 开头，命座技能以 `c1_`、`c2_` 等开头。检查是否有遗漏或不一致。
- [ ] **`SkillType` 字段区分**：伤害使用 `dmg`、治疗使用 `heal`、护盾使用 `shield` 标识。同一技能的不同段数使用数字区分。检查命名是否清晰、有独立名称的技能是否提取了合适表达。
- [ ] **技能等级数组类型**：若技能等级 1-15 的数值各不相同，类型应为 `[f64; 15]`；若各等级数值相同，可以是单一 `f64`。检查是否误用了类型。
- [ ] **百分比数值格式**：所有百分比数值必须以小数形式表示（如 50% 应写为 `0.5`）。检查是否存在直接写百分号数值的错误。
- [ ] **`StaticData` 基础属性**：`hp`、`atk`、`def` 数组长度为 15，对应等级 1/20/20+/40/40+/50/50+/60/60+/70/70+/80/80+/90/100。检查数组长度和数值是否正确。
- [ ] **`sub_stat` 选择**：确认角色突破属性选择正确，如 `CriticalDamage384` 表示 38.4% 暴伤。检查是否与游戏数据一致。
- [ ] **`weapon_type` 和 `star`**：检查武器类型和星级是否正确。
- [ ] **技能名称**：`skill_name1`/`skill_name2`/`skill_name3` 需要中英文两种语言。检查翻译是否准确。
- [ ] **`CharacterName` 枚举注册**：确认角色名称已追加到 `CharacterName` 枚举末尾。检查是否遗漏注册。

### 1.2 Effect 部分

- [ ] **`Effect` 结构体**：必须包含 `common_data: CharacterCommonData` 字段。检查是否有遗漏。
- [ ] **`change_attribute` 实现**：天赋 1/2 需要通过 `self.common_data.has_talent1` / `has_talent2` 判断；命座通过 `self.common_data.constellation >= N` 判断。检查条件判断是否正确。
- [ ] **配置项划分**：属于角色配置（`CONFIG_DATA`）还是技能配置（`CONFIG_SKILL`）？静态/半静态配置放入 `CONFIG_DATA`，随技能释放变化的配置放入 `CONFIG_SKILL`。检查配置归属是否合理。
- [ ] **`CONFIG_DATA` 配置项注册**：每个配置项的 `name` 必须与 Effect 结构体字段名一致，`title` 需中英文，`config` 类型选择正确。检查是否有遗漏或不一致。
- [ ] **`new_effect` 函数**：所有配置项必须添加到 `CharacterConfig` 枚举中，并在 `new_effect` 中正确解构。检查是否有遗漏的配置项。
- [ ] **`CharacterConfig` 注册**：确认新增的配置变体已添加到 `CharacterConfig` 枚举。检查是否遗漏。

### 1.3 DamageEnum 部分

- [ ] **`damage_enum!` 宏声明**：普通攻击以 `A` 开头，元素战技以 `E` 开头，元素爆发以 `Q` 开头，天赋技能以 `P1`/`P2`/`P3` 开头，命座以 `C1`/`C2` 等开头。检查命名规范性。
- [ ] **技能拆分**：造成多次相同倍率伤害的技能是否拆分为单次伤害和总伤害两个变体？检查是否有遗漏。
- [ ] **合并处理**：分开书写但不会同时出现、紧密关联的技能（如条件替换伤害）是否使用同一 DamageEnum？检查是否正确处理。
- [ ] **`get_element` 实现**：普通攻击一般为 `Physical` 或对应元素，元素战技/爆发为对应元素，治疗一般为 `Physical`，护盾返回护盾元素类型。检查元素类型是否正确。
- [ ] **`get_element` 与染色**：若角色存在染色伤害，`get_element` 是否需要传入参数进行判断？检查是否正确实现。
- [ ] **`get_skill_type` 实现**：技能类型应基于实际触发方式而非描述所在位置。协同攻击一般基于初始触发类型；"视为元素战技伤害"应严格按描述判断。检查技能类型是否正确。
- [ ] **擢升反应技能类型**：任何技能触发的擢升反应伤害（月感电/月绽放/月结晶/星超导/星扩散）技能类型必须是 `SkillType::Elevative`。检查是否遗漏。
- [ ] **`get_elevative_type` 实现**：若角色触发擢升反应伤害，是否实现了此方法？检查返回值是否正确。
- [ ] **`ElevativeReaction` 类型**：在 `DamageEnum` 中不允许出现 `LunarChargedReaction` 或 `LunarCrystallizeReaction`（这两种通过元素反应触发），也不允许出现 `StellarSwirlReactionAnemo` 或 `StellarSwirlReactionCryo`（星扩散反应风伤/冰伤通过元素反应触发）。检查是否正确使用 `LunarCharged`/`LunarBloom`/`LunarCrystallize`/`StellarSwirlAnemo`/`StellarSwirlCryo`。
- [ ] **命座伤害拆分**：当同一命座伤害可能由不同技能触发时，是否拆分为多个 DamageEnum 变体（如 C4E/C4Q）并分别归入对应 skill_map？检查是否正确拆分。
- [ ] **`DEFAULT_TAGS`**：月兆角色需要 `CharacterTag::Moonsign`，魔导角色需要 `CharacterTag::Hexerei`。检查标签设置是否正确。
- [ ] **`SKILL_MAP`**：所有需要对用户展示的技能是否都在 `skill_map!` 中注册？检查名称是否准确、中英文是否齐全。

### 1.4 EffectSkill 部分

- [ ] **`CONFIG_SKILL` 配置项**：每个配置项的 `name` 必须与 `CharacterSkillConfig` 中对应字段名一致。检查是否有遗漏或不一致。
- [ ] **`CharacterSkillConfig` 注册**：确认新增的技能配置变体已添加到 `CharacterSkillConfig` 枚举。检查是否遗漏。
- [ ] **`change_attribute`（技能）**：天赋和命座的条件判断是否使用 `common_data.has_talent1`/`has_talent2` 和 `common_data.constellation`？检查条件判断是否正确。
- [ ] **配置项解析**：`change_attribute` 中是否正确解析了 `CharacterConfig` 和 `CharacterSkillConfig`？检查默认值处理是否合理。

### 1.5 Damage 部分

- [ ] **`damage_internal` 实现**：是否先处理角色配置和技能配置，再将技能相关效果应用到 builder，然后获取技能倍率，最后调用 builder 方法？检查顺序是否正确。
- [ ] **条件伤害检查**：条件伤害（需要元素转化、天赋、命座等）是否在计算倍率之前检查？不满足时是否返回 `builder.none()`？检查检查顺序（先数据条件，再天赋/命座条件）。
- [ ] **DamageBuilder 方法选择**：普通伤害用 `builder.damage`，擢升反应伤害用 `builder.elevative`，治疗量用 `builder.heal`，护盾用 `builder.shield`，纯数值用 `builder.number`，不触发用 `builder.none`。检查方法选择是否正确。
- [ ] **不应使用 `builder.transformative`**：角色文件中原则上不应出现 `builder.transformative` 方法。检查是否有误用。
- [ ] **倍率获取**：倍率是否正确从 `SkillType` 中获取？技能等级 `s1`/`s2`/`s3` 对应关系是否正确（`s1`=普通攻击等级, `s2`=元素战技等级, `s3`=元素爆发等级）？
- [ ] **额外效果添加**：`add_extra_*` 方法的 key 命名是否清晰标识了效果来源？数值是否正确传递？

---

## 二、武器文件（Weapon）

### 2.1 武器配置项

- [ ] **配置项声明**：武器所需的所有配置项是否都在 Effect 结构体中声明？检查是否有遗漏。
- [ ] **平均层数处理**：对于层数较多或因素复杂的效果，是否通过配置项设置平均层数替代？注释中是否明确说明？检查合理性。

### 2.2 武器效果实现

- [ ] **`apply` 方法**：是否包含了武器所有效果的修改逻辑？是否通过 `data.refine` 获取精炼等级？检查是否有遗漏效果。
- [ ] **精炼等级数值**：精炼相关数值是否正确使用了 `refine` 变量计算？检查公式是否正确。
- [ ] **key 命名**：一般为"武器名称+被动"，如"霜结的誓金枝被动"。检查命名是否规范。
- [ ] **叠加处理**：默认情况下同名武器效果不可叠加，是否使用了 `set_value_to` 等接口避免重复计算？除非明确声明"可以叠加"。

### 2.3 武器基本信息

- [ ] **`weapon_base`**：基础攻击力是否从 `WeaponBaseATKFamily` 中选择正确值？检查满级攻击力是否匹配。
- [ ] **`weapon_sub_stat`**：副属性类型和数值是否正确？检查是否从 `WeaponSubStatFamily` 中选择正确值。
- [ ] **武器名称注册**：是否追加到 `WeaponName` 枚举末尾？
- [ ] **特效描述**：精炼等级数值是否使用 `<span style=\"color: #409EFF;\">{1}-{2}-{3}-{4}-{5}</span>` 格式填写？百分号是否包含在内？

### 2.4 武器配置项注册

- [ ] **`CONFIG_DATA`**：每个配置项 `name` 是否与 Effect 结构体字段名一致？`title` 是否有中英文？`config` 类型是否正确？
- [ ] **`WeaponConfig` 注册**：新增配置是否添加到 `WeaponConfig` 枚举？
- [ ] **`get_effect` 函数**：是否正确解析了所有配置项？默认值处理是否合理？

---

## 三、圣遗物文件（Artifact）

### 3.1 圣遗物配置项

- [ ] **配置项声明**：圣遗物所需的所有配置项是否都在 Effect 结构体中声明？检查是否有遗漏。
- [ ] **平均层数处理**：对于层数较多或因素复杂的效果，是否通过配置项设置平均层数替代？注释中是否明确说明？

### 3.2 圣遗物效果实现

- [ ] **`effect{n}` 方法**：是否包含了圣遗物所有效果的修改逻辑？例如 `effect2` 和 `effect4` 是否都已实现？
- [ ] **辅助类圣遗物效果**：对于应用到所有角色的效果，是否使用了 `set_value_to` 避免重复计算？检查是否正确使用。
- [ ] **key 命名**：一般为"套装名+n"，如"风起之日4"。检查命名是否规范。

### 3.3 圣遗物基本信息

- [ ] **名称注册**：是否追加到 `ArtifactSetName` 枚举末尾？
- [ ] **各部件名称**：flower/feather/sand/goblet/head 的中英文名称是否完整？
- [ ] **可用稀有度**：`star` 字段是否设置了正确的稀有度范围？
- [ ] **套装效果描述**：`effect2`/`effect4` 的描述是否准确摘抄了游戏原文？
- [ ] **`internal_id`**：内部 ID 是否正确设置？

### 3.4 圣遗物配置项注册

- [ ] **全局联动配置**：圣遗物配置是否需要注册为全局联动配置？键名格式是否为 `[套装标识]配置名称`？检查格式正确性。
- [ ] **`CONFIG4`**：配置项 `name` 是否与 Effect 结构体字段名一致？`title` 是否有中英文？`config` 类型是否正确？
- [ ] **注册位置**：是否在 `ArtifactConfigInterface`、`ArtifactEffectConfig`、`ArtifactEffectConfigBuilder` 等位置完成了注册？没有配置项的圣遗物不需要注册。
- [ ] **`create_effect` 函数**：是否正确解析了所有配置项？

---

## 四、Buff 文件

### 4.1 Buff 适用性原则

- [ ] **作用范围**：buff 只应对当前角色生效，不允许使用角色选择器操作其他角色。检查是否有越权操作。
- [ ] **不读取其他角色属性**：buff 不应通过 Attribute 系统读取其他角色的属性，所有效果应通过配置项实现。检查是否有违规读取。
- [ ] **不应重复实现**：是否与角色/武器/圣遗物文件中的实现存在冲突？同一效果若已在其他位置实现，是否需要通过 `set_value_to` 避免叠加？

### 4.2 Buff 配置项

- [ ] **配置项合理性**：如果整个 buff 效果需要某条件为真才生效，是否省略了该条件配置项（应用即视为条件成立）？
- [ ] **平均层数处理**：对于复杂效果，是否通过配置项设置平均值？注释是否说明？

### 4.3 Buff 效果实现

- [ ] **`change_attribute` 方法**：是否包含了所有效果的修改逻辑？
- [ ] **key 命名**：是否与真实来源的命名一致？如"哥伦比娅Q技能"、"霜结的誓金枝被动"等。
- [ ] **叠加避免**：是否使用了 `set_value_to` 等接口避免错误叠加？
- [ ] **角色倍率引用**：如有需要，是否通过 `use` 正确引用了角色倍率？

### 4.4 Buff 基本信息

- [ ] **名称注册**：是否追加到 `BuffName` 枚举末尾？命名格式是否为"来源标识-「效果名称」"？
- [ ] **效果描述**：是否以来源描述开头（如"哥伦比娅天赋3："）？是否摘抄原文并剔除无关部分？是否在合适位置使用 `<br>` 换行？

### 4.5 Buff 配置项注册

- [ ] **`CONFIG`**：配置项 `name` 是否与 Effect 结构体字段名一致？`title` 是否有中英文？`config` 类型是否正确？
- [ ] **`BuffConfig` 注册**：新增配置是否添加到 `BuffConfig` 枚举？
- [ ] **`create` 函数**：是否正确解析了所有配置项？默认值处理是否合理？

---

## 五、Target Function 文件

### 5.1 Target Function 配置项

- [ ] **配置项沟通**：target_function 与角色实际情况关系极大，是否已与用户充分沟通确认？AI 是否擅自猜测了配置项？

### 5.2 Target Function 基本信息

- [ ] **名称注册**：是否追加到 `TargetFunctionName` 枚举末尾？
- [ ] **名称和描述**：`chs`/`description`/`tags` 是否根据用户指示填写？
- [ ] **`four` 字段**：是否正确关联到对应角色 `CharacterName`？

### 5.3 Target Function 配置项注册

- [ ] **`CONFIG`**：配置项 `name` 是否与 Effect 结构体字段名一致？`title` 是否有中英文？`config` 类型是否正确？
- [ ] **额外配置项**：用户是否指示了未出现在 Effect 结构体中的配置项（如覆写全局联动圣遗物配置）？不应擅自添加到 Effect 结构体中。
- [ ] **`TargetFunctionConfig` 注册**：新增配置是否添加到 `TargetFunctionConfig` 枚举？

### 5.4 Target Function 效果实现

- [ ] **角色文件引用**：是否正确引用了对应角色文件？
- [ ] **`target` 方法参数使用**：`character`/`weapon`/`artifacts` 参数大多只用到 `character.common_data`，是否未经指示使用了其他方法？检查是否有越权使用。
- [ ] **技能配置创建**：每一套技能配置是否创建了独立的 `CharacterSkillConfig`？是否通过 `change_attribute` 处理后创建独立的 `DamageContext`？
- [ ] **伤害计算**：是否使用了 `damage`/`transformative_damage`/`elevative_damage` 等正确的方法？检查方法选择是否正确。
- [ ] **擢升反应伤害系数**：狭义擢升反应伤害（`LunarChargedReaction`/`LunarCrystallizeReaction`）需要乘以角色在反应中的伤害系数，是否已处理？
- [ ] **特殊条件处理**：最低攻击力/充能效率等特殊要求是否通过查询 `attribute` 进行了判断？不满足时是否返回 0？
- [ ] **结果类型**：target_function 的结果可能不是伤害，而是攻击力等数值，是否按照用户指示正确计算？

### 5.5 词条权重设置

- [ ] **`get_target_function_opt_config`**：是否已经根据角色进行设置？可忽略权重与套装正确性检查。

### 5.6 默认圣遗物配置

- [ ] **`get_default_artifact_config`**：是否根据角色需要修改了部分圣遗物的默认配置项？可忽略套装正确性检查。

---

## 六、Attribute 系统使用

### 6.1 面板属性 vs 非面板属性

- [ ] **面板属性使用**：是否只使用了 `is_panel()` 返回 `true` 的 `AttributeName`？检查是否误用了非面板属性名称。
- [ ] **非面板属性声明**：是否正确使用 `InvisibleAttributeType::new_*` 方法声明非面板属性？检查 `attribute_variable_type`/`element`/`skill`/`reaction` 参数是否正确。
- [ ] **一般伤害叙述对应**："基于特定数值提升伤害"→`BaseDamage`；"按百分比提升伤害"→`Bonus`；"提升反应伤害/加成系数"→`ReactionEnhance`；"提升暴击率"→`CriticalRate`；"提升暴击伤害"→`CriticalDamage`；"无视防御力"→`DefPenetration`。检查对应关系是否正确。
- [ ] **聚变反应叙述对应**："提升反应伤害"→`ReactionEnhance`；"基于数值提升反应伤害"→`ReactionExtra`；反应暴击率→`CriticalRate`；反应暴击伤害→`CriticalDamage`。检查对应关系是否正确。
- [ ] **擢升反应叙述对应**：特别注意"提升反应基础伤害"→`ElevativeBase`；"反应擢升"→`ElevativeElevate`；星超导附着次数→`ElevativeCoefficient`；星扩散风涡系数→`ElevativeCoefficient`。检查对应关系是否正确。
- [ ] **治疗效果叙述对应**："治疗加成提升"→`HealingBonus`；"受治疗加成提升"→`IncomingHealingBonus`；治疗暴击率→`HealingCriticalRate`；治疗暴击伤害→`HealingCriticalDamage`。
- [ ] **护盾效果叙述对应**："护盾强效提升"→`ShieldStrength`。
- [ ] **敌人效果叙述对应**："降低抗性"→`ResMinus`；"降低防御力"→`DefMinus`。

### 6.2 月曜反应通用处理

- [ ] **月曜反应列表**：作用于所有月曜反应的效果是否使用了 `ReactionType::get_elevative_reaction()` 获取列表以保证可扩展性？检查是否硬编码了月曜反应。

### 6.3 攻击力/防御力/生命值处理 ⚠️ 高频错误

- [ ] **`ATKBase`**：除非明确说"提升基础攻击力"，否则不得修改。检查是否有误修改。
- [ ] **`ATKPercentage`**：不可直接 `set_value_to` 或 `set_value_by`。必须使用 `add_atk_percentage` 便捷方法或通过 `ATKBase -> ATKPercentage` 的 `EdgePriority::Base` 属性边实现。检查是否有直接修改的错误。
- [ ] **`ATK`**：不允许有指向 `ATK` 的属性边。所有基于攻击力的加成应从 `ATK` 引出边。检查是否有违规入边。
- [ ] **对其他角色添加攻击力百分比**：必须使用 `add_edge_s1ton`（在目标角色的 Base→Percentage 间建边）而非 `add_edge_s1to1`（从当前角色建边）。检查边方向是否正确。
- [ ] **防御力/生命值同理**：`DEF`/`HP` 的处理规则与 `ATK` 一致，检查是否同样遵循。

### 6.4 减抗/减防处理

- [ ] **选择器**：减抗（`ResMinus`）和减防（`DefMinus`）效果是否总是使用 `CharacterSelector::select_all(attribute)`？检查是否错误使用了其他选择器。

### 6.5 叠加处理

- [ ] **`set_value_to` vs `set_value_by`**：默认同一来源不可叠加，是否优先使用 `set_value_to`/`set_value_to_t`/`set_value_to_s`？检查是否有应使用 `set_value_to` 但用了 `set_value_by` 的情况。
- [ ] **键名一致性**：同一键名不应同时使用 `set_value_by` 和 `set_value_to`，也不应多次调用 `set_value_to`。检查是否有冲突。
- [ ] **同增益键名**：对于本质上来自同一来源的增益效果（如在角色文件与 buff 文件出现两次的增益），是否使用了相同的键名？

### 6.6 EdgePriority 使用

- [ ] **`Static` 优先级**：原则上不允许在角色/武器/圣遗物等接口中使用。检查是否有误用。
- [ ] **`Base` 优先级**：仅用于从 `ATKBase`/`DEFBase`/`HPBase` 引出的属性边。检查是否有误用到其他场景。
- [ ] **`Common` 优先级**：用于面板属性到面板属性的属性边。检查使用场景是否正确。
- [ ] **`Invisible` 优先级**：用于面板属性到非面板属性的属性边。检查使用场景是否正确。
- [ ] **`Last` 优先级**：原则上不允许使用。检查是否有误用。

### 6.7 CharacterSelector 使用

- [ ] **选择器选择**：是否正确选择了合适的 `CharacterSelector`？如仅对自己生效用 `select_self`，对全队用 `select_team`，对场上角色用 `select_onfield` 等。
- [ ] **`select_all` 配合减抗/减防**：减抗减防效果总是使用 `select_all`，检查是否正确。

### 6.8 来源命名

- [ ] **key 命名规范**：是否遵循"效果来源对象+效果来源位置"的格式？如"杜林天赋1"、"黑蚀被动"、"风起之日4"。
- [ ] **角色来源**：普攻/战技/爆发 → "技能A"/"技能E"/"技能Q"；天赋 → "天赋1"/"天赋2"/"天赋3"（排除生活天赋）；命座 → "命座1"到"命座6"。
- [ ] **武器来源**："武器名+被动"，如"黑蚀被动"。
- [ ] **圣遗物来源**："套装名+n"，如"风起之日4"。
- [ ] **buff 来源**：与其真实来源命名一致，如"哥伦比娅Q技能"。

### 6.9 天赋编号 ⚠️ 高频错误

- [ ] **生活天赋排除**：对战斗天赋编号时是否排除了生活天赋？一般情况下一个角色至多三个战斗天赋。检查编号是否正确。
- [ ] **生活天赋命名**：若生活天赋与战斗相关，来源命名为"天赋4"。

---

## 七、Config 配置项

### 7.1 配置类型选择

- [ ] **`Float` vs `FloatInput`**：浮点滑块（范围较小）用 `Float`，浮点输入框（范围较大）用 `FloatInput`。
- [ ] **`Int` vs `IntInput`**：小范围整数（如技能等级）用 `Int`，大范围整数用 `IntInput`。
- [ ] **`Option` vs `Option2`**：有实际意义表述的选项建议用 `Option2`（包含中英文）。
- [ ] **`FloatPercentageInput`**：不建议使用。检查是否有误用。
- [ ] **`Element`/`Element4`/`Element8` 等**：根据需求选择合适的元素选择器。检查选择是否正确。

### 7.2 全局联动配置

- [ ] **`GlobalLink` 配置项**：`key` 是否唯一标识了全局联动配置？`priority` 是否使用了正确的优先级常量？`team_shared` 是否正确设置？
- [ ] **优先级常量**: 
  - `PRIORITY_DEFAULT` (0)：默认
  - `PRIORITY_CHARACTERSKILL` (1)：角色技能配置
  - `PRIORITY_ARTIFACT` (2)：圣遗物配置
  - `PRIORITY_TARGETFUNCTION` (3)：target_function 配置
  - `PRIORITY_BUFF` (4)：buff 配置
  - `PRIORITY_WEAPON` (5)：武器配置
  - `PRIORITY_CHARACTER` (6)：角色配置
- [ ] **预设全局联动**：月兆配置（`MOONSIGN_GLOBAL`）、魔导配置（`HEXEREI_SECRET_RITE_GLOBAL`）、星超导附着次数（`STELLAR_CONDUCT_APPLICATION_COUNT`）、辉映·星烁状态（`STELLAR_GLIMMER_STATE`）是否在相关位置正确使用？星扩散风涡系数配置位置待定，待确定后检查是否在相关位置正确使用。

### 7.3 配置解析

- [ ] **解析完整性**：所有配置项是否都在 `new_effect`/`create`/`get_effect`/`create_effect` 等函数中正确解析？检查是否有遗漏。
- [ ] **默认值处理**：解析失败时的默认值是否合理？检查可能出现的 `_ => default` 分支是否合适。

---

## 八、DamageBuilder 使用

### 8.1 效果添加顺序

- [ ] **添加额外效果**：是否按照正确顺序调用 `add_extra_*` 方法？添加的效果是否仅针对特定技能？先添加针对特定技能的效果，再添加倍率。
- [ ] **`add_extra_enhance_melt`/`add_extra_enhance_vaporize`**：历史方法，不应使用。检查是否有误用。

### 8.2 倍率添加

- [ ] **倍率方法选择**：攻击力倍率用 `add_atk_ratio`，防御力用 `add_def_ratio`，生命值用 `add_hp_ratio`，元素精通用 `add_em_ratio`，常数值用 `add_base`。检查方法选择是否正确。
- [ ] **`add_base` 限用**：仅用于治疗量或护盾量的常数值。检查是否正确使用。

### 8.3 结果计算方法选择

- [ ] **`damage`**：普通伤害。传入 `attribute`/`enemy`/`element`/`skill_type`/`character_level`/`fumo`。检查参数是否正确。
- [ ] **`transformative`**：聚变反应伤害/护盾。额外传入 `transformative_type`。角色文件中不应使用。
- [ ] **`elevative`**：擢升反应伤害。额外传入 `elevative_type`。检查 `skill_type` 是否为 `SkillType::Elevative`。
- [ ] **`heal`**：治疗量。仅需 `attribute`。
- [ ] **`shield`**：护盾量。需 `attribute` 和 `element`。
- [ ] **`number`**：纯数值，不受属性加成。如"基础伤害提升数值"。
- [ ] **`none`**：当前配置下不触发的情况。

---

## 九、跨模块一致性

### 11.1 枚举注册

- [ ] **名称枚举**：`CharacterName`/`WeaponName`/`ArtifactSetName`/`BuffName`/`TargetFunctionName` 是否都在对应枚举末尾追加？检查是否遗漏。
- [ ] **配置枚举**：`CharacterConfig`/`CharacterSkillConfig`/`WeaponConfig`/`ArtifactEffectConfig`/`BuffConfig`/`TargetFunctionConfig` 是否都在对应枚举中注册？检查是否遗漏。

### 11.2 mod.rs 导出

- [ ] **模块导出**：新增文件是否在对应 `mod.rs` 中正确声明和导出？如 `pub use new_character::NewCharacter;`。

---

## 十二、常见错误速查表

| 序号 | 错误类型 | 错误示例 | 正确做法 |
|------|---------|---------|---------|
| 1 | 直接修改 ATKPercentage | `set_value_to(ATKPercentage, key, 0.25)` | `add_atk_percentage(key, 0.25)` |
| 2 | 用错边方向（其他角色） | `add_edge_s1to1(target, ATKBase, ATKPercentage)` | `add_edge_s1ton(target, ATKBase, ATKPercentage)` |
| 3 | 减抗不用 select_all | `select_self` / `select_team` | `select_all(attribute)` |
| 4 | 天赋编号含生活天赋 | 天赋1/2/3/4（含生活天赋） | 排除生活天赋后编号1/2/3 |
| 5 | set_value_by 叠加问题 | 同一 key 多次 `set_value_by` | 使用 `set_value_to` |
| 6 | 擢升技能类型错误 | `SkillType::ElementalSkill` | `SkillType::Elevative` |
| 7 | 使用历史方法 | `add_edge1`/`add_atk_percentage_base` | 使用新接口 |
| 8 | 使用历史 DamageBuilder 方法 | `add_extra_enhance_melt` | 不用 |
| 9 | Static/Last 优先级 | 在角色/武器/圣遗物中使用 | 禁止使用 |
| 10 | DamageEnum 与 SkillType 不匹配 | 倍率取 `e_dmg[s3]` | 取对应等级 `e_dmg[s2]` |

---

> **注意**：本清单基于 `mona_docs/src/` 目录下所有文档编写，覆盖了角色、武器、圣遗物、buff、target_function、Attribute 系统、DamageBuilder、Config 等全部模块。Review 时请根据实际变更范围选取相关检查项。
