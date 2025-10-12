use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use crate::artifacts::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::Moonsign;

pub struct SilkenMoonsSerenadeEffect {
    pub moonsign: Moonsign,
    pub gleaming_moon_effect_count: usize,
}

impl<A: Attribute> ArtifactEffect<A> for SilkenMoonsSerenadeEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::Recharge, "纺月的夜歌2", 0.2);
    }

    fn effect4(&self, attribute: &mut A) {
        match self.moonsign {
            Moonsign::Nascent => {
                attribute.set_value_by(AttributeName::ElementalMastery, "纺月的夜歌4", 60.0);
            }
            Moonsign::Ascendant => {
                attribute.set_value_by(AttributeName::ElementalMastery, "纺月的夜歌4", 120.0);
            }
            _ => {}
        }
        
        attribute.set_value_by(AttributeName::EnhanceMoonglare, "BUFF: 纺月的夜歌4", 0.1 * self.gleaming_moon_effect_count as f64);
    }
}

pub struct SilkenMoonsSerenade;

impl ArtifactTrait for SilkenMoonsSerenade {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(SilkenMoonsSerenadeEffect {
            moonsign: config.config_silken_moons_serenade.moonsign,
            gleaming_moon_effect_count: config.config_silken_moons_serenade.gleaming_moon_effect_count,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::SilkenMoonsSerenade,
        name_mona: "SilkenMoonsSerenade",
        name_locale: locale!(zh_cn: "纺月的夜歌", en: "Silken Moon's Serenade"),
        flower: Some(locale!(zh_cn: "流离者的晶泪", en: "Crystal Tear of the Wanderer")),
        feather: Some(locale!(zh_cn: "受福者的白羽", en: "Pristine Plume of the Blessed")),
        sand: Some(locale!(zh_cn: "祭霜者的迷狂", en: "Frost Devotee's Delirium")),
        goblet: Some(locale!(zh_cn: "至纯者的欢荣", en: "Joyous Glory of the Pure")),
        head: Some(locale!(zh_cn: "司信者的圣冕", en: "Holy Crown of the Believer")),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "元素充能效率提高20%。",
            en: "Energy Recharge +20%."
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "造成元素伤害时，获得持续8秒的「月辉明光·崇信」效果：队伍的月兆为初辉/满辉时，队伍中的所有角色的元素精通提高60点/120点。装备者处于后台时也能触发上述效果。队伍中的角色每拥有一种不同的「月辉明光」效果，队伍中的所有角色触发的月曜反应造成的伤害提升10%。由「月辉明光」产生的效果无法叠加。",
            en: "When dealing Elemental DMG, gain the Gleaming Moon: Devotion effect for 8s: Increases all party members' Elemental Mastery by 60/120 when the party's Moonsign is Nascent Gleam/Ascendant Gleam. The equipping character can trigger this effect while off-field. All party members' Lunar Reaction DMG is increased by 10% for each different Gleaming Moon effect that party members have. Effects from Gleaming Moon cannot stack."
        )),
        effect5: None,
        internal_id: 15042
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN3,
        ItemConfig {
            name: "gleaming_moon_effect_count",
            title: locale!(zh_cn: "「月辉明光」数量", en: "Gleaming Moon Effect Count"),
            config: ItemConfigType::Int { min: 0, max: 2, default: 1 },
        }
    ]);
}
