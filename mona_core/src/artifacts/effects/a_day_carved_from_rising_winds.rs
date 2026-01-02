use crate::artifacts::effects::prelude::*;

pub struct ADayCarvedFromRisingWindsEffect {
    pub is_hexerei: bool,
    pub set4_rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for ADayCarvedFromRisingWindsEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.add_atk_percentage("风起之日2", 0.18);
    }

    fn effect4(&self, attribute: &mut A) {
        attribute.add_atk_percentage("风起之日4", 0.25 * self.set4_rate);

        if self.is_hexerei {
            attribute.set_value_by(AttributeName::CriticalBase, "风起之日4", 0.2 * self.set4_rate);
        }
    }
}

pub struct ADayCarvedFromRisingWinds;

impl ArtifactTrait for ADayCarvedFromRisingWinds {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(ADayCarvedFromRisingWindsEffect {
            is_hexerei: config.config_a_day_carved_from_rising_winds.is_hexerei,
            set4_rate: config.config_a_day_carved_from_rising_winds.set4_rate,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::ADayCarvedFromRisingWinds,
        name_mona: "ADayCarvedFromRisingWinds",
        name_locale: locale!(zh_cn: "风起之日", en: "A Day Carved From Rising Winds"),
        flower: Some(locale!(zh_cn: "风花的箴铭", en: "Windborne Flower's Spruchdichtung")),
        feather: Some(locale!(zh_cn: "晨光的明誓", en: "Dawn's Brilliant Oath")),
        sand: Some(locale!(zh_cn: "春律的片刻", en: "A Note in Spring's Leich")),
        goblet: Some(locale!(zh_cn: "未言的宴话", en: "Heldenepos's Unspoken Tale")),
        head: Some(locale!(zh_cn: "哀慕的恋歌", en: "Minnesang of Love and Lament")),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: " 攻击力提高18%。",
            en: "ATK +18%."
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "通攻击、重击、元素战技或元素爆发命中敌人后，将获得持续6秒的「风与牧歌的眷怜」：攻击力提高25%。若装备者已经完成了「魔女的课业」，则「风与牧歌的眷怜」将会升级为「风与牧歌的决意」，额外使通过考验的装备者的暴击率提升20%。装备者处于队伍后台时，也能触发上述效果。",
            en: "After a Normal Attack, Charged Attack, Elemental Skill or Elemental Burst hits an opponent, gain the Blessing of Pastoral Winds effect for 6s: ATK is increased by 25%. If the equipping character has completed Witch's Homework, Blessing of Pastoral Winds will be upgraded to Resolve of Pastoral Winds, which also increases the CRIT Rate of the equipping character by an additional 20%. This effect can be triggered even when the character is off-field."
        )),
        effect5: None,
        internal_id: 15044
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::IS_HEXEREI(false, ItemConfig::PRIORITY_ARTIFACT),
        ItemConfig {
            name: "set4_rate",
            title: locale!(
                zh_cn: "四件套被动比例",
                en: "4-Set Ratio",
            ),
            config: ItemConfigType::GlobalLinkFloat { min: 0.0, max: 1.0, default: 1.0, 
                global_link: GlobalLinkConfig { key: "[a_day_carved_from_rising_winds]set4_rate", priority: ItemConfig::PRIORITY_ARTIFACT, team_shared: false } 
            }
        },
    ]);
}
