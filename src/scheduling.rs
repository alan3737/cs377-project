use super::ps::pqueue;
use super::ps::Process;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_workload(filename:&str) -> pqueue{
    let mut workload:pqueue = BinaryHeap::new();
    if let Ok(file) = File::open(filename) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let mut tokens = line.split_whitespace();
                let mut p1 = Process{arrival: 0, first_run: 0, duration:0, completion:0, order_by:"arrival".to_string()};
                if let Some(token) = tokens.next() {
                    p1.arrival = token.parse().unwrap();
                }
                if let Some(token) = tokens.next() {
                    p1.duration = token.parse().unwrap();
                }
                workload.push(p1);
            }
        }
    }
    workload
}

fn show_workload(workload:pqueue ) {
    let mut xs:pqueue = workload.clone();
    println!("Workload:");
    while let Some(p) = xs.pop() {
        println!("\t{} {}", p.arrival, p.duration);
    }
}

fn show_processes(processes:Vec<Process>) {
    let mut xs = processes;
    println!("Processes:");
    while !xs.is_empty() {
        let p = xs.remove(0);
        println!("\tarrival={}, duration={}, first_run={}, completion={}", p.arrival, p.duration, p.first_run, p.completion);
    }
}

pub fn fifo(workload:pqueue ) -> Vec<Process>{
    let mut complete: Vec<Process> = Vec::new();
    let mut pqa: pqueue = workload.clone();
    let mut cur = 0;
    while let Some(mut p) = pqa.pop() {
        if p.arrival > cur {
            cur = p.arrival;
        }
        p.first_run = cur;
        cur += p.duration;
        p.completion = cur;
        complete.push(p);
    }
    complete
}

pub fn sjf(workload:pqueue) -> Vec<Process>{
    let mut complete: Vec<Process> = Vec::new();
    let mut pqa: pqueue = workload.clone();
    let mut pda: pqueue = pqueue::new();
    let mut time = 0;
    while !pqa.is_empty() || !pda.is_empty() {
        if !pqa.is_empty() && pqa.peek().unwrap().arrival <= time {
            let mut p = pqa.pop().unwrap();
            p.order_by = "duration".to_string();
            pda.push(p);
        } else {
            let mut top = pda.pop().unwrap();
            top.first_run = time;
            time += top.duration;
            top.completion = time;
            top.order_by = "duration".to_string();
            complete.push(top);
        }
    }
    complete
}

pub fn stcf(workload:pqueue ) -> Vec<Process>{
    let mut complete: Vec<Process> = Vec::new();
    let mut pqa: pqueue = workload.clone();
    let mut pda: pqueue = pqueue::new();
    let mut time = 0;
    while !pqa.is_empty() || !pda.is_empty() {
        if !pqa.is_empty() && pqa.peek().unwrap().arrival <= time {
            let mut top = pqa.pop().unwrap();
            top.completion = top.duration;
            top.order_by = "duration".to_string();
            pda.push(top);
        } else {
            let mut top = pda.pop().unwrap();
            if top.duration == top.completion {
                top.first_run = time;
            }
            top.duration -= 1;
            if top.duration == 0 {
                top.duration = top.completion;
                top.completion = time + 1;
                complete.push(top);
            } else {
                pda.push(top);
            }
            time += 1;
        }
    }
    complete
}

pub fn rr(workload:pqueue) -> Vec<Process>{
    let mut complete: Vec<Process> = Vec::new();
    let mut pqa: pqueue = workload.clone();
    let mut time = 0;
    let mut v1:Vec<Process> = Vec::new();
    let mut v2:Vec<i32> = Vec::new();
    while !pqa.is_empty() || !v1.is_empty() {
        if !pqa.is_empty() && pqa.peek().unwrap().arrival <= time {
            let top = pqa.pop().unwrap();
            v1.push(top.clone());
            v2.push(top.duration);
        } else {
            let mut temp1 = v1[0].clone();
            let mut temp2 = v2[0];
            v1.remove(0);
            v2.remove(0);
            if temp1.duration == temp2 {
                temp1.first_run = time;
            }
            temp2 -= temp2;
            if temp2 == 0 {
                temp1.completion = time + 1;
                complete.push(temp1.clone());
            } else {
                v1.push(temp1.clone());
                v2.push(temp2);
            }
            time += 1;
        }
    }

    complete
}

fn avg_turnaround(processes:&Vec<Process>) -> f32 {
    let mut sum = 0.0;
    for i in processes {
        sum += (i.completion - i.arrival) as f32;
    }
    sum / (processes.len() as f32)
}

fn avg_response(processes:&Vec<Process>) -> f32 {
    let mut sum = 0.0;
    for i in processes {
        sum += (i.first_run - i.arrival) as f32;
    }
    sum / (processes.len() as f32)
}

pub fn show_metrics(processes:Vec<Process>) {
    let avg_t = avg_turnaround(&processes);
    let avg_r = avg_response(&processes);
    show_processes(processes);
    println!("");
    println!("Average Turnaround Time: {}", avg_t);
    println!("Average Response Time:   {}", avg_r);
}

