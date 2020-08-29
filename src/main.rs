use clap::{App, Arg, SubCommand};
use anyhow::Result;
use reqq::Reqq;

fn main() -> Result<()> {
    let matches = App::new("reqq").version("1.0.0")
        .author("Seth Etter <mail@sethetter.com>")
        .about("You know..")

        // TODO: optional --dir option to override default of .reqq

        // .arg(Arg::with_name("env")
        //     .short("e")
        //     .long("env")
        //     .value_name("ENV")
        //     .help("Specifies the environment config file to use")
        //     .takes_value(true))

        .arg(Arg::with_name("REQUEST")
            .help("The name of the request to execute.")
            .index(1))
        .subcommand(SubCommand::with_name("list")
            .about("Lists available requests"))
        .get_matches();

    let reqq = Reqq::new(".reqq".to_owned())?;

    if let Some(_) = matches.subcommand_matches("list") {
        // List subcommand.
        for req_name in reqq.list_reqs().into_iter() {
            println!("{}", req_name);
        }
    } else {
        // Default behavior of executing a request
        // let req = matches.value_of("REQUEST").expect("Must provide a request.");
        // reqq.execute(req.to_owned())?;
    }
    Ok(())
}


