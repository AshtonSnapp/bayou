//--> Imports & Modules <--

use clap::{self, Arg, ArgAction, ArgGroup, Command, ValueHint};
use color_eyre;

//--> Type Aliases <--

//--> Structs <--

//--> Enums <--

//--> Functions & Impls <--

#[tokio::main]
async fn main() {
    color_eyre::install();

    dbg!(spdx::text::LICENSE_TEXTS);

    let mut cmd = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("The package & project manager for the Baton runtime.")
        .long_about("\
        Bayou is the package and project management program for languages running on the Baton runtime.\
        You can use Bayou to download already compiled programs or libraries, or to start and manage your own projects.\
        ")
        .arg(Arg::new("version")
            .short('V')
            .long("version")
            .action(ArgAction::Version)
            .exclusive(true)
            .help("Prints the Bayou version.")
        )
        .subcommand(Command::new("start")
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about("Start a project.")
            //.arg(Arg::new("template")
            //    .short('t')
            //    .long("template")
            //    .action(ArgAction::Set)
            //    .value_hint(ValueHint::Url)
            //    .help("Start the project using a remote template.")
            //)
            //.arg(Arg::new("git")
            //    .short('g')
            //    .long("git")
            //    .action(ArgAction::SetTrue)
            //    .default_value(false)
            //    .help("Create a Git repository for this project. Assumed if template ends in '.git'")
            //)
        )
        .subcommand(Command::new("build")
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about("Build your project.")
            .long_about("\
            Build your project. This requires a 'Bayou.ron' project manifest file in the current working directory.\
            \
            In order to build a project, Bayou needs to know several things such as the language you're using, what dependencies your project needs to link to,\
             among multiple other things. All of this information is stored in your project's manifest file, 'Bayou.ron'. This file utilizes the Rusty Object Notation\
             (RON) syntax, and should have been automatically created when you started your project using 'bayou start'.")
        )
        .subcommand(Command::new("run")
            .version(clap::crate_version!())
            .author(clap::crate_authors!())
            .about("Build and run your project.")
        );
    
    cmd.build();

    let args = cmd.get_matches();
}
