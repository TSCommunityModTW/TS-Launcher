use bytes::Bytes;
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{LoadingBarId, emit::emit_loading};

#[tracing::instrument(skip(url))]
pub async fn request_json<T>(url: &str) -> crate::Result<T> where T: DeserializeOwned {
    tracing::debug!("Fetching URL: {}", url);
    let reqwest = reqwest::get(url).await?.error_for_status()?;
    let text = reqwest.text().await?;
    let json = serde_json::from_str::<T>(&text)?;
    Ok(json)
}

pub async fn fetch(url: &str, sha1: Option<&str>, loading_bar: Option<(&LoadingBarId, f64)>) -> crate::Result<Bytes> {

    let result = Client::new()
            .get(url)
            .send()
            .await;

    match result {
        Ok(x) => {

            let bytes = if let Some((loading_bar_id, total)) = &loading_bar {

                let length = x.content_length();

                if let Some(total_size) = length {

                    use futures::StreamExt;
                    
                    let mut stream = x.bytes_stream();
                    let mut bytes = Vec::new();

                    // let mut downloaded: u64 = 0;

                    while let Some(item) = stream.next().await {

                        let chunk = item.or(Err(
                            crate::error::ErrorKind::NoValueFor(
                                "fetch bytes".to_string(),
                            ),
                        ))?;

                        bytes.append(&mut chunk.to_vec());

                        // let new = downloaded + (chunk.len() as u64);
                        // downloaded = new;
                        // let fraction = (new as f64 / total_size as f64) * total;
                        // emit_loading(bar, fraction, None).await?;

                        let fraction = (chunk.len() as f64 / total_size as f64) * total;
                        emit_loading(loading_bar_id, fraction, None).await?;
                    }

                    Ok(bytes::Bytes::from(bytes))

                } else {
                    x.bytes().await
                }
            } else {
                x.bytes().await
            };

            if let Ok(bytes) = bytes {
                
                // TODO: IF sha1

                tracing::trace!("Done downloading URL {url}");
                return Ok(bytes);

            } else if let Err(err) = bytes {

                return Err(err.into());

            }
        },
        Err(_) => (),
    }

    unreachable!()

}