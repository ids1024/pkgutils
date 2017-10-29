use std::path::Path;
use std::fs::{self, File};
use std::io::Write;

use ion_shell::Shell;
use ion_shell::shell::flags::ERR_EXIT;
use ion_shell::shell::library::IonLibrary;

use ::{PackageMeta, Repo, download};

pub struct Recipe {
    target: String,
    shell: Shell,
    #[allow(dead_code)]
    debug: bool,
}

impl Recipe {
    pub fn new(target: String, path: &Path, debug: bool) -> Recipe {
        let mut shell = Shell::new();
        shell.flags |= ERR_EXIT;
        shell.variables.set_var("DEBUG", if debug { "1" } else { "0" });

        shell.execute_script(path).unwrap();

        Recipe { target, shell, debug }
    }

    fn call_func(&mut self, func: &str, args: &[&str]) {
        if self.shell.functions.contains_key(func) || 
           self.shell.variables.aliases.contains_key(func)
        {
            let mut cmd = func.to_string();
            // NOTE no escaping is performed
            for arg in args {
                cmd.push(' ');
                cmd.push_str(arg);
            }
            self.shell.execute_command(&cmd);
        }
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
        let src = self.shell.variables.get_var("SRC").unwrap();
        download(&src, "source.tar").unwrap();
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

    pub fn build(&mut self) {
        self.call_func("build", &[]);
    }

    pub fn test(&mut self) {
        self.call_func("test", &[]);
    }

    pub fn clean(&mut self) {
        self.call_func("clean", &[]);
    }

    pub fn stage(&mut self) {
        self.unstage();
        fs::create_dir("stage").unwrap();
        self.call_func("stage", &["./stage"]);
    }

    pub fn unstage(&self) {
        fs::remove_dir_all("stage").unwrap();
    }
}
