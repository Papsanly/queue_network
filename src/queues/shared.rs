use crate::{
    blocks::BlockId,
    queues::Queue,
    stats::{Stats, StepStats},
};
use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc, time::Duration};

pub struct SharedQueuePool {
    queues: Rc<RefCell<HashMap<BlockId, Box<dyn Queue>>>>,
}

impl Clone for SharedQueuePool {
    fn clone(&self) -> Self {
        Self {
            queues: Rc::clone(&self.queues),
        }
    }
}

impl SharedQueuePool {
    pub fn new() -> Self {
        Self {
            queues: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn add_queue(self, block: BlockId, queue: impl Queue + 'static) -> Self {
        self.queues.borrow_mut().insert(block, Box::new(queue));
        self
    }

    fn redistribute(&mut self, simulation_duration: Duration) -> bool {
        let queues = self.queues.borrow();
        let Some((_, max_length_queue)) = queues.iter().max_by_key(|(_, q)| q.length()) else {
            return false;
        };

        let Some((_, min_length_queue)) = queues.iter().min_by_key(|(_, q)| q.length()) else {
            return false;
        };

        if max_length_queue.length() - min_length_queue.length() < 2 {
            return false;
        }

        drop(queues);

        if let Some((_, max_length_queue)) = self
            .queues
            .borrow_mut()
            .iter_mut()
            .max_by_key(|(_, q)| q.length())
        {
            max_length_queue.dequeue(simulation_duration);
        };

        if let Some((_, min_length_queue)) = self
            .queues
            .borrow_mut()
            .iter_mut()
            .min_by_key(|(_, q)| q.length())
        {
            min_length_queue.enqueue(simulation_duration);
        };

        true
    }

    pub fn get(&self, block: BlockId) -> SharedQueue {
        SharedQueue {
            block,
            pool: self.clone(),
            transitions: 0,
        }
    }
}

pub struct SharedQueue {
    block: BlockId,
    pool: SharedQueuePool,
    transitions: usize,
}

#[derive(Debug)]
struct SharedQueueStats {
    transitions: usize,
}

impl StepStats for SharedQueue {
    fn step_stats(&self) -> Box<dyn Debug> {
        self.stats()
    }
}

impl Stats for SharedQueue {
    fn stats(&self) -> Box<dyn Debug> {
        Box::new(SharedQueueStats {
            transitions: self.transitions,
        })
    }
}

impl Queue for SharedQueue {
    fn length(&self) -> usize {
        self.pool
            .queues
            .borrow()
            .get(self.block)
            .expect("queue should exist in shared queue pool")
            .length()
    }

    fn weighted_total(&self) -> f32 {
        self.pool
            .queues
            .borrow()
            .get(self.block)
            .expect("queue should exist in shared queue pool")
            .weighted_total()
    }

    fn capacity(&self) -> Option<usize> {
        self.pool
            .queues
            .borrow()
            .get(self.block)
            .expect("queue should exist in shared queue pool")
            .capacity()
    }

    fn enqueue(&mut self, simulation_duration: Duration) {
        self.pool
            .queues
            .borrow_mut()
            .get_mut(self.block)
            .expect("queue should exist in shared queue pool")
            .enqueue(simulation_duration);
        if self.pool.redistribute(simulation_duration) {
            self.transitions += 1;
        }
    }

    fn dequeue(&mut self, simulation_duration: Duration) {
        self.pool
            .queues
            .borrow_mut()
            .get_mut(self.block)
            .expect("queue should exist in shared queue pool")
            .dequeue(simulation_duration);
        if self.pool.redistribute(simulation_duration) {
            self.transitions += 1;
        }
    }
}
