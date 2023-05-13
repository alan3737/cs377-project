use std::env;
use std::process;

mod scheduling;
mod ps;
use scheduling::read_workload;
use std::collections::BinaryHeap;


fn main() {
    let args: Vec<String> = env:: args().collect();
    if args.len() != 3 {
        println!("usage: [fifo|sjf|stcf|rr] workload_file");
        process::exit(1);
    }
    let algorithm = &args[1];
    let workload_file = &args[2];

    let workload:BinaryHeap<ps::Process>  = read_workload(workload_file);

    if algorithm == "fifo" {
        scheduling::show_metrics(scheduling::fifo(workload));
    } else if algorithm == "sjf" {
        scheduling::show_metrics(scheduling::sjf(workload));
    } else if algorithm == "stcf" {
        scheduling::show_metrics(scheduling::stcf(workload));
    } else if algorithm == "rr" {
        scheduling::show_metrics(scheduling::rr(workload));
    } else {
        println!("Error: Unknown algorithm: {algorithm}");
        println!("usage: [fifo|sjf|stcf|rr] workload_file");
        process::exit(1);
    }
    // let mut workload: ps::pqueue_arrival = BinaryHeap::new();

    // workload.push(Process{arrival: 4, first_run: 0, duration:4, completion:0});
    // workload.push(Process{arrival: 5, first_run: 0, duration:3, completion:0});
    // workload.push(Process{arrival: 7, first_run: 0, duration:5, completion:0});
    // let mut result = workload.peek();
    // println!("{:?}", result.unwrap());
}
