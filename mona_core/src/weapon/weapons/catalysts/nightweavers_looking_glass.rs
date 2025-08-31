use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct NightweaversLookingGlassEffect {
    pub effect1: bool,
    pub effect2: bool,
}

impl<A: Attribute> WeaponEffect<A> for NightweaversLookingGlassEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {

        if self.effect1 {
            attribute.set_value_by(AttributeName::ElementalMastery, "纺夜天镜：「终北圣言」", 0.45 + 0.15 * data.refine as f64);
        }

        if self.effect2 {
            attribute.set_value_by(AttributeName::ElementalMastery, "纺夜天镜：「朔月诗篇」", 0.45 + 0.15 * data.refine as f64);
        }

        if self.effect1 && self.effect2 {
            attribute.set_value_by(AttributeName::EnhanceBloom, "纺夜天镜：「终北圣言」与「朔月诗篇」", 0.9 + 0.3 * data.refine as f64);
            attribute.set_value_by(AttributeName::EnhanceHyperbloom, "纺夜天镜：「终北圣言」与「朔月诗篇」", 0.6 + 0.2 * data.refine as f64);
            attribute.set_value_by(AttributeName::EnhanceBurgeon, "纺夜天镜：「终北圣言」与「朔月诗篇」", 0.6 + 0.2 * data.refine as f64);
            attribute.set_value_by(AttributeName::EnhanceLunarBloom, "纺夜天镜：「终北圣言」与「朔月诗篇」", 0.3 + 0.1 * data.refine as f64);
        }

    }
}

pub struct NightweaversLookingGlass;

impl WeaponTrait for NightweaversLookingGlass {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::NightweaversLookingGlass,
        internal_name: "Catalyst_Ayus",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM58),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素战技造成水元素或草元素伤害时，装备者获得「终北圣言」效果：元素精通提升<span style=\"color: #409EFF;\">60-75-90-105-120</span>点，持续4.5秒；队伍中附近的角色触发月绽放反应时，装备者获得「朔月诗篇」效果：元素精通提升<span style=\"color: #409EFF;\">60-75-90-105-120</span>点，持续10秒；「终北圣言」效果与「朔月诗篇」效果同时存在时，队伍中附近的所有角色触发的绽放反应造成的伤害提升<span style=\"color: #409EFF;\">120%-150%-180%-210%-240%</span>，超绽放、烈绽放反应造成的伤害提升<span style=\"color: #409EFF;\">80%-100%-120%-140%-160%</span>，月绽放反应伤害提升<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>，该效果无法叠加。装备者处于队伍后台时，依然能触发上述效果。",
            en: "When the equipping character's Elemental Skill deals Hydro or Dendro DMG, they will gain Prayer of the Far North: Elemental Mastery is increased by <span style=\"color: #409EFF;\">60-75-90-105-120</span> for 4.5s. When nearby party members trigger Lunar-Bloom reactions, the equipping character gains New Moon Verse: Elemental Mastery is increased by <span style=\"color: #409EFF;\">60-75-90-105-120</span> for 10s. When both Prayer of the Far North and New Moon Verse are in effect, all nearby party members' Bloom DMG is increased by <span style=\"color: #409EFF;\">120%-150%-180%-210%-240%</span>, their Hyperbloom and Burgeon DMG is increased by <span style=\"color: #409EFF;\">80%-100%-120%-140%-160%</span>, and their Lunar-Bloom DMG is increased by <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>. This effect cannot stack. The aforementioned effects can be triggered even if the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "纺夜天镜",
            en: "Nightweaver's Looking Glass"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "effect1",
            title: locale!(
                zh_cn: "「终北圣言」",
                en: "Prayer of the Far North",
            ),
            config: ItemConfigType::Bool {  default: false },
        },
        ItemConfig {
            name: "effect2",
            title: locale!(
                zh_cn: "「朔月诗篇」",
                en: "New Moon Verse"
            ),
            config: ItemConfigType::Bool {  default: false },
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (effect1, effect2) = match *config {
            WeaponConfig::NightweaversLookingGlass { effect1, effect2 } => (effect1, effect2),
            _ => (false, false)
        };

        Some(Box::new(NightweaversLookingGlassEffect {
            effect1,
            effect2,
        }))
    }
}