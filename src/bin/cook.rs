extern crate ion_shell;
extern crate pkgutils;

use std::path::Path;
use std::fs::{self, File};
use std::io::Write;
use std::env;
use pkgutils::{PackageMeta, Repo};
use ion_shell::{Builtin, Shell};
use ion_shell::shell::library::IonLibrary;
use ion_shell::shell::flags::ERR_EXIT;

fn main() {
    let target = env!("PKG_DEFAULT_TARGET");

    let builtins = Builtin::map();
    let mut shell = Shell::new(&builtins);
    shell.flags |= ERR_EXIT;

    let recipe = Path::new("recipe.ion");
    if !recipe.exists() {
        eprintln!("No recipe.ion in current directory");
    }
    shell.execute_script(recipe).unwrap();

    let cmd = env::args().nth(1).unwrap();

    match cmd.as_str() {
        "dist" => {
            //prepare
            //build
            stage(&mut shell);
            tar(&shell, target);
        }
        "distclean" => { /*XXX*/ }
        "fetch" => fetch(&mut shell),
        "unfetch" => unfetch(),
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
        "unprepare" => unprepare(),
        "version" => { /*XXX*/ }
        "gitversion" => { /*XXX*/ }
        "build" => { /*XXX*/ }
        "test" => { /*XXX*/ }
        "clean" => { /*XXX*/ }
        "stage" => stage(&mut shell),
        "unstage" => unstage(),
        "tar" => tar(&shell, target),
        "untar" => { /*XXX*/ }
        "publish" => { /*XXX*/ }
        "unpublish" => { /*XXX*/ }
        _ => { /*XXX*/ }
    }
}

fn tar(shell: &Shell, target: &str) {
    let version = "1"; // XXX
    let name = shell.variables.get_var("NAME").expect("Package missing NAME");
    let meta = PackageMeta {
        name: name.clone(),
        version: version.to_string(),
        target: target.to_string(),
    };

    fs::create_dir_all("stage/pkg").unwrap();
    let mut manifest = File::create(format!("stage/pkg/{}.toml", name)).unwrap();
    manifest.write_all(meta.to_toml().as_bytes()).unwrap();
    drop(manifest);

    let repo = Repo::new(target);
    repo.create("stage").unwrap();
}

fn call_func(shell: &mut Shell, func: &str, args: &[&str]) -> bool {
    shell.variables.set_var("skip", "0");
    if shell.functions.contains_key(func) {
        let mut cmd = func.to_string();
        // NOTE no escaping is performed
        for arg in args {
            cmd.push(' ');
            cmd.push_str(arg);
        }
        shell.execute_command(&cmd);
    }
    shell.variables.get_var("skip") == Some("1".to_string())
}

fn fetch(shell: &mut Shell) {
}

fn unfetch() {
    fs::remove_dir_all("source");
    fs::remove_file("source.tar");
}

//fn prepare() {
//    unprepare();
//}

fn unprepare() {
    fs::remove_dir_all("build");
}

fn stage(shell: &mut Shell) {
    unstage();
    fs::create_dir("stage");
    let skip = call_func(shell, "stage", &["./stage"]);
}

fn unstage() {
    fs::remove_dir_all("stage");
}
