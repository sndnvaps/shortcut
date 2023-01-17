use mslnk::ShellLink;
use clap::Command;

pub const FULL_TEMPLATE: &str = "\
{before-help}   {name} {version}
Author: {author-with-newline}{about-with-newline}
    Example: {name} -o f:\\rust\\shortcut\\123.txt -d f:\\rust\\shortcut\\355.lnk

{usage-heading} {usage}
{all-args}{after-help}";


fn main()  {

    let cmd = Command::new("shortcut")
        .author("sndnvaps<admin@sndnvaps.com>")
        .bin_name("shortcut")
        .about("make shortcut for windows's app")
        .version("v0.1.0")
        .help_template(FULL_TEMPLATE)
        .arg(clap::arg!(-o --origin <origin> "the windows app file to make shortcut")
            .required(true)
            .value_parser(clap::value_parser!(std::path::PathBuf)))
        .arg(clap::arg!(-d --destination <destination> "the output of the shortcut, save as suffix .lnk")
            .required(true)
            .value_parser(clap::value_parser!(std::path::PathBuf)));
   
    let arg_matches = cmd.get_matches();

    let target = arg_matches.get_one::<std::path::PathBuf>("origin").unwrap();
    
    let lnk =  arg_matches.get_one::<std::path::PathBuf>("destination").unwrap();
    
    let  sl = ShellLink::new(target).unwrap();
    println!("target = {:?},lnk = {:?}",target,lnk);
    sl.create_lnk(lnk).unwrap();
}
