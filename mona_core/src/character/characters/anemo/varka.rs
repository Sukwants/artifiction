use crate::character::characters::prelude::*;

pub struct VarkaSkillType {
    pub a_dmg1: [f64; 15],
    pub a_dmg21: [f64; 15],
    pub a_dmg22: [f64; 15],
    pub a_dmg31: [f64; 15],
    pub a_dmg32: [f64; 15],
    pub a_dmg41: [f64; 15],
    pub a_dmg42: [f64; 15],
    pub a_dmg51: [f64; 15],
    pub a_dmg52: [f64; 15],
    pub z_dmg1: [f64; 15],
    pub z_dmg2: [f64; 15],
    pub x_dmg1: [f64; 15],
    pub x_dmg2: [f64; 15],
    pub x_dmg3: [f64; 15],

    pub e_dmg: [f64; 15],
    pub e_dmg_a1: [f64; 15],
    pub e_dmg_a21: [f64; 15],
    pub e_dmg_a22: [f64; 15],
    pub e_dmg_a31: [f64; 15],
    pub e_dmg_a32: [f64; 15],
    pub e_dmg_a41: [f64; 15],
    pub e_dmg_a42: [f64; 15],
    pub e_dmg_a51: [f64; 15],
    pub e_dmg_a52: [f64; 15],
    pub e_dmg_az1: [f64; 15],
    pub e_dmg_az2: [f64; 15],
    pub e_dmg_e1: [f64; 15],
    pub e_dmg_e2: [f64; 15],
    pub e_dmg_ez1: [f64; 15],
    pub e_dmg_ez2: [f64; 15],
    
    pub q_dmg1: [f64; 15],
    pub q_dmg2: [f64; 15],

    pub c2_dmg: f64,
}

pub const VARKA_SKILL: VarkaSkillType = VarkaSkillType {
    // Normal Attack: Favonius Bladework: Dancing Radiance
    a_dmg1: [0.6546, 0.7079, 0.7612, 0.8373, 0.8906, 0.9515, 1.0352, 1.1189, 1.2026, 1.294, 1.3853, 1.4767, 1.568, 1.6593, 1.7507],
    a_dmg21: [0.2399, 0.2594, 0.2789, 0.3068, 0.3264, 0.3487, 0.3794, 0.41, 0.4407, 0.4742, 0.5077, 0.5411, 0.5746, 0.6081, 0.6416],
    a_dmg22: [0.4455, 0.4818, 0.518, 0.5698, 0.6061, 0.6475, 0.7045, 0.7615, 0.8185, 0.8806, 0.9428, 1.005, 1.0671, 1.1293, 1.1915],
    a_dmg31: [0.3244, 0.3508, 0.3772, 0.4149, 0.4413, 0.4715, 0.5129, 0.5544, 0.5959, 0.6412, 0.6864, 0.7317, 0.777, 0.8222, 0.8675],
    a_dmg32: [0.6024, 0.6514, 0.7005, 0.7705, 0.8195, 0.8756, 0.9526, 1.0297, 1.1067, 1.1908, 1.2748, 1.3589, 1.4429, 1.527, 1.611],
    a_dmg41: [0.5543, 0.5994, 0.6446, 0.709, 0.7541, 0.8057, 0.8766, 0.9475, 1.0184, 1.0957, 1.1731, 1.2504, 1.3278, 1.4051, 1.4825],
    a_dmg42: [0.2985, 0.3228, 0.3471, 0.3818, 0.4061, 0.4338, 0.472, 0.5102, 0.5484, 0.59, 0.6317, 0.6733, 0.715, 0.7566, 0.7983],
    a_dmg51: [0.6975, 0.7543, 0.811, 0.8921, 0.9489, 1.0138, 1.103, 1.1922, 1.2815, 1.3788, 1.4761, 1.5734, 1.6708, 1.7681, 1.8654],
    a_dmg52: [0.3756, 0.4061, 0.4367, 0.4804, 0.511, 0.5459, 0.5939, 0.642, 0.69, 0.7424, 0.7948, 0.8472, 0.8996, 0.952, 1.0044],
    z_dmg1: [0.8564, 0.9261, 0.9958, 1.0954, 1.1651, 1.2448, 1.3543, 1.4638, 1.5734, 1.6929, 1.8124, 1.9319, 2.0513, 2.1708, 2.2903],
    z_dmg2: [0.4611, 0.4987, 0.5362, 0.5898, 0.6274, 0.6703, 0.7292, 0.7882, 0.8472, 0.9115, 0.9759, 1.0402, 1.1046, 1.1689, 1.2333],
    x_dmg1: [0.7459, 0.8066, 0.8673, 0.954, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785, 1.6826, 1.7866, 1.8907, 1.9948],
    x_dmg2: [1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563, 3.3644, 3.5725, 3.7806, 3.9887],
    x_dmg3: [1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.946, 3.1842, 3.4225, 3.6825, 3.9424, 4.2023, 4.4623, 4.7222, 4.9821],

    // Elemental Skill: Windbound Execution
    e_dmg: [2.784, 2.9928, 3.2016, 3.48, 3.6888, 3.8976, 4.176, 4.4544, 4.7328, 5.0112, 5.2896, 5.568, 5.916, 6.264, 6.612],
    e_dmg_a1: [0.8182, 0.8848, 0.9515, 1.0466, 1.1132, 1.1893, 1.294, 1.3986, 1.5033, 1.6175, 1.7316, 1.8458, 1.96, 2.0742, 2.1883],
    e_dmg_a21: [0.2999, 0.3243, 0.3487, 0.3835, 0.4079, 0.4358, 0.4742, 0.5125, 0.5509, 0.5927, 0.6346, 0.6764, 0.7183, 0.7601, 0.8019],
    e_dmg_a22: [0.5569, 0.6022, 0.6475, 0.7123, 0.7576, 0.8094, 0.8806, 0.9519, 1.0231, 1.1008, 1.1785, 1.2562, 1.3339, 1.4116, 1.4893],
    e_dmg_a31: [0.4055, 0.4385, 0.4715, 0.5186, 0.5516, 0.5893, 0.6412, 0.693, 0.7449, 0.8015, 0.8581, 0.9146, 0.9712, 1.0278, 1.0844],
    e_dmg_a32: [0.753, 0.8143, 0.8756, 0.9631, 1.0244, 1.0945, 1.1908, 1.2871, 1.3834, 1.4885, 1.5935, 1.6986, 1.8037, 1.9087, 2.0138],
    e_dmg_a41: [0.6929, 0.7493, 0.8057, 0.8863, 0.9427, 1.0071, 1.0957, 1.1844, 1.273, 1.3697, 1.4664, 1.563, 1.6597, 1.7564, 1.8531],
    e_dmg_a42: [0.3731, 0.4035, 0.4338, 0.4772, 0.5076, 0.5423, 0.59, 0.6377, 0.6855, 0.7375, 0.7896, 0.8416, 0.8937, 0.9458, 0.9978],
    e_dmg_a51: [0.8719, 0.9428, 1.0138, 1.1152, 1.1862, 1.2673, 1.3788, 1.4903, 1.6018, 1.7235, 1.8451, 1.9668, 2.0884, 2.2101, 2.3318],
    e_dmg_a52: [0.4695, 0.5077, 0.5459, 0.6005, 0.6387, 0.6824, 0.7424, 0.8025, 0.8625, 0.928, 0.9935, 1.059, 1.1245, 1.1901, 1.2556],
    e_dmg_az1: [1.0705, 1.1576, 1.2448, 1.3692, 1.4564, 1.5559, 1.6929, 1.8298, 1.9667, 2.1161, 2.2654, 2.4148, 2.5642, 2.7136, 2.8629],
    e_dmg_az2: [0.5764, 0.6233, 0.6703, 0.7373, 0.7842, 0.8378, 0.9115, 0.9853, 1.059, 1.1394, 1.2199, 1.3003, 1.3807, 1.4611, 1.5416],
    e_dmg_e1: [1.7576, 1.8894, 2.0212, 2.197, 2.3288, 2.4606, 2.6364, 2.8122, 2.9879, 3.1637, 3.3394, 3.5152, 3.7349, 3.9546, 4.1743],
    e_dmg_e2: [0.9464, 1.0174, 1.0884, 1.183, 1.254, 1.325, 1.4196, 1.5142, 1.6089, 1.7035, 1.7982, 1.8928, 2.0111, 2.1294, 2.2477],
    e_dmg_ez1: [0.936, 1.0062, 1.0764, 1.17, 1.2402, 1.3104, 1.404, 1.4976, 1.5912, 1.6848, 1.7784, 1.872, 1.989, 2.106, 2.223],
    e_dmg_ez2: [0.504, 0.5418, 0.5796, 0.63, 0.6678, 0.7056, 0.756, 0.8064, 0.8568, 0.9072, 0.9576, 1.008, 1.071, 1.134, 1.197],

    // Elemental Burst: Northwind Avatar
    q_dmg1: [3.3696, 3.6223, 3.875, 4.212, 4.4647, 4.7174, 5.0544, 5.3914, 5.7283, 6.0653, 6.4022, 6.7392, 7.1604, 7.5816, 8.0028],
    q_dmg2: [1.8144, 1.9505, 2.0866, 2.268, 2.4041, 2.5402, 2.7216, 2.903, 3.0845, 3.2659, 3.4474, 3.6288, 3.8556, 4.0824, 4.3092],

    c2_dmg: 8.0,
};

pub const VARKA_STATIC_DATA: CharacterStaticData = CharacterStaticData {
    name: CharacterName::Varka,
    internal_name: "Varka",
    element: Element::Anemo,
    hp: [982, 2547, 3389, 5071, 5669, 6523, 7320, 8182, 8780, 9650, 10249, 11128, 11727, 12613, 13510],
    atk: [27, 71, 95, 142, 159, 182, 205, 229, 246, 270, 287, 311, 328, 353, 432],
    def: [62, 161, 214, 320, 358, 411, 462, 516, 554, 609, 646, 702, 740, 795, 852],
    sub_stat: CharacterSubStatFamily::CriticalDamage384,
    weapon_type: WeaponType::Claymore,
    star: 5,
    skill_name1: locale!(
        zh_cn: "西风剑术·流光之舞",
        en: "Favonius Bladework: Dancing Radiance",
    ),
    skill_name2: locale!(
        zh_cn: "烈风终坠",
        en: "Windbound Execution",
    ),
    skill_name3: locale!(
        zh_cn: "我即朔风",
        en: "Northwind Avatar",
    ),
    name_locale: locale!(
        zh_cn: "法尔伽",
        en: "Varka",
    )
};

pub struct VarkaEffect {
    pub hexerei_secret_rite: bool,
    pub team_elements: ConfigElements8Multi,
    pub repeated_element: bool,
    pub common_data: CharacterCommonData,
}

impl<A: Attribute> ChangeAttribute<A> for VarkaEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.common_data.has_talent1 {
            if self.team_elements.pyro || self.team_elements.hydro || self.team_elements.electro || self.team_elements.cryo {
                attribute.add_edge_n1(AttributeName::ATK, AttributeName::BonusAnemo, Arc::new(move |atk, _ | (atk / 1000.0 * 0.1).min(0.25)), "法尔伽天赋1", EdgePriority::Common);
            }
            if self.team_elements.pyro {
                attribute.add_edge_n1(AttributeName::ATK, AttributeName::BonusPyro, Arc::new(move |atk, _ | (atk / 1000.0 * 0.1).min(0.25)), "法尔伽天赋1", EdgePriority::Common);
            } else if self.team_elements.hydro {
                attribute.add_edge_n1(AttributeName::ATK, AttributeName::BonusHydro, Arc::new(move |atk, _ | (atk / 1000.0 * 0.1).min(0.25)), "法尔伽天赋1", EdgePriority::Common);
            } else if self.team_elements.electro {
                attribute.add_edge_n1(AttributeName::ATK, AttributeName::BonusElectro, Arc::new(move |atk, _ | (atk / 1000.0 * 0.1).min(0.25)), "法尔伽天赋1", EdgePriority::Common);
            } else if self.team_elements.cryo {
                attribute.add_edge_n1(AttributeName::ATK, AttributeName::BonusCryo, Arc::new(move |atk, _ | (atk / 1000.0 * 0.1).min(0.25)), "法尔伽天赋1", EdgePriority::Common);
            }
        }
        
        if self.common_data.constellation >= 4 {
            attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusAnemo), "法尔伽命座4", 0.2);
            if self.team_elements.pyro {
                attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusPyro), "法尔伽命座4", 0.2);
            } else if self.team_elements.hydro {
                attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusHydro), "法尔伽命座4", 0.2);
            } else if self.team_elements.electro {
                attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusElectro), "法尔伽命座4", 0.2);
            } else if self.team_elements.cryo {
                attribute.set_value_by_s(CharacterSelector::select_all(attribute), AttributeType::Panel(AttributeName::BonusCryo), "法尔伽命座4", 0.2);
            }
        }
    }
}

damage_enum!(
    VarkaDamageEnum
    A1
    A21
    A22
    A31
    A32
    A41
    A42
    A51
    A52
    Z1
    Z2
    X1
    X2
    X3
    E
    EA1
    EA21
    EA22
    EA31
    EA32
    EA41
    EA42
    EA51
    EA52
    EAZ1
    EAZ2
    EE1
    EE2
    EEZ1
    EEZ2
    Q1
    Q2
    C2E
    C2Z
);

impl VarkaDamageEnum {
    pub fn get_element(&self, elemental_absorption: Option<Element>) -> Element {
        use VarkaDamageEnum::*;
        match *self {
            A1 | A21 | A22 | A31 | A32 | A41 | A42 | A51 | A52 | Z1 | Z2 | X1 | X2 | X3 => Element::Physical,
            E | EA21 | EA31 | EA42 | EA52 | EAZ2 | EE2 | EEZ2 | Q2 | C2E | C2Z => Element::Anemo,
            EA1 | EA22 | EA32 | EA41 | EA51 | EAZ1 | EE1 | EEZ1 | Q1 => elemental_absorption.unwrap_or(Element::Physical),
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use VarkaDamageEnum::*;
        match *self {
            A1 | A21 | A22 | A31 | A32 | A41 | A42 | A51 | A52 | EA1 | EA21 | EA22 | EA31 | EA32 | EA41 | EA42 | EA51 | EA52 => SkillType::NormalAttack,
            Z1 | Z2 | EAZ1 | EAZ2 | EEZ1 | EEZ2 | C2Z => SkillType::ChargedAttack,
            X1 => SkillType::PlungingAttackInAction,
            X2 | X3 => SkillType::PlungingAttackOnGround,
            E | EE1 | EE2 | C2E => SkillType::ElementalSkill,
            Q1 | Q2 => SkillType::ElementalBurst,
        }
    }
}

pub struct Varka;

impl CharacterTrait for Varka {
    const STATIC_DATA: CharacterStaticData = VARKA_STATIC_DATA;
    type SkillType = VarkaSkillType;
    const SKILL: Self::SkillType = VARKA_SKILL;
    type DamageEnumType = VarkaDamageEnum;
    type RoleEnum = ();

    const DEFAULT_TAGS: Option<&'static [CharacterTag]> = Some(
        &[CharacterTag::Hexerei]
    );

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: skill_map!(
            VarkaDamageEnum
            A1 hit_n_dmg!(1)
            A21 hit_n_dmg!(2, 1)
            A22 hit_n_dmg!(2, 2)
            A31 hit_n_dmg!(3, 1)
            A32 hit_n_dmg!(3, 2)
            A41 hit_n_dmg!(4, 1)
            A42 hit_n_dmg!(4, 2)
            A51 hit_n_dmg!(5, 1)
            A52 hit_n_dmg!(5, 2)
            Z1 charged_dmg!(1)
            Z2 charged_dmg!(2)
            X1 plunging_dmg!(1)
            X2 plunging_dmg!(2)
            X3 plunging_dmg!(3)
            C2Z locale!(zh_cn: "二命伤害-重击", en: "C2 DMG-Charged Attack")
        ),
        skill2: skill_map!(
            VarkaDamageEnum
            E locale!(zh_cn: "技能伤害", en: "Skill DMG")
            EA1 locale!(zh_cn: "狂飙突进·一段伤害", en: "Sturm und Drang 1-Hit DMG")
            EA21 locale!(zh_cn: "狂飙突进·二段伤害-1", en: "Sturm und Drang 2-Hit DMG-1")
            EA22 locale!(zh_cn: "狂飙突进·二段伤害-2", en: "Sturm und Drang 2-Hit DMG-2")
            EA31 locale!(zh_cn: "狂飙突进·三段伤害-1", en: "Sturm und Drang 3-Hit DMG-1")
            EA32 locale!(zh_cn: "狂飙突进·三段伤害-2", en: "Sturm und Drang 3-Hit DMG-2")
            EA41 locale!(zh_cn: "狂飙突进·四段伤害-1", en: "Sturm und Drang 4-Hit DMG-1")
            EA42 locale!(zh_cn: "狂飙突进·四段伤害-2", en: "Sturm und Drang 4-Hit DMG-2")
            EA51 locale!(zh_cn: "狂飙突进·五段伤害-1", en: "Sturm und Drang 5-Hit DMG-1")
            EA52 locale!(zh_cn: "狂飙突进·五段伤害-2", en: "Sturm und Drang 5-Hit DMG-2")
            EAZ1 locale!(zh_cn: "狂飙突进·重击伤害-1", en: "Sturm und Drang Charged Attack DMG-1")
            EAZ2 locale!(zh_cn: "狂飙突进·重击伤害-2", en: "Sturm und Drang Charged Attack DMG-2")
            EE1 locale!(zh_cn: "四风将起伤害-1", en: "Four Winds' Ascension DMG-1")
            EE2 locale!(zh_cn: "四风将起伤害-2", en: "Four Winds' Ascension DMG-2")
            EEZ1 locale!(zh_cn: "苍噬伤害-1", en: "Azure Devour DMG-1")
            EEZ2 locale!(zh_cn: "苍噬伤害-2", en: "Azure Devour DMG-2")
            C2E locale!(zh_cn: "二命伤害-元素战技", en: "C2 DMG-Elemental Skill")
        ),
        skill3: skill_map!(
            VarkaDamageEnum
            Q1 locale!(zh_cn: "技能第一段伤害", en: "Skill 1-Hit DMG")
            Q2 locale!(zh_cn: "技能第二段伤害", en: "Skill 2-Hit DMG")
        )
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::HEXEREI_SECRET_RITE_GLOBAL(false, ItemConfig::PRIORITY_CHARACTER),
        ItemConfig {
            name: "team_elements",
            title: locale!(
                zh_cn: "队伍中角色元素",
                en: "Team Character Elements"
            ),
            config: ItemConfigType::ElementMulti { 
                elements: &[Element::Anemo, Element::Pyro, Element::Hydro, Element::Electro, Element::Cryo], 
                default: ConfigElements8Multi {
                    pyro: false,
                    hydro: false,
                    anemo: false,
                    electro: false,
                    dendro: false,
                    cryo: false,
                    geo: false,
                    physical: false,
                }
            }
        },
        ItemConfig {
            name: "repeated_element",
            title: locale!(
                zh_cn: "队伍中是否至少两名元素类型相同的火、水、雷、冰角色",
                en: "Whether there are at least two characters of the same element among Pyro, Hydro, Electro, Cryo in the team"
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "azure_fang_oath",
            title: locale!(
                zh_cn: "「苍牙之誓」层数",
                en: "Stacks of Azure Fang's Oath"
            ),
            config: ItemConfigType::Int { min: 0, max: 4, default: 4 }
        },
        ItemConfig {
            name: "c1_bonus",
            title: locale!(
                zh_cn: "「歌中的佳酿」效果",
                en: "Lyrical Libation effect"
            ),
            config: ItemConfigType::Bool { default: false }
        },
    ]);

    fn change_attribute<A: Attribute>(attribute: &mut A, common_data: &CharacterCommonData, skill_config: &CharacterSkillConfig) {
        let (hexerei_secret_rite, team_elements, repeated_element) = match &common_data.config {
            CharacterConfig::Varka { hexerei_secret_rite, team_elements, repeated_element } => (*hexerei_secret_rite, *team_elements, *repeated_element),
            _ => (false, ConfigElements8Multi::default(), false),
        };

        let (azure_fang_oath, c1_bonus) = match *skill_config {
            CharacterSkillConfig::Varka { azure_fang_oath, c1_bonus } => (azure_fang_oath, c1_bonus),
            _ => (0, false)
        };

        if common_data.constellation >= 6 {
            attribute.set_value_by(AttributeName::CriticalDamageBase, "法尔伽命座6", azure_fang_oath as f64 * 0.2);
        }
    }

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: VarkaDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        let (hexerei_secret_rite, team_elements, repeated_element) = match &context.character_common_data.config {
            CharacterConfig::Varka { hexerei_secret_rite, team_elements, repeated_element } => (*hexerei_secret_rite, *team_elements, *repeated_element),
            _ => (false, ConfigElements8Multi::default(), false),
        };

        let (azure_fang_oath, c1_bonus) = match *config {
            CharacterSkillConfig::Varka { azure_fang_oath, c1_bonus } => (azure_fang_oath, c1_bonus),
            _ => (0, false)
        };

        let elemental_absorption = if team_elements.pyro {
            Some(Element::Pyro)
        } else if team_elements.hydro {
            Some(Element::Hydro)
        } else if team_elements.electro {
            Some(Element::Electro)
        } else if team_elements.cryo {
            Some(Element::Cryo)
        } else {
            None
        };

        use VarkaDamageEnum::*;
        let mut builder = D::new();

        if (s == C2E || s == C2Z) && context.character_common_data.constellation < 2 {
            return builder.none();
        }

        if context.character_common_data.has_talent2 && (s.get_skill_type() == SkillType::NormalAttack || s.get_skill_type() == SkillType::ChargedAttack || s == EE1 || s == EE2 || s == EEZ1 || s == EEZ2) {
            builder.add_extra_bonus("法尔伽天赋2", azure_fang_oath as f64 * 0.075);
        }

        let ratio = match s {
            A1 => VARKA_SKILL.a_dmg1[s1],
            A21 => VARKA_SKILL.a_dmg21[s1],
            A22 => VARKA_SKILL.a_dmg22[s1],
            A31 => VARKA_SKILL.a_dmg31[s1],
            A32 => VARKA_SKILL.a_dmg32[s1],
            A41 => VARKA_SKILL.a_dmg41[s1],
            A42 => VARKA_SKILL.a_dmg42[s1],
            A51 => VARKA_SKILL.a_dmg51[s1],
            A52 => VARKA_SKILL.a_dmg52[s1],
            Z1 => VARKA_SKILL.z_dmg1[s1],
            Z2 => VARKA_SKILL.z_dmg2[s1],
            X1 => VARKA_SKILL.x_dmg1[s1],
            X2 => VARKA_SKILL.x_dmg2[s1],
            X3 => VARKA_SKILL.x_dmg3[s1],
            E => VARKA_SKILL.e_dmg[s2],
            EA1 => VARKA_SKILL.e_dmg_a1[s2],
            EA21 => VARKA_SKILL.e_dmg_a21[s2],
            EA22 => VARKA_SKILL.e_dmg_a22[s2],
            EA31 => VARKA_SKILL.e_dmg_a31[s2],
            EA32 => VARKA_SKILL.e_dmg_a32[s2],
            EA41 => VARKA_SKILL.e_dmg_a41[s2],
            EA42 => VARKA_SKILL.e_dmg_a42[s2],
            EA51 => VARKA_SKILL.e_dmg_a51[s2],
            EA52 => VARKA_SKILL.e_dmg_a52[s2],
            EAZ1 => VARKA_SKILL.e_dmg_az1[s2],
            EAZ2 => VARKA_SKILL.e_dmg_az2[s2],
            EE1 => VARKA_SKILL.e_dmg_e1[s2],
            EE2 => VARKA_SKILL.e_dmg_e2[s2],
            EEZ1 => VARKA_SKILL.e_dmg_ez1[s2],
            EEZ2 => VARKA_SKILL.e_dmg_ez2[s2],
            Q1 => VARKA_SKILL.q_dmg1[s3],
            Q2 => VARKA_SKILL.q_dmg2[s3],
            C2E => VARKA_SKILL.c2_dmg,
            C2Z => VARKA_SKILL.c2_dmg,
        } * match s {
            EA1 | EA21 | EA22 | EA31 | EA32 | EA41 | EA42 | EA51 | EA52 | EAZ1 | EAZ2 | EE1 | EE2 | EEZ1 | EEZ2 =>
                if team_elements.anemo && repeated_element { 2.2 } else if team_elements.anemo || repeated_element { 1.4 } else { 1.0 },
            _ => 1.0,
        } * match s {
            EE1 | EE2 | EEZ1 | EEZ2 =>
                if context.character_common_data.constellation >= 1 && c1_bonus { 2.0 } else { 1.0 },
            _ => 1.0,
        };

        builder.add_atk_ratio("技能倍率", ratio);

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(elemental_absorption),
            s.get_skill_type(),
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        let (hexerei_secret_rite, team_elements, repeated_element) = match *config {
            CharacterConfig::Varka { hexerei_secret_rite, team_elements, repeated_element } => (hexerei_secret_rite, team_elements, repeated_element),
            _ => (false, ConfigElements8Multi::default(), false),
        };
        Some(Box::new(VarkaEffect {
            hexerei_secret_rite: hexerei_secret_rite,
            team_elements: team_elements,
            repeated_element: repeated_element,
            common_data: common_data.clone(),
        }))
    }

    fn get_target_function_by_role(role_index: usize, _team: &TeamQuantization, _c: &CharacterCommonData, _w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}
