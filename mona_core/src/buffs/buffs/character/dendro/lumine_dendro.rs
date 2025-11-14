use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Element;

pub struct BuffLumineDendroP1 {
    pub overflowing_lotuslight_count: usize,
}

impl<A: Attribute> Buff<A> for BuffLumineDendroP1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "荧-草「蔓生的埜草」", (self.overflowing_lotuslight_count.min(10) as f64) * 6.0);
    }
}

impl BuffMeta for BuffLumineDendroP1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LumineDendroP1,
        name_locale: locale!(
            zh_cn: "荧-草-「蔓生的埜草」",
            en: "Lumine-Dendro-Verdant Overgrowth"
        ),
        image: BuffImage::Avatar(CharacterName::LumineDendro),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "草灯莲将在其存在期间每秒获得一层莲光遍照效果，使其领域内的当前场上角色的元素精通提升6点。草灯莲的莲光遍照效果至多叠加10层。",
            en: "Lea Lotus Lamp will obtain one level of Overflowing Lotuslight every second it is on the field, increasing the Elemental Mastery of active character(s) within its AoE by 6. Overflowing Lotuslight has a maximum of 10 stacks."
        )),
        from: BuffFrom::Character(CharacterName::LumineDendro),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "overflowing_lotuslight_count",
            title: locale!(
                zh_cn: "莲光遍照层数",
                en: "Overflowing Lotuslight Count",
            ),
            config: ItemConfigType::Int { min: 0, max: 10, default: 0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let overflowing_lotuslight_count = match *b {
            BuffConfig::LumineDendroP1 { overflowing_lotuslight_count } => overflowing_lotuslight_count,
            _ => 0
        };
        Box::new(BuffLumineDendroP1 {
            overflowing_lotuslight_count,
        })
    }
}

pub struct BuffLumineDendroC6 {
    pub has_elemental_correspond: bool,
    pub elemental_correspond: Element,
}

impl<A: Attribute> Buff<A> for BuffLumineDendroC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusDendro, "荧-草「蕴思的霜草」", 0.12);

        if self.has_elemental_correspond {
            attribute.set_value_by(AttributeName::bonus_name_by_element(self.elemental_correspond), "荧-草「蕴思的霜草」", 0.12);
        }
    }
}

impl BuffMeta for BuffLumineDendroC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::LumineDendroC6,
        name_locale: locale!(
            zh_cn: "荧-草-「蕴思的霜草」",
            en: "Lumine-Dendro-Withering Aggregation"
        ),
        image: BuffImage::Avatar(CharacterName::LumineDendro),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "处于草灯莲的莲光遍照效果影响下的角色获得12%草元素伤害加成；若草灯莲发生过莲光幻变转化，还将获得12%对应元素伤害加成。",
            en: "The Dendro DMG Bonus of the character under the effect of Overflowing Lotuslight as created by the Lea Lotus Lamp is increased by 12%. If the Lamp has experienced a Lotuslight Transfiguration previously, the character will also gain 12% DMG Bonus for the corresponding element."
        )),
        from: BuffFrom::Character(CharacterName::LumineDendro),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "has_elemental_correspond",
            title: locale!(zh_cn: "是否发生元素转化", en: "Whether Elemental correspond Occurred"),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "elemental_correspond",
            title: locale!(zh_cn: "元素转化类型", en: "Elemental correspond Type"),
            config: ItemConfigType::Element { elements: &[Element::Hydro, Element::Electro, Element::Pyro], default: Element::Hydro }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (has_elemental_correspond, elemental_correspond) = match *b {
            BuffConfig::LumineDendroC6 { has_elemental_correspond, elemental_correspond } => (has_elemental_correspond, elemental_correspond),
            _ => (false, Element::Hydro)
        };
        Box::new(BuffLumineDendroC6 {
            has_elemental_correspond,
            elemental_correspond,
        })
    }
}