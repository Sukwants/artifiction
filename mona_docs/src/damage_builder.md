## DamageBuilder

本文件包含 DamageBuilder 使用说明。

在使用前，首先需要通过 `new` 函数创建一个 `DamageBuilder` 实例，假设我们创建了一个名为 `builder` 的 `DamageBuilder` 实例。

### 添加效果

此后将需要单独在 `builder` 中处理的效果（如针对特定技能的效果）通过 `DamageBuilder` 中的方法添加到 `builder` 中。调用这些方法时均需要传递 `key` 和 `value`，其中 `key` 标识当前属性来源，包含发起角色与技能，如 `杜林天赋2`，`value` 是效果的数值。可调用的方法包括：
- `add_extra_em`：添加额外元素精通。
- `add_extra_atk`：添加额外攻击力。
- `add_extra_def`：添加额外防御力。
- `add_extra_hp`：添加额外生命值上限。
- `add_extra_damage`：添加额外基础伤害，对应 `AttributeVariableType::BaseDamage`。如有任何基于非当前角色攻击力、防御力、生命值上限或元素精通的伤害，请将属性乘以倍率后通过该方法添加到 `builder` 中。
- `add_extra_critical`：添加额外暴击率，对应 `AttributeVariableType::CriticalRate`。
- `add_extra_critical_damage`：添加额外暴击伤害，对应 `AttributeVariableType::CriticalDamage`。
- `add_extra_bonus`：添加额外伤害加成，对应 `AttributeVariableType::Bonus`。
- `add_extra_reaction_enhance`：添加额外反应系数提升，对应 `AttributeVariableType::ReactionEnhance`。
- `add_extra_reaction_extra`：添加额外元素反应额外提升，对应 `AttributeVariableType::ReactionExtra`。
- `add_extra_enhance_melt`：历史方法，不应使用。
- `add_extra_enhance_vaporize`：历史方法，不应使用。
- `add_extra_def_minus`：添加额外防御力降低，对应 `AttributeVariableType::DefMinus`。
- `add_extra_def_penetration`：添加额外防御力穿透，对应 `AttributeVariableType::DefPenetration`。
- `add_extra_res_minus`：添加额外抗性降低，对应 `AttributeVariableType::ResMinus`。

以下为一个示例：

> 施放元素战技天地忽然身时，或队伍中附近的角色触发月笼谐奏时，兹白将获得持续4秒的「太阴降」效果：灵驹飞踏第二段攻击造成的伤害提升，提升值相当于兹白防御力的60%。  

> 施放元素战技天地忽然身后，兹白会立即积攒100点「时隙浮光」，且「月转时隙」模式下灵驹飞踏的最大可用次数增加至5次。  
> 此外，每次切换至「月转时隙」模式后，初次施放灵驹飞踏时，第二段攻击造成的月结晶反应伤害提升220%。  
```rust
if s == E2 {
    if context.character_common_data.constellation >= 1 && activated_c1 {
        builder.add_extra_reaction_enhance("兹白命座1", 2.20);
    }

    if context.character_common_data.has_talent1 {
        if context.character_common_data.constellation >= 2 && moonsign.is_ascendant() {
            builder.add_extra_reaction_extra("兹白天赋1", context.attribute.get_def() * 6.10);
        } else {
            builder.add_extra_reaction_extra("兹白天赋1", context.attribute.get_def() * 0.60);
        }
    }

}
```

### 添加倍率

此后，通过调用以下方法添加技能倍率：
- `add_atk_ratio`：添加攻击力倍率。
- `add_def_ratio`：添加防御力倍率。
- `add_hp_ratio`：添加生命值上限倍率。
- `add_em_ratio`：添加元素精通倍率。
- `add_base`：添加基础伤害，仅用于添加治疗量或护盾量的常数值。

### 结果计算

最后，通过调用以下方法计算最终伤害结果：

```rust
mona::damage::damage_builder::DamageBuilder
pub trait DamageBuilder
pub fn damage(
    &self,
    attribute: &Self::AttributeType,
    enemy: &Enemy,
    element: Element,
    skill_type: SkillType,
    character_level: usize,
    fumo: Option<Element>
) -> Self::Result

pub fn transformative(
    &self,
    attribute: &Self::AttributeType,
    enemy: &Enemy,
    transformative_type: TransformativeType,
    character_level: usize
) -> Self::Result

pub fn moonglare(
    &self,
    attribute: &Self::AttributeType,
    enemy: &Enemy,
    element: Element,
    lunar_type: MoonglareReaction,
    skill_type: SkillType,
    character_level: usize,
    fumo: Option<Element>
) -> Self::Result

pub fn heal(&self, attribute: &Self::AttributeType) -> Self::Result

pub fn shield(&self, attribute: &Self::AttributeType, element: Element) -> Self::Result

pub fn number(&self, attribute: &Self::AttributeType) -> Self::Result

pub fn none(&self) -> Self::Result
```

- `damage` 方法用于计算普通伤害，需要传入 `attribute` 以获取属性，`enemy` 以传入敌人信息，`element` 和 `skill_type` 为伤害元素类型和技能类型，`character_level` 为角色等级，`fumo` 为可选的附魔元素类型。
- `transformative` 方法用于计算聚变反应（扩散、超载、感电、碎冰、超导、绽放、烈绽放、超绽放、燃烧、结晶）伤害，需要额外传入 `transformative_type` 以标识反应类型。
- `moonglare` 方法用于计算月曜反应（月感电、月绽放、月结晶）伤害，需要额外传入 `lunar_type` 以标识月曜反应类型。
- `heal` 方法用于计算治疗量，仅需传入 `attribute`。
- `shield` 方法用于计算护盾量，需要传入 `attribute` 和 `element` 以获取属性和护盾元素类型。
- `number` 方法用于纯数值结果，不受任何属性加成，如“基础伤害提升数值”。
- `none` 方法用于无任何结果的情况，用于该伤害、治疗、护盾在当前配置下不会触发的情况。
