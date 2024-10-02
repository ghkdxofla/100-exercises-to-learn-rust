// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, String> {
        let (sender_ticket_id, receiver_ticket_id) = sync_channel(1);
        let result = self.sender.send(Command::Insert {
            draft,
            response_channel: sender_ticket_id,
        });
        match result {
            Ok(_) => Ok(receiver_ticket_id.recv().unwrap()),
            Err(_) => Err("Failed to send command".into()),
        }
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, String> {
        let (sender_ticket, receiver_ticket) = sync_channel(1);
        let result = self.sender.send(Command::Get {
            id,
            response_channel: sender_ticket,
        });
        match result {
            Ok(_) => Ok(receiver_ticket.recv().unwrap()),
            Err(_) => Err("Failed to send command".into()),
        }
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                   draft,
                   response_channel,
               }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                   id,
                   response_channel,
               }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
