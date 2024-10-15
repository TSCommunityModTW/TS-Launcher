use tracing_appender::non_blocking::WorkerGuard;

use crate::util;

// #[cfg(debug_assertions)]
// pub fn init_logger() -> Option<WorkerGuard> {

//     use tracing_subscriber::prelude::*;

//     let filter = tracing_subscriber::EnvFilter::try_from_default_env()
//         .unwrap_or_else(|_| {
//             tracing_subscriber::EnvFilter::new("allay_core=debug,ts_gui=debug")
//         });

//     let subscriber = tracing_subscriber::registry()
//         .with(tracing_subscriber::fmt::layer())
//         .with(filter)
//         .with(tracing_error::ErrorLayer::default());

//     tracing::subscriber::set_global_default(subscriber)
//         .expect("Setting default subscriber failed.");

//     None
// }

// #[cfg(debug_assertions)]
pub fn init_logger() -> Option<WorkerGuard> {

    use std::fs::OpenOptions;

    use crate::util::app_path;
    use chrono::Local;
    use tracing_subscriber::prelude::*;

    // let filter = tracing_subscriber::EnvFilter::try_from_default_env()
    //     .unwrap_or_else(|_| {
    //         tracing_subscriber::EnvFilter::new("allay_core=debug,ts_gui=debug")
    //     });

    let filter = tracing_subscriber::EnvFilter::new("allay_core=debug,ts_gui=debug");

    // 獲取當前日期，並格式化為 YYYY-MM-DD
    // let date = Local::now().format("%Y-%m-%d").to_string();

    // 自定義日志文件名稱
    let logs_dir = app_path::get_logs_dir_path();
    // let log_file_name = format!("{}/{}_app_log.log", logs_dir.to_string_lossy().to_owned(), date);


    // 打開文件，並確保是追加寫入模式
    // let log_file = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open(log_file_name)
    //     .expect("Failed to open log file");


    let file_appender = tracing_appender::rolling::daily(logs_dir, "allay_core");
    let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer()
            .with_writer(non_blocking_appender)
            .with_ansi(false)
        )
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .with(tracing_error::ErrorLayer::default());

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed.");

    Some(guard)
}

// #[cfg(not(debug_assertions))]
// pub fn init_logger() -> Option<WorkerGuard> {

//     use crate::util::app_path;
//     use tracing_subscriber::prelude::*;

//     // Initialize and get logs directory path
//     let logs_dir = app_path::get_logs_dir_path();

//     let filter = tracing_subscriber::EnvFilter::try_from_default_env()
//         .unwrap_or_else(|_| {
//             tracing_subscriber::EnvFilter::new("allay_core=info")
//         });

//     let file_appender = tracing_appender::rolling::daily(logs_dir, "allay_core.log");
//     let (non_blocking_appender, guard) = tracing_appender::non_blocking(file_appender);

//     let subscriber = tracing_subscriber::registry()
//         .with(tracing_subscriber::fmt::layer()
//             .with_writer(non_blocking_appender)
//             .with_ansi(false))
//         .with(filter)
//         .with(tracing_error::ErrorLayer::default());

//     tracing::subscriber::set_global_default(subscriber)
//         .expect("Setting default subscriber failed.");
    
//     Some(guard)
// }