use crate::{
    blocks::{Block, BlockId},
    events::{Event, EventType},
    stats::Stats,
    weighted_average::weighted_average,
};
use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
    io::stdin,
    thread,
    time::Duration,
};

#[derive(Debug)]
struct QueueNetworkStats {
    average_event_count: f32,
    average_event_duration: f32,
}

pub struct QueueNetwork {
    event_queue: BinaryHeap<Event>,
    speed: Option<f32>,
    step_through: bool,
    on_simulation_step: Box<dyn Fn(&QueueNetwork, Event)>,
    pub blocks: HashMap<BlockId, Box<dyn Block>>,
    pub event_count: usize,
    event_counts: Vec<(Duration, usize)>,
    event_durations: HashMap<usize, (Option<Duration>, Option<Duration>)>,
}

impl Stats for QueueNetwork {
    fn stats(&self) -> Box<dyn Debug> {
        let event_durations = self
            .event_durations
            .iter()
            .filter(|(_, (start, end))| start.is_some() && end.is_some());
        Box::new(QueueNetworkStats {
            average_event_count: weighted_average(&self.event_counts),
            average_event_duration: event_durations
                .clone()
                .map(|(_, (start, end))| end.unwrap() - start.unwrap())
                .sum::<Duration>()
                .as_secs_f32()
                / event_durations.count() as f32,
        })
    }
}

impl QueueNetwork {
    pub fn new() -> QueueNetwork {
        QueueNetwork {
            event_queue: BinaryHeap::new(),
            speed: None,
            step_through: false,
            on_simulation_step: Box::new(|_, _| {}),
            blocks: HashMap::new(),
            event_count: 0,
            event_counts: Vec::new(),
            event_durations: HashMap::new(),
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
                EventType::In => {
                    let accepted = block.process_in(id, &mut self.event_queue, time);
                    if block.kind() == "dispose" {
                        self.event_count -= 1;
                        self.event_counts.push((time, self.event_count));
                        self.event_durations.entry(id).or_default().1 = Some(time);
                    }
                    if accepted && block.kind() == "process" {
                        self.event_count += 1;
                        self.event_counts.push((time, self.event_count));
                    }
                }
                EventType::Out => {
                    block.process_out(id, &mut self.event_queue, time);
                    if block.kind() == "create" {
                        self.event_durations.entry(id).or_default().0 = Some(time);
                    }
                    if let Some(next) = next {
                        self.event_queue.push(Event(time, next, EventType::In, id));
                    }
                }
            }
            (self.on_simulation_step)(self, Event(time, block_id, event_type, id));
        }
    }
}
