use exec;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use tempdir::TempDir;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ExecError(exec::Error)
}

pub struct Wrapper<'a> {
    cmd: &'a str,
    status: Option<i32>,
    tmpdir: &'a TempDir
}

impl<'a> Wrapper<'a> {
    pub fn new(cmd: &'a str, tmpdir: &'a TempDir) -> Wrapper<'a> {
        Wrapper {
            cmd: cmd,
            status: None,
            tmpdir: tmpdir
        }
    }

    pub fn exec(&mut self) -> Error {
        match self.write_script() {
            Ok(_) => {
                let error = exec::Command::new("bash").arg(&self.script_name()).exec();
                Error::ExecError(error)
            },
            Err(error) => Error::IoError(error)
        }
    }

    pub fn write_script(&mut self) -> io::Result<()> {
        let script = format!("#!/usr/bin/env bash\n{}\nprintf $? > {:?}", self.cmd, self.status_name());
        match File::create(&self.script_name()) {
            Ok(mut file) => {
                let _ = file.write_all(script.as_bytes());
                let meta = file.metadata().unwrap();
                let mut perm = meta.permissions();
                perm.set_mode(0o744);
                Ok(())
            },
            Err(error) => Err(error)
        }
    }

    pub fn status(&mut self) -> Option<i32> {
        if self.status.is_some() { return self.status; }

        match File::open(&self.status_name()) {
            Ok(ref mut file) => {
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);
                self.status = Some(contents.trim().parse::<i32>().unwrap());
                self.status
            },
            _ => None
        }
    }

    pub fn script_name(&self) -> PathBuf {
        let mut pb = PathBuf::new();
        pb.push(self.tmpdir.path());
        pb.set_file_name("wrapper.sh");
        pb
    }

    pub fn status_name(&self) -> PathBuf {
        let mut pb = PathBuf::new();
        pb.push(self.tmpdir.path());
        pb.set_file_name("wrapper.status");
        pb
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn write_script() {
        let cmd = "ruby stream.rb";
        let tmpdir = TempDir::new("redactor").unwrap();
        let mut wrapper = Wrapper::new(cmd, &tmpdir);
        let result = wrapper.write_script();
        assert!(result.is_ok());

        match File::open(wrapper.script_name()) {
            Ok(mut file) => {
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);
                assert_eq!(
                    format!("#!/usr/bin/env bash\nruby stream.rb\nprintf $? > {:?}", wrapper.status_name()),
                    contents
                );
            },
            _ => panic!("Script not found")
        }
    }
}
