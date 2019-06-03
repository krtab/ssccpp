use ssccpp::Parser;
use std::path::PathBuf;

use clap::{App, Arg};
use hostname::get_hostname;

use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::io::{self, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let hostname = get_hostname().unwrap();
    let cliargs = App::new("SSCCPP, the Simple Switch Case Config PreProcessor")
        .version("0.1.0")
        .author("Arthur Carcano <arthur.carcano@inria.fr>")
        .about("ssccpp-batch applies ssccpp to all files in a directory, copying the directory structure into another one.")
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .value_name("DELIMITER")
                .takes_value(true)
                .default_value(">>>>>>>>"),
        )
        .arg(
            Arg::with_name("ident")
                .short("id")
                .long("ident")
                .value_name("MACHINE_NAME")
                .takes_value(true)
                .default_value(&hostname),
        )
        .arg(
            Arg::with_name("from")
            .required(true)
            .help("Path of the input directory."),
        )
        .arg(
            Arg::with_name("into")
            .required(true)
            .help("Path of the output directory.")
        )
        .get_matches();

    // Opts unwrapping
    let fromstr = cliargs.value_of("from").unwrap();
    let intostr = cliargs.value_of("into").unwrap();
    let frompath = Path::new(fromstr);
    let intopath = Path::new(intostr);
    let delimiter = cliargs.value_of("delimiter").unwrap();
    let ident = cliargs.value_of("ident").unwrap();

    let parser = Parser::new(ident, delimiter).unwrap();

    let mut pathqueue: VecDeque<PathBuf> = VecDeque::new();
    pathqueue.push_back(frompath.to_path_buf());
    while !pathqueue.is_empty() {
        let entryfrompath = pathqueue.pop_front().unwrap();
        let basename = entryfrompath.strip_prefix(frompath).unwrap();
        let entryisdir = entryfrompath.is_dir();
        let entryintopath = intopath.join(basename);
        println!(
            "{} -> {} ({})",
            entryfrompath.display(),
            entryintopath.display(),
            if entryisdir { "D" } else { "F" }
        );
        if entryisdir {
            for subentry in fs::read_dir(entryfrompath)? {
                pathqueue.push_back(subentry?.path())
            }
            {
                use std::io::ErrorKind;
                match fs::create_dir(entryintopath) {
                    Ok(()) => (),
                    Err(ref e) if e.kind() == ErrorKind::AlreadyExists => (),
                    err => return err,
                }
            }
        } else {
            let mut intofile = OpenOptions::new()
                .write(true)
                .create(true)
                .open(entryintopath)?;
            let fromfile = OpenOptions::new().read(true).open(entryfrompath)?;
            let fromfile = BufReader::new(fromfile);
            parser.process(fromfile, &mut intofile)?;

        }
    }
    Ok(())
}
