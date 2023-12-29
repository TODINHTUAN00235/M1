use serde::{Deserialize, Serialize};
use super::{ReleaseOperations, Release};
use crate::util::location::{Location, self};
use std::path::PathBuf;
use std::io::Write;
use reqwest;
use crate::util::util::Version;
use crate::util::sys::{Arch, OS};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HttpGET(String);

impl HttpGET {

    pub fn new(url : String) -> Self {
        Self(url)
    }

    pub fn url(&self) -> &String {
        &self.0
    }

    pub async fn download_to_path(&self, path : &PathBuf) -> Result<(), anyhow::Error> {
        let mut response = reqwest::get(self.url()).await?;
        match response.status().is_success() {
            true => {
                let mut file = std::fs::File::create(path)?;
                while let Some(chunk) = response.chunk().await? {
                    file.write_all(&chunk)?;
                }
                Ok(())
            },
            false => {
                anyhow::bail!("Failed to download file from url: {}", self.url());
            }
        }
    }

}

#[async_trait::async_trait]
impl ReleaseOperations for HttpGET {

    async fn get(&self, location : &Location) -> Result<(), anyhow::Error> {

        match location {
            Location::TempDir(temp)=>{
                self.download_to_path(&temp.get_release_tempfile_path()).await
            }
            _ => {
                anyhow::bail!("Cannot get a file release to a non-release location.");
            }
        }
     
        
    }

    fn with_version(mut self, version : &Version) -> Self {
        self
    }

    fn with_arch(mut self, arch : &Arch) -> Self {
        self
    }

    fn with_os(mut self, os : &OS) -> Self {
        self
    }

}

impl Into<Release> for HttpGET {
    fn into(self) -> Release {
        Release::HttpGET(self)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_get_release_to_file() -> Result<(), anyhow::Error> {

        let release_text = "hello";

        let release = HttpGET::new(
            String::from("https://github.com/movemntdev/resources/releases/download/v0.0.0/hello.txt")
        );

        let dir = tempdir().unwrap();
        let path = dir.path().join("test.txt");

        let location = Location::temp(
            "test.txt".to_string(), 
            &PathBuf::from("test.txt")
        );
    
        release.get(&location).await.unwrap();

        let contents = std::fs::read_to_string(&path).unwrap();

        assert_eq!(contents, release_text);

        Ok(())

    }

}