use crate::attribute::{AttributeName, Attribute, AttributeCommon};
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

pub struct SacrificersStaffEffect {
    pub stack: f64,
}

impl<T: Attribute> WeaponEffect<T> for SacrificersStaffEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        attribute.add_atk_percentage("圣祭者的辉杖「未染的觖望」", (0.06 + 0.02 * refine) * self.stack);
        attribute.set_value_by(AttributeName::Recharge, "圣祭者的辉杖「未染的觖望」", (0.045 + 0.015 * refine) * self.stack);
    }
}

impl SacrificersStaffEffect {
    pub fn new(config: &WeaponConfig) -> SacrificersStaffEffect {
        let stack = match *config {
            WeaponConfig::SacrificersStaff { stack } => stack,
            _ => 0.0
        };

        SacrificersStaffEffect {
            stack,
        }
    }
}

pub struct SacrificersStaff;

impl WeaponTrait for SacrificersStaff {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SacrificersStaff,
        internal_name: "Polearm_SacrificersStaff",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate20),
        weapon_base: WeaponBaseATKFamily::ATK620,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(locale!(
            zh_cn: "元素战技命中敌人后的6秒内，攻击力提升 <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> ，元素充能效率提升 <span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span> 。该效果至多叠加3层，装备者处于队伍后台时依然能触发。",
            en: "For 6s after an Elemental Skill hits an opponent, ATK is increased by <span style=\"color: #409EFF;\">8%-10%-12%-14%-16%</span> and Energy Recharge is increased by <span style=\"color: #409EFF;\">6%-7.5%-9%-10.5%-12%</span> . Max 3 stacks. This effect can be triggered even when the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: locale!(
            zh_cn: "圣祭者的辉杖",
            en: "Sacrificer's Staff"
        ),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: locale!(
                zh_cn: "平均层数",
                en: "Avg Stack"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 0.0 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SacrificersStaffEffect::new(config)))
    }
}
