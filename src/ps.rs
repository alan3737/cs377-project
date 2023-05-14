use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Process {
  pub arrival: i32,
  pub first_run: i32,
  pub duration: i32,
  pub completion: i32,
  pub order_by: String
}

// Instructs how the Binary Heap should be sorted
 impl Ord for Process {
    fn cmp(&self, other: &Self) -> Ordering {
      if self.order_by == "arrival" || other.order_by == "arrival" {
        // Sort by arrival
        if self.arrival != other.arrival {
          return other.arrival.cmp(&self.arrival);
        } else {
          return other.duration.cmp(&self.duration);
        }
      } else {
        // Sort by duration
        if self.duration != other.duration {
          return other.duration.cmp(&self.duration);
        } else {
          return other.arrival.cmp(&self.arrival);
        }
      } 
    } 
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub type Pqueue = BinaryHeap<Process>;