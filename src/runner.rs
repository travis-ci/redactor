use exec;
use siphasher::sip::SipHasher;
use std::fs::{self, File};
use std::hash::Hasher;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;

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

    fn script_name(&self) -> String {
        format!("{}.sh", self.hash)
    }

    fn status_name(&self) -> String {
        format!("{}.status", self.hash)
    }

    pub fn exec(&mut self) -> exec::Error {
        // insert cmd into bash script
        let script = format!("#!/usr/bin/env bash\n{}\nprintf $? > {}.status", self.cmd, self.hash);
        let mut file = match File::create(&self.script_name()) {
            Ok(file) => file,
            _ => panic!("Could not create bash script")
        };
        let _ = file.write_all(script.as_bytes());

        // set chmod +x
        let meta = file.metadata().unwrap();
        let mut perm = meta.permissions();
        perm.set_mode(0o744);

        // exec
        exec::Command::new("bash").arg(&self.script_name()).exec()
    }

    pub fn status(&mut self) -> Option<i32> {
        if self.status.is_some() { return self.status; }

        // if status file exists read status, convert to int and return
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
}
