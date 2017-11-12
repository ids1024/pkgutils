extern crate ion_shell;
extern crate pkgutils;
extern crate clap;
extern crate termion;

use std::path::Path;
use std::process;

use pkgutils::{Recipe, CookError};
use clap::{App, Arg};
use termion::{color, style};

/// Print the name of a cook command being executed
fn print_cmd(cmd: &str) {
    eprintln!("{}{}cook - {}{}{}", 
             style::Bold,
             color::Fg(color::Yellow),
             cmd,
             color::Fg(color::Reset),
             style::Reset);
}

fn dist(recipe: &mut Recipe) -> Result<(), CookError> {
    print_cmd("prepare");
    recipe.prepare()?;
    print_cmd("build");
    recipe.build()?;
    print_cmd("stage");
    recipe.stage()?;
    print_cmd("tar");
    recipe.tar()?;
    Ok(())
}

fn main() {
    let matches = App::new("cook")
        .arg(Arg::with_name("target")
             .long("target")
             .takes_value(true)
             )
        .arg(Arg::with_name("debug")
             .long("debug")
             )
        .arg(Arg::with_name("template_dir")
             .long("template_dir")
             // TODO: Do not require this
             .required(true)
             .takes_value(true)
             )
        .arg(Arg::with_name("command")
             .multiple(true)
             .required(true)
             )
        .get_matches();

    let target = matches.value_of("target").unwrap_or(env!("PKG_DEFAULT_TARGET"));
    let debug = matches.is_present("debug");
    let template_dir = matches.value_of("template_dir").unwrap();

    let recipe_path = Path::new("recipe.ion");
    if !recipe_path.exists() {
        eprintln!("No recipe.ion in current directory");
    }

    let mut recipe = Recipe::new(target.to_string(), template_dir, recipe_path, debug).unwrap();

    for cmd in matches.values_of("command").unwrap() {
        print_cmd(cmd);

        let res = match cmd {
            "dist" => dist(&mut recipe),
            "distclean" => Ok(()), // XXX
            "fetch" => recipe.fetch(),
            "unfetch" => recipe.unfetch(),
            "status" => Ok(()), // XXX
            "status_origin" => Ok(()), // XXX
            "status_upstream" => Ok(()), // XXX
            "diff" => Ok(()), // XXX
            "diff_origin" => Ok(()), // XXX
            "diff_upstream" => Ok(()), // XXX
            "difftool" => Ok(()), // XXX
            "difftool_origin" => Ok(()), // XXX
            "difftool_upstream" => Ok(()), // XXX
            "update" => Ok(()), // XXX
            "prepare" => Ok(()), // XXX
            "unprepare" => recipe.unprepare(),
            "version" => match recipe.version() {
                Ok(version) => {
                    println!("{}", version);
                    Ok(())
                }
                Err(e) => Err(e)
            }
            "gitversion" => Ok(()), // XXX
            "build" => recipe.build(),
            "test" => recipe.test(),
            "clean" => recipe.clean(),
            "stage" => recipe.stage(),
            "unstage" => recipe.unstage(),
            "tar" => recipe.tar(),
            "untar" => recipe.untar(),
            "publish" => Ok(()), // XXX
            "unpublish" => Ok(()), // XXX
            _ => Ok(()) // XXX
        };

        if let Err(err) = res {
            eprintln!("cook: {} error: {}", cmd, err);
            process::exit(1);
        }
    }
}
