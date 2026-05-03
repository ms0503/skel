use crate::SKEL_REPO;
use crate::SKEL_REPO_REV;
use flate2::read::GzDecoder;
use std::io::ErrorKind;
use std::io::Read;
use tar::Archive;

pub async fn run(skel_dir: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Sync from GitHub ms0503/skeletons.");
    println!("Download skeletons...");
    let tarball = reqwest::get(format!("{}/archive/{}.tar.gz", SKEL_REPO, SKEL_REPO_REV));
    if let Err(err) = tokio::fs::remove_dir_all(&skel_dir).await {
        match err.kind() {
            ErrorKind::NotFound => {}
            _ => Err(err)?
        }
    }
    tokio::fs::create_dir(&skel_dir).await?;
    let tarball = tarball
        .await?
        .bytes()
        .await?
        .into_iter()
        .collect::<Vec<u8>>();
    println!("Extract archive...");
    let tarball = GzDecoder::new(&tarball[..]);
    let mut tarball = Archive::new(tarball);
    for file in tarball.entries()? {
        let mut file = file?;
        let filename = file.header().path()?;
        let filename = filename.to_str().unwrap();
        if filename == "pax_global_header" || filename == "skeletons-main/" {
            continue;
        }
        let filename = filename.replace("skeletons-main/", "");
        let is_dir = filename.ends_with("/");
        if is_dir {
            tokio::fs::create_dir(format!("{}/{}", &skel_dir, filename)).await?;
        } else {
            let mut contents: Vec<u8> = vec![];
            file.read_to_end(&mut contents)?;
            tokio::fs::write(format!("{}/{}", &skel_dir, filename), contents).await?;
        }
    }
    println!("Complete syncing!");
    Ok(())
}
