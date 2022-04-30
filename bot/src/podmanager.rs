use std::process::{Command, Stdio};
use std::io::Write;

pub struct Pod {
    id: String,
}

impl Pod {
    fn new_from_tag(tag: &str) -> Result<Pod, String> {
        // id="$(podman container create "$tag")"
        // podman container start "$id"

        let output = Command::new("podman")
            .arg("container")
            .arg("create")
            .arg("--network=none")
            .arg(tag)
            .arg("sleep")
            .arg("10")
            .stderr(Stdio::inherit())
            .output();
        let output = match output {
            Ok(output) => output,
            Err(err) => return Err(format!("Creating pod failed: {}", err)),
        };

        if !output.status.success() {
            return Err("Creating pod failed".into());
        }

        let id = match String::from_utf8(output.stdout) {
            Ok(id) => id.trim().to_string(),
            Err(err) => return Err(format!("Creating pod failed: {}", err)),
        };

        let output = Command::new("podman")
            .arg("container")
            .arg("start")
            .arg(&id)
            .stderr(Stdio::inherit())
            .output();
        let output = match output {
            Ok(output) => output,
            Err(err) => return Err(format!("Creating pod failed: {}", err)),
        };

        if !output.status.success() {
            return Err("Starting pod failed".into());
        }

        Ok(Pod { id })
    }

    pub fn execute(&self, language: &str, content: &str) -> Result<String, String> {
        // echo "$content" | podman exec -i "$id" ./scripts/run.sh "$language"

        let child = Command::new("podman")
            .arg("exec")
            .arg("-i")
            .arg(&self.id)
            .arg("./scripts/run.sh")
            .arg(language)
            .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
            .spawn();
        let mut child = match child {
            Ok(child) => child,
            Err(err) => return Err(format!("Running program failed: {}", err)),
        };

        if let Some(stdin) = &mut child.stdin {
            match stdin.write_all(&content.as_bytes()) {
                Ok(()) => (),
                Err(err) => return Err(format!("Running program failed: {}", err)),
            }
        }

        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(err) => return Err(format!("Running program failed: {}", err)),
        };

        let mut errmsg: Option<String> = None;
        {
            let msg = String::from_utf8_lossy(&output.stderr).trim().to_string();
            if msg.len() > 0 {
                errmsg = Some(msg);
            }
        }

        let mut outmsg: Option<String> = None;
        {
            let msg = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if msg.len() > 0 {
                outmsg = Some(msg);
            }
        }

        if !output.status.success() {
            let exiterr = match output.status.code() {
                Some(code) => format!("Code {}", code),
                None => "Exited by signal".into(),
            };

            if let Some(errmsg) = errmsg {
                return Err(format!("Running program failed: {}\nSTDERR: {}", exiterr, errmsg));
            } else {
                return Err(format!("Running program failed: {}", exiterr));
            }
        }

        if errmsg.is_some() && outmsg.is_some() {
            Ok(format!("STDERR: {}\n\n{}", errmsg.unwrap(), outmsg.unwrap()))
        } else if let Some(errmsg) = errmsg {
            Ok(format!("STDERR: {}", errmsg))
        } else if let Some(outmsg) = outmsg {
            Ok(format!("{}", outmsg))
        } else {
            Ok("(No output)".into())
        }
    }
}

impl Drop for Pod {
    fn drop(&mut self) {
        // podman container kill "$id"
        // podman container rm "$id"

        let output = Command::new("podman")
            .arg("container")
            .arg("kill")
            .arg(&self.id)
            .output();
        match output {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Killing container {} failed: {}", self.id, err);
                return;
            }
        };
        // We don't really care if the kill command fails; that just means
        // the container has already exited

        let output = Command::new("podman")
            .arg("container")
            .arg("rm")
            .arg(&self.id)
            .stderr(Stdio::inherit())
            .output();
        let output = match output {
            Ok(output) => output,
            Err(err) => {
                eprintln!("Removing container {} failed: {}", self.id, err);
                return;
            }
        };
        if !output.status.success() {
            eprintln!("Removing container {} failed", self.id);
        }
    }
}

pub struct PodManager {
    tag: String,
}

impl PodManager {
    pub fn new(tag: String) -> Self {
        Self { tag }
    }

    pub fn get_pod(&self) -> Result<Pod, String> {
        Pod::new_from_tag(&self.tag)
    }
}
