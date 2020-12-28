use thiserror::Error;
use super::preprocess::CPPDirective;

#[derive(Error, Debug)]
pub enum CInputError {
    #[error("On line {0}, cannot have preprocessor macro {1}")]
    PreprocessorError(usize, String, CPPDirective),
    #[error("LangC Error {0}")]
    LangCSyntaxError(lang_c::driver::SyntaxError),
}