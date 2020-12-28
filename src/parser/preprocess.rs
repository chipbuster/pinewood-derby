/*! "Preprocesses" the source code by producing an error if a C preprocessor
directive is found. Not exactly elegant, but it works. */

use super::error::CInputError;

use lazy_static::lazy_static;
use regex::RegexSet;
use std::fmt;

use num;
use num_derive::FromPrimitive;

#[derive(Debug, PartialEq, Eq, Copy, Clone, FromPrimitive)]
pub enum CPPDirective {
    Define = 0,
    Include,
    Undef,
    Ifdef,
    Ifndef,
    If,
    Else,
    Elif,
    Endif,
    Error,
    Pragma,
    Date,
    Time,
    Timestamp,
    File,
    Line,
    Stdc,
}

impl fmt::Display for CPPDirective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CPPDirective::Define => write!(f, "#define"),
            CPPDirective::Include => write!(f, "#include"),
            CPPDirective::Undef => write!(f, "#undef"),
            CPPDirective::Ifdef => write!(f, "#ifdef"),
            CPPDirective::Ifndef => write!(f, "#ifndef"),
            CPPDirective::If => write!(f, "#if"),
            CPPDirective::Else => write!(f, "#else"),
            CPPDirective::Elif => write!(f, "#elif"),
            CPPDirective::Endif => write!(f, "#endif"),
            CPPDirective::Error => write!(f, "#error"),
            CPPDirective::Pragma => write!(f, "#pragma"),
            CPPDirective::Date => write!(f, "__DATE__"),
            CPPDirective::Time => write!(f, "__TIME__"),
            CPPDirective::Timestamp => write!(f, "__TIMESTAMP__"),
            CPPDirective::File => write!(f, "__FILE__"),
            CPPDirective::Line => write!(f, "__LINE__"),
            CPPDirective::Stdc => write!(f, "__STDC__"),
        }
    }
}

///Search a single line of text (w/o terminal newline) for a preprocessor directive.
fn preprocess_line(line: &str) -> Option<CPPDirective> {
    lazy_static! {
        // Important
        static ref CPP_DIRECTIVE_RE: RegexSet = RegexSet::new(&[
            r"^\p{White_Space}*#define",
            r"^\p{White_Space}*#include",
            r"^\p{White_Space}*#undef",
            r"^\p{White_Space}*#ifdef",
            r"^\p{White_Space}*#ifndef",
            r"^\p{White_Space}*#if",
            r"^\p{White_Space}*#else",
            r"^\p{White_Space}*#elif",
            r"^\p{White_Space}*#endif",
            r"^\p{White_Space}*#error",
            r"^\p{White_Space}*#pragma",
            r"__DATE__",
            r"__TIME__",
            r"__TIMESTAMP__",
            r"__FILE__",
            r"__LINE__",
            r"__STDC__",
        ]).unwrap();
    }

    let matches = CPP_DIRECTIVE_RE.matches(line);
    if matches.matched_any() {
        for i in matches.iter() {
            let directive = num::FromPrimitive::from_usize(i)
                .expect("Could not turn match index into pragma: mismatch in RegexSet/Enum?");
            return Some(directive);
        }
        None
    } else {
        None
    }
}

/// "Preprocesses" code by throwing an error when a directive is encountered.
/// Awful, but it probably works. Mostly.
pub fn check_preprocessor_tokens(contents: &str) -> Result<(), CInputError> {
    for (lineno, linecontent) in contents.lines().enumerate() {
        if let Some(directive) = preprocess_line(linecontent) {
            return Err(CInputError::PreprocessorError(
                lineno,
                linecontent.to_owned(),
                directive,
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{check_preprocessor_tokens, CPPDirective};

    #[test]
    fn normal_code() {
        let contents = r#"
int main(void){
    printf("Hello world from test1\n");
    return 0;
}        
"#;
        let res = check_preprocessor_tokens(contents);
        assert!(res.is_ok());
    }

    #[test]
    fn pragmaed_code() {
        let contents = r#"
#include<stdio.h>
int main(void){
    printf("Hello world from test1\n");
    return 0;
}        
"#;
        let res = check_preprocessor_tokens(contents);
        if let super::super::error::CInputError::PreprocessorError(_, _, directive) =
            res.unwrap_err()
        {
            assert_eq!(directive, CPPDirective::Include);
        } else {
            panic!("Was not a preprocessor error!");
        }
    }

    #[test]
    fn tokened_code() {
        let contents = r#"
int main(void){
    printf("Hello world from test1\n");
    printf("I'm on line %d\n", __LINE__);
    return 0;
}        
"#;
        let res = check_preprocessor_tokens(contents);
        if let super::super::error::CInputError::PreprocessorError(_, _, directive) =
            res.unwrap_err()
        {
            assert_eq!(directive, CPPDirective::Line);
        } else {
            panic!("Was not a preprocessor error!");
        }
    }
}
