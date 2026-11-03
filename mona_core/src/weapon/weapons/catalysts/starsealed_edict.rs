// Genertaed by GitHub Copilot (DeepSeek V4.1 Flash 1017)
use crate::weapon::weapons::prelude::*;
use crate::common::StellarGlimmerState;

pub struct StarsealedEdictEffect {
    pub rate: f64,
    pub stack: f64,
    pub stellar_glimmer_state: StellarGlimmerState,
}

impl<A: Attribute> WeaponEffect<A> for StarsealedEdictEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;

        // 暴击伤害提升。
        attribute.set_value_to(AttributeName::CriticalDamageBase, "秘星典谕被动", 0.18 + 0.06 * refine);

        // 「晶质调合」：施放元素战技时获得，持续 20 秒，以配置项「覆盖率」近似其生效时间占比。
        // 「炬中法度」：处于「晶质调合」期间，队伍中附近的角色触发元素反应时获得一层，
        // 层数取决于队伍触发反应的频率，以配置项「等效层数」近似其生效期间的平均层数。
        if self.rate > 0.0 {
            let elemental_mastery = (36.0 + 12.0 * refine + (18.0 + 6.0 * refine) * self.stack) * self.rate;
            attribute.set_value_to(AttributeName::ElementalMastery, "秘星典谕被动", elemental_mastery);

            // 辉映·星超导：每层「炬中法度」使装备者造成的星超导反应伤害的暴击伤害提升。
            if self.stellar_glimmer_state.is_stellar_conduct() && self.stack > 0.0 {
                attribute.set_value_to_t(
                    AttributeType::Invisible(InvisibleAttributeType::new_reaction(
                        AttributeVariableType::CriticalDamage,
                        ReactionType::StellarConduct,
                    )),
                    "秘星典谕被动",
                    (0.09 + 0.03 * refine) * self.stack * self.rate,
                );
            }
        }
    }
}

pub struct StarsealedEdict;

impl WeaponTrait for StarsealedEdict {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::StarsealedEdict,
        internal_name: "Catalyst_Chernaya",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some(crate::common::i18n::locale!(
            zh_cn: "暴击伤害提升 <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> 。施放元素战技时，装备者获得「晶质调合」效果：元素精通提升 <span style=\"color: #409EFF;\">48-60-72-84-96</span> 点，持续20秒；持续期间，队伍中附近的角色触发元素反应时，装备者还会获得一层「炬中法度」，使元素精通进一步提升 <span style=\"color: #409EFF;\">24-30-36-42-48</span> 点，该效果至多叠加3层，持续至「晶质调合」效果结束或再次施放元素战技。<br>辉映·星超导：每层「炬中法度」还会使装备者造成的星超导反应伤害的暴击伤害提升 <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> 。<br>装备者处于队伍后台时，依然能触发上述效果。",
            en: "Increases CRIT DMG by <span style=\"color: #409EFF;\">24%-30%-36%-42%-48%</span> . The equipping character gains the Crystal Compounding effect when they use their Elemental Skill, increasing Elemental Mastery by <span style=\"color: #409EFF;\">48-60-72-84-96</span> for 20s. During this time, the equipping character gains a stack of Law of Combustion when a nearby party member triggers an Elemental Reaction. Each stack increases Elemental Mastery by an additional <span style=\"color: #409EFF;\">24-30-36-42-48</span> . Max 3 stacks. These stacks last until Crystal Compounding ends or the equipping character uses their Elemental Skill again.<br>Radiance: Stellar-Conduct: Each Law of Combustion stack also increases the equipping character's Stellar-Conduct CRIT DMG by <span style=\"color: #409EFF;\">12%-15%-18%-21%-24%</span> .<br>The aforementioned effects can still trigger even when the equipping character is not on the field."
        )),
        #[cfg(not(target_family = "wasm"))]
        name_locale: crate::common::i18n::locale!(
            zh_cn: "秘星典谕",
            en: "Starsealed Edict"
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: locale!(zh_cn: "「晶质调合」覆盖率", en: "\"Crystal Compounding\" Uptime Ratio"),
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 },
        },
        ItemConfig {
            name: "stack",
            title: locale!(zh_cn: "「炬中法度」等效层数", en: "Avg \"Law of Combustion\" Stacks"),
            config: ItemConfigType::Float { min: 0.0, max: 3.0, default: 3.0 },
        },
        ItemConfig::STELLAR_GLIMMER_STATE(StellarGlimmerState::None, ItemConfig::PRIORITY_WEAPON),
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let (rate, stack, stellar_glimmer_state) = match *config {
            WeaponConfig::StarsealedEdict { rate, stack, stellar_glimmer_state } => (rate, stack, stellar_glimmer_state),
            _ => (0.0, 0.0, StellarGlimmerState::None),
        };

        Some(Box::new(StarsealedEdictEffect {
            rate,
            stack,
            stellar_glimmer_state,
        }))
    }
}
