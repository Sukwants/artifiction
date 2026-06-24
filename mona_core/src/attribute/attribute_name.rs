use crate::{common::{Element, ElevativeReaction, ReactionType, SkillType, TransformativeType}, damage::transformative_damage};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum AttributeName {
    // 自定义数据，应当只用在角色的特定的Effect中，否则容易使用不当，产生冲突
    USER1,
    USER2,

    NULL,

    HealingBonus,
    IncomingHealingBonus,
    ElementalMastery,
    // 不参与精通转换的计算，例如草神天赋不被船桨计算
    ElementalMasteryExtra,
    Recharge,
    RechargeExtra,
    ShieldStrength,

    DefMinus,
    DefPenetration,
    ResMinusBase,
    ResMinusElectro,
    ResMinusPyro,
    ResMinusHydro,
    ResMinusCryo,
    ResMinusGeo,
    ResMinusAnemo,
    ResMinusDendro,
    ResMinusPhysical,

    SpeedNormalAttack,
    SpeedChargedAttack,

    HPBase,
    HPFixed,
    HPPercentage,  // 并非防御力百分比，如需添加百分比攻击请使用 add_edge: DEFBase -> DEFPercentage 或 add_atk_percentage
    HP,

    ATKBase,
    ATKFixed,
    ATKPercentage, // 并非攻击力百分比，如需添加百分比攻击请使用 add_edge: ATKBase -> ATKPercentage 或 add_atk_percentage
    ATK,

    DEFBase,
    DEFFixed,
    DEFPercentage,  // 并非生命值上限百分比，如需添加百分比攻击请使用 add_edge: HPBase -> HPPercentage 或 add_atk_percentage
    DEF,

    // not character attributes, but needed
    ATKBonusForOther,
    HealBonusForOther,

    CriticalBase,
    CriticalAttacking,          // critical when attack enemy, but not counted in real panel
    CriticalNormalAttack,
    CriticalChargedAttack,
    CriticalPlungingAttack,
    CriticalElementalSkill,
    CriticalElementalBurst,
    CriticalElectro,
    CriticalPyro,
    CriticalHydro,
    CriticalCryo,
    CriticalAnemo,
    CriticalGeo,
    CriticalDendro,
    CriticalPhysical,

    CriticalDamageBase,
    CriticalDamageNormalAttack,
    CriticalDamageChargedAttack,
    CriticalDamagePlungingAttack,
    CriticalDamageElementalSkill,
    CriticalDamageElementalBurst,
    CriticalDamageElectro,
    CriticalDamagePyro,
    CriticalDamageHydro,
    CriticalDamageCryo,
    CriticalDamageAnemo,
    CriticalDamageGeo,
    CriticalDamageDendro,
    CriticalDamagePhysical,

    BonusBase,
    BonusNormalAttack,
    BonusChargedAttack,
    BonusPlungingAttack,
    BonusElementalSkill,
    BonusElementalBurst,
    BonusElectro,
    BonusPyro,
    BonusHydro,
    BonusCryo,
    BonusAnemo,
    BonusGeo,
    BonusDendro,
    BonusPhysical,
    BonusNormalAndElemental, // 普通攻击&元素伤害 todo 以后应该重构掉

    EnhanceBurgeon,
    EnhanceHyperbloom,
    EnhanceBloom,
    EnhanceOverload,
    EnhanceBurning,
    EnhanceShatter,
    EnhanceElectroCharged,
    EnhanceSuperconduct,
    EnhanceSwirlElectro,
    EnhanceSwirlPyro,
    EnhanceSwirlHydro,
    EnhanceSwirlCryo,
    EnhanceSwirlBase,
    EnhanceVaporize,
    EnhanceMelt,
    EnhanceAggravate,
    EnhanceSpread,
    EnhanceElevative,
    EnhanceLunarCharged,
    EnhanceLunarBloom,

    IncreaseLunarCharged, // 天赋「月兆祝赐」给出的月曜反应基础提升
    IncreaseLunarBloom,

    ElevateLunarCharged, // 月曜反应擢升
    ElevateLunarBloom,

    ExtraIncreaseBurgeon, // 部分角色天赋给出的额外提升，不受益于精通和反应增伤
    ExtraIncreaseHyperBloom,
    ExtraIncreaseBloom,
    ExtraIncreaseLunarCharged, // 月曜反应额外提升，（大概）应由 add_edge 给出
    ExtraIncreaseLunarBloom,

    CriticalElevative,
    CriticalLunarCharged,
    CriticalLunarBloom,

    CriticalDamageElevative,
    CriticalDamageLunarCharged,
    CriticalDamageLunarBloom,

    HPRatioBase,
    HPRatioNormalAttack,
    HPRatioChargedAttack,
    HPRatioPlungingAttack,
    HPRatioElementalSkill,
    HPRatioElementalBurst,
    HPRatioElectro,
    HPRatioPyro,
    HPRatioHydro,
    HPRatioCryo,
    HPRatioAnemo,
    HPRatioGeo,
    HPRatioDendro,
    HPRatioPhysical,

    DEFRatioBase,
    DEFRatioNormalAttack,
    DEFRatioChargedAttack,
    DEFRatioPlungingAttack,
    DEFRatioElementalSkill,
    DEFRatioElementalBurst,
    DEFRatioElectro,
    DEFRatioPyro,
    DEFRatioHydro,
    DEFRatioCryo,
    DEFRatioAnemo,
    DEFRatioGeo,
    DEFRatioDendro,
    DEFRatioPhysical,

    ATKRatioBase,
    ATKRatioNormalAttack,
    ATKRatioChargedAttack,
    ATKRatioPlungingAttack,
    ATKRatioElementalSkill,
    ATKRatioElementalBurst,
    ATKRatioElectro,
    ATKRatioPyro,
    ATKRatioHydro,
    ATKRatioCryo,
    ATKRatioAnemo,
    ATKRatioGeo,
    ATKRatioDendro,
    ATKRatioPhysical,

    ExtraDmgBase,
    ExtraDmgNormalAttack,
    ExtraDmgChargedAttack,
    ExtraDmgPlungingAttack,
    ExtraDmgPlungingAttackLowHigh, // 坠地冲击额外伤害，由于闲云而首次引进
    ExtraDmgElementalSkill,
    ExtraDmgElementalBurst,
    ExtraDmgElectro,
    ExtraDmgPyro,
    ExtraDmgHydro,
    ExtraDmgCryo,
    ExtraDmgAnemo,
    ExtraDmgGeo,
    ExtraDmgDendro,
    ExtraDmgPhysical,

    // introduced because of YumemizukiMizuki C1
    SwirlExtraDmg,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
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

    ElevativeBase, // 擢升反应基础提升
    ElevativeElevate, // 擢升反应擢升

    HealingBonus, // 治疗加成
    IncomingHealingBonus, // 受治疗加成
    HealingCriticalRate, // 治疗暴击率
    HealingCriticalDamage, // 治疗暴击伤害

    ShieldStrength, // 护盾强效
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub struct InvisibleAttributeType {
    pub attribute_variable_type: AttributeVariableType,
    pub element: Option<Element>,
    pub skill: Option<SkillType>,
    pub reaction: Option<ReactionType>,
}

impl InvisibleAttributeType {
    pub fn new(
        attribute_variable_type: AttributeVariableType,
        element: Option<Element>,
        skill: Option<SkillType>,
        reaction: Option<ReactionType>,
    ) -> Self {
        InvisibleAttributeType {
            attribute_variable_type,
            element,
            skill,
            reaction,
        }
    }

    pub fn new_any(attribute_variable_type: AttributeVariableType) -> Self {
        InvisibleAttributeType {
            attribute_variable_type,
            element: None,
            skill: None,
            reaction: None,
        }
    }

    pub fn new_element(attribute_variable_type: AttributeVariableType, element: Element) -> Self {
        InvisibleAttributeType {
            attribute_variable_type,
            element: Some(element),
            skill: None,
            reaction: None,
        }
    }

    pub fn new_skill(attribute_variable_type: AttributeVariableType, skill: SkillType) -> Self {
        InvisibleAttributeType {
            attribute_variable_type,
            element: None,
            skill: Some(skill),
            reaction: None,
        }
    }

    pub fn new_reaction(attribute_variable_type: AttributeVariableType, reaction: ReactionType) -> Self {
        InvisibleAttributeType {
            attribute_variable_type,
            element: None,
            skill: None,
            reaction: Some(reaction),
        }
    }
}

impl AttributeName {
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
    
    pub fn bonus_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::BonusElectro,
            Element::Hydro => AttributeName::BonusHydro,
            Element::Anemo => AttributeName::BonusAnemo,
            Element::Pyro => AttributeName::BonusPyro,
            Element::Cryo => AttributeName::BonusCryo,
            Element::Dendro => AttributeName::BonusDendro,
            Element::Geo => AttributeName::BonusGeo,
            Element::Physical => AttributeName::BonusPhysical,
        }
    }

    pub fn bonus_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::BonusNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::BonusChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::BonusPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::BonusElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::BonusElementalBurst),
            _ => None,
        }
    }

    pub fn enhance_name_by_elevative_reaction(lunar_type: ElevativeReaction) -> AttributeName {
        match lunar_type {
            ElevativeReaction::LunarChargedReaction | ElevativeReaction::LunarCharged => AttributeName::EnhanceLunarCharged,
            ElevativeReaction::LunarBloom => AttributeName::EnhanceLunarBloom,
            _ => AttributeName::NULL,
        }
    }

    pub fn increase_name_by_elevative_reaction(lunar_type: ElevativeReaction) -> AttributeName {
        match lunar_type {
            ElevativeReaction::LunarChargedReaction | ElevativeReaction::LunarCharged => AttributeName::IncreaseLunarCharged,
            ElevativeReaction::LunarBloom => AttributeName::IncreaseLunarBloom,
            _ => AttributeName::NULL,
        }
    }

    pub fn elevate_name_by_elevative_reaction(lunar_type: ElevativeReaction) -> AttributeName {
        match lunar_type {
            ElevativeReaction::LunarChargedReaction | ElevativeReaction::LunarCharged => AttributeName::ElevateLunarCharged,
            ElevativeReaction::LunarBloom => AttributeName::ElevateLunarBloom,
            _ => AttributeName::NULL,
        }
    }

    pub fn extra_increase_name_by_reaction(reaction_type: ReactionType) -> Option<AttributeName> {
        match reaction_type {
            ReactionType::Burgeon => Some(AttributeName::ExtraIncreaseBurgeon),
            ReactionType::Hyperbloom => Some(AttributeName::ExtraIncreaseHyperBloom),
            ReactionType::Bloom => Some(AttributeName::ExtraIncreaseBloom),
            ReactionType::LunarCharged => Some(AttributeName::ExtraIncreaseLunarCharged),
            ReactionType::LunarBloom => Some(AttributeName::ExtraIncreaseLunarBloom),
            _ => None,
        }
    }

    pub fn critical_rate_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::CriticalElectro,
            Element::Hydro => AttributeName::CriticalHydro,
            Element::Anemo => AttributeName::CriticalAnemo,
            Element::Pyro => AttributeName::CriticalPyro,
            Element::Cryo => AttributeName::CriticalCryo,
            Element::Dendro => AttributeName::CriticalDendro,
            Element::Geo => AttributeName::CriticalGeo,
            Element::Physical => AttributeName::CriticalPhysical,
        }
    }

    pub fn critical_rate_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::CriticalNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::CriticalChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::CriticalPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::CriticalElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::CriticalElementalBurst),
            _ => None,
        }
    }

    pub fn critical_rate_name_by_elevative_reaction(lunar_type: ElevativeReaction) -> AttributeName {
        match lunar_type {
            ElevativeReaction::LunarChargedReaction | ElevativeReaction::LunarCharged => AttributeName::CriticalLunarCharged,
            ElevativeReaction::LunarBloom => AttributeName::CriticalLunarBloom,
            _ => AttributeName::NULL,
        }
    }

    pub fn critical_damage_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::CriticalDamageElectro,
            Element::Hydro => AttributeName::CriticalDamageHydro,
            Element::Anemo => AttributeName::CriticalDamageAnemo,
            Element::Pyro => AttributeName::CriticalDamagePyro,
            Element::Cryo => AttributeName::CriticalDamageCryo,
            Element::Dendro => AttributeName::CriticalDamageDendro,
            Element::Geo => AttributeName::CriticalDamageGeo,
            Element::Physical => AttributeName::CriticalDamagePhysical,
        }
    }

    pub fn critical_damage_name_by_skill_name(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::CriticalDamageNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::CriticalDamageChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::CriticalDamagePlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::CriticalDamageElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::CriticalDamageElementalBurst),
            _ => None,
        }
    }

    pub fn critical_damage_name_by_elevative_reaction(lunar_type: ElevativeReaction) -> AttributeName {
        match lunar_type {
            ElevativeReaction::LunarChargedReaction | ElevativeReaction::LunarCharged => AttributeName::CriticalDamageLunarCharged,
            ElevativeReaction::LunarBloom => AttributeName::CriticalDamageLunarBloom,
            _ => AttributeName::NULL,
        }
    }

    pub fn hp_ratio_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::HPRatioElectro,
            Element::Hydro => AttributeName::HPRatioHydro,
            Element::Anemo => AttributeName::HPRatioAnemo,
            Element::Pyro => AttributeName::HPRatioPyro,
            Element::Cryo => AttributeName::HPRatioCryo,
            Element::Dendro => AttributeName::HPRatioDendro,
            Element::Geo => AttributeName::HPRatioGeo,
            Element::Physical => AttributeName::HPRatioPhysical,
        }
    }

    pub fn hp_ratio_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::HPRatioNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::HPRatioChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::HPRatioPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::HPRatioElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::HPRatioElementalBurst),
            _ => None,
        }
    }

    pub fn def_ratio_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::DEFRatioElectro,
            Element::Hydro => AttributeName::DEFRatioHydro,
            Element::Anemo => AttributeName::DEFRatioAnemo,
            Element::Pyro => AttributeName::DEFRatioPyro,
            Element::Cryo => AttributeName::DEFRatioCryo,
            Element::Dendro => AttributeName::DEFRatioDendro,
            Element::Geo => AttributeName::DEFRatioGeo,
            Element::Physical => AttributeName::DEFRatioPhysical,
        }
    }

    pub fn def_ratio_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::DEFRatioNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::DEFRatioChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::DEFRatioPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::DEFRatioElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::DEFRatioElementalBurst),
            _ => None,
        }
    }

    pub fn atk_ratio_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Electro => AttributeName::ATKRatioElectro,
            Element::Hydro => AttributeName::ATKRatioHydro,
            Element::Anemo => AttributeName::ATKRatioAnemo,
            Element::Pyro => AttributeName::ATKRatioPyro,
            Element::Cryo => AttributeName::ATKRatioCryo,
            Element::Dendro => AttributeName::ATKRatioDendro,
            Element::Geo => AttributeName::ATKRatioGeo,
            Element::Physical => AttributeName::ATKRatioPhysical,
        }
    }

    pub fn atk_ratio_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::ATKRatioNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::ATKRatioChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::ATKRatioPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::ATKRatioElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::ATKRatioElementalBurst),
            _ => None,
        }
    }

    pub fn extra_dmg_name_by_element(element: Element) -> AttributeName {
        match element {
            Element:: Electro => AttributeName::ExtraDmgElectro,
            Element::Hydro => AttributeName::ExtraDmgHydro,
            Element::Anemo => AttributeName::ExtraDmgAnemo,
            Element::Pyro => AttributeName::ExtraDmgPyro,
            Element::Cryo => AttributeName::ExtraDmgCryo,
            Element::Dendro => AttributeName::ExtraDmgDendro,
            Element::Geo => AttributeName::ExtraDmgGeo,
            Element::Physical => AttributeName::ExtraDmgPhysical,
        }
    }

    pub fn extra_dmg_name_by_skill_type(skill_type: SkillType) -> Option<AttributeName> {
        match skill_type {
            SkillType::NormalAttack => Some(AttributeName::ExtraDmgNormalAttack),
            SkillType::ChargedAttack => Some(AttributeName::ExtraDmgChargedAttack),
            SkillType::PlungingAttackOnGround | SkillType::PlungingAttackInAction => Some(AttributeName::ExtraDmgPlungingAttack),
            SkillType::ElementalSkill => Some(AttributeName::ExtraDmgElementalSkill),
            SkillType::ElementalBurst => Some(AttributeName::ExtraDmgElementalBurst),
            _ => None,
        }
    }

    pub fn res_minus_name_by_element(element: Element) -> AttributeName {
        match element {
            Element::Cryo => AttributeName::ResMinusCryo,
            Element::Pyro => AttributeName::ResMinusPyro,
            Element::Geo => AttributeName::ResMinusGeo,
            Element::Electro => AttributeName::ResMinusElectro,
            Element::Hydro => AttributeName::ResMinusHydro,
            Element::Anemo => AttributeName::ResMinusAnemo,
            Element::Dendro => AttributeName::ResMinusDendro,
            Element::Physical => AttributeName::ResMinusPhysical,
        }
    }
}
