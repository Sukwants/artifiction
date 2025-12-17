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

pub struct AthameArtisEffect {
    pub hexerei_secret_rite: bool,
    pub effect: bool,
}

impl<A: Attribute> WeaponEffect<A> for AthameArtisEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::CriticalDamageElementalBurst, "黑蚀被动", 0.12 + 0.04 * refine);

        let rate = if self.hexerei_secret_rite { 1.75 } else { 1.0 };

        if self.effect {
            attribute.add_atk_percentage("黑蚀「白昼之刃」", (0.15 + 0.05 * refine) * rate);
        }
    }
}

pub struct AthameArtis;

impl WeaponTrait for AthameArtis {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AthameArtis,
        internal_name: "Sword_AthameArtis",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate72),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "元素爆发造成的暴击伤害提升 <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> ；元素爆发命中敌人时，将获得「白昼之刃」效果：攻击力提升 <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> ，除装备者以外，队伍中附近的当前场上角色攻击力提升 <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> ，持续3秒。此外，队伍拥有「魔导·秘仪」效果时，「白昼之刃」的效果额外提升75%。装备者处于队伍后台时，依然能触发上述效果。",
            en: "CRIT DMG from Elemental Bursts is increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> . When an Elemental Burst hits an opponent, gain the Blade of the Daylight Hours effect: ATK is increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> . Nearby active party members other than the equipping character have their ATK increased by <span style=\"color: #409EFF;\">16%-20%-24%-28%-32%</span> for 3s. Additionally, when the party possesses Hexerei: Secret Rite effects, the effects of Blade of the Daylight Hours are increased by an additional 75%. This effect can be triggered even if the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "黑蚀",
            en: "Athame Artis"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_WEAPON),
        ItemConfig {
            name: "effect",
            title: locale!(
                zh_cn: "「白昼之刃」",
                en: "the Daylight Hours"
            ),
            config: ItemConfigType::Bool {  default: false },
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (hexerei_secret_rite, effect) = match *config {
            WeaponConfig::AthameArtis { hexerei_secret_rite, effect } => (hexerei_secret_rite, effect),
            _ => (false, false)
        };

        Some(Box::new(AthameArtisEffect {
            hexerei_secret_rite,
            effect,
        }))
    }
}