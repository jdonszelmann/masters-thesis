use super::Location;
use crate::input::subsystems::lsp::lsp_types::jsonrpc::{Nullable, Union};

/// The definition of a symbol represented as one or many [locations](#Location).
/// For most programming languages there is only one location at which a symbol is
/// defined. If no definition can be found `null` is returned.
pub type Definition = Nullable<Union<Location, Vec<Location>>>;
