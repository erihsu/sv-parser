#[macro_use]
pub mod utils;
pub use utils::*;

pub mod behavioral_statements;
pub mod declarations;
pub mod expressions;
pub mod general;
pub mod instantiations;
pub mod primitive_instances;
pub mod source_text;
pub mod specify_section;
pub mod udp_declaration_and_instantiation;
pub use behavioral_statements::*;
pub use declarations::*;
pub use expressions::*;
pub use general::*;
pub use instantiations::*;
pub use primitive_instances::*;
pub use source_text::*;
pub use specify_section::*;
pub use udp_declaration_and_instantiation::*;

pub type Span<'a> = nom_locate::LocatedSpanEx<&'a str, u64>;

// IDs for left recursion detection
static REC_PRIMARY: u32 = 0;
