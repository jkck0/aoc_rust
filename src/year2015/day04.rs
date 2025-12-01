use std::{
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    thread,
};

use md5;

pub fn parse(data: &str) -> &str {
    data
}

fn count_zeroes(hash: &str) -> u32 {
    hash.chars().take_while(|c| *c == '0').count() as u32
}

struct State {
    key: String,
    zeroes: u32,
    chunk_size: u32,

    next_chunk: AtomicU32,
    answer: AtomicU32,
}

impl State {
    fn new(key: String, zeroes: u32, chunk_size: u32) -> Self {
        State {
            key: key,
            zeroes: zeroes,
            chunk_size: chunk_size,
            next_chunk: AtomicU32::new(0),
            answer: AtomicU32::new(0),
        }
    }

    // returns the next chunk to search, until we find an answer
    fn next_chunk(&self) -> Option<u32> {
        if self.answer.load(Ordering::Relaxed) == 0 {
            Some(
                self.next_chunk
                    .fetch_add(self.chunk_size, Ordering::Relaxed),
            )
        } else {
            None
        }
    }
}

fn find_hash(state: State) -> u32 {
    let num_threads = thread::available_parallelism().unwrap().get();
    let state_arc = Arc::new(state);

    let mut handles = Vec::with_capacity(num_threads);
    for _ in 0..num_threads {
        let state_clone = state_arc.clone();
        handles.push(thread::spawn(move || {
            worker(state_clone);
        }));
    }

    for handle in handles {
        _ = handle.join();
    }
    state_arc.answer.load(Ordering::Relaxed)
}

fn worker(state: Arc<State>) {
    while let Some(chunk) = state.next_chunk() {
        for x in chunk..chunk + state.chunk_size {
            let hash = format!("{:x}", md5::compute(state.key.clone() + &x.to_string()));
            if count_zeroes(&hash) >= state.zeroes {
                state.answer.store(x, Ordering::Relaxed);
                return;
            }
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let state = State::new(input.to_owned(), 5, 1000);
    find_hash(state)
}

pub fn part2(input: &str) -> u32 {
    let state = State::new(input.to_owned(), 6, 1000);
    find_hash(state)
}
