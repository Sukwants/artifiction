use crate::artifacts::effects::prelude::*;

pub struct AubadeOfMorningstarAndMoonEffect {
    pub moonsign: Moonsign,
    pub set4_rate: f64,
}

impl<A: Attribute> ArtifactEffect<A> for AubadeOfMorningstarAndMoonEffect {
    fn effect2(&self, attribute: &mut A) {
        attribute.set_value_by(AttributeName::ElementalMastery, "晨星与月的晓歌2", 80.0);
    }

    fn effect4(&self, attribute: &mut A) {
        let val = if self.moonsign.is_ascendant() { 0.60 } else { 0.20 };
        for reaction in [ReactionType::LunarCharged, ReactionType::LunarBloom, ReactionType::LunarCrystallize] {
            attribute.set_value_by_s(CharacterSelector::select_self_offfield(attribute),
                AttributeType::Invisible(InvisibleAttributeType::new_reaction(AttributeVariableType::ReactionEnhance, reaction)),
                "晨星与月的晓歌4", val);
        }
    }
}

pub struct AubadeOfMorningstarAndMoon;

impl ArtifactTrait for AubadeOfMorningstarAndMoon {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(AubadeOfMorningstarAndMoonEffect {
            moonsign: config.config_aubade_of_morningstar_and_moon.moonsign,
            set4_rate: config.config_aubade_of_morningstar_and_moon.set4_rate,
        })
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::AubadeOfMorningstarAndMoon,
        name_mona: "AubadeOfMorningstarAndMoon",
        name_locale: locale!(zh_cn: "晨星与月的晓歌", en: "Aubade of Morningstar and Moon"),
        flower: Some(locale!(zh_cn: "献与月的华梦", en: "Moonlit Offering's Opulent Dream")),
        feather: Some(locale!(zh_cn: "献与月的离光", en: "Moonlit Offering's Parting Light")),
        sand: Some(locale!(zh_cn: "献与月的终时", en: "Moonlit Offering's Final Hour")),
        goblet: Some(locale!(zh_cn: "献与月的酹祭", en: "Moonlit Offering's Libation")),
        head: Some(locale!(zh_cn: "献与月的银冕", en: "Moonlit Offering's Silver Crown")),
        star: (4, 5),
        effect1: None,
        effect2: Some(locale!(
            zh_cn: "元素精通提高80点。",
            en: "Increases Elemental Mastery by 80."
        )),
        effect3: None,
        effect4: Some(locale!(
            zh_cn: "装备者处于队伍后台时，造成的月曜反应伤害提升20%；队伍的月兆等级至少为满辉时，造成的月曜反应伤害进一步提升40%。上述效果将在装备者位于场上3秒后移除。",
            en: "When the equipping character is off-field, Lunar Reaction DMG is increased by 20%. When the party's Moonsign Level is at least Ascendant Gleam, Lunar Reaction DMG will be further increased by 40%. This effect will disappear after the equipping character is active for 3s."
        )),
        effect5: None,
        internal_id: 15043
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN_GLOBAL(Moonsign::None, ItemConfig::PRIORITY_ARTIFACT),
        ItemConfig {
            name: "set4_rate",
            title: locale!(
                zh_cn: "四件套被动比例",
                en: "4-Set Ratio",
            ),
            config: ItemConfigType::GlobalLinkFloat { min: 0.0, max: 1.0, default: 0.0, 
                global_link: GlobalLinkConfig { key: "[aubade_of_morningstar_and_moon]set4_rate", priority: ItemConfig::PRIORITY_ARTIFACT, team_shared: false } 
            }
        },
    ]);
}
