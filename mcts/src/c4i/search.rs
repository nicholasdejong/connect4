use crate::{
    montecarlo::mcts,
    types::{board::Board, r#move::Move},
};
use std::{
    sync::{atomic::AtomicBool, Arc, Mutex}, thread::{self, JoinHandle}, time::Duration
};

type Handle = JoinHandle<Move>;

#[derive(Default)]
pub struct SearchHandler {
    should_stop: Arc<Mutex<AtomicBool>>,
    handle: Option<Handle>,
    searching: bool
}

impl SearchHandler {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn search(&mut self, board: Board, time: Duration) {
        if self.is_searching() { return }
        self.searching = true;
        self.should_stop.clone().lock().unwrap().store(false, std::sync::atomic::Ordering::Release);
        let should_stop = self.should_stop.clone();
        let handle = thread::spawn(move || {
            let mv = mcts(&mut board.clone(), time, should_stop);
            mv
        });
        self.handle = Some(handle);
    }

    pub fn stop_search(&mut self) {
        self.should_stop.clone().lock().unwrap().store(true, std::sync::atomic::Ordering::Release);
        self.searching = false;
    }

    pub fn is_searching(&self) -> bool {
        if let Some(handle) = &self.handle {
            !handle.is_finished()
        } else {
            false
        }
    }

    // pub fn get_mv(&mut self) -> Option<Move> {
    //     if let Some(handle) = &mut self.handle {
    //         println!("getting mv");
    //         if handle.is_finished() || !self.searching {
    //             let mv = self.handle.take().unwrap().join().expect("search failed");
    //             Some(mv)
    //         } else {
    //             None
    //         }
    //     } else {
    //         None
    //     }
    // }
}
