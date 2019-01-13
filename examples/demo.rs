use rain::Graph;
use rand::distributions::{Distribution, Range};
use std::{thread, time::Duration};

fn main() {
    let g = &mut Graph::new();
    let l1 = "Alpha";
    let l2 = "Beta";
    let l3 = "Gamma";

    fn sleep() {
        let between = Range::new(100, 800);
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(between.sample(&mut rng)));
    }

    fn print(graph: &mut Graph<u8>) {
        assert!(graph.print().is_ok());
        sleep();
    }

    fn add(graph: &mut Graph<u8>, line: &str, value: u8) {
        assert!(graph.add(line, value).is_ok());
    }

    fn rm(graph: &mut Graph<u8>, line: &str) {
        assert!(graph.remove(line).is_ok());
    }

    add(g, l1, 20);
    print(g);

    add(g, l1, 2);
    print(g);

    add(g, l1, 3);
    add(g, l2, 25);
    print(g);

    for i in 3..10 {
        add(g, l1, i);
        add(g, l2, 25 - i);
        print(g);
    }

    for i in 1..10 {
        add(g, l3, 15 + i);
        add(g, l1, 15 - i);
        print(g);
    }

    for i in 1..4 {
        add(g, l2, 15 + i);
        print(g);
    }

    rm(g, l2);
    print(g);
    add(g, l1, 8);
    add(g, l3, 25);
    print(g);

    for i in 1..6 {
        add(g, l2, i);
        if i == 4 {
            rm(g, l1);
        }
        print(g);
    }

    add(g, l3, 22);
    add(g, l2, 10);
    print(g);
    add(g, l3, 18);
    add(g, l2, 20);
    print(g);
    add(g, l3, 14);
    print(g);
    rm(g, l3);
    print(g);
    print(g);
    print(g);
    rm(g, l2);
    for _ in 1..8 {
        print(g);
    }
}
