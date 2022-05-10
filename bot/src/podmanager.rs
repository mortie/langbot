use std::io::Write;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;
use std::io::Cursor;
use tar;

type Archive = tar::Archive<Cursor<Vec<u8>>>;

pub struct Pod {
    id: String,
    killed: bool,
}

pub struct ExecResult {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub status: ExitStatus,
    pub files: Option<Arc<Mutex<Archive>>>,
}

impl Pod {
    fn new_from_tag(tag: &str) -> Result<Pod, String> {
        let output = Command::new("podman")
            .arg("container")
            .arg("create")
            .arg("--rm")
            .arg("--network=none")
            .arg(tag)
            .arg("tail")
            .arg("-f")
            .arg("/dev/null")
            .stderr(Stdio::inherit())
            .output();
        let output = match output {
            Ok(output) => output,
            Err(err) => return Err(format!("Creating container failed: {}", err)),
        };

        if !output.status.success() {
            return Err("Creating cointainer failed".into());
        }

        let id = match String::from_utf8(output.stdout) {
            Ok(id) => id.trim().to_string(),
            Err(err) => return Err(format!("Podman retuned invalid UTF-8: {}", err)),
        };

        let output = Command::new("podman")
            .arg("container")
            .arg("start")
            .arg(&id)
            .stderr(Stdio::inherit())
            .output();
        let output = match output {
            Ok(output) => output,
            Err(err) => return Err(format!("Creating container failed: {}", err)),
        };

        if !output.status.success() {
            return Err("Starting container failed".into());
        }

        Ok(Pod { id, killed: false })
    }

    pub fn execute(&mut self, language: &str, content: &str) -> Result<ExecResult, String> {
        let exited = Arc::new(AtomicBool::new(false));

        let exited_th = exited.clone();
        let id_th = self.id.clone();
        thread::spawn(move || {
            for _ in 0..3 {
                thread::sleep(time::Duration::from_millis(1000));
                if exited_th.as_ref().load(Ordering::Relaxed) {
                    break;
                }
            }

            let output = Command::new("podman")
                .arg("container")
                .arg("kill")
                .arg(&id_th)
                .stderr(Stdio::inherit())
                .output();
            match output {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Killing container {} failed: {}", id_th, err);
                    return;
                }
            }
        });

        let child = Command::new("podman")
            .arg("exec")
            .arg("-i")
            .arg(&self.id)
            .arg("./scripts/run.sh")
            .arg(language)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
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

        let child = Command::new("podman")
            .arg("exec")
            .arg("-i")
            .arg(&self.id)
            .arg("./scripts/get-files.sh")
            .arg(language)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn();
        let child = match child {
            Ok(child) => child,
            Err(err) => return Err(format!("Getting files failed: {}", err)),
        };

        let tar_output = match child.wait_with_output() {
            Ok(output) => output,
            Err(err) => return Err(format!("Getting files failed: {}", err)),
        };

        exited.as_ref().store(true, Ordering::Relaxed);
        self.killed = true;

        let mut errmsg: Option<String> = None;
        {
            let msg = String::from_utf8_lossy(&output.stderr).trim_end().to_string();
            if msg.len() > 0 {
                errmsg = Some(msg);
            }
        }

        let mut outmsg: Option<String> = None;
        {
            let msg = String::from_utf8_lossy(&output.stdout).trim_end().to_string();
            if msg.len() > 0 {
                outmsg = Some(msg);
            }
        }

        let files = if tar_output.stdout.len() > 0 && tar_output.status.success() {
            Some(Arc::new(Mutex::new(Archive::new(Cursor::new(tar_output.stdout)))))
        } else {
            None
        };

        Ok(ExecResult {
            stdout: outmsg,
            stderr: errmsg,
            status: output.status,
            files,
        })
    }
}

impl Drop for Pod {
    fn drop(&mut self) {
        if !self.killed {
            let output = Command::new("podman")
                .arg("container")
                .arg("kill")
                .arg(&self.id)
                .stderr(Stdio::inherit())
                .output();
            match output {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Killing container {} failed: {}", self.id, err);
                    return;
                }
            }
        }
    }
}

enum Request {
    CreatePod,
    Terminate,
}

type Response = Result<Pod, String>;

fn pod_server(tag: String, req_ch: mpsc::Receiver<Request>, resp_ch: mpsc::Sender<Response>) {
    loop {
        let pod_res = Pod::new_from_tag(&tag);

        let req = req_ch.recv().unwrap();
        match req {
            Request::Terminate => return,
            Request::CreatePod => (),
        }

        resp_ch.send(pod_res).unwrap();
    }
}

pub struct PodManager {
    server: Option<thread::JoinHandle<()>>,
    req_ch: mpsc::Sender<Request>,
    resp_ch: mpsc::Receiver<Response>,
}

unsafe impl Send for PodManager {}
unsafe impl Sync for PodManager {}

impl PodManager {
    pub fn new(tag: String) -> Self {
        let (req_send, req_recv) = mpsc::channel();
        let (resp_send, resp_recv) = mpsc::channel();
        let handle = thread::spawn(move || {
            pod_server(tag, req_recv, resp_send);
        });

        Self {
            server: Some(handle),
            req_ch: req_send,
            resp_ch: resp_recv,
        }
    }

    pub fn get_pod(&self) -> Result<Pod, String> {
        self.req_ch.send(Request::CreatePod).unwrap();
        self.resp_ch.recv().unwrap()
    }
}

impl Drop for PodManager {
    fn drop(&mut self) {
        self.req_ch.send(Request::Terminate).unwrap();
        self.server.take().unwrap().join().unwrap();
    }
}
