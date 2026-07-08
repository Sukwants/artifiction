## Effect

本文件包含角色文件中 Effect 部分书写说明。

Effect 部分包含角色效果配置声明与角色效果接口实现，以及 `CharacterTrait` 中的 `CONFIG_DATA`。

一个 Effect 部分示例如下：

```rust
pub struct DurinEffect {
    pub hexerei_secret_rite: bool,
    pub essential_transmutation: usize,
    pub elements: ConfigElements8Multi,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for DurinEffect {
    fn change_attribute(&self, attribute: &mut A) {

        if self.common_data.has_talent1 {
            if self.essential_transmutation == 0 {
                
                let ratio = if self.hexerei_secret_rite { 0.35 } else { 0.20 };

                if self.elements.pyro {
                    attribute.set_value_by_s(CharacterSelector::select_all(attribute),
                        AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::ResMinus, Element::Pyro)),
                        "杜林天赋1", ratio,
                    );
                }
                if self.elements.anemo {
                    attribute.set_value_by_s(CharacterSelector::select_all(attribute),
                        AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::ResMinus, Element::Anemo)),
                        "杜林天赋1", ratio,
                    );
                }
                if self.elements.electro {
                    attribute.set_value_by_s(CharacterSelector::select_all(attribute),
                        AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::ResMinus, Element::Electro)),
                        "杜林天赋1", ratio,
                    );
                }
                if self.elements.dendro {
                    attribute.set_value_by_s(CharacterSelector::select_all(attribute),
                        AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::ResMinus, Element::Dendro)),
                        "杜林天赋1", ratio,
                    );
                }
                if self.elements.geo {
                    attribute.set_value_by_s(CharacterSelector::select_all(attribute),
                        AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::ResMinus, Element::Geo)),
                        "杜林天赋1", ratio,
                    );
                }
            } else {
                attribute.set_value_by_t(
                    AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, ReactionType::Vaporize)),
                    "杜林天赋1", if self.hexerei_secret_rite { 0.7 } else { 0.4 }
                );
                attribute.set_value_by_t(
                    AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, ReactionType::Melt)),
                    "杜林天赋1", if self.hexerei_secret_rite { 0.7 } else { 0.4 }
                );
            }
        }

        if self.common_data.constellation >= 4 {
            attribute.set_value_by_t(
                AttributeType::Invisible(InvisibleAttributeType::new_skill(AttributeVariableType::Bonus, SkillType::ElementalBurst)),
                "杜林命座4",
                0.4
            );
        }

        if self.common_data.constellation >= 6 {
            attribute.set_value_by_t(
                AttributeType::Invisible(InvisibleAttributeType::new_skill(AttributeVariableType::DefPenetration, SkillType::ElementalBurst)),
                "杜林命座6",
                0.3
            );
        }
    }
}

impl CharacterTrait for Durin {

    // ...
    
    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig {
            name: "essential_transmutation",
            title: locale!(
                zh_cn: "精质转变",
                en: "Essential Transmutation"
            ),
            config: ItemConfigType::Option2 { options_zh: "白化之是,黑度之否", options_en: "Confirmation of Purity,Denial of Darkness", default: 0 }
        },
        ItemConfig {
            name: "elements",
            title: locale!(
                zh_cn: "反应元素",
                en: "Reaction Elements"
            ),
            config: ItemConfigType::ElementMulti {
                elements: &[Element::Pyro, Element::Hydro, Element::Anemo, Element::Electro, Element::Dendro, Element::Cryo, Element::Geo],
                default: ConfigElements8Multi {
                    pyro: true,
                    hydro: false,
                    anemo: false,
                    electro: false,
                    dendro: false,
                    cryo: false,
                    geo: false,
                    physical: false,
                }
            }
        },
    ]);

    // ...

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (hexerei_secret_rite, essential_transmutation, elements) = match *config {
            CharacterConfig::Durin { hexerei_secret_rite, essential_transmutation, elements } => (hexerei_secret_rite, essential_transmutation, elements),
            _ => (false, 0, ConfigElements8Multi::default()),
        };
        Some(Box::new(DurinEffect {
            hexerei_secret_rite: hexerei_secret_rite,
            essential_transmutation: essential_transmutation,
            elements,
            common_data: common_data.clone(),
        }))
    }

    // ...
}
```

其中，`Effect` 的声明部分包含了部分角色配置所需的配置项，应包含所有不由 target_function 决定的配置项（如当前队伍月兆等级、当前队伍中角色元素类型等），原则上必须包含一个 `CharacterCommonData` 类型的字段 `common_data`，以及其他根据角色配置需要的字段。

`change_attribute` 接口实现部分包含了角色配置对属性的修改逻辑，应包含所有不由 target_function 决定所影响的属性修改逻辑。接口调用方法参见 `mona_docs/src/attribute.md`。

`CONFIG_DATA` 包含了需要用户给出的配置项，每一个配置项应包含以下字段：
- `name`：配置项的绑定名称，必须与 Effect 结构体中对应字段的名称一致。
- `title`：配置项的显示名称，需要中英文两种语言。
- `config`：配置项的类型定义，包含了配置项的类型、可选项、默认值等信息。可以使用的配置类型详见 `mona_docs/src/config.md`。

`new_effect` 函数用于将用户输入的配置项转换为 Effect 结构体，每一个配置项都应添加到 `mona::character::character_config::CharacterConfig` 中，并在 `new_effect` 函数中进行匹配解析。

请额外注意，天赋 1 和天赋 2 需要通过 `has_talent1` 和 `has_talent2` 来判断是否生效，天赋 3 或生活天赋则总是生效，不需要判断，命座需要通过 `common_data.constellation` 来判断是否生效。
