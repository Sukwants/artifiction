## Damage

本文件包含角色文件中 Damage 部分书写说明。

Damage 部分包含声明 `builder`，将与具体技能相关的配置应用进 `builder`，并调用 `builder` 中的方法完成计算。

一个 Damage 部分示例如下：

```rust
impl CharacterTrait for Durin {
    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: DurinDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (hexerei_secret_rite, essential_transmutation, elements) = match &context.character_common_data.config {
            CharacterConfig::Durin { hexerei_secret_rite, essential_transmutation, elements } => (*hexerei_secret_rite, *essential_transmutation, *elements),
            _ => (false, 0, ConfigElements8Multi::default()),
        };

        let (activated_res, primordial_fusion, cycle_of_enlightenment, activated_reaction) = match *config {
            CharacterSkillConfig::Durin { activated_res, primordial_fusion, cycle_of_enlightenment, activated_reaction } => (activated_res, primordial_fusion, cycle_of_enlightenment, activated_reaction),
            _ => (false, false, false, false)
        };

        use DurinDamageEnum::*;
        let mut builder = D::new();

        let ratio = match s {
            A1 => DURIN_SKILL.a_dmg1[s1],
            A2 => DURIN_SKILL.a_dmg2[s1],
            A31 => DURIN_SKILL.a_dmg31[s1],
            A32 => DURIN_SKILL.a_dmg32[s1],
            A4 => DURIN_SKILL.a_dmg4[s1],
            Z => DURIN_SKILL.z_dmg[s1],
            X1 => DURIN_SKILL.x_dmg1[s1],
            X2 => DURIN_SKILL.x_dmg2[s1],
            X3 => DURIN_SKILL.x_dmg3[s1],
            EP => DURIN_SKILL.e_dmgp[s2],
            ED1 => DURIN_SKILL.e_dmgd1[s2],
            ED2 => DURIN_SKILL.e_dmgd2[s2],
            ED3 => DURIN_SKILL.e_dmgd3[s2],
            QP1 => DURIN_SKILL.q_dmgp1[s3],
            QP2 => DURIN_SKILL.q_dmgp2[s3],
            QP3 => DURIN_SKILL.q_dmgp3[s3],
            QP => DURIN_SKILL.q_dmgp[s3],
            QD1 => DURIN_SKILL.q_dmgd1[s3],
            QD2 => DURIN_SKILL.q_dmgd2[s3],
            QD3 => DURIN_SKILL.q_dmgd3[s3],
            QD => DURIN_SKILL.q_dmgd[s3],
        };

        let extra_ratio = if context.character_common_data.has_talent2 && primordial_fusion {
            (context.attribute.get_atk() / 100.0 * 0.03).min(0.75)
        } else { 0.0 } * ratio;

        builder.add_atk_ratio("技能倍率", ratio);
        if extra_ratio > 0.0 {
            builder.add_atk_ratio("天赋2：混沌如黑夜构成", extra_ratio);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }
}
```

此处可以通过解析 `context.character_common_data.config` 获取角色配置，解析 `config` 获取角色技能配置。

首先需要处理角色配置与角色技能配置，然后将与具体技能相关的配置（如“灵驹飞踏第二段攻击造成的伤害提升”）应用进 `builder`，再根据具体技能获得技能倍率，最后调用 `builder` 中的方法完成计算。

- 对于普通伤害，需要调用 `builder.damage` 方法。
- 对于擢升反应伤害（包含月曜反应伤害、星超导反应伤害、星扩散反应伤害），需要调用 `builder.elevative` 方法。
- 对于治疗量，需要调用 `builder.heal` 方法。
- 对于护盾量，需要调用 `builder.shield` 方法。
- 对于纯数值（如“基础伤害提升数值”），需要调用 `builder.number` 方法。
- 对于当前配置下不会触发的伤害、治疗或护盾，调用 `builder.none` 方法。

原则上角色伤害计算部分不应出现 `builder.transformative` 方法。

### 条件伤害处理

某些伤害仅在特定条件下才能触发（如需要元素转化存在、需要解锁特定天赋或命座），应在计算倍率之前检查条件，不满足时返回 `builder.none()`：

```rust
// 需要元素转化才能触发的伤害
if (s == P1 || s == C4E) && elemental_absorption == None {
    return builder.none();
}

// 需要解锁天赋才能触发的伤害
if s == P1 && !context.character_common_data.has_talent1 {
    return builder.none();
}

// 需要特定命座才能触发的伤害
if s == C4E && context.character_common_data.constellation < 4 {
    return builder.none();
}
```

注意检查顺序：先检查元素转化等数据条件，再检查天赋和命座条件。