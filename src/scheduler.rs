pub trait Scheduler {
    type Process;

    fn new() -> Self;
    fn add(&mut self, process: Self::Process);
    fn print_execution(&self);
    fn start(&mut self);
}

