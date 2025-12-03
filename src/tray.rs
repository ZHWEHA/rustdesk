use crate::client::translate;
#[cfg(windows)]
use crate::ipc::Data;
#[cfg(windows)]
use hbb_common::tokio;
use hbb_common::{allow_err, log};
use std::sync::{Arc, Mutex};
#[cfg(windows)]
use std::time::Duration;

pub fn start_tray() {}

fn make_tray() -> hbb_common::ResultType<()> {
    Ok(())
}

#[cfg(windows)]
#[tokio::main(flavor = "current_thread")]
async fn start_query_session_count(sender: std::sync::mpsc::Sender<Data>) {
}

fn load_icon_from_asset() -> Option<image::DynamicImage> {

    None
}
