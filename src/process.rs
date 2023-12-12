use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Process {
    pub p_id: u32,
    pub wait_times: u32,
    pub arrival_time: u32,
    pub response_time: u32,
    pub turnaround_time: u32,
    pub queue: u32,

    pub first_run: bool,

    data: Vec<u32>,
    pub cpu_bursts: Vec<u32>,
    pub io_bursts: VecDeque<u32>,
}

impl Process {
    pub fn new(p_id: u32, data: Vec<u32>) -> Self {
        Process {
            p_id,
            wait_times: 0,
            arrival_time: 0,
            response_time: 0,
            turnaround_time: 0,
            queue: 1,

            first_run: true,

            data,
            cpu_bursts: Vec::new(),
            io_bursts: VecDeque::new(),
        }
    }

    pub fn parse(&mut self) {
        for i in 0..self.data.len() {
            if i % 2 == 0 {
                self.cpu_bursts.push(self.data[i]);
            } else {
                self.io_bursts.push_back(self.data[i]);
            }
        }
    }
}

