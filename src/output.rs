use super::crawler::Visited;
use tokio::{fs::File, io::AsyncWriteExt};
use tracing::info;

pub async fn write_results(vis: &Visited, path: std::path::PathBuf) -> anyhow::Result<()> {
    let list = vis.all();
    let mut file = File::create(&path).await?;
    if path.extension().map(|e| e == "json").unwrap_or(false) {
        let json = serde_json::to_string_pretty(&list)?;
        file.write_all(json.as_bytes()).await?;
    } else {
        for url in &list {
            file.write_all(url.as_bytes()).await?;
            file.write_all(b"\n").await?;
        }
    }
    info!(count = list.len(), path = %path.display(), "saved");
    Ok(())
}
