pub use element::Element;
pub use stat::{StatName, SUB_STAT_VALUE_5};
pub use weapon_type::WeaponType;
pub use change_attribute::ChangeAttribute;
pub use skill_type::SkillType;
pub use crate::damage::damage_result::DamageResult;
pub use entry_type::EntryType;
pub use reaction_type::{ReactionType, TransformativeType, MoonglareReaction};
pub use moonsign::Moonsign;

pub mod stat;
pub mod element;
pub mod weapon_type;
pub mod change_attribute;
pub mod skill_type;
pub mod entry_type;
pub mod moonsign;
pub mod max_trait;
pub mod reaction_type;
pub mod item_config_type;
pub mod code_escape;
pub mod i18n;

pub struct Span {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

impl Span {
    pub fn from_pest_span(span: &pest::Span) -> Span {
        let (start_row, start_col) = span.start_pos().line_col();
        let (end_row, end_col) = span.end_pos().line_col();
        Span {
            start_row,
            start_col,
            end_row,
            end_col
        }
    }
}