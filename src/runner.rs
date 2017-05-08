use exec;
use siphasher::sip::SipHasher;
use std::fs::{self, File};
use std::hash::Hasher;
use std::io::{self, Read, Write};
use std::os::unix::fs::PermissionsExt;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    ExecError(exec::Error)
}

pub struct Runner<'a> {
    cmd: &'a str,
    hash: u64,
    status: Option<i32>
}

impl<'a> Runner<'a> {
    pub fn new(cmd: &'a str) -> Runner<'a> {
        let mut hasher = SipHasher::new();
        hasher.write(cmd.as_bytes());

        Runner {
            cmd: cmd,
            hash: hasher.finish(),
            status: None
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
        let script = format!("#!/usr/bin/env bash\n{}\nprintf $? > {}.status", self.cmd, self.hash);
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

    pub fn cleanup(&self) {
        let _ = fs::remove_file(&self.script_name());
        let _ = fs::remove_file(&self.status_name());
    }

    pub fn cleanup_status(&mut self) -> Option<i32> {
        let status = self.status();
        self.cleanup();
        status
    }

    fn script_name(&self) -> String {
        format!("{}.sh", self.hash)
    }

    fn status_name(&self) -> String {
        format!("{}.status", self.hash)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn hash() {
        let cmd = "ruby stream.rb";
        let runner = Runner::new(cmd);
        assert_eq!(2392779401433226153, runner.hash);
        runner.cleanup();
    }

    #[test]
    fn write_script() {
        let cmd = "ruby stream.rb";
        let mut runner = Runner::new(cmd);
        let _ = runner.write_script();
        match File::open("2392779401433226153.sh") {
            Ok(mut file) => {
                let mut contents = String::new();
                let _ = file.read_to_string(&mut contents);
                assert_eq!(String::from("#!/usr/bin/env bash\nruby stream.rb\nprintf $? > 2392779401433226153.status"), contents);
            },
            _ => panic!("Script not found")
        }
        runner.cleanup();
    }
}
