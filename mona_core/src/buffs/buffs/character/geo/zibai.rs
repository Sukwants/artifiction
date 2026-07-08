use crate::buffs::buffs::prelude::*;

pub struct BuffZibaiP3 {
    pub def: f64,
}

impl<A: Attribute> Buff<A> for BuffZibaiP3 {
    fn change_attribute(&self, attribute: &mut A) {
        attribute.set_value_by_s(
            CharacterSelector::select_all(attribute),
            AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                AttributeVariableType::ElevativeBase,
                ReactionType::LunarCrystallize,
            )),
            "兹白天赋3",
            (self.def / 100.0 * 0.007).min(0.14),
        );
    }
}

impl BuffMeta for BuffZibaiP3 {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::ZibaiP3,
        name_locale: locale!(
            zh_cn: "兹白-「月兆祝赐·浮明若流」",
            en: "Zibai-Moonsign Benediction: The Coursing Sun and Moon"
        ),
        image: BuffImage::Avatar(CharacterName::Zibai),
        genre: BuffGenre::Character,
        description: Some(locale!(
            zh_cn: "兹白天赋3：队伍中的角色触发水元素结晶反应时，将转为触发月结晶反应，且基于兹白的防御力，提升队伍中角色造成的月结晶反应的基础伤害：每100点防御力都将提升0.7%月结晶反应的基础伤害，至多通过这种方式提升14%伤害。",
            en: "Zibai Talent 3: When a party member triggers a Hydro Crystallize reaction, it will be converted into the Lunar-Crystallize reaction, with every 100 DEF that Zibai has increasing Lunar-Crystallize's Base DMG by 0.7%, up to a maximum of 14%."
        )),
        from: BuffFrom::Character(CharacterName::Zibai),
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "def",
            title: locale!(
                zh_cn: "防御力",
                en: "DEF"
            ),
            config: ItemConfigType::FloatInput { default: 0.0 }
        },
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let def = match *b {
            BuffConfig::ZibaiP3 { def } => def,
            _ => 0.0
        };
        Box::new(BuffZibaiP3 {
            def,
        })
    }
}