# rain
[![Build Status](https://travis-ci.org/saschagrunert/rain.svg)](https://travis-ci.org/saschagrunert/rain) [![Coverage Status](https://coveralls.io/repos/github/saschagrunert/rain/badge.svg)](https://coveralls.io/github/saschagrunert/rain?branch=master) [![master doc rain](https://img.shields.io/badge/master_doc-rain-blue.svg)](https://saschagrunert.github.io/rain) [![License MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/saschagrunert/rain/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/v/rain.svg)](https://crates.io/crates/rain) [![doc.rs](https://docs.rs/rain/badge.svg)](https://docs.rs/rain)
## Visualize vertical data inside your terminal
This library helps you to display line based data vertically within your terminal. The color of the actual graph
represents its value, whereas blue is low and red is high. These color bounds will be calculated automatically during
runtime. Beside this, the terminal dimensions are adapted during runtime, too. If no data was added to a line, their
terminal line is dashed.


## Example usage

```rust
use rain::Graph;

// Get a drawing area
let mut graph = Graph::new();

// Get some line identifiers
let l1 = "Line 1";
let l2 = "Line 1";
let l3 = "Line 1";

// Add some values and print
assert!(graph.add(l1, 0).is_ok());
assert!(graph.add(l2, 0).is_ok());
graph.print();

// Add more values and print
assert!(graph.add(l2, 5).is_ok());
assert!(graph.add(l3, 10).is_ok());
graph.print();

// Remove a line and print
assert!(graph.remove(l1).is_ok());
graph.print();
```

## Contributing
You want to contribute to this project? Wow, thanks! So please just fork it and send me a pull request.
