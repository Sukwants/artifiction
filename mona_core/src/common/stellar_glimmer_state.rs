use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{Error as DeError, Visitor};
use std::fmt;
use strum_macros::Display;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive as NumFromPrimitive;

/// 辉映·星烁状态（`ItemConfig::STELLAR_GLIMMER_STATE` 的取值语义）。
///
/// - `None`：无（对应配置值 0）
/// - `StellarConduct`：辉映·星超导（对应配置值 1）
/// - `StellarSwirl`：辉映·星扩散（对应配置值 2）
///
/// 该枚举直接作为 `CharacterConfig` / `BuffConfig` 字段与配置项的 `default` 类型（模仿
/// `Moonsign` 的模式），序列化为字符串（如 `"StellarConduct"`），并反序列化时兼容字符串与
/// 旧的数字（0/1/2）两种形式，以便迁移历史数据。
#[derive(Serialize, Display, FromPrimitive)]
#[derive(Default, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum StellarGlimmerState {
    #[default]
    None = 0,
    StellarConduct = 1,
    StellarSwirl = 2,
}

impl<'de> Deserialize<'de> for StellarGlimmerState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StellarGlimmerStateVisitor;

        impl<'de> Visitor<'de> for StellarGlimmerStateVisitor {
            type Value = StellarGlimmerState;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a stellar glimmer state string (\"None\" | \"StellarConduct\" | \"StellarSwirl\") or an integer (0 | 1 | 2)")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                match v {
                    "None" => Ok(StellarGlimmerState::None),
                    "StellarConduct" => Ok(StellarGlimmerState::StellarConduct),
                    "StellarSwirl" => Ok(StellarGlimmerState::StellarSwirl),
                    _ => Err(E::custom(format!("invalid stellar glimmer state: {}", v))),
                }
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                Ok(StellarGlimmerState::from_usize(v as usize))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: DeError,
            {
                if v < 0 {
                    Ok(StellarGlimmerState::None)
                } else {
                    Ok(StellarGlimmerState::from_usize(v as usize))
                }
            }
        }

        deserializer.deserialize_any(StellarGlimmerStateVisitor)
    }
}

impl StellarGlimmerState {
    /// 从配置值（0/1/2）转换为枚举；越界值按 `None` 处理。
    pub fn from_usize(v: usize) -> Self {
        <Self as NumFromPrimitive>::from_usize(v).unwrap_or(Self::None)
    }

    pub fn is_none(&self) -> bool {
        *self == Self::None
    }

    pub fn is_stellar_conduct(&self) -> bool {
        *self == Self::StellarConduct
    }

    pub fn is_stellar_swirl(&self) -> bool {
        *self == Self::StellarSwirl
    }

    /// 是否处于辉映·星烁状态（辉映·星超导或辉映·星扩散）。
    pub fn is_stellar_glimmer(&self) -> bool {
        !self.is_none()
    }
}
