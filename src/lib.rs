use std::io::{BufRead, Write, Error as IoError};
use regex::{Regex, self};

#[derive(Debug)]
pub struct LineDiscriminer {
    delimiter_regex : Regex,
    switchon_regex : Regex,
}

#[derive(Debug)]
pub enum LineAction {
    SwitchOn,
    SwitchOff,
    Process,
}

impl LineDiscriminer {

    pub fn new(ident: &str, delimiter : &str) -> Result<LineDiscriminer,regex::Error> {
        let delimiter_regex_string = format!(r#"^\s*{delimiter}\s*"#, delimiter = regex::escape(delimiter));
        let switchon_regex_string = format!(r#"^({ident}\s*)?$"#, ident = regex::escape(ident));
        Ok(LineDiscriminer {
            delimiter_regex: Regex::new(&delimiter_regex_string)?,
            switchon_regex: Regex::new(&switchon_regex_string)?
        })
    }


    pub fn discriminate(self : &Self, s : &str) -> LineAction {
        use LineAction::*;
        match self.delimiter_regex.find(s) {
            None => Process,
            Some(match_range) => {
                if self.switchon_regex.is_match(&s[match_range.end()..]) {
                    SwitchOn
                } else {
                    SwitchOff
                }
            }
        }
    }

    pub fn process<I,O>(self : &Self, input : I, output : &mut O) -> Result<(),IoError>
    where
        I : BufRead,
        O : Write,
        {
            use LineAction::*;
            let mut output_next = true;
            for line in input.lines() {
                let line = line?;
                match self.discriminate(&line) {
                    Process => {
                        if output_next {
                            writeln!(output,"{}",&line)?
                        }
                    }
                    SwitchOn => output_next = true,
                    SwitchOff => output_next = false,
                    }
                }
            Ok(())
        }
}
