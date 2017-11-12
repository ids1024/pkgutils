use std;
use std::path::Path;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::fmt::{self, Display, Formatter};
use std::ffi::OsStr;

use ion_shell::{Shell, Capture, IonError};

use ::{PackageMeta, Repo, download};

#[allow(dead_code)]
enum Source {
    Git(String, Option<String>),
    Tar(String)
}

#[derive(Debug)]
pub enum CookError {
    IO(io::Error),
    Ion(IonError),
    MissingVar(String),
    NonZero(String, i32),
}

impl From<io::Error> for CookError {
    fn from(err: io::Error) -> CookError {
        CookError::IO(err)
    }
}

impl From<IonError> for CookError {
    fn from(err: IonError) -> CookError {
        CookError::Ion(err)
    }
}

impl Display for CookError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            CookError::IO(ref e) => e.fmt(fmt),
            CookError::Ion(ref e) => e.fmt(fmt),
            CookError::MissingVar(ref var) =>
                fmt.write_fmt(format_args!("Recipe missing '{}' variable", var)),
            CookError::NonZero(ref func, status) =>
                fmt.write_fmt(format_args!("Function {}() returned {}'", func, status)),
        }
    }
}

type Result<T> = std::result::Result<T, CookError>;

pub struct Recipe {
    target: String,
    shell: Shell,
    #[allow(dead_code)]
    debug: bool,
}

fn call_func(shell: &mut Shell, func: &str, args: &[&str]) -> Result<()> {
    let mut args_vec = vec![func];
    args_vec.extend(args);
    match shell.execute_function(func, &args_vec) {
        Err(IonError::DoesNotExist) => Ok(()),
        Err(e) => Err(e.into()),
        Ok(0) => Ok(()),
        Ok(status) => Err(CookError::NonZero(func.to_string(), status)),
    }
}

impl Recipe {
    pub fn new<T1: AsRef<Path>, T2: AsRef<Path>>(target: String, template_dir: T1, path: T2, debug: bool) -> Result<Recipe> {
        let mut shell = Shell::new();
        //XXX shell.flags |= ERR_EXIT;
        shell.set_var("DEBUG", if debug { "1" } else { "0" });

        for entry in fs::read_dir(template_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() &&
               entry.path().extension() == Some(OsStr::new("ion")) {
                shell.execute_script(entry.path())?;
            }
        }

        shell.execute_script(path.as_ref())?;

        Ok(Recipe { target, shell, debug })
    }

    fn call_func(&mut self, func: &str, args: &[&str]) -> Result<()> {
        call_func(&mut self.shell, func, args)
    }

    /// Return the metadata, from which /pkg/<package>.toml is generated.
    /// This calls the recipe's version(), so it will fail if that does.
    pub fn meta(&mut self) -> Result<PackageMeta> {
        let version = self.version()?;
        let name = self.shell.get_var("name")
            .ok_or(CookError::MissingVar("name".to_string()))?;
        let depends = self.shell.get_array("depends").unwrap_or(&[]);
        Ok(PackageMeta {
            name: name.clone(),
            version: version.to_string(),
            target: self.target.clone(),
            depends: depends.to_vec(),
        })
    }

    pub fn tar(&mut self) -> Result<()> {
        let meta = self.meta()?;
        fs::create_dir_all("stage/pkg")?;
        let mut manifest = File::create(format!("stage/pkg/{}.toml", meta.name))?;
        manifest.write_all(meta.to_toml().as_bytes())?;
        drop(manifest);

        let repo = Repo::new(&self.target);
        repo.create("stage")?;
        Ok(())
    }

    pub fn untar(&self) -> Result<()> {
        if let Err(err) = fs::remove_file("stage.tar") {
            if err.kind() != io::ErrorKind::NotFound {
                return Err(err.into());
            }
        }
        Ok(())
    }

    pub fn fetch(&self) -> Result<()> {
        let src = self.shell.get_var("src")
            .ok_or(CookError::MissingVar("src".to_string()))?;
        download(&src, "source.tar")?;
        Ok(())
    }

    pub fn unfetch(&self) -> Result<()> {
        fs::remove_dir_all("source")?;
        fs::remove_file("source.tar")?;
        Ok(())
    }

    //fn prepare(&self) {
    //    unprepare();
    //}

    pub fn unprepare(&self) -> Result<()> {
        if let Err(err) = fs::remove_dir_all("build") {
            if err.kind() != io::ErrorKind::NotFound {
                return Err(err.into());
            }
        }
        Ok(())
    }

    pub fn build(&mut self) -> Result<()> {
        self.call_func("build", &[])
    }

    pub fn test(&mut self) -> Result<()> {
        self.call_func("test", &[])
    }

    pub fn clean(&mut self) -> Result<()> {
        self.call_func("clean", &[])
    }

    pub fn stage(&mut self) -> Result<()> {
        self.unstage()?;
        fs::create_dir("stage")?;
        let path = fs::canonicalize("./stage")?;
        self.call_func("stage", &[path.to_str().unwrap()])
    }

    pub fn unstage(&self) -> Result<()> {
        if let Err(err) = fs::remove_dir_all("stage") {
            if err.kind() != io::ErrorKind::NotFound {
                return Err(err.into());
            }
        }
        Ok(())
    }

    pub fn version(&mut self) -> Result<String> {
        let mut ver = String::new();
        let res = self.shell.fork(Capture::Stdout, |shell| {
            call_func(shell, "version", &[]).unwrap();
        })?;
        res.stdout.unwrap().read_to_string(&mut ver)?;
        // XXX non-zero return
        if ver.ends_with("\n") {
            ver.pop();
        }
        Ok(ver)
    }
}
