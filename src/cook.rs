use std::path::Path;
use std::fs::{self, File};
use std::io::{Read, Write};

use ion_shell::Shell;
use ion_shell::shell::IonError;
use ion_shell::shell::flags::ERR_EXIT;

use ::{PackageMeta, Repo, download};

#[allow(dead_code)]
enum Source {
    Git(String, Option<String>),
    Tar(String)
}

pub struct Recipe {
    target: String,
    shell: Shell,
    #[allow(dead_code)]
    debug: bool,
}

fn call_func(shell: &mut Shell, func: &str, args: &[&str]) {
    match shell.execute_function(func, args) {
        Err(IonError::DoesNotExist) => {},
        Err(e) => Err(e).unwrap(),
        Ok(_status) => {},
    }
}

impl Recipe {
    pub fn new(target: String, path: &Path, debug: bool) -> Recipe {
        let mut shell = Shell::new();
        shell.flags |= ERR_EXIT;
        shell.set_var("DEBUG", if debug { "1" } else { "0" });

        shell.execute_script(path).unwrap();

        Recipe { target, shell, debug }
    }

    fn call_func(&mut self, func: &str, args: &[&str]) {
        call_func(&mut self.shell, func, args);
    }

    pub fn tar(&mut self) {
        let version = self.version();
        let name = self.shell.get_var("name").expect("Package missing 'name'");
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
        let src = self.shell.get_var("src").unwrap();
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
        let _ = fs::remove_dir_all("stage");
    }

    pub fn version(&mut self) -> String {
        let mut ver = String::new();
        let mut res = self.shell.fork(|shell| call_func(shell, "version", &["version"])).unwrap();
        res.stdout.read_to_string(&mut ver).unwrap();
        if ver.ends_with("\n") {
            ver.pop();
        }
        ver
    }
}
