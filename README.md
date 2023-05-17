# Final Project

## Descriptions
  This project translates project3 from c/c++ into rust. Rust is a very unqiue programming lanaguage. It allows for other people who don't have rust installed to run the executable. For this version it utilizes cargo which is Rust's build system and package manager. It includes a very cool feature that compiles the code with optimization so the code can run faster. As a whole the functionality of the project is still the same. For those who aren't familiar with the what functionality is then I will discuss it. The project is a simluation of scheduling that occurs between programs to allow resources to be managed efficiently. This project contains four different type of scheduling.

## Changes
  Implementation of priority queue. This version usess BinaryHeap. In order to use Process as a genereic type of binary heap, it is required to override ord. Ord indicates how the binary heap is sorted. There are two ways it can be sorted which is by arrival and the other is duration. Ord doesn't allow multiple implementation of ord for the same struct so I added an attribute called order_by to Process. The Ord fucntion would check if the order_by is arrival or duration and sort it accordingly.

  The other change is using mod. The files are sorted hierachically as modules.

  The privacy of each function is changed. Rust automatically assume each function is private to that mod unless you declare it as public. Therefore, only the necessary methods that are needed are changed to public.

  The variables are changed such that not every variable is mutable which help prevent errors that shouldn't occur.

## Challenges
  There were many challenges that occured when translating it to Rust as the two languages contains many different design choices. That includes types, priveleges, file management system, and organization. It took a quite of bit of time for me to get used to the syntax and the return types of some functions. 

## Instructions To Run The Program
  
  "cargo run <algorithm> <inputfile>" - the command runs the program and it takes in a algorithm and text file as argument

  "cargo build" - compiles the program

  "cargo build --release" - compiles the program with optimization

  "./target/debug/cs377-project <algorithm> <inputfile>" - runs the excutable

  "cargo test" - to test the code

  algorithm example: fifo and sjf

  inputfile example: workloads/workload_01.txt

## Link to Video

https://drive.google.com/file/d/1Gm9lqtgoSZ-VPq4ovUG0jc01vPyBNZ1Y/view?usp=sharing
