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

pub struct TheDaybreakChroniclesEffect {
    pub rate1: f64,
    pub rate2: f64,
    pub rate3: f64,
}

impl<A: Attribute> WeaponEffect<A> for TheDaybreakChroniclesEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.set_value_by(AttributeName::BonusNormalAttack, "黎明破晓之史「渐起的晓风」", (0.45 + 0.15 * refine) * self.rate1);
        attribute.set_value_by(AttributeName::BonusElementalSkill, "黎明破晓之史「渐起的晓风」", (0.45 + 0.15 * refine) * self.rate2);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "黎明破晓之史「渐起的晓风」", (0.45 + 0.15 * refine) * self.rate3);
    }
}

pub struct TheDaybreakChronicles;

impl WeaponTrait for TheDaybreakChronicles {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheDaybreakChronicles,
        internal_name: "Bow_TheDaybreakChronicles",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage96),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "装备者获得「渐起的晓风」：脱离战斗状态3秒后，普通攻击、元素战技和元素爆发造成的伤害提升 <span style=\"color: #409EFF;\">60%-75%-90%-105%-120% </span>。在战斗状态下，上述伤害提升效果每秒降低 <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span> ，直到降低至0%；装备者的普通攻击、元素战技或元素爆发命中敌人时，对应类别的伤害提升效果提升 <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span> ，直到提升至 <span style=\"color: #409EFF;\">60%-75%-90%-105%-120%</span> ，上述每种类别的攻击每0.1秒至多触发一次该效果。装备者处于队伍后台时，依然能触发上述效果。\
                <br>此外，队伍拥有「魔导·秘仪」效果时，装备者的普通攻击、元素战技或元素爆发命中敌人时，改为使所有类别的伤害提升效果提升 <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> 。",
            en: "The equipping character gains Stirring Dawn Breeze: 3s after leaving combat, Normal Attack, Elemental Skill, and Elemental Burst DMG is increased by <span style=\"color: #409EFF;\">60%-75%-90%-105%-120%</span> . While in combat, this DMG Bonus will decrease by <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span> per second until it reaches 0%. When the equipping character's Normal Attacks, Elemental Skills, or Elemental Bursts hit an opponent, the DMG Bonus for the corresponding DMG type is increased by <span style=\"color: #409EFF;\">10%-12.5%-15%-17.5%-20%</span> until it reaches <span style=\"color: #409EFF;\">60%-75%-90%-105%-120%</span> . This effect can be triggered once every 0.1s for each of the attack types mentioned above. This effect can be triggered even if the equipping character is off-field. \
                <br>Additionally, when the party possesses Hexerei: Secret Rite effects, when the equipping character's Normal Attacks, Elemental Skills, or Elemental Bursts hit an opponent, the DMG Bonus for all these DMG types is increased by <span style=\"color: #409EFF;\">20%-25%-30%-35%-40%</span> instead."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "黎明破晓之史",
            en: "The Daybreak Chronicles"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate1",
            title: locale!(
                zh_cn: "普通攻击伤害提升效果比例",
                en: "Normal Attack DMG Bonus Effect Proportion"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "rate2",
            title: locale!(
                zh_cn: "元素战技伤害提升效果比例",
                en: "Elemental Skill DMG Bonus Effect Proportion"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
        ItemConfig {
            name: "rate3",
            title: locale!(
                zh_cn: "元素爆发伤害提升效果比例",
                en: "Elemental Burst DMG Bonus Effect Proportion"
            ),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 0.0 },
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (rate1, rate2, rate3) = match *config {
            WeaponConfig::TheDaybreakChronicles { rate1, rate2, rate3 } => (rate1, rate2, rate3),
            _ => (0.0, 0.0, 0.0)
        };

        Some(Box::new(TheDaybreakChroniclesEffect {
            rate1,
            rate2,
            rate3,
        }))
    }
}