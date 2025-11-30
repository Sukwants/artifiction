use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::character::CharacterName;
use crate::common::i18n::locale;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};

pub struct BuffAlbedoTalent2;

impl<A: Attribute> Buff<A> for BuffAlbedoTalent2 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "阿贝多「瓶中人的天慧」", 125.0);
    }
}

impl BuffMeta for BuffAlbedoTalent2 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AlbedoTalent2,
        name_locale: locale!(
            zh_cn: "阿贝多-「瓶中人的天慧」",
            en: "Albedo-「Homuncular Nature」",
        ),
        image: BuffImage::Avatar(CharacterName::Albedo),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "阿贝多天赋2：释放诞生式·大地之潮时,使附近的队伍中角色的元素精通提高125点，持续10秒",
            en: "Albedo Talent2: Using Rite of Progeniture: Tectonic Tide increases the Elemental Mastery of nearby party members by 125 for 10s.",
        )),
        from: BuffFrom::Character(CharacterName::Albedo),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAlbedoTalent2)
    }
}

pub struct BuffAlbedoTalent3 {
    pub def: f64,
    pub is_hexerei: bool,
}

impl<A: Attribute> Buff<A> for BuffAlbedoTalent3 {
    fn change_attribute(&self, attribute: &mut A) {
        let val = if self.is_hexerei {
            (self.def / 1000.0 * 0.14).min(0.42)
        } else {
            (self.def / 1000.0 * 0.04).min(0.12)
        };
        attribute.set_value_by(AttributeName::BonusNormalAttack, "阿贝多「魔女的前夜礼·白芒之书」", val);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "阿贝多「魔女的前夜礼·白芒之书」", val);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "阿贝多「魔女的前夜礼·白芒之书」", val);
        attribute.set_value_by(AttributeName::BonusElementalSkill, "阿贝多「魔女的前夜礼·白芒之书」", val);
        attribute.set_value_by(AttributeName::BonusElementalBurst, "阿贝多「魔女的前夜礼·白芒之书」", val);
    }
}

impl BuffMeta for BuffAlbedoTalent3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AlbedoTalent3,
        name_locale: locale!(
            zh_cn: "阿贝多-「瓶中人的天慧」",
            en: "Albedo-「Homuncular Nature」",
        ),
        image: BuffImage::Avatar(CharacterName::Albedo),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "阿贝多天赋3：魔导·秘仪：\
                <br>·炼成阳华后的20秒内，基于阿贝多的防御力，提升队伍中附近的角色的普通攻击、重击、下落攻击、元素战技和元素爆发造成的伤害：每1000点防御力都将提升4%伤害，至多通过这种方式提升12%伤害。\
                <br>·炼成瑰银后的20秒内，基于阿贝多的防御力，提升队伍中附近的魔导角色的普通攻击、重击、下落攻击、元素战技和元素爆发造成的伤害：每1000点防御力都将提升10%伤害，至多通过这种方式提升30%伤害。",
            en: "Albedo Talent3: Hexerei: Secret Rite: \
                <br>· After creating a Solar Isotoma, nearby party members' Normal Attack, Charged Attack, Plunging Attack, Elemental Skill, and Elemental Burst DMG are increased by 4% for every 1,000 DEF Albedo has for 20s. The maximum increase that can be achieved this way is 12%.\
                <vr>· After generating a Silver Isotoma, nearby Hexerei party members' Normal Attack, Charged Attack, Plunging Attack, Elemental Skill, and Elemental Burst DMG are increased by 10% for every 1,000 DEF Albedo has for 20s. The maximum increase that can be achieved this way is 30%.",
        )),
        from: BuffFrom::Character(CharacterName::Albedo),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "def",
            title: locale!(
                zh_cn: "防御力",
                en: "DEF",
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
        ItemConfig {
            name: "is_hexerei",
            title: locale!(
                zh_cn: "当前角色为魔导角色",
                en: "Current character is Hexerei party",
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (def, is_hexerei) = match *_b {
            BuffConfig::AlbedoTalent3 { def, is_hexerei } => (def, is_hexerei),
            _ => (0.0, false)
        };
        Box::new(BuffAlbedoTalent3{
            def,
            is_hexerei,
        })
    }
}

pub struct BuffAlbedoC4 {
    pub silver_isotoma: bool,
}

impl<A: Attribute> Buff<A> for BuffAlbedoC4 {
    fn change_attribute(&self, attribute: &mut A) {
        if self.silver_isotoma {
            attribute.set_value_by(AttributeName::BonusPlungingAttack, "阿贝多「神性之陨」", 0.6);
        } else {
            attribute.set_value_by(AttributeName::BonusPlungingAttack, "阿贝多「神性之陨」", 0.3);
        }
    }
}

impl BuffMeta for BuffAlbedoC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AlbedoC4,
        name_locale: locale!(
            zh_cn: "阿贝多-「神性之陨」",
            en: "Albedo-Descent of Divinity",
        ),
        image: BuffImage::Avatar(CharacterName::Albedo),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "阿贝多命座4：处于阳华的领域中的队伍中当前场上角色，造成的下落攻击伤害提高30%。\
                <br>此外，队伍中附近的当前场上角色在瑰银附近跳跃时，将会摧毁瑰银，大幅提升本次跳跃的高度，并在接下来的3秒内，使该角色的下落攻击坠地冲击造成的伤害提升30%。该效果将在下落攻击坠地冲击造成伤害后的0.1秒后解除。",
            en: "Albedo C4: Active party members within the Solar Isotoma field have their Plunging Attack DMG increased by 30%. \
                <br>Additionally, when a nearby active character on the team jumps near a Silver Isotoma, the Silver Isotoma will be destroyed while greatly increasing the height of that jump, and for the next 3s, said character will deal 30% more Plunging Attack ground impact DMG. This effect is removed 0.1s after Plunging Attack ground impact DMG is dealt.",
        )),
        from: BuffFrom::Character(CharacterName::Albedo),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "silver_isotoma",
            title: locale!(
                zh_cn: "存在瑰银",
                en: "Near Silver Isotoma",
            ),
            config: ItemConfigType::Bool { default: false }
        }
    ]);

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        let silver_isotoma = match *_b {
            BuffConfig::AlbedoC4 { silver_isotoma } => silver_isotoma,
            _ => false
        };
        Box::new(BuffAlbedoC4{
            silver_isotoma,
        })
    }
}

pub struct BuffAlbedoC6;

impl<A: Attribute> Buff<A> for BuffAlbedoC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::BonusBase, "阿贝多「无垢之土」", 0.17);
    }
}

impl BuffMeta for BuffAlbedoC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::AlbedoC6,
        name_locale: locale!(
            zh_cn: "阿贝多-「无垢之土」",
            en: "Albedo-「Dust of Purification」",
        ),
        image: BuffImage::Avatar(CharacterName::Albedo),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "阿贝多命座6：处在阳华的领域中的队伍中当前场上角色，若处于结晶反应产生的护盾庇护下，造成的伤害提高17%。",
            en: "Albedo C6: Active party members within the Solar Isotoma field who are protected by a shield created by Crystallize have their DMG increased by 17%.",
        )),
        from: BuffFrom::Character(CharacterName::Albedo),
    };

    fn create<A: Attribute>(_b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffAlbedoC6)
    }
}
