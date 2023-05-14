#[cfg(test)]
mod tests {
    use crate::scheduling_process::Process;
    use crate::read_workload;
    use std::collections::BinaryHeap;
    #[test]
    fn test_read_workload1() {
        let pq: BinaryHeap<Process> = read_workload("workloads/workload_01.txt");
        assert_eq!(pq.len(), 3);
    }

    #[test]
    fn test_read_workload2() {
        let pq: BinaryHeap<Process> = read_workload("workloads/not_a_file.txt");
        assert_eq!(pq.len(), 0);
    }

    #[test]
    fn test_read_workload3() {
        let mut pq: BinaryHeap<Process> = read_workload("workloads/workload_01.txt");

        assert!(!pq.is_empty());
        let x1 = pq.pop().unwrap();
        assert_eq!(x1.arrival, 0);

        assert!(!pq.is_empty());
        let x2 = pq.pop().unwrap();
        assert_eq!(x2.arrival, 0);

        assert!(!pq.is_empty());
        let x3 = pq.pop().unwrap();
        assert_eq!(x3.arrival, 0);
    }

    #[test]
    fn test_read_workload4() {
        let mut pq: BinaryHeap<Process> = read_workload("workloads/workload_01.txt");

        assert!(!pq.is_empty());
        let x1 = pq.pop().unwrap();
        assert_eq!(x1.duration, 10);

        assert!(!pq.is_empty());
        let x2 = pq.pop().unwrap();
        assert_eq!(x2.duration, 10);

        assert!(!pq.is_empty());
        let x3 = pq.pop().unwrap();
        assert_eq!(x3.duration, 10);
    }
}