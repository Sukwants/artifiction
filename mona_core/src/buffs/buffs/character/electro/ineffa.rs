use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffIneffa {
    pub atk: f64,
    pub has_p2: bool,
    pub has_c1: bool,
}

impl<A: Attribute> Buff<A> for BuffIneffa {
    fn change_attribute(&self, attribute: &mut A) {

        if self.has_p2 {
            attribute.set_value_by(AttributeName::ElementalMastery, "伊涅芙「全相重构协议」", self.atk * 0.06);
        }

        attribute.set_value_by(AttributeName::IncreaseLunarCharged, "伊涅芙「月兆祝赐·象拟中继」", (self.atk * 0.00007).min(0.14));

        if self.has_c1 {
            attribute.set_value_by(AttributeName::EnhanceLunarCharged, "伊涅芙「载流复合」", (self.atk * 0.00025).min(0.5));
        }
    }
}

impl BuffMeta for BuffIneffa {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::Ineffa,
        name_locale: locale!(
            zh_cn: "伊涅芙-「轰隆雷鸣波」",
            en: "Ineffa-Boom Boom Thunderwave"
        ),
        image: BuffImage::Avatar(CharacterName::Ineffa),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "天赋 - 全相重构协议：施放元素爆发至高律令·全域扫灭时，将为队伍中自己的所有角色赋予「参数重构」效果：基于伊涅芙攻击力的6%，提升伊涅芙与队伍中自己当前场上角色的元素精通，持续20秒。\
                <br>天赋 - 月兆祝赐·象拟中继：队伍中的角色触发感电反应时，将转为触发月感电反应，且基于伊涅芙的攻击力，提升月感电反应的基础伤害：每100点攻击力都将提升0.7%基础伤害，至多通过这种方式提升14%伤害。\
                <br>C1：伊涅芙展开光流屏障护盾时，将为队伍中附近的所有角色赋予持续20秒的「载流复合」效果，使月感电反应造成的伤害提升，提升值基于伊涅芙的攻击力：每100点攻击力都将提升2.5%伤害，至多通过这种方式提升50%伤害。",
            en: "Talent - Panoramic Permutation Protocol: When using the Elemental Burst Supreme Instruction: Cyclonic Exterminator, all your party members will gain the Parameter Permutation effect: Increases Ineffa and your own active party member's Elemental Mastery by 6% of Ineffa's ATK for 20s.\
                <br>Talent - Moonsign Benediction: Assemblage Hub: When a party member triggers an Electro-Charged reaction, it will be converted into the Lunar-Charged reaction, with every 100 ATK that Ineffa has increasing Lunar-Charged's Base DMG by 0.7%, up to a maximum of 14%.\
                <br>C1: When Ineffa activates her Optical Flow Shield Barrier, all nearby party members will gain the Carrier Flow Composite effect for 20s: Increases Lunar-Charged DMG by 2.5% for every 100 ATK that Ineffa has, up to a maximum of 50%."
        )),
        from: BuffFrom::Character(CharacterName::Ineffa),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "atk",
            title: locale!(
                zh_cn: "攻击力",
                en: "Attack"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
        ItemConfig {
            name: "has_p2",
            title: locale!(
                zh_cn: "天赋：全相重构协议",
                en: "Talent: Panoramic Permutation Protocol"
            ),
            config: ItemConfigType::Bool { default: false }
        },
        ItemConfig {
            name: "has_c1",
            title: locale!(
                zh_cn: "C1",
                en: "C1"
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (atk, has_p2, has_c1) = match *b {
            BuffConfig::Ineffa { atk, has_p2, has_c1 } => (atk, has_p2, has_c1),
            _ => (0.0, false, false)
        };
        Box::new(BuffIneffa {
            atk,
            has_p2,
            has_c1
        })
    }
}
