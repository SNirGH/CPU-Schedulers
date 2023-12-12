mod fcfs;
mod mlfq;
mod process;
mod scheduler;
mod sjf;

use fcfs::FCFS;
use mlfq::MLFQ;
use process::Process;
use scheduler::Scheduler;
use sjf::SJF;

fn main() {
    let mut p1: Process = Process::new(1, vec![5, 27, 3, 31, 5, 43, 4, 18, 6, 22, 4, 26, 3, 24, 4]);
    let mut p2: Process = Process::new(
        2,
        vec![4, 48, 5, 44, 7, 42, 12, 37, 9, 76, 4, 41, 9, 31, 7, 43, 8],
    );
    let mut p3: Process = Process::new(
        3,
        vec![
            8, 33, 12, 41, 18, 65, 14, 21, 4, 61, 15, 18, 14, 26, 5, 31, 6,
        ],
    );
    let mut p4: Process = Process::new(
        4,
        vec![3, 35, 4, 41, 5, 45, 3, 51, 4, 61, 5, 54, 6, 82, 5, 77, 3],
    );
    let mut p5: Process = Process::new(
        5,
        vec![
            16, 24, 17, 21, 5, 36, 16, 26, 7, 31, 13, 28, 11, 21, 6, 13, 3, 11, 4,
        ],
    );
    let mut p6: Process = Process::new(
        6,
        vec![11, 22, 4, 8, 5, 10, 6, 12, 7, 14, 9, 18, 12, 24, 15, 30, 8],
    );
    let mut p7: Process = Process::new(
        7,
        vec![14, 46, 17, 41, 11, 42, 15, 21, 4, 32, 7, 19, 16, 33, 10],
    );
    let mut p8: Process = Process::new(8, vec![4, 14, 5, 33, 6, 51, 14, 73, 16, 87, 6]);

    p1.parse();
    p2.parse();
    p3.parse();
    p4.parse();
    p5.parse();
    p6.parse();
    p7.parse();
    p8.parse();

    let mut fcfs = FCFS::new();
    fcfs.add(p1);
    fcfs.add(p2);
    fcfs.add(p3);
    fcfs.add(p4);
    fcfs.add(p5);
    fcfs.add(p6);
    fcfs.add(p7);
    fcfs.add(p8);
    fcfs.start();

    // let mut sjf = SJF::new();
    // sjf.add(p1);
    // sjf.add(p2);
    // sjf.add(p3);
    // sjf.add(p4);
    // sjf.add(p5);
    // sjf.add(p6);
    // sjf.add(p7);
    // sjf.add(p8);
    // sjf.start();

    // let mut mlfq = MLFQ::new();
    // mlfq.add(p1);
    // mlfq.add(p2);
    // mlfq.add(p3);
    // mlfq.add(p4);
    // mlfq.add(p5);
    // mlfq.add(p6);
    // mlfq.add(p7);
    // mlfq.add(p8);
    // mlfq.start();
}
