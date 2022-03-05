use tokio::sync::mpsc::{self, error::TryRecvError};

use crate::turtle::ipc_protocol::{
    ServerOneshotSender,
    ServerResponse,
};
use crate::turtle::Event;

use super::HandlerError;

pub(crate) fn poll_event(
    conn: ServerOneshotSender,
    events_receiver: &mut mpsc::UnboundedReceiver<Event>,
) -> Result<(), HandlerError> {
    let event = match events_receiver.try_recv() {
        Ok(event) => Some(event),
        Err(TryRecvError::Empty) => None,
        // The main thread must have ended so no more events will be sent ever
        Err(TryRecvError::Closed) => return Ok(()),
    };

    conn.send(ServerResponse::Event(event))?;

    Ok(())
}
