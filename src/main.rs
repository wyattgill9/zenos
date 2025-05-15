#![allow(dead_code)]

use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TaskState {
    Ready,
    Running,
    Blocked,
    Suspended,
    Terminated,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

static NEXT_TASK_ID: AtomicU32 = AtomicU32::new(1);

#[derive(Debug)]
pub struct Task<F>
where
    F: Fn() + Copy,
{
    id: u32,
    name: [u8; 32], // fixed-size task name
    name_len: usize,
    state: TaskState,
    priority: TaskPriority,

    period: u64,
    last_run: u64,
    execution_time: u64,
   
    action: F,
}

impl<F> Task<F>
where
    F: Fn() + Copy,
{
    pub fn new(name: &str, period: u64, execution_time: u64, action: F) -> Self {
        let mut name_buf = [0u8; 32];
        let bytes = name.as_bytes();
        let len = bytes.len().min(32);
        name_buf[..len].copy_from_slice(&bytes[..len]);

        Self {
            id: NEXT_TASK_ID.fetch_add(1, Ordering::SeqCst),
            name: name_buf,
            name_len: len,
            state: TaskState::Ready,
            priority: TaskPriority::Normal,
            period,
            last_run: 0,
            execution_time,
            action,
        }
    }

    pub fn execute(&self) {
        (self.action)();
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        std::str::from_utf8(&self.name[..self.name_len]).unwrap_or("<invalid utf8>")
    }

    pub fn state(&self) -> TaskState {
        self.state
    }

    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }

    pub fn priority(&self) -> TaskPriority {
        self.priority
    }

    pub fn set_priority(&mut self, priority: TaskPriority) {
        self.priority = priority;
    }

    pub fn update_last_run(&mut self, time: u64) {
        self.last_run = time;
    }

    pub fn is_ready_to_run(&self, current_time: u64) -> bool {
        self.state == TaskState::Ready && current_time >= self.last_run + self.period
    }
}

fn example_task_usage() {
    let simple_task = Task::new("SimpleTask", 1000, 50, || {
        println!("Simple task executed!");
    });

    let complex_task = Task::new("ComplexTask", 500, 100, || {
        println!("Complex task executed!");
    });

    println!("Created tasks: {} and {}", simple_task.name(), complex_task.name());

    simple_task.execute();
    complex_task.execute();
}

fn main() {
    example_task_usage();
}
