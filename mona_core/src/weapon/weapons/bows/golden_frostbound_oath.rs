use crate::weapon::weapons::prelude::*;

pub struct GoldenFrostboundOathEffect {
    pub rate1: f64,
    pub rate2: f64,
}

impl<A: Attribute> WeaponEffect<A> for GoldenFrostboundOathEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.add_def_percentage("霜结的誓金枝被动", 0.12 + 0.04 * refine);

        if self.rate1 > 0.0 {
            attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new_element(
                AttributeVariableType::Bonus,
                Element::Geo,
            )), "霜结的誓金枝被动", (0.3 + 0.1 * refine) * self.rate1);
            attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::ReactionEnhance,
                ReactionType::LunarCrystallize,
            )), "霜结的誓金枝被动", (0.3 + 0.1 * refine) * self.rate1);
        }
        if self.rate2 > 0.0 {
            attribute.set_value_by_s(
                CharacterSelector::select_all_except_self(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_element(
                AttributeVariableType::Bonus,
                Element::Geo,
            )), "霜结的誓金枝被动", (0.15 + 0.05 * refine) * self.rate2);
            attribute.set_value_by_s(
                CharacterSelector::select_all_except_self(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::ReactionEnhance,
                ReactionType::LunarCrystallize,
            )), "霜结的誓金枝被动", (0.15 + 0.05 * refine) * self.rate2);
        }
    }
}

pub struct GoldenFrostboundOath;

impl WeaponTrait for GoldenFrostboundOath {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::GoldenFrostboundOath,
        internal_name: "Bow_GoldenFrostboundOath",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "防御力提高 <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> 。装备者的元素战技或月结晶攻击命中敌人时，将获得持续6秒的「霜妖精的报恩」效果：装备者造成的岩元素伤害提升 <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> ，月结晶反应伤害提升 <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> 。持续期间，若装备者附近存在月笼，则队伍中附近的所有其他角色还会获得「霜妖精的恶戏」效果：造成的岩元素伤害提升 <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> ，月结晶反应伤害提升 <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> 。装备者处于队伍后台时，依然能触发上述效果。",
            en: "Increase DEF by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span>. When the equipping character's Elemental Skill or Lunar-Crystallize attack(s) hits enemies, gain the Frost Fae's Favor effect for 6s: Geo DMG inflicted by the equipping character increases by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>, Lunar-Crystallize Reaction DMG increases by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>. While this effect is active, if there are Moondrift(s) near the equipping character, all other nearby party members will gain the Frost Fae's Mischief effect: Geo DMG dealt increases by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> and Lunar-Crystallize Reaction DMG increases by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. This effect can be triggered even when the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "霜结的誓金枝",
            en: "Lightbearing Moonshard"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(zh_cn: "被动1应用比例", en: "Avg Effect 1 Ratio"),
            config: ItemConfig::RATE01_TYPE,
        },
        ItemConfig {
            name: "rate2",
            title: locale!(zh_cn: "被动2应用比例", en: "Avg Effect 2 Ratio"),
            config: ItemConfig::RATE01_TYPE,
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (rate1, rate2) = match *config {
            WeaponConfig::GoldenFrostboundOath { rate1, rate2 } => (rate1, rate2),
            _ => (0.0, 0.0),
        };

        Some(Box::new(GoldenFrostboundOathEffect {
            rate1,
            rate2,
        }))
    }
}