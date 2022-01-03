use std::path::PathBuf;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use dgit::LocalRepository;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("arg")
                .required(false)
                .help("Paths or urls to the git repositories to upload")
                .index(1)
                .multiple(true),
        )
        .get_matches();

    match matches.values_of("arg") {
        None => (),
        Some(values) => values.for_each(|path| {
            match LocalRepository::new(path) {
                Err(e) => panic!("{}", e),
                Ok(repo) => repo,
            }
            .objects()
            .for_each(|x| match x {
                Ok(x) => println!("{}", hex::encode(x)),
                Err(e) => eprintln!("{}", e),
            });
        }),
    }
}
