//! The Simple Switch Cases Configuration PreProcessor
//!
//! Introduction is in the Readme.
//!
//! Typical usage of the library is as follow:
//!
//! ```
//! use ssccpp::Parser;
//! let content =
//! b"1\n\
//! >>> thisone\n\
//! 2\n\
//! >>> other\n\
//! XXX\n\
//! >>>\n\
//! 3";
//! let parser = Parser::new("thisone",">>>").unwrap();
//! let mut res : Vec<u8> = Vec::new();
//! parser.process(&content[..], &mut res).unwrap();
//! let expected_result = b"1\n2\n3\n";
//! assert_eq!(expected_result, res.as_slice());
//! ```

use regex::{self, Regex, RegexSet};
use std::io::{BufRead, Error as IoError, Write};

const REGEX_IDX_DELIMITER_ONLY: usize = 0;
const REGEX_IDX_STAR: usize = 1;
const REGEX_IDX_IDENT: usize = 2;

#[derive(Debug)]
enum LineAction {
    EnterSpecific,
    EnterIgnore,
    EnterOtherwise,
    EnterGeneral,
    Process,
}

/// The struct implementing the actual logic.
///
/// See crate-level doc for typcial usage.
#[derive(Debug)]
pub struct Parser {
    delimiter_regex: Regex,
    caseswitch_regexset: RegexSet,
}

impl Parser {
    /// Build a new parser, with `ident` as identifier, and `delimiter` as delimiter.
    pub fn new(ident: &str, delimiter: &str) -> Result<Parser, regex::Error> {
        let delimiter_regex_string =
            format!(r"^\s*{delimiter}\s*", delimiter = regex::escape(delimiter));
        Ok(Parser {
            delimiter_regex: Regex::new(&delimiter_regex_string)?,
            caseswitch_regexset: RegexSet::new(&[
                r"^$",
                r"(?:[,\s]|^)\*(?:[,\s]|$)",
                &format!(r"\b{ident}\b", ident = regex::escape(ident)),
            ])?,
        })
    }

    fn discriminate(self: &Self, s: &str) -> LineAction {
        use LineAction::*;
        match self.delimiter_regex.find(s) {
            None => Process,
            Some(match_range) => {
                let matches = self.caseswitch_regexset.matches(&s[match_range.end()..]);
                if matches.matched(REGEX_IDX_DELIMITER_ONLY) {
                    EnterGeneral
                } else if matches.matched(REGEX_IDX_STAR) {
                    EnterOtherwise
                } else if matches.matched(REGEX_IDX_IDENT) {
                    EnterSpecific
                } else {
                    EnterIgnore
                }
            }
        }
    }

    /// Process an `input` (which should implement `BufRead`) and writes the corresponding result in
    /// `output`.
    pub fn process<I, O>(self: &Self, input: I, output: &mut O) -> Result<(), IoError>
    where
        I: BufRead,
        O: Write,
    {
        use LineAction::*;
        let mut entered_specific = false;
        let mut output_next = true;
        for line in input.lines() {
            let line = line?;
            match self.discriminate(&line) {
                Process => {
                    if output_next {
                        writeln!(output, "{}", &line)?
                    }

                }
                EnterSpecific => {
                    output_next = true;
                    entered_specific = true;
                }
                EnterIgnore => output_next = false,
                EnterOtherwise => output_next = !entered_specific,
                EnterGeneral => {
                    output_next = true;
                    entered_specific = false;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing1() {
        let content =
        b"1\n\
        >>> thisone\n\
        2\n\
        >>> other\n\
        XXX\n\
        >>>\n\
        3";
        let parser = Parser::new("thisone",">>>").unwrap();
        let mut res : Vec<u8> = Vec::new();
        parser.process(&content[..], &mut res).unwrap();
        let res_string = String::from_utf8(res).unwrap();
        let expected_result = "1\n2\n3\n";
        assert_eq!(expected_result, res_string);
    }

    #[test]
    fn parsing2() {
        let content =
        b"1\n\
        >>> thisone\n\
        2\n\
        >>> other\n\
        XXX\n\
        >>> *\n\
        XXX\n\
        >>>\n\
        3";
        let parser = Parser::new("thisone",">>>").unwrap();
        let mut res : Vec<u8> = Vec::new();
        parser.process(&content[..], &mut res).unwrap();
        let res_string = String::from_utf8(res).unwrap();
        let expected_result = "1\n2\n3\n";
        assert_eq!(expected_result, res_string);
    }

    #[test]
    fn parsing3() {
        let content =
        b"1\n\
        >>> other\n\
        XXX\n\
        >>> *\n\
        2\n\
        >>>\n\
        3";
        let parser = Parser::new("thisone",">>>").unwrap();
        let mut res : Vec<u8> = Vec::new();
        parser.process(&content[..], &mut res).unwrap();
        let res_string = String::from_utf8(res).unwrap();
        let expected_result = "1\n2\n3\n";
        assert_eq!(expected_result, res_string);
    }

    #[test]
    fn parsing4() {
        let content =
        b"1\n\
        \t\t>>>other\n\
        XXX>>>   \n\
        XXX\n\
        >>>thisone\n\
        2\n\
        >>>\n\
        3
        >>>*\n\
        4";
        let parser = Parser::new("thisone",">>>").unwrap();
        let mut res : Vec<u8> = Vec::new();
        parser.process(&content[..], &mut res).unwrap();
        let res_string = String::from_utf8(res).unwrap();
        let expected_result = "1\n2\n3\n4\n";
        assert_eq!(expected_result, res_string);
    }

    #[test]
    fn parsing5() {
        let content =
        b"1\n\
        >>> another, thisone, andanother\n\
        2\n\
        >>> other\n\
        XXX\n\
        >>>\n\
        3";
        let parser = Parser::new("thisone",">>>").unwrap();
        let mut res : Vec<u8> = Vec::new();
        parser.process(&content[..], &mut res).unwrap();
        let res_string = String::from_utf8(res).unwrap();
        let expected_result = "1\n2\n3\n";
        assert_eq!(expected_result, res_string);
    }
}
