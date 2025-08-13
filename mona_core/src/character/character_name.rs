// required by strum_derive::EnumString
use std::str::FromStr;

use mona_derive::{CharacterData, EnumLen};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum::*;
use strum_macros::{Display, EnumString, EnumIter};

use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::character_static_data::CharacterStaticData;
use crate::character::CharacterConfig;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterTrait};
use crate::common::ChangeAttribute;
use crate::common::element::Element;
use crate::common::item_config_type::ItemConfig;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[derive(Display, FromPrimitive, EnumString, CharacterData, EnumLen, EnumIter)]
pub enum CharacterName {
    AetherAnemo,
    Amber,
    Kaeya,
    Lisa,
    Barbara,
    Razor,
    Xiangling,
    Beidou,
    Xingqiu,
    Ningguang,
    Fischl,
    Bennett,
    Noelle,
    Chongyun,
    Sucrose,
    Jean,
    Diluc,
    Qiqi,
    Mona,
    Keqing,
    Venti,
    Klee,
    Diona,
    Tartaglia,
    Xinyan,
    Zhongli,
    Albedo,
    Ganyu,
    Xiao,
    HuTao,
    Rosaria,
    Yanfei,
    Eula,
    KaedeharaKazuha,
    KamisatoAyaka,
    Sayu,
    Yoimiya,
    Aloy,
    KujouSara,
    RaidenShogun,
    SangonomiyaKokomi,
    Thoma,
    Gorou,
    AratakiItto,
    Yunjin,
    Shenhe,
    YaeMiko,
    KamisatoAyato,
    Yelan,
    KukiShinobu,
    ShikanoinHeizou,
    Collei,
    Tighnari,
    Dori,
    Candace,
    Cyno,
    Nilou,
    Nahida,
    Layla,
    Faruzan,
    Wanderer,
    Yaoyao,
    Alhaitham,
    Dehya,
    Mika,
    Kaveh,
    Baizhu,
    Kirara,
    Lynette,
    Lyney,
    Freminet,
    Neuvillette,
    Wriothesley,
    Charlotte,
    Furina,
    Navia,
    Chevreuse,
    Gaming,
    Xianyun,
    Chiori,
    Arlecchino,
    Sethos,
    Clorinde,
    Sigewinne,
    Emilie,
    Kachina,
    Mualani,
    Kinich,
    Xilonen,
    Ororon,
    Chasca,
    Citlali,
    Mavuika,
    Lanyan,
    YumemizukiMizuki,
    Iansan,
    Varesa,
    Ifa,
    Escoffier,
    Dahlia,
    Skirk,
}