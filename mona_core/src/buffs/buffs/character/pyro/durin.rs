use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
#[cfg(not(target_family = "wasm"))]
use crate::common::Element;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ConfigElements8Multi, ItemConfig, ItemConfigType};

pub struct BuffDurinP1 {
    pub hexerei_secret_rite: bool,
    pub elements: ConfigElements8Multi,
}

impl<A: Attribute> Buff<A> for BuffDurinP1 {
    fn change_attribute(&self, attribute: &mut A) {
        let ratio = if self.hexerei_secret_rite { 0.35 } else { 0.20 };

        if self.elements.pyro { attribute.set_value_by(AttributeName::ResMinusPyro, "杜林「光灵遵神数显现」", ratio); }
        if self.elements.hydro { attribute.set_value_by(AttributeName::ResMinusHydro, "杜林「光灵遵神数显现」", ratio); }
        if self.elements.anemo { attribute.set_value_by(AttributeName::ResMinusAnemo, "杜林「光灵遵神数显现」", ratio); }
        if self.elements.electro { attribute.set_value_by(AttributeName::ResMinusElectro, "杜林「光灵遵神数显现」", ratio); }
        if self.elements.dendro { attribute.set_value_by(AttributeName::ResMinusDendro, "杜林「光灵遵神数显现」", ratio); }
        if self.elements.cryo { attribute.set_value_by(AttributeName::ResMinusCryo, "杜林「光灵遵神数显现」", ratio); }
        if self.elements.geo { attribute.set_value_by(AttributeName::ResMinusGeo, "杜林「光灵遵神数显现」", ratio); }
    }
}

impl BuffMeta for BuffDurinP1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::DurinP1,
        name_locale: locale!(
            zh_cn: "杜林-「光灵遵神数显现」",
            en: "Durin-Light Manifest of the Divine Calculus"
        ),
        image: BuffImage::Avatar(CharacterName::Durin),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "杜林天赋1：白焰之龙：队伍中附近的角色对敌人触发燃烧、超载、火元素扩散、火元素结晶反应后，或对处于燃烧状态下的敌人造成火元素伤害或草元素伤害时，该敌人的火元素抗性与参与反应的对应元素抗性降低20%，持续6秒。",
            en: "Durin Talent1: Dragon of White Flame: After nearby party members trigger Burning, Overloaded, Pyro Swirl, or Pyro Crystallize reactions on an opponent, or deal Pyro DMG or Dendro DMG to an opponent affected by Burning, that opponent's Pyro RES, and the RES for the corresponding Element involved in the reaction, are decreased by 20% for 6s."
        )),
        from: BuffFrom::Character(CharacterName::Durin),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_BUFF),
        ItemConfig {
            name: "elements",
            title: locale!(
                zh_cn: "反应元素",
                en: "Reaction Elements"
            ),
            config: ItemConfigType::ElementMulti {
                elements: &[Element::Pyro, Element::Hydro, Element::Anemo, Element::Electro, Element::Dendro, Element::Cryo, Element::Geo],
                default: ConfigElements8Multi {
                    pyro: true,
                    hydro: false,
                    anemo: false,
                    electro: false,
                    dendro: false,
                    cryo: false,
                    geo: false,
                    physical: false,
                }
            }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (hexerei_secret_rite, elements) = match *b {
            BuffConfig::DurinP1 { hexerei_secret_rite, elements } => (hexerei_secret_rite, elements),
            _ => (false, ConfigElements8Multi { pyro: false, electro: false, dendro: false, cryo: false, anemo: false, geo: false, hydro: false, physical: false })
        };
        Box::new(BuffDurinP1 {
            hexerei_secret_rite,
            elements,
        })
    }
}

pub struct BuffDurinC1 {
    pub atk: f64,
}

impl<A: Attribute> Buff<A> for BuffDurinC1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ExtraDmgNormalAttack, "杜林「红土之逆」", self.atk * 0.6);
        attribute.set_value_by(AttributeName::ExtraDmgChargedAttack, "杜林「红土之逆」", self.atk * 0.6);
        attribute.set_value_by(AttributeName::ExtraDmgPlungingAttack, "杜林「红土之逆」", self.atk * 0.6);
        attribute.set_value_by(AttributeName::ExtraDmgElementalSkill, "杜林「红土之逆」", self.atk * 0.6);
        attribute.set_value_by(AttributeName::ExtraDmgElementalBurst, "杜林「红土之逆」", self.atk * 0.6);
    }
}

impl BuffMeta for BuffDurinC1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::DurinC1,
        name_locale: locale!(
            zh_cn: "杜林-「红土之逆」",
            en: "Durin-Adamah's Redemption"
        ),
        image: BuffImage::Avatar(CharacterName::Durin),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "杜林命座1：白化法·如光流变：队伍中附近的所有其他角色获得20层「轮变启迪」，持续20秒。拥有「轮变启迪」的当前场上角色的普通攻击、重击、下落攻击、元素战技或元素爆发造成伤害时，将消耗1层「轮变启迪」，提升造成的伤害，提升值相当于杜林攻击力的60%。队伍中拥有「轮变启迪」的角色，其生效次数单独计算。",
            en: "Durin C1: Principle of Purity: As the Light Shifts: All other nearby party members gain 20 stacks of Cycle of Enlightenment lasting 20s. When nearby active characters deal Normal Attack, Charged Attack, Plunging Attack, Elemental Skill, or Elemental Burst DMG, 1 stack of Cycle of Enlightenment will be consumed to increase the DMG dealt by 60% of Durin's ATK. Stack counts for characters in the party who have Cycle of Enlightenment are managed individually."
        )),
        from: BuffFrom::Character(CharacterName::Durin),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "atk",
            title: locale!(
                zh_cn: "攻击力",
                en: "ATK"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let atk = match *b {
            BuffConfig::DurinC1 { atk } => atk,
            _ => 0.0
        };
        Box::new(BuffDurinC1 {
            atk,
        })
    }
}

pub struct BuffDurinC6 {
}

impl<A: Attribute> Buff<A> for BuffDurinC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::DefMinus, "杜林「双重诞生」", 0.3);
    }
}

impl BuffMeta for BuffDurinC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::DurinC6,
        name_locale: locale!(
            zh_cn: "杜林-「双重诞生」",
            en: "Durin-Dual Birth"
        ),
        image: BuffImage::Avatar(CharacterName::Durin),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "杜林命座6：元素爆发白化法·如光流变与「白焰之龙」命中敌人时，该敌人的防御力降低30%，持续6秒",
            en: "Durin C6: After Principle of Purity: As the Light Shifts and Dragon of White Flame hit an opponent, that opponent's DEF is decreased by 30% for 6s."
        )),
        from: BuffFrom::Character(CharacterName::Durin),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffDurinC6 {
        })
    }
}