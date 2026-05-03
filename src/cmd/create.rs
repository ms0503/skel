use crate::VisitDir;
use crate::error::SkeletonNotFoundError;
use crate::error::SkeletonsDirNotFoundError;
use std::io::ErrorKind;
use std::io::stdin;
use tokio::io::AsyncWriteExt;
use tokio::io::stderr;

pub async fn run(
    skel_dir: String,
    skel_name: String,
    target_dir: String
) -> Result<(), Box<dyn std::error::Error>> {
    if !tokio::fs::try_exists(&skel_dir).await? {
        eprintln!("Skeletons directory does not exist!");
        eprintln!("Please sync skeletons directory first.");
        Err(SkeletonsDirNotFoundError)?;
    }
    if !tokio::fs::try_exists(format!("{}/{}", &skel_dir, &skel_name)).await? {
        eprintln!("Skeleton '{}' does not exist!", &skel_name);
        Err(SkeletonNotFoundError)?;
    }
    println!("Create project from '{}'...", &skel_name);
    if let Err(err) = tokio::fs::create_dir(&target_dir).await {
        match err.kind() {
            ErrorKind::AlreadyExists => {
                eprintln!(
                    "Warning: Target directory '{}' already exists.",
                    &target_dir
                );
                loop {
                    eprint!("Run anyway? [y/N] ");
                    stderr().flush().await?;
                    let mut ans = String::new();
                    stdin().read_line(&mut ans)?;
                    let ans = ans.trim().to_lowercase();
                    match ans.as_str() {
                        "n" | "no" => {
                            println!("Aborted.");
                            return Ok(());
                        }
                        "y" | "yes" => break,
                        _ => {}
                    }
                }
            }
            _ => Err(err)?
        }
    }
    for file in VisitDir::new(format!("{}/{}", &skel_dir, &skel_name))?.entries() {
        let file = file?;
        let src = file.path();
        let src = src.to_str().unwrap();
        let relative_path = src.replace(&format!("{}/{}/", &skel_dir, &skel_name), "");
        let dst = format!("{}/{}", &target_dir, &relative_path);
        if file.file_type()?.is_dir() {
            println!("Create directory '{}'...", &relative_path);
            tokio::fs::create_dir(&dst).await?;
        } else {
            println!("Copy file '{}'...", &relative_path);
            tokio::fs::copy(&src, &dst).await?;
        }
    }
    Ok(())
}
