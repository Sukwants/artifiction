use crate::target_functions::target_functions::prelude::*;
use crate::character::characters::Columbina;

pub struct ColumbinaOffFieldTargetFunction {
    pub lunar_charged_coefficient: f64,
    pub lunar_crystallize_coefficient: f64,
    pub hp_demand: f64,
}

impl TargetFunctionMetaTrait for ColumbinaOffFieldTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::ColumbinaOffField,
        name_locale: locale!(
            zh_cn: "哥伦比娅-后台输出",
            en: "Columbina-Off Field"
        ),
        description: locale!(
            zh_cn: "后台一轮输出总伤害乘以生命值上限满足需求比例",
            en: "Off Field Total Damage multiplied by HP Max to meet demand ratio"
        ),
        tags: "副C",
        four: TargetFunctionFor::SomeWho(CharacterName::Columbina),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "lunar_charged_coefficient",
            title: locale!(
                zh_cn: "月感电伤害系数",
                en: "Lunar Charged DMG Coefficient",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "lunar_crystallize_coefficient",
            title: locale!(
                zh_cn: "月结晶伤害系数",
                en: "Lunar Crystallize DMG Coefficient",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 }
        },
        ItemConfig {
            name: "hp_demand",
            title: locale!(
                zh_cn: "生命值上限需求",
                en: "HP Demand Ratio",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
        ItemConfig {
            name: "[aubade_of_morningstar_and_moon]set4_rate",
            title: locale!(
                zh_cn: "「晨星与月的晓歌」四件套被动比例",
                en: "[Aubade of Morningstar and Moon] 4-Set Ratio",
            ),
            config: ItemConfigType::GlobalLinkFloat { min: 0.0, max: 1.0, default: 1.0, 
                global_link: GlobalLinkConfig { key: "[aubade_of_morningstar_and_moon]set4_rate", priority: ItemConfig::PRIORITY_TARGETFUNCTION, team_shared: false } 
            }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        let (lunar_charged_coefficient, lunar_crystallize_coefficient, hp_demand) = match *config {
            TargetFunctionConfig::ColumbinaOffField { lunar_charged_coefficient, lunar_crystallize_coefficient, hp_demand } => (lunar_charged_coefficient, lunar_crystallize_coefficient, hp_demand),
            _ => (0.0, 0.0, 0.0)
        };
        Box::new(ColumbinaOffFieldTargetFunction {
            lunar_charged_coefficient,
            lunar_crystallize_coefficient,
            hp_demand,
        })
    }
}

impl TargetFunction for ColumbinaOffFieldTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            // .aubade_of_morningstar_and_moon(1.0)
            .build()
    }

    fn target(&self, attribute: &TargetFunctionAttributeType, character: &Character<TargetFunctionAttributeType>, weapon: &Weapon<TargetFunctionAttributeType>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let config1 = CharacterSkillConfig::Columbina { activated_q: true, activated_c4: false };
        let config2 = CharacterSkillConfig::Columbina { activated_q: true, activated_c4: true };
        
        let context1 = {
            let mut attribute_temp = (*attribute).clone();
            Columbina::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config1);
            DamageContext { character_common_data: &character.common_data, attribute: &attribute_temp.solve(), enemy }
        };
        let context2 = {
            let mut attribute_temp = (*attribute).clone();
            Columbina::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config2);
            DamageContext { character_common_data: &character.common_data, attribute: &attribute_temp.solve(), enemy }
        };

        let main_element = match &character.common_data.config {
            CharacterConfig::Columbina { moonsign, main_element, reacted_element } => *main_element,
            _ => None,
        };

        let demand_ratio = if context1.attribute.get_hp() < self.hp_demand {
            context1.attribute.get_hp() / self.hp_demand
        } else { 1.0 };

        type Ty = <Columbina as CharacterTrait>::DamageEnumType;

        let dmg_e1 = Columbina::damage::<SimpleDamageBuilder>(&context1, Ty::E, &config1, None).normal.expectation
            + Columbina::damage::<SimpleDamageBuilder>(&context1, Ty::EGC, &config1, None).normal.expectation * 9.0;
        let dmg_e2 = Columbina::damage::<SimpleDamageBuilder>(&context1, Ty::EGI, &config1, None).normal.expectation
            * if character.common_data.constellation >= 2 && character.common_data.constellation < 4 { 4.0 } else { 3.0 }
            + if character.common_data.constellation >= 4 {
                Columbina::damage::<SimpleDamageBuilder>(&context2, Ty::EGI, &config2, None).normal.expectation
            } else { 0.0 };
        let dmg_q = Columbina::damage::<SimpleDamageBuilder>(&context1, Ty::Q, &config1, None).normal.expectation;

        let dmg_lunar_charged = Columbina::moonglare_damage::<SimpleDamageBuilder>(&context1, MoonglareReaction::LunarChargedReaction).normal.expectation;
        let dmg_lunar_crystallize = Columbina::moonglare_damage::<SimpleDamageBuilder>(&context1, MoonglareReaction::LunarCrystallizeReaction).normal.expectation;

        (dmg_e1 + dmg_e2 * if main_element == Some(Element::Dendro) { 5.0 } else { 1.0 }
            + dmg_q + dmg_lunar_charged * self.lunar_charged_coefficient + dmg_lunar_crystallize * self.lunar_crystallize_coefficient) * demand_ratio
    }
}

pub struct ColumbinaOnFieldTargetFunction {
}

impl TargetFunctionMetaTrait for ColumbinaOnFieldTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::ColumbinaOnField,
        name_locale: locale!(
            zh_cn: "哥伦比娅-前台输出",
            en: "Columbina-On Field"
        ),
        description: locale!(
            zh_cn: "前台一轮输出总伤害乘以生命值上限满足需求比例",
            en: "On Field Total Damage"
        ),
        tags: "主C",
        four: TargetFunctionFor::SomeWho(CharacterName::Columbina),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "[aubade_of_morningstar_and_moon]set4_rate",
            title: locale!(
                zh_cn: "「晨星与月的晓歌」四件套被动比例",
                en: "[Aubade of Morningstar and Moon] 4-Set Ratio",
            ),
            config: ItemConfigType::GlobalLinkFloat { min: 0.0, max: 1.0, default: 1.0, 
                global_link: GlobalLinkConfig { key: "[aubade_of_morningstar_and_moon]set4_rate", priority: ItemConfig::PRIORITY_TARGETFUNCTION, team_shared: false } 
            }
        },
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(ColumbinaOnFieldTargetFunction {
        })
    }
}

impl TargetFunction for ColumbinaOnFieldTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        unimplemented!()
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            // .aubade_of_morningstar_and_moon(1.0)
            .build()
    }

    fn target(&self, attribute: &TargetFunctionAttributeType, character: &Character<TargetFunctionAttributeType>, weapon: &Weapon<TargetFunctionAttributeType>, artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let config1 = CharacterSkillConfig::Columbina { activated_q: true, activated_c4: false };
        let config2 = CharacterSkillConfig::Columbina { activated_q: true, activated_c4: true };
        
        let context1 = {
            let mut attribute_temp = (*attribute).clone();
            Columbina::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config1);
            DamageContext { character_common_data: &character.common_data, attribute: &attribute_temp.solve(), enemy }
        };
        let context2 = {
            let mut attribute_temp = (*attribute).clone();
            Columbina::change_attribute::<TargetFunctionAttributeType>(&mut attribute_temp, &character.common_data, &config2);
            DamageContext { character_common_data: &character.common_data, attribute: &attribute_temp.solve(), enemy }
        };

        let main_element = match &character.common_data.config {
            CharacterConfig::Columbina { moonsign, main_element, reacted_element } => *main_element,
            _ => None,
        };

        type Ty = <Columbina as CharacterTrait>::DamageEnumType;

        let dmg_e1 = Columbina::damage::<SimpleDamageBuilder>(&context1, Ty::E, &config1, None).normal.expectation
            + Columbina::damage::<SimpleDamageBuilder>(&context1, Ty::EGC, &config1, None).normal.expectation * 9.0;
        let dmg_e2 = Columbina::damage::<SimpleDamageBuilder>(&context1, Ty::EGI, &config1, None).normal.expectation
            * if character.common_data.constellation >= 2 && character.common_data.constellation < 4 { 4.0 } else { 3.0 }
            + if character.common_data.constellation >= 4 {
                Columbina::damage::<SimpleDamageBuilder>(&context2, Ty::EGI, &config2, None).normal.expectation
            } else { 0.0 };
        let dmg_q = Columbina::damage::<SimpleDamageBuilder>(&context1, Ty::Q, &config1, None).normal.expectation;

        let dmg_z = Columbina::damage::<SimpleDamageBuilder>(&context1, Ty::ZM, &config1, None).normal.expectation * 6.0;

        dmg_e1 + dmg_e2 * if main_element == Some(Element::Dendro) { 5.0 } else { 1.0 } + dmg_q + dmg_z
    }
}
