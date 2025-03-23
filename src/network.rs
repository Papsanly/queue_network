use crate::{
    blocks::{Block, BlockId},
    events::{Event, EventType},
};
use std::{
    collections::{BinaryHeap, HashMap},
    io::stdin,
    thread,
    time::Duration,
};

pub struct QueueNetwork {
    event_queue: BinaryHeap<Event>,
    speed: Option<f32>,
    step_through: bool,
    on_simulation_step: Box<dyn Fn(&QueueNetwork, Event)>,
    pub blocks: HashMap<BlockId, Box<dyn Block>>,
}

impl QueueNetwork {
    pub fn new() -> QueueNetwork {
        QueueNetwork {
            event_queue: BinaryHeap::new(),
            speed: None,
            step_through: false,
            on_simulation_step: Box::new(|_, _| {}),
            blocks: HashMap::new(),
        }
    }

    pub fn real_time(mut self) -> Self {
        self.speed = Some(1.0);
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = Some(speed);
        self
    }

    pub fn step_through(mut self) -> Self {
        self.step_through = true;
        self
    }

    pub fn add_block(mut self, block: impl Block + 'static) -> Self {
        self.blocks.insert(block.id(), Box::new(block));
        self
    }

    pub fn on_simulation_step(
        mut self,
        on_simulation_step: impl Fn(&QueueNetwork, Event) + 'static,
    ) -> Self {
        self.on_simulation_step = Box::new(on_simulation_step);
        self
    }

    pub fn simulate(&mut self, duration: Duration) {
        for block in self.blocks.values_mut() {
            block.init(&mut self.event_queue);
        }

        let mut prev_time = Duration::from_secs(0);
        while let Some(Event(time, block_id, event_type, id)) = self.event_queue.pop() {
            if self.step_through {
                stdin().read_line(&mut String::new()).unwrap();
            } else if let Some(speed) = self.speed {
                thread::sleep(Duration::from_secs_f32(
                    (time - prev_time).as_secs_f32() / speed,
                ));
                prev_time = time
            }
            if time >= duration {
                break;
            }
            let expect_message = "event queue should only contain valid block ids";
            let next = self
                .blocks
                .get(block_id)
                .expect(expect_message)
                .next(&self.blocks);
            let block = self.blocks.get_mut(block_id).expect(expect_message);
            match event_type {
                EventType::In => block.process_in(id, &mut self.event_queue, time),
                EventType::Out => {
                    block.process_out(id, &mut self.event_queue, time);
                    if let Some(next) = next {
                        self.event_queue.push(Event(time, next, EventType::In, id));
                    }
                }
            }
            (self.on_simulation_step)(self, Event(time, block_id, event_type, id));
        }
    }
}
