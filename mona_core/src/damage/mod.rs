pub use simple_damage_builder::SimpleDamageBuilder;
pub use damage_analysis::DamageAnalysis;
pub use complicated_damage_builder::ComplicatedDamageBuilder;
pub use damage_context::DamageContext;

pub mod simple_damage_builder;
pub mod complicated_damage_builder;
pub mod damage_analysis;
pub mod damage_context;
pub mod reaction;
// pub mod damage_type;
pub mod damage_builder;
pub mod damage_result;
pub mod transformative_damage;
pub mod level_coefficient;
