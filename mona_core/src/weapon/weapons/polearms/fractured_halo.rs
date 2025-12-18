use crate::attribute::*;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct FracturedHaloEffect {
    electrifying_edict: bool
}

impl FracturedHaloEffect {
    pub fn new(config: &WeaponConfig) -> FracturedHaloEffect {
        match *config {
            WeaponConfig::FracturedHalo { electrifying_edict } => FracturedHaloEffect {
                electrifying_edict
            },
            _ => FracturedHaloEffect {
                electrifying_edict: false
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for FracturedHaloEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        attribute.add_atk_percentage("支离轮光被动", refine * 0.06 + 0.18);
        if self.electrifying_edict {
            attribute.set_value_by(AttributeName::EnhanceLunarCharged, "支离轮光被动", refine * 0.1 + 0.3);
        }
    }
}

pub struct FracturedHalo;

impl WeaponTrait for FracturedHalo {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FracturedHalo,
        internal_name: "",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage144),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "施放元素战技或元素爆发后的20秒内，攻击力提升<span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span>。持续期间内，若装备者创造了护盾，则接下来的20秒内，还会获得「流电圣敕」效果：队伍中附近所有角色触发的月感电反应造成的伤害提升<span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span>。",
            en: "After an Elemental Skill or Elemental Burst is used, ATK is increased by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> for 20s. If the equipping character creates a Shield while this effect is active, they will gain the Electrifying Edict effect for 20s: All nearby party members deal <span style=\"color: #409EFF;\">40%-50%-60%-70%-80%</span> more Lunar-Charged DMG.",
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "支离轮光",
            en: "Fractured Halo"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "electrifying_edict",
            title: locale!(
                zh_cn: "「流电圣敕」",
                en: "Electrifying Edict",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(FracturedHaloEffect::new(config)))
    }
}
