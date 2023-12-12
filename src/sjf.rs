use crate::{process::Process, scheduler::Scheduler};
use std::collections::VecDeque;

pub struct SJF {
    ready: VecDeque<Process>,
    completed: Vec<Process>,
    execution_time: u32,
    total_time: u32,
}

impl Scheduler for SJF {
    type Process = Process;

    fn new() -> Self {
        SJF {
            ready: VecDeque::new(),
            completed: Vec::new(),
            execution_time: 0,
            total_time: 0,
        }
    }

    fn add(&mut self, process: Self::Process) {
        self.ready.push_back(process);
    }

    fn print_execution(&self) {
        println!("[Execution Time: {}]", self.execution_time);
        print!("\n\n");
    }

    fn start(&mut self) {
        for p in self.ready.iter_mut() {
            self.total_time += p.cpu_bursts.iter().sum::<u32>();
        }
        loop {
            self.print_execution();

            for i in 0..self.ready.len() {
                if self.ready[i].cpu_bursts.is_empty() && self.ready[i].io_bursts.is_empty() {
                    self.completed.push(self.ready.remove(i).unwrap());
                }
            }

            self.ready.make_contiguous().sort_by(|a, b| {
                if a.cpu_bursts[0] == b.cpu_bursts[0] {
                    a.p_id.cmp(&b.p_id)
                } else {
                    a.cpu_bursts[0].cmp(&b.cpu_bursts[0])
                }
            });

            let mut i = 0;
            if !self.ready.is_empty() {
                while self.ready[0].arrival_time > self.execution_time {
                    if i == self.ready.len() {
                        self.ready
                            .make_contiguous()
                            .sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));
                        break;
                    }
                    let not_ready = self.ready.pop_front().unwrap();
                    self.ready.push_back(not_ready);
                    i += 1;
                }
            }

            match self.ready.remove(0) {
                Some(mut p) => {
                    if p.first_run {
                        p.response_time = self.execution_time;
                        p.first_run = false;
                    }

                    if p.arrival_time <= self.execution_time {
                        p.wait_times += self.execution_time - p.arrival_time;
                    }

                    let cpu_burst = p.cpu_bursts.remove(0);
                    let io_burst = match p.io_bursts.pop_front() {
                        Some(io) => io,
                        None => 0,
                    };

                    if p.arrival_time > self.execution_time {
                        self.execution_time = p.arrival_time;
                        self.print_execution();
                    }

                    self.execution_time += cpu_burst;
                    p.arrival_time = self.execution_time + io_burst;

                    println!("{:?}", p);
                    self.ready.push_back(p);
                }

                None => {
                    self.completed.sort_by(|a, b| a.p_id.cmp(&b.p_id));

                    for i in 0..self.completed.len() {
                        self.completed[i].turnaround_time = self.completed[i].arrival_time;
                        println!("\nProcess: {:?}", self.completed[i].p_id);
                        println!("Wait Time: {:?}", self.completed[i].wait_times);
                        println!("Response Time: {:?}", self.completed[i].response_time);
                        println!("Turnaround Time: {:?}", self.completed[i].turnaround_time);
                    }
                    let (mut wait_time_avg, mut turnaround_time_avg, mut response_time_avg) =
                        (0.0, 0.0, 0.0);
                    for p in self.completed.iter() {
                        wait_time_avg += p.wait_times as f32 / 8.0;
                        turnaround_time_avg += p.turnaround_time as f32 / 8.0;
                        response_time_avg += p.response_time as f32 / 8.0;
                    }
                    println!("\nWait Time Average: {:.2}", wait_time_avg);
                    println!("Turnaround Time Average: {:.2}", turnaround_time_avg);
                    println!("Response Time Average: {:.2}", response_time_avg);
                    println!(
                        "\nCPU Utilization: {:.2}%",
                        self.total_time as f32 / self.execution_time as f32 * 100.0
                    );
                    break;
                }
            }
        }
    }
}
