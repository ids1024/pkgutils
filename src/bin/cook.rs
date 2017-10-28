extern crate ion_shell;
extern crate pkgutils;
extern crate clap;

use std::path::Path;

use pkgutils::Recipe;
use clap::{App, Arg};

fn main() {
    let matches = App::new("cook")
        .arg(Arg::with_name("target")
             .long("target")
             .takes_value(true)
             )
        .arg(Arg::with_name("debug")
             .long("debug")
             )
        .arg(Arg::with_name("command")
             .multiple(true)
             .required(true)
             )
        .get_matches();

    let target = matches.value_of("target").unwrap_or(env!("PKG_DEFAULT_TARGET"));
    let debug = matches.is_present("debug");

    let recipe_path = Path::new("recipe.ion");
    if !recipe_path.exists() {
        eprintln!("No recipe.ion in current directory");
    }

    let mut recipe = Recipe::new(target.to_string(), recipe_path, debug);

    for cmd in matches.values_of("command").unwrap() {
        match cmd {
            "dist" => {
                //prepare
                //build
                recipe.stage();
                recipe.tar();
            }
            "distclean" => { /*XXX*/ }
            "fetch" => recipe.fetch(),
            "unfetch" => recipe.unfetch(),
            "status" => { /*XXX*/ }
            "status_origin" => { /*XXX*/ }
            "status_upstream" => { /*XXX*/ }
            "diff" => { /*XXX*/ }
            "diff_origin" => { /*XXX*/ }
            "diff_upstream" => { /*XXX*/ }
            "difftool" => { /*XXX*/ }
            "difftool_origin" => { /*XXX*/ }
            "difftool_upstream" => { /*XXX*/ }
            "update" => { /*XXX*/ }
            "prepare" => { /*XXX*/ }
            "unprepare" => recipe.unprepare(),
            "version" => { /*XXX*/ }
            "gitversion" => { /*XXX*/ }
            "build" => { /*XXX*/ }
            "test" => { /*XXX*/ }
            "clean" => { /*XXX*/ }
            "stage" => recipe.stage(),
            "unstage" => recipe.unstage(),
            "tar" => recipe.tar(),
            "untar" => { /*XXX*/ }
            "publish" => { /*XXX*/ }
            "unpublish" => { /*XXX*/ }
            _ => { /*XXX*/ }
        }
    }
}
