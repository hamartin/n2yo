pub fn get_arguments() -> clap::ArgMatches<'static> {
    clap::App::new("N2YO")
        .version("0.1")
        .author("Hans Åge Martinsen <hamartin@moshwire.com>")
        .about("A utility to get location of satellites from N2YO.")
        .arg(clap::Arg::with_name("id")
            .help("The NORAD ID of the satellite.")
            .long("id")
            .required(true)
            .takes_value(true)
            .value_name("ID"))
        .arg(clap::Arg::with_name("seconds")
            .help("The span in seconds to get information for. Defaults to 1 seconds.")
            .long("seconds")
            .required(false)
            .takes_value(true)
            .default_value("1")
            .value_name("SECONDS"))
        .arg(clap::Arg::with_name("filename")
            .help("The filename to store satellite information in.")
            .long("filename")
            .required(false)
            .takes_value(true)
            .default_value("n2yo.json")
            .value_name("FILENAME"))
        .after_help(
"
This application retrieves information about a specified satellite.

The general argument format for using this application is
satellite --id 12345
satellite --id 12345 --seconds 2

Don't set seconds to anything other than 1 for now.

"
        )
        .get_matches()
}
