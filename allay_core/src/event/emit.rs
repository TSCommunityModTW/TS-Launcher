use super::LoadingBarId;
use futures::{TryStream, Future};
use uuid::Uuid;

use crate::event::{
    LoadingBarType, LoadingBar, EventError, LoadingPayload
};

#[cfg(feature = "tauri")]
use tauri::Manager;

pub async fn init_loading(bar_type: LoadingBarType, total: f64, title: &str) -> crate::Result<LoadingBarId> {

    let event_state = crate::EventState::get().await?;
    let id = LoadingBarId(Uuid::new_v4());

    event_state.loading_bars.write().await.insert(
        id.0,
        LoadingBar {
            loading_bar_uuid: id.0,
            message: title.to_string(),
            total,
            bar_type,
            current: 0.0,
            last_sent: 0.0
        },
    );

    // attempt an initial loading_emit event to the frontend
    emit_loading(&id, 0.0, None).await?;

    Ok(id)
}

pub async fn emit_loading(id: &LoadingBarId, increment_fraction: f64, message: Option<&str>) -> crate::Result<()> {

    let event_state = crate::EventState::get().await?;

    let mut loading_bar = event_state.loading_bars.write().await;
    let loading_bar = match loading_bar.get_mut(&id.0) {
        Some(f) => f,
        None => {
            return Err(EventError::NoLoadingBar(id.0).into());
        }
    };

    loading_bar.current += increment_fraction;
    let display_frac = loading_bar.current / loading_bar.total;
    let opt_display_frac = if display_frac >= 1.0 {
        1.0
    } else {
        display_frac
    };

    let opt_display_frac = opt_display_frac * 100.0;

    // println!("================================================");
    // println!("increment_fraction: {:?}", increment_fraction);
    // println!("loading_bar.current: {:?}", loading_bar.current);
    // println!("display_frac: {:?}", display_frac);
    // println!("opt_display_frac: {:?}", opt_display_frac);

    // Emit event to tauri
    #[cfg(feature = "tauri")]
    event_state.app.emit_all(
        "loading",
        LoadingPayload {
            fraction: opt_display_frac,
            message: message.unwrap_or(&loading_bar.message).to_string(),
            event: loading_bar.bar_type.clone(),
            loader_uuid: loading_bar.loading_bar_uuid,
        }
    ).map_err(EventError::from)?;

    loading_bar.last_sent = display_frac;

    Ok(())
}

pub async fn loading_try_for_each_concurrent<I, F, Fut, T>(
    stream: I,
    limit: Option<usize>,
    key: Option<&LoadingBarId>,
    total: f64,
    num_futs: usize,
    message: Option<&str>,
    f: F,
) -> crate::Result<()>
where
    I: futures::TryStreamExt<Error = crate::Error> + TryStream<Ok = T>,
    F: FnMut(T) -> Fut + Send,
    Fut: Future<Output = crate::Result<()>> + Send,
    T: Send,
{
    let mut f = f;

    stream.try_for_each_concurrent(limit, |item| {
        let f = f(item);
        async move {
            f.await?;
            if let Some(key) = key {
                emit_loading(key, total / (num_futs as f64), message).await?;
                }
            Ok(())
        }
    }).await
}