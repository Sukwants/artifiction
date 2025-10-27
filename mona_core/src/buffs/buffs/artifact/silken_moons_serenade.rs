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

pub struct BuffSilkenMoonsSerenade4 {
    pub moonsign: Moonsign,
    pub gleaming_moon_effect_count: usize,
}

impl<A: Attribute> Buff<A> for BuffSilkenMoonsSerenade4 {
    fn change_attribute(&self, attribute: &mut A) {
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

impl BuffMeta for BuffSilkenMoonsSerenade4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SilkenMoonsSerenade4,
        name_locale: crate::common::i18n::locale!(
            zh_cn: "纺月的夜歌4",
            en: "Silken Moons Serenade 4",
        ),
        image: BuffImage::Artifact(ArtifactSetName::SilkenMoonsSerenade),
        genre: BuffGenre::Artifact,
        description: Some(crate::common::i18n::locale!(
            zh_cn: "造成元素伤害时，获得持续8秒的「月辉明光·崇信」效果：队伍的月兆为初辉/满辉时，队伍中的所有角色的元素精通提高60点/120点。装备者处于后台时也能触发上述效果。队伍中的角色每拥有一种不同的「月辉明光」效果，队伍中的所有角色触发的月曜反应造成的伤害提升10%。由「月辉明光」产生的效果无法叠加。",
            en: "When dealing Elemental DMG, gain the Gleaming Moon: Devotion effect for 8s: Increases all party members' Elemental Mastery by 60/120 when the party's Moonsign is Nascent Gleam/Ascendant Gleam. The equipping character can trigger this effect while off-field. All party members' Lunar Reaction DMG is increased by 10% for each different Gleaming Moon effect that party members have. Effects from Gleaming Moon cannot stack.",
        )),
        from: BuffFrom::Artifact(ArtifactSetName::SilkenMoonsSerenade)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN3,
        ItemConfig {
            name: "gleaming_moon_effect_count",
            title: locale!(zh_cn: "「月辉明光」数量（如效果重复请填入 0）", en: "Gleaming Moon Effect Count (Enter 0 if effects are duplicated)"),
            config: ItemConfigType::Int { min: 0, max: 2, default: 1 },
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (moonsign, gleaming_moon_effect_count) = match *b {
            BuffConfig::SilkenMoonsSerenade4 { moonsign, gleaming_moon_effect_count } => (moonsign, gleaming_moon_effect_count),
            _ => (Moonsign::None, 0)
        };
        Box::new(BuffSilkenMoonsSerenade4 { moonsign, gleaming_moon_effect_count })
    }
}