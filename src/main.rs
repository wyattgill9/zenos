#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TaskState {
    Ready,
    Running,
    Blocked,
    Suspended,
    Terminated,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum TaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

type TaskId = u32;

fn main() {}
