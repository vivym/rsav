use std::error::Error;
use std::process::Command;
use std::path::Path;

use vergen::EmitBuilder;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

fn mkdir(dir_name: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(dir_name)
}

fn cd(dir_name: &str) -> std::io::Result<()> {
    std::env::set_current_dir(dir_name)
}

fn pwd() -> std::io::Result<std::path::PathBuf> {
    std::env::current_dir()
}

fn prepare_libav() -> Result<()> {
    let status_path = Path::new("tmp/ffmpeg.done");

    // Check if the ffmpeg build has already been done
    if status_path.exists() {
        return Ok(());
    }

    mkdir("tmp")?;
    cd("tmp")?;

    let branch = "release/7.0";

    if std::fs::metadata("ffmpeg").is_err() {
        Command::new("git")
            .arg("clone")
            .arg("--single-branch")
            .arg("--branch")
            .arg(branch)
            .arg("--depth")
            .arg("1")
            .arg("https://git.ffmpeg.org/ffmpeg.git")
            .status()?;
    }

    cd("ffmpeg")?;

    Command::new("git")
        .arg("fetch")
        .arg("origin")
        .arg(&branch)
        .arg("--depth")
        .arg("1")
        .status()?;

    Command::new("git")
        .arg("checkout")
        .arg("FETCH_HEAD")
        .status()?;

    Ok(())
}

fn main() -> Result<()> {
    let current_dir = pwd()?;

    prepare_libav()?;

    cd(&current_dir.to_string_lossy())?;

    // Try to get the git sha from the local git repository
    if EmitBuilder::builder()
        .fail_on_error()
        .git_sha(false)
        .emit()
        .is_err()
    {
        // Unable to get the git sha
        if let Ok(sha) = std::env::var("GIT_SHA") {
            // Set it from an env var
            println!("cargo:rustc-env=VERGEN_GIT_SHA={sha}");
        }
    }

    // Set docker label if present
    if let Ok(label) = std::env::var("DOCKER_LABEL") {
        // Set it from an env var
        println!("cargo:rustc-env=DOCKER_LABEL={label}");
    }

    Ok(())
}
