use crate::artifacts::ArtifactSetName;
use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
#[cfg(not(target_family = "wasm"))]
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType, ConfigElements8Multi};
use crate::common::{Element, Moonsign};
use crate::enemies::Enemy;

pub struct BuffScrollOfTheHeroOfCinderCity4 {
    pub elements: ConfigElements8Multi,
    pub nightsouls_blessing: bool,
}

impl<A: Attribute> Buff<A> for BuffScrollOfTheHeroOfCinderCity4 {
    fn change_attribute(&self, attribute: &mut A) {
        for &elements in self.elements.collect_elements().iter() {
            let bonus_attribute_name = AttributeName::bonus_name_by_element(elements);
            attribute.set_value_by(bonus_attribute_name, "烬城勇者绘卷4", if self.nightsouls_blessing { 0.40 } else { 0.12 });
        }
    }
}

impl BuffMeta for BuffScrollOfTheHeroOfCinderCity4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ScrollOfTheHeroOfCinderCity4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "烬城勇者绘卷4",
            en: "ScrollOfTheHeroOfCinderCity4",
        ),
        image: BuffImage::Artifact(ArtifactSetName::ScrollOfTheHeroOfCinderCity),
        genre: BuffGenre::Artifact,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "装备者触发其对应元素类型的相关反应后，队伍中附近的所有角色的该元素反应相关的元素伤害加成提升12%，持续15秒。若触发该效果时，装备者处于夜魂加持状态下，还将使队伍中附近的所有角色的与该元素反应相关的元素伤害加成提升28%，持续20秒。装备者处于后台时也能触发上述效果。同名圣遗物套装产生的伤害加成效果无法叠加。",
            en: "After the equipping character triggers a reaction related to their Elemental Type, all nearby party members gain a 12% Elemental DMG Bonus for the Elemental Types involved in the elemental reaction for 15s. If the equipping character is in the Nightsoul's Blessing state when triggering this effect, all nearby party members gain an additional 28% Elemental DMG Bonus for the Elemental Types involved in the elemental reaction for 20s. The equipping character can trigger this effect while off-field, and the DMG bonus from Artifact Sets with the same name do not stack."
        )),
        from: BuffFrom::Artifact(ArtifactSetName::ScrollOfTheHeroOfCinderCity)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "elements",
            title: crate::common::i18n::locale!(zh_cn: "增伤元素", en: "Enhance Element"),
            config: ItemConfigType::Element8Multi {
                default: ConfigElements8Multi {
                    pyro: false,
                    electro: false,
                    dendro: false,
                    cryo: false,
                    anemo: false,
                    geo: false,
                    hydro: false,
                    physical: false,
                }
            }
        },
        ItemConfig {
            name: "nightsouls_blessing",
            title: locale!(zh_cn: "夜魂加持状态", en: "Nightsouls Blessing"),
            config: ItemConfigType::Bool { default: false },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (elements, nightsouls_blessing) = match *b {
            BuffConfig::ScrollOfTheHeroOfCinderCity4 { elements, nightsouls_blessing } => (elements, nightsouls_blessing),
            _ => (ConfigElements8Multi { pyro: false, electro: false, dendro: false, cryo: false, anemo: false, geo: false, hydro: false, physical: false }, false)
        };
        Box::new(BuffScrollOfTheHeroOfCinderCity4 { elements, nightsouls_blessing })
    }
}