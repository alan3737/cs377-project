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

 impl Ord for Process {
    fn cmp(&self, other: &Self) -> Ordering {
      if self.order_by == "arrival" && other.order_by == "arrival" {
        if self.arrival != other.arrival {
          return self.arrival.cmp(&other.arrival);
        } else {
          return self.duration.cmp(&other.duration);
        }
      } else {
        if self.duration != other.duration {
          return self.duration.cmp(&other.duration);
        } else {
          return self.arrival.cmp(&other.arrival);
        }
      } 
    } 
}

impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}




pub type pqueue = BinaryHeap<Process>;