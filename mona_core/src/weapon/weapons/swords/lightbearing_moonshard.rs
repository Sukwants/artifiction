use crate::weapon::weapons::prelude::*;

pub struct LightbearingMoonshardEffect {
    pub rate: f64,
}

impl<A: Attribute> WeaponEffect<A> for LightbearingMoonshardEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.add_def_percentage("朏魄含光被动", 0.15 + 0.05 * refine);

        if self.rate > 0.0 {
            attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::CriticalDamage,
                ReactionType::LunarCrystallize,
            )), "朏魄含光被动", (0.48 + 0.16 * refine) * self.rate);
        }
    }
}

pub struct LightbearingMoonshard;

impl WeaponTrait for LightbearingMoonshard {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::LightbearingMoonshard,
        internal_name: "Sword_LightbearingMoonshard",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "防御力提高 <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> 。装备者施放元素战技后的5秒内，月结晶反应伤害提升 <span style=\"color: #409EFF;\">64%-80%-96%-112%-128%</span> 。",
            en: "Increases DEF by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> . DMG inflicted by Lunar-Crystallize reactions increases by <span style=\"color: #409EFF;\">64%-80%-96%-112%-128%</span> for 5s after the equipping character uses an Elemental Skill."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "朏魄含光",
            en: "Lightbearing Moonshard"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: ItemConfig::DEFAULT_RATE_TITLE,
            config: ItemConfig::RATE01_TYPE,
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::LightbearingMoonshard { rate } => rate,
            _ => 0.0,
        };

        Some(Box::new(LightbearingMoonshardEffect {
            rate,
        }))
    }
}