use crate::buffs::buffs::prelude::*;
use crate::character::characters::illuga::ILLUGA_SKILL;

pub struct BuffIllugaQ {
    pub em: f64,
    pub level_q: usize,
    pub hydro_geo_count: usize,
}

impl<A: Attribute> Buff<A> for BuffIllugaQ {
    fn change_attribute(&self, attribute: &mut A) {
        let extra_geo_ratio = [0.0, 0.07, 0.14, 0.24][self.hydro_geo_count.min(3)];
        let extra_lunar_ratio = [0.0, 0.48, 0.96, 1.60][self.hydro_geo_count.min(3)];

        attribute.set_value_by_s(
            CharacterSelector::select_all_onfield(attribute),
            AttributeType::Invisible(InvisibleAttributeType::new_element(
                AttributeVariableType::BaseDamage,
                Element::Geo,
            )),
            "叶洛亚Q技能",
            self.em * (ILLUGA_SKILL.q_increase_geo[self.level_q - 1] + extra_geo_ratio),
        );
        attribute.set_value_by_s(
            CharacterSelector::select_all_onfield(attribute),
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::ReactionExtra,
                ReactionType::LunarCrystallize,
            )),
            "叶洛亚Q技能",
            self.em * (ILLUGA_SKILL.q_increase_lunar[self.level_q - 1] + extra_lunar_ratio),
        );
    }
}

impl BuffMeta for BuffIllugaQ {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::IllugaQ,
        name_locale: locale!(
            zh_cn: "叶洛亚-「夜莺之歌」",
            en: "Illuga-Shadowless Reflection"
        ),
        image: BuffImage::Avatar(CharacterName::Illuga),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "叶洛亚Q技能：队伍中附近的当前场上角色的普通攻击、重击、下落攻击、元素战技或元素爆发对敌人造成岩元素伤害时，将消耗1层「夜莺之歌」，提升造成的伤害，提升值基于叶洛亚的元素精通；若该伤害为月结晶反应造成的伤害，还会进一步提升造成的伤害。",
            en: "Illuga Skill Q: When Normal Attacks, Charged Attacks, Plunging Attacks, Elemental Skills, and Elemental Bursts of nearby active party members deal Geo DMG to opponents, 1 stack of Nightingale's Song is consumed to increase the DMG dealt. Increase in DMG is based on Illuga's Elemental Mastery. If DMG is inflicted by Lunar-Crystallize, DMG dealt will increase further."
        )),
        from: BuffFrom::Character(CharacterName::Illuga),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "em",
            title: locale!(
                zh_cn: "元素精通",
                en: "Elemental Mastery"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
        ItemConfig {
            name: "level_q",
            title: locale!(
                zh_cn: "技能等级",
                en: "Skill Level"
            ),
            config: ItemConfigType::Int { min: 1, max: 15, default: 10 }
        },
        ItemConfig {
            name: "hydro_geo_count",
            title: locale!(
                zh_cn: "水元素或岩元素角色数量",
                en: "Number of Hydro or Geo Characters"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 1 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (em, level_q, hydro_geo_count) = match *b {
            BuffConfig::IllugaQ { em, level_q, hydro_geo_count } => (em, level_q, hydro_geo_count),
            _ => (0.0, 0, 0)
        };
        Box::new(BuffIllugaQ {
            em,
            level_q,
            hydro_geo_count,
        })
    }
}


pub struct BuffIllugaP1 {
    pub moonsign: Moonsign,
    pub has_c6: bool,
}

impl<A: Attribute> Buff<A> for BuffIllugaP1 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_s(
            CharacterSelector::select_all_except_self(attribute),
            AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::CriticalRate, Element::Geo)),
            "叶洛亚天赋1",
            if self.has_c6 { 0.10 } else { 0.05 },
        );
        attribute.set_value_by_s(
            CharacterSelector::select_all_except_self(attribute),
            AttributeType::Invisible(InvisibleAttributeType::new_element(AttributeVariableType::CriticalDamage, Element::Geo)),
            "叶洛亚天赋1",
            if self.has_c6 { 0.30 } else { 0.10 },
        );

        if self.moonsign.is_ascendant() {
            attribute.set_value_by_s(
                CharacterSelector::select_all_except_self(attribute),
                AttributeType::Panel(AttributeName::ElementalMastery),
                "叶洛亚天赋1",
            if self.has_c6 { 80.0 } else { 50.0 },
            );
        }
    }
}

impl BuffMeta for BuffIllugaP1 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::IllugaP1,
        name_locale: locale!(
            zh_cn: "叶洛亚-「狩魔者的黄昏」",
            en: "Illuga-Moonsign Benediction: Moonlight, Lent Unto You"
        ),
        image: BuffImage::Avatar(CharacterName::Illuga),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "叶洛亚天赋1：施放元素战技衔莺破晓或元素爆发鉴照无影后，队伍中附近的其他角色将获得持续20秒的「执灯之誓」效果：对敌人造成的岩元素伤害，暴击率提升5%，暴击伤害提升10%。\
                <br>月兆·满辉：处于「执灯之誓」效果影响下的角色，元素精通提升50点。",
            en: "Illuga Talent 1: After unleashing Elemental Skill Dawnbearing Songbird or Elemental Burst Shadowless Reflection, nearby party members will gain the Lightkeeper's Oath effect for 20s: For Geo DMG dealt to opponents, CRIT Rate increases by 5% and CRIT DMG increases by 10%.\
                <br>Moonsign: Ascendant Gleam: When party members are affected by the Lightkeeper's Oath effect, Elemental Mastery is increased by 50."
        )),
        from: BuffFrom::Character(CharacterName::Illuga),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::Nascent, ItemConfig::PRIORITY_BUFF),
        ItemConfig {
            name: "has_c6",
            title: locale!(
                zh_cn: "六命",
                en: "Constellation 6"
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (moonsign, has_c6) = match *b {
            BuffConfig::IllugaP1 { moonsign, has_c6 } => (moonsign, has_c6),
            _ => (Moonsign::None, false)
        };
        Box::new(BuffIllugaP1 {
            moonsign,
            has_c6,
        })
    }
}


pub struct BuffIllugaC4 {
}

impl<A: Attribute> Buff<A> for BuffIllugaC4 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_s(
            CharacterSelector::select_all_onfield(attribute),
            AttributeType::Panel(AttributeName::DEFFixed),
            "叶洛亚命座4",
            200.0,
        );
    }
}

impl BuffMeta for BuffIllugaC4 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::IllugaC4,
        name_locale: locale!(
            zh_cn: "叶洛亚-「逐日之狼」",
            en: "Illuga-Solarhunter Wolf"
        ),
        image: BuffImage::Avatar(CharacterName::Illuga),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "叶洛亚命座4：元素爆发鉴照无影中的「魇夜的莺歌」效果持续期间，队伍中附近的当前场上角色防御力提升200点。",
            en: "Illuga C4: When Elemental Burst Shadowless Reflection's Haunted Night's Oriole-Song effect is active, nearby active party members will have their DEF increased by 200."
        )),
        from: BuffFrom::Character(CharacterName::Illuga),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        Box::new(BuffIllugaC4 {
        })
    }
}
