use crate::buffs::buffs::prelude::*;

pub struct BuffBeidouC6 {
    pub stellar_glimmer_state: StellarGlimmerState,
}

impl<A: Attribute> Buff<A> for BuffBeidouC6 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_to_t(
            AttributeType::Invisible(InvisibleAttributeType::new_element(
                AttributeVariableType::ResMinus,
                Element::Electro,
            )),
            "北斗命座6",
            0.15,
        );

        if self.stellar_glimmer_state.is_stellar_conduct() {
            attribute.set_value_to_t(
                AttributeType::Invisible(InvisibleAttributeType::new_element(
                    AttributeVariableType::ResMinus,
                    Element::Cryo,
                )),
                "北斗命座6",
                0.15,
            );
            attribute.set_value_to(AttributeName::ElementalMastery, "北斗命座6", 200.0);
        }
    }
}

impl BuffMeta for BuffBeidouC6 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::BeidouC6,
        name_locale: locale!(
            zh_cn: "北斗-「北斗祓幽孽」",
            en: "Beidou-「Bane of Evil」",
        ),
        image: BuffImage::Avatar(CharacterName::Beidou),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "北斗命座6：斫雷持续期间，周围敌人的雷元素抗性降低15%。\
                <br>辉映·星超导：斫雷持续期间，附近敌人的冰元素抗性还会降低15%，且当前场上角色元素精通提升200点。",
            en: "Beidou C6: During Stormbreaker, opponents' Electro RES is decreased by 15%.\
                <br>Glimmering Stellar-Conduct: During Stormbreaker, opponents' Cryo RES will also be decreased by 15%, and the current active character gains 200 Elemental Mastery.",
        )),
        from: BuffFrom::Character(CharacterName::Beidou),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STELLAR_GLIMMER_STATE(StellarGlimmerState::None, ItemConfig::PRIORITY_BUFF),
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let stellar_glimmer_state = match *b {
            BuffConfig::BeidouC6 { stellar_glimmer_state } => stellar_glimmer_state,
            _ => StellarGlimmerState::None,
        };
        Box::new(BuffBeidouC6 {
            stellar_glimmer_state,
        })
    }
}
