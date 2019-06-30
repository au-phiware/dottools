#![feature(proc_macro_span)]

extern crate petgraph;
extern crate proc_macro;

#[cfg(test)]
mod tests;

use petgraph::graphmap::UnGraphMap;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use unicode_normalization::char::is_combining_mark;

#[derive(Debug)]
pub enum Error {
    TODO,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct LineColumn {
    line: usize,
    column: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum Brush {
    NorthSouth(char),
    EastWest(char),
    NorthEastSouthWest(char),
    NorthWestSouthEast(char),
}

#[derive(Debug, Copy, Clone)]
pub struct Edge(Option<char>, Brush, Option<char>);

#[derive(Debug)]
struct Graph(UnGraphMap<LineColumn, Edge>);

/*
"  ┌──
 ─╲┼╱┐
  │╳ │
  └──┘
   ┏━━
 ━╲╋╱┓
  ┃╳ ┃
  ┗━━┛
   +--
 -\+/+
  |X |
  +--+
   ╔══
 ═╲╬╱╗
  ║╳ ║
  ╚══╝
   ╱╲
   ╲╱
"
*/

impl From<proc_macro::LineColumn> for LineColumn {
    fn from(loc: proc_macro::LineColumn) -> Self {
        let proc_macro::LineColumn { line, column } = loc;
        LineColumn { line, column }
    }
}

/*
impl From<char> for Brush {
    fn from(c: char) -> Option<Self> {
        match c {
            '│' | '║' | '┃' | '┊' | '┋' | '┆' | '┇' | '╎' | '╏' | '|' => Some(Brush::NorthSouth(c)),
            '─' | '═' | '━' | '┈' | '┉' | '┄' | '┅' | '╌' | '╍' | '-' => Some(Brush::EastWest(c)),
            '╱' | '/' => Some(Brush::NorthEastSouthWest(c)),
            '╲' | '\\' => Some(Brush::NorthWestSouthEast(c)),
            _ => None,
        }
    }
}
*/

impl Ord for LineColumn {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.line.cmp(&other.line) {
            Ordering::Equal => self.column.cmp(&other.column),
            o => o,
        }
    }
}

impl PartialOrd for LineColumn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Deref for Graph {
    type Target = UnGraphMap<LineColumn, Edge>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Copy, Clone)]
enum Tx {
    Initial,
    Build { start: LineColumn, edge: Edge },
}

struct State {
    graph: UnGraphMap<LineColumn, Edge>,
    ports: Vec<(LineColumn, LineColumn, Edge)>,
    source_location: LineColumn,
    visible_location: LineColumn,
    tx: Tx,
}

impl State {
    fn next(&mut self, c: char) {
        self.tx = match self.tx {
            Tx::Initial => match c {
                '─' | '═' | '━' | '┈' | '┉' | '┄' | '┅' | '╌' | '╍' | '-'
                | '=' => {
                    let start = self.graph.add_node(self.source_location.clone());
                    let edge = Edge(None, Brush::EastWest(c), None);
                    Tx::Build { start, edge }
                }
                '│' | '║' | '┃' | '┊' | '┋' | '┆' | '┇' | '╎' | '╏' | '|' => {
                    let start = self.graph.add_node(self.source_location.clone());
                    self.ports.push((
                        self.visible_location,
                        start,
                        Edge(None, Brush::NorthSouth(c), None),
                    ));
                    Tx::Initial
                }
                _ => Tx::Initial,
            },
            Tx::Build { start, edge } => match edge.1 {
                Brush::EastWest(chr) if c == chr => self.tx,
                _ => {
                    let mut source_location = self.source_location.clone();
                    source_location.column -= 1;
                    let end = self.graph.add_node(source_location);
                    self.graph.add_edge(start, end, edge);
                    Tx::Initial
                }
            },
        }
    }

    fn finish(&mut self) {
        match self.tx {
            Tx::Initial => (),
            _ => self.next('\n'),
        }
    }
}

impl FromStr for Graph {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut state = &mut State {
            graph: UnGraphMap::new(),
            ports: Vec::new(),
            source_location: LineColumn { line: 1, column: 0 },
            visible_location: LineColumn { line: 1, column: 0 },
            tx: Tx::Initial,
        };

        for c in input.chars() {
            if !is_combining_mark(c) {
                state.next(c);

                state.visible_location.column += 1;
            }

            // TODO drop unreachable ports

            if c == '\n' {
                state.visible_location.line += 1;
                state.visible_location.column = 0;
                state.source_location.line += 1;
                state.source_location.column = 0;
            } else {
                state.source_location.column += 1;
            }
        }
        state.finish();

        Ok(Graph(state.graph.to_owned()))
    }
}
