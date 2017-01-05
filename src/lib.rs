//! # Visualize vertical data inside your terminal
//!
//! This library helps you to display line based data vertically within your terminal. The color of
//! the actual graph represents its value, whereas blue is low and red is high. These color bounds
//! will be calculated automatically during runtime. Beside this, the terminal dimensions are
//! adapted during runtime, too. If no data was added to a line, their terminal line is dashed.
//!
//! # Example usage
//! ```
//! use rain::Graph;
//!
//! // Get a drawing area
//! let mut graph = Graph::new();
//!
//! // Get some line identifiers
//! let l1 = "Line 1";
//! let l2 = "Line 1";
//! let l3 = "Line 1";
//!
//! // Add some values and print
//! assert!(graph.add(l1, 0).is_ok());
//! assert!(graph.add(l2, 0).is_ok());
//! graph.print();
//!
//! // Add more values and print
//! assert!(graph.add(l2, 5).is_ok());
//! assert!(graph.add(l3, 10).is_ok());
//! graph.print();
//!
//! // Remove a line and print
//! assert!(graph.remove(l1).is_ok());
//! graph.print();
//! ```
#![deny(missing_docs)]

#[macro_use]
extern crate log;
extern crate mowl;
extern crate termion;

#[macro_use]
pub mod error;

use std::u8;
use std::cmp::max;
use std::{convert, fmt, iter};

use error::{RainResult, ErrorType};

use log::LogLevel;
use termion::color::{self, LightBlack, Reset, Fg};

/// The graph drawing structure
pub struct Graph<V> {
    lines_to_be_removed: Vec<String>,
    columns: Vec<Column<V>>,
    prefix_len: usize,
}

impl<V> Graph<V>
    where V: Clone + Default + Ord + PartialEq + fmt::Debug,
          f64: convert::From<V>
{
    /// Create a new `Graph` for drawing
    ///
    /// # Example
    /// ```
    /// use rain::Graph;
    ///
    /// let _ :Graph<u8> = Graph::new();
    /// ```
    pub fn new() -> Self {
        Self::with_prefix_length(8)
    }

    /// Create a new `Graph` for drawing with a custom length of the identifier (prefix)
    ///
    /// # Example
    /// ```
    /// use rain::Graph;
    ///
    /// let _ :Graph<u8> = Graph::with_prefix_length(25);
    /// ```
    pub fn with_prefix_length(length: usize) -> Self {
        Graph {
            lines_to_be_removed: vec![],
            columns: vec![],
            prefix_len: length + 3,
        }
    }


    /// Set the global log level for reporting
    pub fn set_log_level(self, level: LogLevel) -> Self {
        // Setup the logger if not already set
        if mowl::init_with_level(level).is_err() {
            warn!("Logger already set.");
        } else {
            info!("Log level set to: {:?}", level);
        }
        self
    }

    /// Add a data value to the graph by some identifier which can be displayed somehow.
    ///
    /// # Example
    /// ```
    /// use rain::Graph;
    ///
    /// let mut graph = Graph::new();
    /// let line = graph.add("Line 1", 0).unwrap();
    ///
    /// assert_eq!(line, "Line 1");
    /// ```
    pub fn add<T>(&mut self, identifier: T, value: V) -> RainResult<T>
        where T: fmt::Display
    {
        /// Get a line name string from the identifier
        let line_name = format!("{}", identifier);
        debug!("Adding value {:?} to line '{}'", value, line_name);

        // Just add the value if the line already exist
        let add_new_line = {
            if let Some(line) = self.line_already_existing(&line_name) {
                debug!("Line already exist, just adding the value");
                line.add_value(value.clone());
                false
            } else {
                true
            }
        };

        // Add a new line and set the column as used
        if add_new_line {
            debug!("Adding new line");
            let column = self.get_next_free_column();
            let mut line = Line::new(&line_name);
            line.add_value(value);
            *column = Column::Used(line);
        }

        Ok(identifier)
    }

    /// Remove a line from the graph
    ///
    /// # Example
    /// ```
    /// use rain::Graph;
    ///
    /// let mut graph = Graph::new();
    /// let line = graph.add("Line 1", 0).unwrap();
    ///
    /// let removed_line = graph.remove(line).unwrap();
    /// assert_eq!(removed_line, "Line 1");
    /// ```
    pub fn remove<T>(&mut self, identifier: T) -> RainResult<T>
        where T: fmt::Display
    {
        // Check if the line exists
        let line_name = format!("{}", identifier);
        if let None = self.line_already_existing(&line_name) {
            bail!(ErrorType::LineDoesNotExist,
                  "Line does not exist and can not be removed");
        }

        // Just push the line into a temporarily vector
        self.lines_to_be_removed.push(line_name);

        Ok(identifier)
    }

    /// Prints the graph
    ///
    /// # Example
    /// ```
    /// use rain::Graph;
    ///
    /// let mut graph = Graph::new();
    ///
    /// assert!(graph.add("Line 1", 0).is_ok());
    /// assert!(graph.add("Line 2", 0).is_ok());
    ///
    /// graph.print();
    /// ```
    pub fn print(&mut self) -> RainResult<()> {
        /// Prints the fillchar to the terminal
        fn fillchar() -> String {
            format!("{}┈{}", Fg(LightBlack), Fg(Reset))
        }

        // Do the actual printing per column
        let start_ch = "┬";
        let line_chr = "│";
        let nodata_c = "╎";
        let end_char = "┴";
        let col_width = 2;

        let (width, _) = termion::terminal_size()?;

        let mut cursor = self.prefix_len as u16;
        let end_cursor = if width % 2 == 0 { width - 2 } else { width - 1 };

        // A string representation for a row to be printed
        struct Row {
            content: String,
            prefix: Option<String>,
        }
        let mut row = Row {
            content: String::with_capacity(width as usize),
            prefix: None,
        };

        // Returns the maximum or minimum value of all available values
        macro_rules! get_value {
            ($($p:ident)*) => (
                $(self.columns.iter().filter_map(|c| {
                    match *c {
                        Column::Used(ref line) => line.values.iter().$p(),
                        _ => None,
                    }
                }).$p().cloned().unwrap_or_default())*
            )
        }

        // Get the current minimum and maximum values from all lines
        let (min, max) = (get_value!(min), get_value!(max));

        // Gather all columns together
        for column in self.columns.iter_mut() {
            // Check if we an print more columns
            if end_cursor < cursor + col_width {
                row.content += "…";
                cursor += 1;
                break;
            }

            // Column can be printed
            let free_column = match *column {
                Column::Used(ref mut line) => {
                    // Get a row prefix format and keep three characters left
                    let mut row_prefix = format!("{:>w$.*}",
                                                 self.prefix_len - 3,
                                                 line.name,
                                                 w = self.prefix_len - 3);

                    // Get the character to be printed
                    let (c, free_column) = if line.started {
                        // Check if the line is done an can be used later on
                        if self.lines_to_be_removed.contains(&line.name) {
                            row_prefix += " ← ";
                            row.prefix = Some(row_prefix);
                            (end_char, true)
                        } else {
                            (if line.got_data { line_chr } else { nodata_c }, false)
                        }
                    } else {
                        row_prefix += " → ";
                        row.prefix = Some(row_prefix);
                        line.started = true;
                        (start_ch, false)
                    };

                    // Get the rgb value for the character
                    let value = line.values.last().cloned().unwrap_or_default();
                    let (r, g, b) = Self::rgb(min.clone(), max.clone(), value.clone());

                    row.content += &format!("{}{}{}", Fg(color::Rgb(r, g, b)), c, Fg(Reset));
                    row.content += &fillchar();

                    // Reset the line indicator for the data
                    line.got_data = false;

                    free_column
                }
                Column::Free => {
                    row.content += &fillchar();
                    row.content += &fillchar();
                    false
                }
            };
            if free_column {
                *column = Column::Free;
            }

            cursor += col_width;
        }

        // Fill rest of the screen
        for _ in cursor..width {
            row.content += &fillchar();
        }

        // Print the row including the prefix if set
        let prefix_string = match row.prefix {
            Some(prefix) => prefix,
            _ => iter::repeat(' ').take(self.prefix_len).collect::<String>(),
        };
        println!("{}{}", prefix_string, row.content);

        // Cleanup lines to be removed
        self.lines_to_be_removed.clear();
        Ok(())
    }

    /// Print only if new data is available. Returns an indicator if somethings was printed or not.
    ///
    /// # Example
    /// ```
    /// use rain::Graph;
    ///
    /// let mut graph = Graph::new();
    ///
    /// assert!(graph.add("Line 1", 0).is_ok());
    /// assert!(graph.add("Line 2", 0).is_ok());
    ///
    /// graph.print_if_new_data();
    /// graph.print_if_new_data();
    /// ```
    pub fn print_if_new_data(&mut self) -> RainResult<bool> {
        if !self.lines_to_be_removed.is_empty() ||
           self.columns
            .iter()
            .filter(|c| match **c {
                Column::Used(ref line) if line.got_data => true,
                _ => false,
            })
            .count() > 0 {
            self.print()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get the next free column and set the column as used
    fn get_next_free_column(&mut self) -> &mut Column<V> {
        macro_rules! free_column_iter {
            () => (self.columns.iter_mut().filter(|c| **c == Column::Free))
        }

        let free_column_count = free_column_iter!().count();

        if free_column_count == 0 {
            self.columns.push(Column::Free);
            self.columns.iter_mut().rev().next().unwrap()
        } else {
            free_column_iter!().next().unwrap()
        }
    }

    // Returns a line if the name already exist within all columns
    fn line_already_existing(&mut self, line_name: &str) -> Option<&mut Line<V>> {
        let line_string = line_name.to_owned();
        self.columns
            .iter_mut()
            .filter_map(|c| match *c {
                Column::Used(ref mut line) if line_string == line.name => Some(line),
                _ => None,
            })
            .next()
    }

    fn rgb(minimum: V, maximum: V, value: V) -> (u8, u8, u8) {
        // Lightens up the colors
        let soft_scale = 125;
        if minimum == maximum {
            return (soft_scale, soft_scale, u8::MAX);
        }

        // Calculate a RGB value over the complete gradient
        let minimum = f64::from(minimum);
        let maximum = f64::from(maximum);
        let value = f64::from(value);
        let ratio = 2f64 * (value - minimum) / (maximum - minimum);
        let mut b = max(0, (255f64 * (1f64 - ratio)) as i64) as u8;
        let mut r = max(0, (255f64 * (ratio - 1f64)) as i64) as u8;
        let mut g = (255 - b - r) as u8;

        // Lighten up the values
        b = b.saturating_add(soft_scale);
        r = r.saturating_add(soft_scale);
        g = g.saturating_add(soft_scale);

        (r, g, b)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Representation of a set of data `Point` values
struct Line<V> {
    got_data: bool,
    name: String,
    started: bool,
    values: Vec<V>,
}

impl<V> Line<V> {
    /// Creates a new `Line`
    fn new(name: &str) -> Self {
        Line {
            got_data: false,
            name: name.to_owned(),
            started: false,
            values: vec![],
        }
    }

    /// Adds a value to a line
    fn add_value(&mut self, value: V) {
        self.values.push(value);
        self.got_data = true;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Specifies if a column can be used or not
enum Column<V> {
    /// Column free for usage
    Free,

    /// Column already in use
    Used(Line<V>),
}
