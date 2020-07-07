#[macro_use]
extern crate serde_derive;

use kube::{
    api::{Informer, Object, Event, RawApi, Void, WatchEvent},
    client::APIClient,
    config,
};

fn main() {
    let kubeconfig = config::load_kube_config().expect("kubeconfig failed to load");

    let client = APIClient::new(kubeconfig);

    let namespace = "default";

    let resource = RawApi::v1Event()
        .within(&namespace);

    let informer = Informer::raw(client, resource)
        .init()
        .expect("informer init failed");

    loop {
        informer.poll().expect("informer poll failed");

        while let Some(event) = informer.pop() {
            handle(event);
        }
    }
}

fn handle(event: WatchEvent<Event>) {
    match event {
        WatchEvent::Added(book) => println!(
            "Added a book {} with title '{}'",
            book.metadata.name, book.spec.title
        ),
        WatchEvent::Deleted(book) => println!("Deleted a book {}", book.metadata.name),
        _ => println!("another event"),
    }
}