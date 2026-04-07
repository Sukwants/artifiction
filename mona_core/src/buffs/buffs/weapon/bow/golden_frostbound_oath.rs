use crate::buffs::buffs::prelude::*;

pub struct BuffGoldenFrostboundOath {
    pub refine: usize,
    pub rate: f64,
}

impl<A: Attribute> Buff<A> for BuffGoldenFrostboundOath {
    fn change_attribute(&self, attribute: &mut A) {
        let refine: f64 = self.refine as f64;

        attribute.set_value_by_t(
            AttributeType::Invisible(InvisibleAttributeType::new_element(
            AttributeVariableType::Bonus,
            Element::Geo,
        )), "霜结的誓金枝被动", (0.15 + 0.05 * refine) * self.rate);
        attribute.set_value_by_t(
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(
            AttributeVariableType::ReactionEnhance,
            ReactionType::LunarCrystallize,
        )), "霜结的誓金枝被动", (0.15 + 0.05 * refine) * self.rate);
    }
}

impl BuffMeta for BuffGoldenFrostboundOath {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::GoldenFrostboundOath,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "霜结的誓金枝-「霜妖精的恶戏」",
            en: "Golden Frostbound Oath-Frost Fae's Mischief",
        ),
        image: BuffImage::Weapon(WeaponName::GoldenFrostboundOath),
        genre: BuffGenre::Weapon,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "装备者的元素战技或月结晶攻击命中敌人时，将获得持续6秒的「霜妖精的报恩」效果。持续期间，若装备者附近存在月笼，则队伍中附近的所有其他角色还会获得「霜妖精的恶戏」效果：造成的岩元素伤害提升 <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> ，月结晶反应伤害提升 <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> 。装备者处于队伍后台时，依然能触发上述效果。",
            en: "When the equipping character's Elemental Skill or Lunar-Crystallize attack(s) hits enemies, gain the Frost Fae's Favor effect for 6s. While this effect is active, if there are Moondrift(s) near the equipping character, all other nearby party members will gain the Frost Fae's Mischief effect: Geo DMG dealt increases by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> and Lunar-Crystallize Reaction DMG increases by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span>. This effect can be triggered even when the equipping character is off-field."
        )),
        from: BuffFrom::Weapon(WeaponName::GoldenFrostboundOath)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (refine, rate) = match *b {
            BuffConfig::GoldenFrostboundOath { refine, rate } => (refine, rate),
            _ => (1, 0.0)
        };

        Box::new(BuffGoldenFrostboundOath { refine, rate })
    }
}
