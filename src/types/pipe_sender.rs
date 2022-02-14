use std::{
    fs::OpenOptions,
    io::Write,
    os::unix::fs::OpenOptionsExt,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use glob::glob;

#[derive(Clone)]
pub struct PipeSender {
    pub bar_pipe_glob: Arc<Mutex<String>>,
}
impl PipeSender {
    pub fn new(glob_str: String) -> PipeSender {
        Self {
            bar_pipe_glob: Arc::new(Mutex::new(glob_str)),
        }
    }
    pub fn send(&self, msg: &str) {
        let pipe_glob = self.bar_pipe_glob.lock().unwrap();
        if let Ok(bars) = glob(pipe_glob.as_str()) {
            for bar in bars {
                if let Ok(pipe) = bar {
                    if let Some(fname) = pipe.to_str() {
                        if let Ok(mut fid) = OpenOptions::new()
                            .write(true)
                            .append(true)
                            .custom_flags(libc::O_NONBLOCK)
                            .open(fname)
                        {
                            if let Err(e) = fid.write(&msg.as_bytes()) {
                                eprintln!("Error writing to pipe: {}", e);
                            }
                            if let Err(e) = fid.flush() {
                                eprintln!("Error flushing pipe buffer: {}", e);
                            }
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(2)); // give the bar time to process the message before allowing the next
    }
}
