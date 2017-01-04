extern crate rain;
extern crate rand;
extern crate log;

use std::thread;
use std::time::Duration;

use rain::Graph;
use log::LogLevel;
use rand::distributions::{IndependentSample, Range};

#[test]
fn add_remove_success_1() {
    let mut graph = Graph::new().set_log_level(LogLevel::Warn);
    assert!(graph.add("Line 1", 0).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.add("Line 2", 0).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.add("Line 3", 0).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.remove("Line 2").is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.add("Line 3", 0).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.add("Line 4", 0).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.add("Line 3", 0).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.remove("Line 1").is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.add("Line 3", 0).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.add("Line 1", 0).is_ok());
    assert!(graph.print().is_ok());
}

#[test]
fn add_remove_success_2() {
    let mut graph = Graph::new().set_log_level(LogLevel::Warn);
    for i in 1..50 {
        if i > 25 {
            assert!(graph.remove(&(i - 25).to_string()).is_ok());
        }
        assert!(graph.add(&i.to_string(), i).is_ok());
    }
    for _ in 0..5 {
        assert!(graph.print().is_ok());
    }
}

#[test]
fn add_remove_success_3() {
    let mut graph = Graph::new();
    assert!(graph.add("0", 100).is_ok());
    for i in 1..101 {
        assert!(graph.add("0", i).is_ok());
        assert!(graph.print().is_ok());
    }
    assert!(graph.remove("0").is_ok());
    assert!(graph.print().is_ok());
}

#[test]
fn add_remove_success_4() {
    let mut graph = Graph::new();
    let l1 = "Line 1";
    let l2 = "Line 2";
    assert!(graph.add(l1, 0).is_ok());
    assert!(graph.add(l2, 0).is_ok());
    for _ in 0..3 {
        assert!(graph.print().is_ok());
    }
    for _ in 0..3 {
        assert!(graph.add(l2, 0).is_ok());
        assert!(graph.print().is_ok());
    }
    for _ in 0..3 {
        assert!(graph.add(l1, 0).is_ok());
        assert!(graph.print().is_ok());
    }
    assert!(graph.remove(l1).is_ok());
    assert!(graph.remove(l2).is_ok());
    assert!(graph.print().is_ok());
}

#[test]
fn print_if_new_data_success() {
    let mut graph = Graph::new();
    let l1 = "Line 1";
    let l2 = "Line 2";

    assert!(graph.add(l1, 0).is_ok());
    assert!(graph.add(l2, 0).is_ok());
    assert_eq!(graph.print_if_new_data().unwrap(), true);
    assert_eq!(graph.print_if_new_data().unwrap(), false);

    assert!(graph.remove(l2).is_ok());
    assert_eq!(graph.print_if_new_data().unwrap(), true);
    assert_eq!(graph.print_if_new_data().unwrap(), false);

    assert!(graph.add(l2, 0).is_ok());
    assert_eq!(graph.print_if_new_data().unwrap(), true);
    assert_eq!(graph.print_if_new_data().unwrap(), false);
    assert_eq!(graph.print_if_new_data().unwrap(), false);
}

#[test]
fn random_add_remove_success() {
    let mut graph = Graph::new();
    let between = Range::new(0, 10);
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let a = between.ind_sample(&mut rng);
        let b = between.ind_sample(&mut rng);

        if b % 3 == 0 {
            assert!(graph.add(a, b).is_ok());
        }

        if a % 3 == 0 {
            graph.remove(b).ok();
        }
        assert!(graph.print_if_new_data().is_ok());
        thread::sleep(Duration::from_millis(10));
    }
}

#[test]
fn add_remove_success_signed_integer() {
    let mut graph = Graph::new();
    let l1 = "L 1";
    let l2 = "L 2";
    let l3 = "Too long line 3";
    assert!(graph.add(l1, -10).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.add(l2, 0).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.add(l3, 10).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.remove(l1).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.remove(l2).is_ok());
    assert!(graph.print().is_ok());
    assert!(graph.remove(l3).is_ok());
    assert!(graph.print().is_ok());
}
