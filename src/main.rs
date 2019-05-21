use clap::{App, Arg};
use hostname::get_hostname;
use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let hostname = get_hostname().unwrap();
    let cliargs = App::new("SSCCPP, the Simple Switch Case Config PreProcessor")
        .version("0.1.0")
        .author("Arthur Carcano <arthur.carcano@inria.fr>")
        .about("SSCCPP parses a configuration file to adapt it to the current host.")
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .value_name("DELIMITER")
                .takes_value(true)
                .default_value(">>>>>>>>"),
        )
        .arg(
            Arg::with_name("path")
                .required(true)
                .index(1)
                .help("Path of the input file to use."),
        )
        .arg(
            Arg::with_name("ident")
                .short("id")
                .long("ident")
                .value_name("MACHINE_NAME")
                .takes_value(true)
                .default_value(&hostname),
        )
        .get_matches();

    // Opts unwrapping
    let filepath = cliargs.value_of("path").unwrap();
    let delimiter = cliargs.value_of("delimiter").unwrap();
    let ident = cliargs.value_of("ident").unwrap();

    // Regex creation
    let regex_string = format!(r#"^\s*{delimiter}\s*(\S+)?\s*$"#, delimiter = delimiter);
    let re = Regex::new(&regex_string).unwrap();

    let file = File::open(filepath)?;
    let bufed_file = BufReader::new(file);
    let mut print_next = true;
    for line in bufed_file.lines() {
        let line = line?;
        let captures = re.captures(&line);
        match captures {
            None => {
                if print_next {
                    println!("{}", &line)
                }
            }
            Some(caps) => match caps.get(1) {
                None => print_next = true,
                Some(captured_id) => print_next = captured_id.as_str() == ident,
            },
        }
    }

    Ok(())
}
