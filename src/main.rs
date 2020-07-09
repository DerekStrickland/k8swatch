// ripped off directly from here
// https://github.com/clux/kube-rs/blob/master/kube/examples/event_informer.rs

#[macro_use] extern crate log;
use k8s_openapi::api::core::v1::Event;
use kube::{
    api::{Api, ListParams, WatchEvent},
    runtime::Informer,
    Client,
};

use futures::{StreamExt, TryStreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "info,kube=debug");
    env_logger::init();
    let client = Client::try_default().await?;

    let events: Api<Event> = Api::all(client);
    let lp = ListParams::default();
    let ei = Informer::new(events).params(lp);

    loop {
        let mut events = ei.poll().await?.boxed();

        while let Some(event) = events.try_next().await? {
            handle_event(event)?;
        }
    }
}

// This function lets the app handle an event from kube
fn handle_event(ev: WatchEvent<Event>) -> anyhow::Result<()> {
    match ev {
        WatchEvent::Added(o) => {
            info!(
                "New Event: {} (via {} {})",
                o.message.unwrap(),
                o.involved_object.kind.unwrap(),
                o.involved_object.name.unwrap()
            );
        }
        WatchEvent::Modified(o) => {
            info!("Modified Event: {}", o.reason.unwrap());
        }
        WatchEvent::Deleted(o) => {
            info!("Deleted Event: {}", o.message.unwrap());
        }
        WatchEvent::Error(e) => {
            warn!("Error event: {:?}", e);
        }
        _ => {}
    }
    Ok(())
}