use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

use actix_rt::time::interval;
use actix_web_lab::sse::{self, ChannelStream};
use futures_util::future;

pub struct EventSender {
    pub clients: HashMap<String, Vec<sse::Sender>>,
}

impl EventSender {
    pub fn new() -> Arc<Mutex<Self>> {
        let mutex_self = Arc::new(Mutex::new(Self {
            clients: HashMap::new(),
        }));

        Self::spawn_ping(mutex_self.clone());

        mutex_self
    }

    pub async fn send(&self, msg: &str, id: String) {
        let send_futures = self
            .clients
            .get(&id)
            .unwrap()
            .iter()
            .map(|client| client.send(sse::Data::new(msg)));

        let _ = future::join_all(send_futures).await;
    }

    pub async fn new_client(&mut self, id: String) -> sse::Sse<ChannelStream> {
        let (tx, rx) = sse::channel(10);

        tx.send(sse::Data::new("connected")).await.unwrap();

        if let Some(clients) = self.clients.get_mut(&id) {
            clients.push(tx);
        } else {
            self.clients.insert(id, vec![tx]);
        }

        rx
    }

    fn spawn_ping(mutex_self: Arc<Mutex<Self>>) {
        actix_web::rt::spawn(async move {
            let mut interval = interval(Duration::from_secs(10));

            loop {
                interval.tick().await;
                mutex_self.lock().unwrap().remove_dead_clients().await;
            }
        });
    }

    pub async fn remove_dead_clients(&mut self) {
        let mut tmp: HashMap<String, Vec<sse::Sender>> = HashMap::new();

        for (id, clients) in self.clients.clone() {
            let mut t = vec![];
            for client in clients {
                if client
                    .send(sse::Event::Comment("ping".into()))
                    .await
                    .is_ok()
                {
                    t.push(client.clone());
                }
            }
            tmp.insert(id.clone(), t);
        }
    }
}
