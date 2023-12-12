# CPU Schedulers
## Schedulers.rs
---
The point of this file is to create a public trait that all CPU schedulers will have to include.

The ***new*** function is responsible to initializing each of the three CPU schedulers.
This is used to setup the rest of the functions with default values and types.

The ***add*** function is responsible for adding processes to a specific CPU scheduler.
For example, if you have "fcfs.add(p1)", it will add a process "p1" to the FCFS scheduler for processing.

The ***print_execution*** function is responsible for printing out each iteration that the scheduler performs.
Here is an example output:  
Process { p_id: 1, wait_times: 0, arrival_time: 32, response_time: 0, turnaround_time: 0, queue: 1, first_run: false, data: \[5, 27, 3, 31, 5, 43, 4, 18, 6, 22, 4, 26, 3, 24, 4], cpu_bursts: [3, 5, 4, 6, 4, 3, 4], io_bursts: [31, 43, 18, 22, 26, 24\] }  
\[Execution Time: 5\]

The ***start*** function is responsible for starting each CPU scheduler.

## Process.rs 
---
The point of this file is to initialize and parse all processes being added.

The ***new*** function initializes all processes with a p\_id and a vector of data.

The ***parse*** function takes the given data and parses it into two separate vectors.  
The first vector "*cpu\_bursts*" will take all CPU bursts and the second vector "*io\_bursts*" will take all the I\/O Bursts.
By doing this I can work on each of them individually, making it easier to keep track of each value coming in through the schedulers.
