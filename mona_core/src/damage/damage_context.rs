use crate::attribute::*;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::Element;
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::transformative_damage::TransformativeDamage;
use crate::enemies::Enemy;

pub struct DamageContext<'a, A> {
    pub character_common_data: &'a CharacterCommonData,
    pub attribute: &'a A,
    pub enemy: &'a Enemy,
}

impl<'a, A: Attribute> DamageContext<'a, A> {
    // pub fn transformative(&self) -> TransformativeDamage {
    //     let level = self.character_common_data.level;

    //     transformative_damage::<A>(level, &self.attribute, &self.enemy)
    // }
}
