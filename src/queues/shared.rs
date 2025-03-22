use crate::{blocks::BlockId, queues::Queue};
use std::{cell::RefCell, collections::HashMap, rc::Rc, time::Duration};

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

    fn redistribute(&mut self, simulation_duration: Duration) {
        let queues = self.queues.borrow();
        let Some((_, max_length_queue)) = queues.iter().max_by_key(|(_, q)| q.length()) else {
            return;
        };

        let Some((_, min_length_queue)) = queues.iter().min_by_key(|(_, q)| q.length()) else {
            return;
        };

        if max_length_queue.length() - min_length_queue.length() < 2 {
            return;
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
    }

    pub fn get(&self, block: BlockId) -> SharedQueue {
        SharedQueue {
            block,
            pool: self.clone(),
        }
    }
}

pub struct SharedQueue {
    block: BlockId,
    pool: SharedQueuePool,
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
        self.pool.redistribute(simulation_duration);
    }

    fn dequeue(&mut self, simulation_duration: Duration) {
        self.pool
            .queues
            .borrow_mut()
            .get_mut(self.block)
            .expect("queue should exist in shared queue pool")
            .dequeue(simulation_duration);
        self.pool.redistribute(simulation_duration);
    }

    fn total_weighted_time(&self) -> f32 {
        self.pool
            .queues
            .borrow_mut()
            .get_mut(self.block)
            .expect("queue should exist in shared queue pool")
            .total_weighted_time()
    }

    fn duration(&self) -> Duration {
        self.pool
            .queues
            .borrow_mut()
            .get_mut(self.block)
            .expect("queue should exist in shared queue pool")
            .duration()
    }

    fn average_length(&self) -> f32 {
        self.pool
            .queues
            .borrow_mut()
            .get_mut(self.block)
            .expect("queue should exist in shared queue pool")
            .average_length()
    }
}
