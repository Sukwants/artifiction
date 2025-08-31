use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Moonsign;

pub struct NightOfTheSkysUnveilingEffect {
    pub moonsign: Moonsign,
    pub gleaming_moon_effect_count: usize,
}

impl<A: Attribute> ArtifactEffect<A> for NightOfTheSkysUnveilingEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "穹境示现之夜2", 80.0);
    }

    fn effect4(&self, attribute: &mut A) {
        match self.moonsign {
            Moonsign::Nascent => {
                attribute.set_value_by(AttributeName::CriticalBase, "穹境示现之夜4", 0.15);
            }
            Moonsign::Ascendant => {
                attribute.set_value_by(AttributeName::CriticalBase, "穹境示现之夜4", 0.30);
            }
            _ => {}
        }

        attribute.set_value_by(AttributeName::EnhanceMoonglare, "穹境示现之夜4", 0.1 * self.gleaming_moon_effect_count as f64);
    }
}

pub struct NightOfTheSkysUnveiling;

impl ArtifactTrait for NightOfTheSkysUnveiling {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(NightOfTheSkysUnveilingEffect {
            moonsign: match config.config_night_of_the_skys_unveiling.moonsign {
                0 => Moonsign::None,
                1 => Moonsign::Nascent,
                2 => Moonsign::Ascendant,
                _ => Moonsign::None,
            },
            gleaming_moon_effect_count: config.config_night_of_the_skys_unveiling.gleaming_moon_effect_count,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::NightOfTheSkysUnveiling,
        name_mona: "NightOfTheSkysUnveiling",
        name_locale: locale!(zh_cn: "穹境示现之夜", en: "Night of the Sky's Unveiling"),
        flower: Some(locale!(zh_cn: "渴真之花", en: "Bloom of the Mind's Desire")),
        feather: Some(locale!(zh_cn: "深罪之羽", en: "Feather of Indelible Sin")),
        sand: Some(locale!(zh_cn: "谕告之钟", en: "Revelation's Toll")),
        goblet: Some(locale!(zh_cn: "满溢之壶", en: "Vessel of Plenty")),
        head: Some(locale!(zh_cn: "永劫之冕", en: "Crown of the Befallen")),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "元素精通提高80点。",
            en: "Increases Elemental Mastery by 80."
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "队伍中附近的角色触发月曜反应时，若装备者在场上，将获得持续4秒的「月辉明光·蓄念」效果：队伍的月兆为初辉/满辉时，暴击率提升15%/30%。队伍中的角色每拥有一种不同的「月辉明光」效果，队伍中的所有角色触发的月曜反应造成的伤害提升10%。由「月辉明光」产生的效果无法叠加。",
            en: "When nearby party members trigger Lunar Reactions, if the equipping character is on the field, gain the Gleaming Moon: Intent effect for 4s: Increases CRIT Rate by 15%/30% when the party's Moonsign is Nascent Gleam/Ascendant Gleam. All party members' Lunar Reaction DMG is increased by 10% for each different Gleaming Moon effect that party members have. Effects from Gleaming Moon cannot stack."
        )),
        effect5: None,
        internal_id: 15041
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "moonsign",
            title: locale!(
                zh_cn: "月兆",
                en: "Moonsign",
            ),
            config: ItemConfigType::Option {
                options: "无 None,初辉 Nascent Gleam,满辉 Ascendant Gleam",
                default: 0
            }
        },
        ItemConfig {
            name: "gleaming_moon_effect_count",
            title: locale!(zh_cn: "「月辉明光」数量", en: "Gleaming Moon Effect Count"),
            config: ItemConfigType::Int { min: 1, max: 2, default: 1 },
        }
    ]);
}
