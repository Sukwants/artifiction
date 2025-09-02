use std::cmp::max;
use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;

pub struct BuffNahidaTalent1 {
    pub max_em: f64,
}

impl<A: Attribute> Buff<A> for BuffNahidaTalent1 {
    fn change_attribute(&self, attribute: &mut A) {
        let value = (self.max_em * 0.25).min(250.0);
        attribute.set_value_by(AttributeName::ElementalMasteryExtra, "BUFF: 纳西妲天赋1", value);
    }
}

impl BuffMeta for BuffNahidaTalent1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NahidaTalent1,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "纳西妲-「净善摄受明论」",
            en: "Nahida-「Compassion Illuminated」",
        ),
        image: BuffImage::Avatar(CharacterName::Nahida),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "施放心景幻成时，摩耶之殿将获得以下效果：<br>依据队伍中元素精通最高的角色的元素精通数值的25%，提高领域内当前场上角色的元素精通。通过这种方式，至多提升250点元素精通。",
            en: "When unleashing Illusory Heart, the Shrine of Maya will gain the following effects:<br>The Elemental Mastery of the active character within the field will be increased by 25% of the Elemental Mastery of the party member with the highest Elemental Mastery. You can gain a maximum of 250 Elemental Mastery in this manner."
        )),
        from: BuffFrom::Character(CharacterName::Nahida)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "max_em",
            title: crate::common::i18n::locale!(
                zh_cn: "队伍最大元素精通",
                en: "Max EM in Team",
            ),
            config: ItemConfigType::Float { min: 0.0, max: 3000.0, default: 1000.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let max_em = match *b {
            BuffConfig::NahidaTalent1 { max_em } => max_em,
            _ => 0.0
        };
        Box::new(BuffNahidaTalent1 {
            max_em
        })
    }
}

pub struct BuffNahidaC2 {
    de_def: bool,
}

impl<A: Attribute> Buff<A> for BuffNahidaC2 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.de_def {
            attribute.set_value_by(AttributeName::DefMinus, "纳西妲「正等善见之根」", 0.3);
        }

        attribute.set_value_by(AttributeName::CriticalDamageBloom, "纳西妲「正等善见之根」", 1.0);
        attribute.set_value_by(AttributeName::CriticalDamageHyperbloom, "纳西妲「正等善见之根」", 1.0);
        attribute.set_value_by(AttributeName::CriticalDamageBurgeon, "纳西妲「正等善见之根」", 1.0);
        attribute.set_value_by(AttributeName::CriticalDamageBurning, "纳西妲「正等善见之根」", 1.0);

        attribute.set_value_to(AttributeName::CriticalBloom, "纳西妲「正等善见之根」", 0.2);
        attribute.set_value_to(AttributeName::CriticalHyperbloom, "纳西妲「正等善见之根」", 0.2);
        attribute.set_value_to(AttributeName::CriticalBurgeon, "纳西妲「正等善见之根」", 0.2);
        attribute.set_value_to(AttributeName::CriticalBurning, "纳西妲「正等善见之根」", 0.2);

        attribute.set_value_by(AttributeName::CriticalDamageLunarBloom, "纳西妲「正等善见之根」", 0.2);
        attribute.set_value_by(AttributeName::CriticalLunarBloom, "纳西妲「正等善见之根」", 0.1);
    }
}

impl BuffMeta for BuffNahidaC2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NahidaC2,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "纳西妲「正等善见之根」",
            en: "Nahida-「The Root of All Fullness」",
        ),
        image: BuffImage::Avatar(CharacterName::Nahida),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "处于纳西妲自身施加的蕴种印状态下的敌人，将受到以下效果影响：\
                <br>· 受到的燃烧、绽放、超绽放、烈绽放反应伤害能够造成暴击，暴击率固定为20%，暴击伤害固定为100%，该效果提供的暴击率可以与使对应元素反应能够造成暴击的同类效果提供的暴击率叠加；\
                <br> 受到原激化、超激化、蔓激化反应影响后的8秒内，防御力降低30%；\
                <br> 受到月绽放反应伤害的暴击率提升10%，暴击伤害提升20%。",
            en: "Opponents that are marked by Seeds of Skandha applied by Nahida herself will be affected by the following effects:\
                <br>· Burning, Bloom, Hyperbloom, and Burgeon Reaction DMG they receive can score CRIT Hits. CRIT Rate and CRIT DMG are fixed at 20% and 100% respectively. CRIT Rate from this effect stacks with CRIT Rate from similar effects that allow these Elemental Reactions to CRIT.\
                <br>· Within 8s of being affected by Quicken, Aggravate, or Spread, DEF is decreased by 30%.\
                <br>· The CRIT Rate and CRIT DMG of Lunar-Bloom DMG received are increased by 10% and 20% respectively."
        )),
        from: BuffFrom::Character(CharacterName::Nahida)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "de_def",
            title: crate::common::i18n::locale!(
                zh_cn: "防御力降低",
                en: "Defense DEF",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let de_def = match *b {
            BuffConfig::NahidaC2 { de_def } => de_def,
            _ => false
        };
        Box::new(BuffNahidaC2 {
            de_def
        })
    }
}
