use crate::attribute::*;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::{Moonsign, WeaponType};
use super::super::super::weapon_effect::WeaponEffect;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::WeaponName;
use super::super::super::weapon_config::WeaponConfig;

pub struct MoonweaversDawnEffect {
    pub energy: usize,
}

impl<T: Attribute> WeaponEffect<T> for MoonweaversDawnEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        let val = (refine * 0.05 + 0.15) +
            if self.energy <= 40 { refine * 0.07 + 0.21 }
            else if self.energy <= 60 { refine * 0.04 + 0.12 }
            else { 0.0 };

        attribute.set_value_by(AttributeName::BonusElementalBurst, "Buff: 织月者的曙色「秘银的血告」", val);
    }
}

impl MoonweaversDawnEffect {
    pub fn new(config: &WeaponConfig) -> MoonweaversDawnEffect {
        let energy = match *config {
            WeaponConfig::MoonweaversDawn { energy } => energy,
            _ => 0
        };

        MoonweaversDawnEffect {
            energy,
        }
    }
}

pub struct MoonweaversDawn;

impl WeaponTrait for MoonweaversDawn {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MoonweaversDawn,
        internal_name: "Sword_MoonweaversDawn",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::ATK60),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "元素爆发造成的伤害提高 <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> ；装备者的元素能量上限不超过60/40点时，元素爆发造成的伤害额外提高 <span style=\"color: #409EFF;\">16%/28%-20%/35%-24%/42%-28%/49%-32%/56%</span> 。",
            en: "Increases Elemental Burst DMG by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> . When the equipping character's Energy Capacity does not exceed 60/40, their Elemental Burst DMG is increased by an additional <span style=\"color: #409EFF;\">16%/28%-20%/35%-24%/42%-28%/49%-32%/56%</span> ."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "织月者的曙色",
            en: "Moonweaver's Dawn"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "energy",
            title: locale!(
                zh_cn: "装备者元素爆发能量上限",
                en: "Equipping Character's Elemental Burst Energy Capacity"
            ),
            config: ItemConfigType::Int { min: 0, max: 100, default: 70 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MoonweaversDawnEffect::new(config)))
    }
}
