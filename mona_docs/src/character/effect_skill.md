## EffectSkill

本文件包含角色文件中 EffectSkill 部分书写说明。

EffectSkill 部分包含角色技能效果配置声明与角色技能效果接口实现，以及 `CharacterTrait` 中的 `CONFIG_SKILL`。

一个 EffectSkill 部分示例如下：

```rust
impl CharacterTrait for Durin {
    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "activated_res",
            title: locale!(
                zh_cn: "白焰之龙减抗",
                en: "Dragon of White Flame RES Reduction"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "primordial_fusion",
            title: locale!(
                zh_cn: "存在「肇象」",
                en: "Has primordial Fusion"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "cycle_of_enlightenment",
            title: locale!(
                zh_cn: "存在「轮变启迪」",
                en: "Has Cycle of Enlightenment"
            ),
            config: ItemConfigType::Bool { default: true }
        },
        ItemConfig {
            name: "activated_reaction",
            title: locale!(
                zh_cn: "触发火元素相关反应",
                en: "Activated Pyro Element Reactions"
            ),
            config: ItemConfigType::Bool { default: true }
        },
    ]);

    fn change_attribute<A: Attribute>(attribute: &mut A, common_data: &CharacterCommonData, skill_config: &CharacterSkillConfig) {

        let (hexerei_secret_rite, essential_transmutation, elements) = match &common_data.config {
            CharacterConfig::Durin { hexerei_secret_rite, essential_transmutation, elements } => (*hexerei_secret_rite, *essential_transmutation, *elements),
            _ => (false, 0, ConfigElements8Multi::default()),
        };

        let (activated_res, primordial_fusion, cycle_of_enlightenment, activated_reaction) = match *skill_config {
            CharacterSkillConfig::Durin { activated_res, primordial_fusion, cycle_of_enlightenment, activated_reaction } => (activated_res, primordial_fusion, cycle_of_enlightenment, activated_reaction),
            _ => (false, false, false, false)
        };

        if common_data.constellation >= 1 && cycle_of_enlightenment {
            if essential_transmutation == 0 {
                for skill in [SkillType::NormalAttack, SkillType::ChargedAttack, SkillType::PlungingAttackInAction, SkillType::PlungingAttackOnGround, SkillType::ElementalSkill, SkillType::ElementalBurst].into_iter() {
                    attribute.add_edge_s1to1(
                        CharacterSelector::select_all_except_self(attribute),
                        AttributeType::Panel(AttributeName::ATK),
                        AttributeType::Invisible(InvisibleAttributeType::new_skill(AttributeVariableType::BaseDamage, skill)),
                        Arc::new(move |atk: f64, _ | atk * 0.6 ),
                        "杜林命座1",
                        EdgePriority::Invisible,
                    );
                }
            } else {
                attribute.add_edge_t1(
                    AttributeType::Panel(AttributeName::ATK),
                    AttributeType::Invisible(InvisibleAttributeType::new_skill(AttributeVariableType::BaseDamage, SkillType::ElementalBurst)),
                    Arc::new(move |atk: f64, _ | atk * 1.5 ),
                    "杜林命座1",
                    EdgePriority::Invisible,
                );
            }
        }

        if common_data.constellation >= 2 {
            if elements.pyro { attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusPyro), "杜林天赋2", 0.5); }
            if elements.hydro { attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusHydro), "杜林天赋2", 0.5); }
            if elements.anemo { attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusAnemo), "杜林天赋2", 0.5); }
            if elements.electro { attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusElectro), "杜林天赋2", 0.5); }
            if elements.dendro { attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusDendro), "杜林天赋2", 0.5); }
            if elements.cryo { attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusCryo), "杜林天赋2", 0.5); }
            if elements.geo { attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusGeo), "杜林天赋2", 0.5); }
        }

        if common_data.constellation >= 6 {
            if essential_transmutation == 0 {
                attribute.set_value_by_s(
                    CharacterSelector::select_all(attribute),
                    AttributeType::Invisible(InvisibleAttributeType::new_any(AttributeVariableType::DefMinus)),
                    "杜林命座6",
                    0.3
                );
            } else {
                attribute.set_value_by_t(
                    AttributeType::Invisible(InvisibleAttributeType::new_skill(AttributeVariableType::DefPenetration, SkillType::ElementalBurst)),
                    "杜林命座6",
                    0.4
                );
            }
        }
        
    }
}
```

角色技能配置与角色配置的区别在于，角色技能配置应为计算过程中需要由 target_function 决定的配置项（如角色当前形态、某个效果是否触发），并需要写入到 `CharacterSkillConfig` 中（位于 `mona_core/src/character/skill_config.rs`）。

`change_attribute` 接口实现部分包含了角色技能配置对属性的修改逻辑，应包含所有不由具体技能决定的属性修改逻辑，通过直接解析 `skill_config` 获取配置数据。接口调用方法参见 `mona_docs/src/attribute.md`。

`CONFIG_SKILL` 包含了需要用户给出的配置项，每一个配置项应包含以下字段：
- `name`：配置项的绑定名称，必须与 CharacterSkillConfig 中对应字段的名称一致。
- `title`：配置项的显示名称，需要中英文两种语言。
- `config`：配置项的类型定义，包含了配置项的类型、可选项、默认值等信息。可以使用的配置类型详见 `mona_docs/src/config.md`。
