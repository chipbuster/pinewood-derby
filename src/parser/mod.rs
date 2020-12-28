pub mod preprocess;
pub mod error;

use lang_c::driver::{Config, Parse};
use error::CInputError;
use std::fs;
use std::path::Path;

pub fn parse<P: AsRef<Path>>(source: P) -> Result<Parse, CInputError>{
    let config = Config::default();
    let contents = fs::read_to_string(source).expect("Could not read input file");
    preprocess::check_preprocessor_tokens(&contents)?;

    lang_c::driver::parse_preprocessed(&config, contents).map_err(CInputError::LangCSyntaxError)
}