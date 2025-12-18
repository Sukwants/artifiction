use crate::attribute::*;
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffIfa {
    pub re_count: usize,
}

impl<A: Attribute> Buff<A> for BuffIfa {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::EnhanceSwirlCryo, "伊法「救援要义」", self.re_count as f64 * 0.015);
        attribute.set_value_by(AttributeName::EnhanceSwirlPyro, "伊法「救援要义」", self.re_count as f64 * 0.015);
        attribute.set_value_by(AttributeName::EnhanceSwirlHydro, "伊法「救援要义」", self.re_count as f64 * 0.015);
        attribute.set_value_by(AttributeName::EnhanceSwirlElectro, "伊法「救援要义」", self.re_count as f64 * 0.015);
        attribute.set_value_by(AttributeName::EnhanceElectroCharged, "伊法「救援要义」", self.re_count as f64 * 0.015);
        attribute.set_value_by(AttributeName::EnhanceLunarCharged, "伊法「救援要义」", self.re_count as f64 * 0.002);
    }
}

impl BuffMeta for BuffIfa {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::Ifa,
        name_locale: locale!(
            zh_cn: "伊法-「救援要义」",
            en: "Ifa-Rescue Essentials"
        ),
        image: BuffImage::Avatar(CharacterName::Ifa),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "救援要义：上限为150点。基于伊法持有的救援要义，每1点救援要义会使队伍中附近的角色触发的扩散反应与感电反应造成的伤害提升1.5%，月感电反应造成的伤害提升0.2%。救援要义将在伊法的夜魂加持结束时被移除。\
                <br>伊法天赋 1：场中医者视野：伊法处于夜魂加持状态下时，将基于队伍中所有角色当前夜魂值的总和，每1点夜魂值都将使伊法获得1点「救援要义」。救援要义可以提升队伍中附近的角色触发的扩散、感电、月感电反应造成的伤害。\
                <br>伊法命座 2：祈祷弹道的助灵：伊法处于夜魂加持状态下时，基于队伍中所有角色当前夜魂值的总和超过60点的部分，每1点夜魂值都将使伊法额外获得4点「救援要义」。此外，伊法持有「救援要义」的上限提升50点。",
            en: "Rescue Essentials: Max 150 points. Each Rescue Essentials point Ifa has will increase the Swirl and Electro-Charged DMG dealt by nearby party members by 1.5%, and their Lunar-Charged DMG dealt by 0.2%. These points will be cleared when his Nightsoul's Blessing state ends.\
                <br>Ifa Talent 1: Field Medic's Vision: When Ifa is in the Nightsoul's Blessing state, every 1 Nightsoul point out of the total in his entire party will grant him 1 Rescue Essentials point. Rescue Essentials will increase the Swirl, Electro-Charged, and Lunar-Charged DMG dealt by nearby party members.\
                <br>Ifa C2: Guiding Spirit of Ballistic Prayer: When Ifa is in the Nightsoul's Blessing state, every Nightsoul point the party has above a total of 60 will grant Ifa 4 additional Rescue Essentials points. Additionally, Ifa's Rescue Essentials limit is increased by 50."
        )),
        from: BuffFrom::Character(CharacterName::Ifa),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "re_count",
            title: locale!(
                zh_cn: "「救援要义」数量",
                en: "Rescue Essentials Count"
            ),
            config: ItemConfigType::Int { min: 0, max: 200, default: 0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let re_count = match *b {
            BuffConfig::Ifa { re_count } => re_count,
            _ => 0
        };
        Box::new(BuffIfa {
            re_count,
        })
    }
}
