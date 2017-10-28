use std::path::Path;
use std::fs::{self, File};
use std::io::Write;

use ion_shell::Shell;
use ion_shell::shell::flags::ERR_EXIT;
use ion_shell::shell::library::IonLibrary;

use ::{PackageMeta, Repo};

pub struct Recipe {
    target: String,
    shell: Shell,
}

impl Recipe {
    pub fn new(target: String, path: &Path) -> Recipe {
        let mut shell = Shell::new();
        shell.flags |= ERR_EXIT;

        shell.execute_script(path).unwrap();

        Recipe { target, shell }
    }

    fn call_func(&mut self, func: &str, args: &[&str]) -> bool {
        self.shell.variables.set_var("skip", "0");
        if self.shell.functions.contains_key(func) {
            let mut cmd = func.to_string();
            // NOTE no escaping is performed
            for arg in args {
                cmd.push(' ');
                cmd.push_str(arg);
            }
            self.shell.execute_command(&cmd);
        }
        self.shell.variables.get_var("skip") == Some("1".to_string())
    }

    pub fn tar(&self) {
        let version = "1"; // XXX
        let name = self.shell.variables.get_var("NAME").expect("Package missing NAME");
        let meta = PackageMeta {
            name: name.clone(),
            version: version.to_string(),
            target: self.target.clone(),
        };

        fs::create_dir_all("stage/pkg").unwrap();
        let mut manifest = File::create(format!("stage/pkg/{}.toml", name)).unwrap();
        manifest.write_all(meta.to_toml().as_bytes()).unwrap();
        drop(manifest);

        let repo = Repo::new(&self.target);
        repo.create("stage").unwrap();
    }

    pub fn fetch(&self) {
    }

    pub fn unfetch(&self) {
        fs::remove_dir_all("source").unwrap();
        fs::remove_file("source.tar").unwrap();
    }

    //fn prepare(&self) {
    //    unprepare();
    //}

    pub fn unprepare(&self) {
        fs::remove_dir_all("build").unwrap();
    }

    pub fn stage(&mut self) {
        self.unstage();
        fs::create_dir("stage").unwrap();
        let _skip = self.call_func("stage", &["./stage"]);
    }

    pub fn unstage(&self) {
        fs::remove_dir_all("stage").unwrap();
    }
}
