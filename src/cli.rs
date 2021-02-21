use clap::{App, Arg, ArgMatches};

pub fn args<'a>() -> ArgMatches<'a> {
    App::new("Serve")
        .version("0.1")
        .about("Simple http server based on iouring")
        .arg(
            Arg::with_name("ip")
                .long("ip")
                .value_name("ADDRESS")
                .help("Ip address to bind to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("PORT")
                .help("Port number to listen to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level ov verbosity"),
        )
        .get_matches()
}
