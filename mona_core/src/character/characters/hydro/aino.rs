use num_traits::FromPrimitive;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::character::macros::{damage_enum, skill_map};
use crate::common::{ChangeAttribute, Element, MoonglareReaction, Moonsign, SkillType, WeaponType};
use crate::common::i18n::{locale, hit_n_dmg, plunging_dmg, charged_dmg};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
// use crate::target_functions::target_functions::AinoDefaultTargetFunction;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapons::moonpiercer;

pub struct AinoSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg2: [f64; 15],
    pub a_dmg3: [f64; 15],
    pub z_dmg1: [f64; 15],
    pub z_dmg2: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg1: [f64; 15],
    pub e_dmg2: [f64; 15],

    pub q_dmg: [f64; 15],

    pub c2_dmg_atk: f64,
    pub c2_dmg_em: f64,
}

pub const AINO_SKILL: AinoSkillType = AinoSkillType {
    // Normal Attack: Bish-Bash-Bosh Repair
    a_dmg1: [0.664986, 0.719113, 0.77324, 0.850564, 0.904691, 0.96655, 1.051606, 1.136663, 1.221719, 1.314508, 1.407297, 1.500086, 1.592874, 1.685663, 1.778452],
    a_dmg2: [0.661916, 0.715793, 0.76967, 0.846637, 0.900514, 0.962087, 1.046751, 1.131415, 1.216079, 1.308439, 1.400799, 1.49316, 1.58552, 1.677881, 1.770241],
    a_dmg3: [0.492165, 0.532225, 0.572285, 0.629514, 0.669573, 0.715356, 0.778308, 0.841259, 0.90421, 0.972885, 1.041559, 1.110233, 1.178907, 1.247581, 1.316256],
    z_dmg1: [0.62522, 0.67611, 0.727, 0.7997, 0.85059, 0.90875, 0.98872, 1.06869, 1.14866, 1.2359, 1.32314, 1.41038, 1.49762, 1.58486, 1.6721],
    z_dmg2: [1.1309, 1.22295, 1.315, 1.4465, 1.53855, 1.64375, 1.7884, 1.93305, 2.0777, 2.2355, 2.3933, 2.5511, 2.7089, 2.8667, 3.0245],
    x_dmg1: [0.745878, 0.806589, 0.8673, 0.95403, 1.014741, 1.084125, 1.179528, 1.274931, 1.370334, 1.47441, 1.578486, 1.682562, 1.786638, 1.890714, 1.99479],
    x_dmg2: [1.49144, 1.612836, 1.734233, 1.907656, 2.029052, 2.167791, 2.358556, 2.549322, 2.740087, 2.948195, 3.156303, 3.364411, 3.572519, 3.780627, 3.988735],
    x_dmg3: [1.862889, 2.01452, 2.16615, 2.382765, 2.534396, 2.707688, 2.945964, 3.184241, 3.422517, 3.682455, 3.942393, 4.202331, 4.462269, 4.722207, 4.982145],

    // Elemental Skill: Musecatcher
    e_dmg1: [0.656, 0.7052, 0.7544, 0.82, 0.8692, 0.9184, 0.984, 1.0496, 1.1152, 1.1808, 1.2464, 1.312, 1.394, 1.476, 1.558],
    e_dmg2: [1.888, 2.0296, 2.1712, 2.36, 2.5016, 2.6432, 2.832, 3.0208, 3.2096, 3.3984, 3.5872, 3.776, 4.012, 4.248, 4.484],

    // Elemental Burst: Runo: All Hearts Become the Beating Moon
    q_dmg: [0.20112, 0.216204, 0.231288, 0.2514, 0.266484, 0.281568, 0.30168, 0.321792, 0.341904, 0.362016, 0.382128, 0.40224, 0.42738, 0.45252, 0.47766],

    c2_dmg_atk: 0.25,
    c2_dmg_em: 1.0,
};

pub const AINO_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Aino,
    internal_name: "Aino",
    element: Element::Hydro,
    hp: [939, 2413, 3114, 4665, 5163, 5939, 6604, 7379, 7878, 8653, 9151, 9927, 10425, 11201, 11976],
    atk: [20, 52, 67, 101, 112, 128, 143, 160, 170, 187, 198, 215, 225, 242, 304],
    def: [51, 131, 169, 253, 280, 322, 358, 400, 427, 469, 496, 538, 565, 607, 649],
    sub_stat: CharacterSubStatFamily::ElementalMastery96,
    weapon_type: WeaponType::Claymore,
    star: 4,
    skill_name1: locale!(
        zh_cn: "敲打修理法",
        en: "Bish-Bash-Bosh Repair",
    ),
    skill_name2: locale!(
        zh_cn: "妙思捕手",
        en: "Musecatcher",
    ),
    skill_name3: locale!(
        zh_cn: "精密水冷仪",
        en: "Precision Hydronic Cooler",
    ),
    name_locale: locale!(
        zh_cn: "爱诺",
        en: "Aino",
    )
};

pub struct AinoEffect {
    pub has_p1: bool,
    pub has_p2: bool,
    pub has_c1: bool,
    pub has_c6: bool,
    pub moonsign: Moonsign,
}

impl<A: Attribute> ChangeAttribute<A> for AinoEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.has_p2 {
            attribute.add_edge1(
                AttributeName::ElementalMastery,
                AttributeName::ExtraDmgElementalBurst,
                Box::new(move |em, _| {
                    em * 0.5
                }),
                Box::new(move |em, _, grad| (0.0, 0.0)),
                "天赋：结构化功率提升"
            );
        }

        if self.has_c1 {
            attribute.set_value_by(AttributeName::ElementalMastery, "C1：灰与力场的平衡理论", 80.0);
        }

        if self.has_c6 {
            let val = match self.moonsign {
                Moonsign::Ascendant => 0.35,
                Moonsign::Nascent => 0.15,
                Moonsign::None => 0.0
            };

            attribute.set_value_by(AttributeName::EnhanceElectroCharged, "C6：天才之为构造之责任", val);
            attribute.set_value_by(AttributeName::EnhanceBloom, "C6：天才之为构造之责任", val);
            attribute.set_value_by(AttributeName::EnhanceLunarCharged, "C6：天才之为构造之责任", val);
            attribute.set_value_by(AttributeName::EnhanceLunarBloom, "C6：天才之为构造之责任", val);
        }

        if self.has_p1 {
            attribute.set_value_by(AttributeName::USER1, "Talent1: 模块式高效运作", if self.moonsign.is_ascendant() { 0.7 } else { 1.5 });   // 元素爆发伤害间隔(s)
        }
    }
}

damage_enum!(
    AinoDamageEnum
    A1
    A2
    A3
    Z1
    Z2
    X1
    X2
    X3
    E1
    E2
    Q
    C2
);

impl AinoDamageEnum {
    pub fn get_element(&self) -> Element {
        use AinoDamageEnum::*;
        match *self {
            E1 | E2 | Q | C2 => Element::Hydro,
            _ => Element::Physical,
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use AinoDamageEnum::*;
        match *self {
            A1 | A2 | A3 => SkillType::NormalAttack,
            Z1 | Z2 => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E1 | E2 => SkillType::ElementalSkill,
            Q | C2 => SkillType::ElementalBurst,
        }
    }
}

pub struct Aino;

impl CharacterTrait for Aino {
    const STATIC_DATA: CharacterStaticData = AINO_STATIC_DATA;
    type SkillType = AinoSkillType;
    const SKILL: Self::SkillType = AINO_SKILL;
    type DamageEnumType = AinoDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            AinoDamageEnum
            A1 hit_n_dmg!(1)
            A2 hit_n_dmg!(2)
            A3 hit_n_dmg!(3)
            Z1 charged_dmg!("loop1")
            Z2 charged_dmg!("loop2")
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
        ),
        skill2: skill_map!(
            AinoDamageEnum
            E1 locale!(zh_cn: "一段伤害", en: "1-Hit DMG")
            E2 locale!(zh_cn: "二段伤害", en: "2-Hit DMG")
        ),
        skill3: skill_map!(
            AinoDamageEnum
            Q locale!(zh_cn: "水弹伤害", en: "Water Ball DMG")
            C2 locale!(zh_cn: "二命额外伤害", en: "C2 Extra DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::MOONSIGN2
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: AinoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use AinoDamageEnum::*;
        let mut builder = D::new();

        let ratio = match s {
            A1 => AINO_SKILL.a_dmg1[s1],
            A2 => AINO_SKILL.a_dmg2[s1],
            A3 => AINO_SKILL.a_dmg3[s1],
            Z1 => AINO_SKILL.z_dmg1[s1],
            Z2 => AINO_SKILL.z_dmg2[s1],
            X1 => AINO_SKILL.x_dmg1[s1],
            X2 => AINO_SKILL.x_dmg2[s1],
            X3 => AINO_SKILL.x_dmg3[s1],
            E1 => AINO_SKILL.e_dmg1[s2],
            E2 => AINO_SKILL.e_dmg2[s2],
            Q => AINO_SKILL.q_dmg[s3],
            C2 => AINO_SKILL.c2_dmg_atk,
            _ => 0.0
        };

        builder.add_atk_ratio("技能倍率", ratio);

        if s == C2 {
            builder.add_em_ratio("技能倍率", AINO_SKILL.c2_dmg_em);
        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let moonsign = match config {
            CharacterConfig::Aino { moonsign } => *moonsign,
            _ => Moonsign::None,
        };
        Some(Box::new(AinoEffect {
            has_p1: common_data.has_talent1,
            has_p2: common_data.has_talent2,
            has_c1: common_data.constellation >= 1,
            has_c6: common_data.constellation >= 6,
            moonsign: moonsign,
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
