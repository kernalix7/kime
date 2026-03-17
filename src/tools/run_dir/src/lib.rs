use std::env;
use std::path::PathBuf;

pub fn get_run_dir() -> PathBuf {
    let path = get_run_dir_impl();

    if !path.exists() {
        std::fs::create_dir(&path).ok();
    }

    path
}

pub fn get_run_dir_impl() -> PathBuf {
    if let Ok(dir) = env::var("XDG_RUNTIME_DIR") {
        dir.into()
    } else {
        // SAFETY: getuid() is always safe to call and has no preconditions
        let uid = unsafe { libc::getuid() };
        PathBuf::from(format!("/tmp/kime-{}", uid))
    }
}
