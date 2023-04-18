use abi::Reservation;
use futures::Stream;
use reservation::ReservationManager;
use std::pin::Pin;
use tokio::sync::mpsc;
use tonic::Status;

mod service;

pub struct RsvpService {
    manager: ReservationManager,
}

pub struct TonicReceiverStream<T> {
    inner: mpsc::Receiver<Result<T, abi::Error>>,
}

type ReservationStream = Pin<Box<dyn Stream<Item = Result<Reservation, Status>> + Send>>;
