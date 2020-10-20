use clap::{App, Arg, SubCommand, AppSettings, crate_version, crate_authors};

pub fn init() -> App<'static, 'static> {
    App::new("kitty")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A tool for interacting with Kattis via the command line")
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::SubcommandRequired)
        .subcommand(SubCommand::with_name("test")
                    .about("Runs a solution through the official test cases")
                    .setting(AppSettings::DisableVersion)
                    .arg(Arg::with_name("PATH")
                         .help("Path to problem directory")
                         .default_value(".")
                         .index(1))
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .takes_value(true)
                         .help("Name of source file to test. Necessary when there are multiple valid sources or when the program cannot recognise the file extension"))
                    .arg(Arg::with_name("language")
                         .short("l")
                         .long("lang")
                         .takes_value(true)
                         .help("Programming language of the solution. Write the typical file extension for the language (java for Java, py for python, js for JavaScript, etc.)."))
                   )
        .subcommand(SubCommand::with_name("get")
                    .about("Fetches a problem from Kattis by creating a directory of the same name and downloading the official test cases")
                    .setting(AppSettings::DisableVersion)
                    .arg(Arg::with_name("PROBLEM ID")
                         .help("ID of the Kattis problem. You can find the id in the URL of the problem page on kattis: open.kattis.com/problems/<PROBLEM ID>")
                         .required(true)
                         .index(1))
                   )
        .subcommand(SubCommand::with_name("submit")
                    .about("Submits a solution to Kattis")
                    .setting(AppSettings::DisableVersion)
                    .arg(Arg::with_name("PATH")
                         .help("Path to problem directory. Note that the directory name must match the problem id")
                         .default_value(".")
                         .index(1))
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .takes_value(true)
                         .help("Name of source file to test. Necessary when there are multiple valid sources or when the program cannot recognise the file extension"))
                    .arg(Arg::with_name("language")
                         .short("l")
                         .long("lang")
                         .takes_value(true)
                         .help("Programming language of the solution. Write the typical file extension for the language (java for Java, py for python, js for JavaScript, etc.)."))
                    .arg(Arg::with_name("yes")
                         .short("y")
                         .long("yes")
                         .help("Bypass the confirmation prompt by saying \"yes\" in advance"))
                   )
        .subcommand(SubCommand::with_name("history")
                    .about("Shows a list of your submissions to Kattis as seen on your profile page")
                    .setting(AppSettings::DisableVersion)
                    .arg(Arg::with_name("count")
                         .short("c")
                         .long("count")
                         .help("How many submissions to show")
                         .default_value("10"))
                    .arg(Arg::with_name("all")
                         .short("a")
                         .long("all")
                         .help("Show all submissions (if --count is present too, it is ignored)"))
                   )
        .subcommand(SubCommand::with_name("open")
                    .about("Opens a problem in the browser")
                    .setting(AppSettings::DisableVersion)
                    .arg(Arg::with_name("PROBLEM ID")
                         .help("The ID of the problem as seen in its URL. [default: the name of the current directory]")
                         .index(1))
                   )
}
