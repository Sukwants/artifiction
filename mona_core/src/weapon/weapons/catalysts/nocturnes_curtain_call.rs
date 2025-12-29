use crate::weapon::weapons::prelude::*;

pub struct NocturnesCurtainCallEffect {
    pub effect: bool,
}

impl<A: Attribute> WeaponEffect<A> for NocturnesCurtainCallEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        attribute.add_hp_percentage("帷间夜曲被动", 0.08 + 0.02 * refine);

        if self.effect {
            attribute.add_hp_percentage("帷间夜曲被动", 0.12 + 0.02 * refine);

            for reaction in [ReactionType::LunarCharged, ReactionType::LunarBloom, ReactionType::LunarCrystallize] {
                attribute.set_value_by_t(AttributeType::Invisible(InvisibleAttributeType::new(
                    AttributeVariableType::CriticalDamage,
                    None, None, Some(reaction)
                )), "帷间夜曲被动", 0.40 + 0.20 * refine);
            }
        }

    }
}

pub struct NocturnesCurtainCall;

impl WeaponTrait for NocturnesCurtainCall {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::NocturnesCurtainCall,
        internal_name: "Catalyst_NocturnesCurtainCall",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "生命值上限提高 <span style=\"color: #409EFF;\">10%-12%-14%-16%-18%</span> 。装备者触发月曜反应或对敌人造成月曜反应伤害时，将为装备者恢复 <span style=\"color: #409EFF;\">14-15-16-17-18</span> 点元素能量，并获得持续12秒的「丰饶海的神酒」效果：生命值上限进一步提高 <span style=\"color: #409EFF;\">14%-16%-18%-20%-22%</span> ，月曜反应伤害的暴击伤害提升 <span style=\"color: #409EFF;\">60%-80%-100%-120%-140%</span> 。恢复元素能量效果每18秒至多触发一次，装备者处于队伍后台时，依然能触发上述效果。",
            en: "Max HP increases by <span style=\"color: #409EFF;\">10%-12%-14%-16%-18%</span> . After triggering Lunar reactions or inflicting Lunar Reaction DMG, the equipping character will recover <span style=\"color: #409EFF;\">12-13-14-15-16</span> Energy, and receive the Bountiful Sea's Sacred Wine effect for 12s: Max HP increases by an additional <span style=\"color: #409EFF;\">14%-16%-18%-20%-22%</span> , CRIT DMG from Lunar Reaction DMG increases by <span style=\"color: #409EFF;\">60%-80%-100%-120%-140%</span> . The Energy recovery effect can be triggered at most once every 18s, and can be triggered even when the equipping character is off-field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "帷间夜曲",
            en: "Nocturne's Curtain Call"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "effect",
            title: locale!(
                zh_cn: "「丰饶海的神酒」",
                en: "the Bountiful Sea's Sacred Wine",
            ),
            config: ItemConfigType::Bool {  default: false },
        },
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let effect = match *config {
            WeaponConfig::NocturnesCurtainCall { effect } => effect,
            _ => false
        };

        Some(Box::new(NocturnesCurtainCallEffect {
            effect,
        }))
    }
}