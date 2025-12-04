use crate::artifacts::ArtifactSetName;
use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
#[cfg(not(target_family = "wasm"))]
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Moonsign;
use crate::enemies::Enemy;

pub struct BuffNightOfTheSkysUnveiling4 {
    pub gleaming_moon_effect_count: usize,
}

impl<A: Attribute> Buff<A> for BuffNightOfTheSkysUnveiling4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceMoonglare, "BUFF: 穹境示现之夜4", 0.1 * self.gleaming_moon_effect_count as f64);
    }
}

impl BuffMeta for BuffNightOfTheSkysUnveiling4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::NightOfTheSkysUnveiling4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "穹境示现之夜4",
            en: "Night Of The Sky's Unveiling 4",
        ),
        image: BuffImage::Artifact(ArtifactSetName::NightOfTheSkysUnveiling),
        genre: BuffGenre::Artifact,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "队伍中附近的角色触发月曜反应时，若装备者在场上，将获得持续4秒的「月辉明光·蓄念」效果：队伍的月兆为初辉/满辉时，暴击率提升15%/30%。队伍中的角色每拥有一种不同的「月辉明光」效果，队伍中的所有角色触发的月曜反应造成的伤害提升10%。由「月辉明光」产生的效果无法叠加。",
            en: "When nearby party members trigger Lunar Reactions, if the equipping character is on the field, gain the Gleaming Moon: Intent effect for 4s: Increases CRIT Rate by 15%/30% when the party's Moonsign is Nascent Gleam/Ascendant Gleam. All party members' Lunar Reaction DMG is increased by 10% for each different Gleaming Moon effect that party members have. Effects from Gleaming Moon cannot stack."
        )),
        from: BuffFrom::Artifact(ArtifactSetName::NightOfTheSkysUnveiling)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "gleaming_moon_effect_count",
            title: locale!(zh_cn: "「月辉明光」数量（如效果重复请填入 0）", en: "Gleaming Moon Effect Count (Enter 0 if effects are duplicated)"),
            config: ItemConfigType::Int { min: 0, max: 2, default: 1 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let  gleaming_moon_effect_count = match *b {
            BuffConfig::NightOfTheSkysUnveiling4 { gleaming_moon_effect_count } => gleaming_moon_effect_count,
            _ => 0
        };
        Box::new(BuffNightOfTheSkysUnveiling4 { gleaming_moon_effect_count })
    }
}