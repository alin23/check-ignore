use ansi_term::Colour::{Green, Yellow};
use atty::Stream;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::{path::Path, path::PathBuf};
use structopt::clap::AppSettings;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "check-ignore",
    about = "Exits with non-zero code if files provided don't match at least one pattern.\nOutputs results of the form `pattern => file`.",
    global_settings = &[AppSettings::ColoredHelp]
)]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Use global gitignore
    #[structopt(short, long)]
    global: bool,

    /// Also print whitelisted files
    #[structopt(short, long)]
    whitelist: bool,

    /// Disable colorful output
    #[structopt(long, env)]
    no_color: bool,

    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// .gitignore file
    #[structopt(short, long, default_value = ".gitignore", parse(from_os_str))]
    ignore_file: PathBuf,

    /// Root for checking file
    #[structopt(short, long, parse(from_os_str))]
    root: Option<PathBuf>,

    /// Files to check
    #[structopt(name = "FILES", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn new_gitignore_with_root<P: AsRef<Path>>(gitignore_path: P, root: Option<&Path>) -> Gitignore {
    let path = gitignore_path.as_ref();
    let parent = root.unwrap_or(path.parent().unwrap_or(Path::new("/")));
    let mut builder = GitignoreBuilder::new(parent);
    builder.add(path);
    match builder.build() {
        Ok(gi) => gi,
        Err(_) => Gitignore::empty(),
    }
}

fn main() {
    let opt = Opt::from_args();
    let gitignore: Gitignore;
    if opt.global {
        gitignore = Gitignore::global().0;
    } else if let Some(root) = opt.root.clone() {
        gitignore = new_gitignore_with_root(opt.ignore_file.clone(), Some(&root));
    } else {
        gitignore = Gitignore::new(opt.ignore_file.clone()).0;
    }

    let matched_files = opt.files.iter().filter_map(|file| {
        let file_path = file.clone();
        let canonical_file = file.canonicalize().unwrap_or(file.clone());
        match gitignore.matched_path_or_any_parents(canonical_file, file_path.is_dir()) {
            ignore::Match::None => None,
            ignore::Match::Ignore(glob) => Some((glob, file)),
            ignore::Match::Whitelist(glob) => {
                if opt.whitelist {
                    Some((glob, file))
                } else {
                    None
                }
            }
        }
    });

    #[cfg(windows)]
    let ansi_colors_support =
        ansi_term::enable_ansi_support().is_ok() || std::env::var_os("TERM").is_some();

    #[cfg(not(windows))]
    let ansi_colors_support = true;

    let interactive_terminal = atty::is(Stream::Stdout);
    let colored_output = !opt.no_color && ansi_colors_support && interactive_terminal;

    let mut status_code = 1;
    for (glob, file) in matched_files {
        status_code = 0;
        if !colored_output {
            println!("{} => {}", glob.original(), file.to_string_lossy());
        } else if glob.is_whitelist() {
            println!(
                "{} => {}",
                Green.paint(glob.original()),
                Green.paint(file.to_string_lossy())
            );
        } else {
            println!(
                "{} => {}",
                Yellow.paint(glob.original()),
                Yellow.paint(file.to_string_lossy())
            );
        }
    }
    std::process::exit(status_code)
}
