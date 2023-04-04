

mod sourcecode;

pub mod input;
pub mod output;
pub mod analysis;
pub mod parse;

pub use sourcecode::SourceCode;
pub use analysis::Analysis;
pub use output::Annotater;


#[cfg(test)]
mod tests;




