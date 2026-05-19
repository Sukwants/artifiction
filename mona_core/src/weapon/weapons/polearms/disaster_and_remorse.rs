use crate::weapon::weapons::prelude::*;

pub struct DisasterAndRemorseEffect {
    pub rate1: f64,
    pub rate2: f64,
    pub hexerei_secret_rite: bool,
}

impl<A: Attribute> WeaponEffect<A> for DisasterAndRemorseEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let base_bonus = 0.3 + 0.1 * refine;
        // 魔导·秘仪：所有伤害提升效果提升75%
        let multiplier = if self.hexerei_secret_rite { 1.75 } else { 1.0 };
        let bonus = base_bonus * multiplier;

        // 无赦：普通攻击与重击伤害提升
        if self.rate1 > 0.0 {
            attribute.set_value_by_t(
                AttributeType::Invisible(InvisibleAttributeType::new_skill(
                    AttributeVariableType::Bonus,
                    SkillType::NormalAttack,
                )),
                "灾悔被动",
                bonus * self.rate1,
            );
            attribute.set_value_by_t(
                AttributeType::Invisible(InvisibleAttributeType::new_skill(
                    AttributeVariableType::Bonus,
                    SkillType::ChargedAttack,
                )),
                "灾悔被动",
                bonus * self.rate1,
            );
        }

        // 无愈：元素战技与元素爆发伤害提升
        if self.rate2 > 0.0 {
            attribute.set_value_by_t(
                AttributeType::Invisible(InvisibleAttributeType::new_skill(
                    AttributeVariableType::Bonus,
                    SkillType::ElementalSkill,
                )),
                "灾悔被动",
                bonus * self.rate2,
            );
            attribute.set_value_by_t(
                AttributeType::Invisible(InvisibleAttributeType::new_skill(
                    AttributeVariableType::Bonus,
                    SkillType::ElementalBurst,
                )),
                "灾悔被动",
                bonus * self.rate2,
            );
        }
    }
}

pub struct DisasterAndRemorse;

impl WeaponTrait for DisasterAndRemorse {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::DisasterAndRemorse,
        internal_name: "Polearm_DisasterAndRemorse",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate48),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "装备者施放元素战技后，获得持续17秒的「纷争之途」，和分别持续3秒的「无赦」与「无愈」效果，每18秒至多触发一次。无赦：装备者的普通攻击与重击伤害提升 <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> 。无愈：装备者的元素战技与元素爆发伤害提升 <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> 。「纷争之途」期间，装备者的普通攻击与重击命中敌人时，将延长1秒「无愈」；装备者的元素战技与元素爆发命中敌人时，将延长1秒「无赦」。上述效果每0.1秒分别至多触发一次。「纷争之途」结束或装备者退场将解除「无赦」与「无愈」。<br>魔导·秘仪：上述所有伤害提升效果提升75%。",
            en: "After the equipping character uses an Elemental Skill, they gain \"Path of Conflict\" for 17s, as well as \"Unforgivable\" and \"Irreparable\" for 3s each. This effect can trigger once every 18s. Unforgivable: Increases the equipping character's Normal Attack and Charged Attack DMG by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> . Irreparable: Increases the equipping character's Elemental Skill and Elemental Burst DMG by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> . While Path of Conflict is in effect, when the equipping character hits an opponent with a Normal Attack or Charged Attack, Irreparable's duration will be increased by 1s. When the equipping character hits an opponent with their Elemental Skill or Elemental Burst, Unforgivable's duration will be increased by 1s. Each of the above effects can be triggered once every 0.1s. When Path of Conflict ends or the equipping character leaves the field, both Unforgivable and Irreparable will be removed.<br>Hexerei: Secret Rite: The above DMG boosts are increased by 75%."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "灾悔",
            en: "Disaster and Remorse"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(zh_cn: "无赦应用比例", en: "Unforgivable Ratio"),
            config: ItemConfig::RATE01_TYPE,
        },
        ItemConfig {
            name: "rate2",
            title: locale!(zh_cn: "无愈应用比例", en: "Irreparable Ratio"),
            config: ItemConfig::RATE01_TYPE,
        },
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_WEAPON),
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (rate1, rate2, hexerei_secret_rite) = match *config {
            WeaponConfig::DisasterAndRemorse { rate1, rate2, hexerei_secret_rite } => (rate1, rate2, hexerei_secret_rite),
            _ => (0.0, 0.0, false),
        };

        Some(Box::new(DisasterAndRemorseEffect {
            rate1,
            rate2,
            hexerei_secret_rite,
        }))
    }
}
