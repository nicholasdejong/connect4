use std::{
    sync::{atomic::AtomicBool, Arc},
    thread::{self, JoinHandle},
    time::Duration,
};
use {
    crate::montecarlo::mcts,
    types::{board::Board, r#move::Move},
};

type Handle = JoinHandle<Move>;

#[derive(Default)]
pub struct SearchHandler {
    should_stop: Arc<AtomicBool>,
    handle: Option<Handle>,
    searching: Arc<AtomicBool>,
}

impl SearchHandler {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn search(&mut self, board: Board, time: Duration) {
        if self.is_searching() {
            // try again
            self.search(board, time);
            return;
        }
        self.searching.as_ref().store(true, std::sync::atomic::Ordering::Release);
        self.should_stop
            .clone()
            .store(false, std::sync::atomic::Ordering::Release);
        let should_stop = self.should_stop.clone();
        let searching = self.searching.clone();
        let handle = thread::spawn(move || {
            let mv = mcts(&mut board.clone(), time, should_stop, searching);
            mv
        });
        self.handle = Some(handle);
    }

    pub fn stop_search(&mut self) {
        self.should_stop
            .clone()
            .store(true, std::sync::atomic::Ordering::Release);
    }

    pub fn is_searching(&self) -> bool {
        self.searching.as_ref().load(std::sync::atomic::Ordering::Acquire)
    }
}
