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
    ports: Vec<(LineColumn, LineColumn, LineColumn, Edge)>,
    source_location: LineColumn,
    visible_location: LineColumn,
    tx: Tx,
}

impl State {
    fn next(&mut self, c: char) {
        let LineColumn { line, column } = self.visible_location;
        let ports = self.ports.to_owned();
        let mut pass = false;
        self.ports = Vec::with_capacity(ports.len() + 1);
        for (mut location, start, end, edge) in ports {
            let brush = edge.1;
            if location.line + 1 >= line {
                if match brush {
                    Brush::NorthSouth(brush) => location.column == column && brush == c,
                    Brush::EastWest(_) => false,
                    Brush::NorthEastSouthWest(brush) => location.column + 1 == column && brush == c,
                    Brush::NorthWestSouthEast(brush) => location.column == column + 1 && brush == c,
                } {
                    location.line = line;
                    location.column = column;
                    self.ports
                        .push((location, start, self.source_location, edge));
                    pass = true;
                } else if location.column == column
                    && match brush {
                        Brush::NorthSouth(brush) => match brush {
                            '│' | '┊' | '┆' | '╎' => match c {
                                '└' | '╘' | '╰' | '┕' | '┘' | '╛' | '╯' | '┙'
                                | '┴' | '┵' | '┶' | '┷' | '╧' | '╵' | '├' | '┝'
                                | '┟' | '┢' | '┤' | '┥' | '┧' | '┪' | '╞' | '╡'
                                | '┼' | '┽' | '┾' | '┿' | '╁' | '╅' | '╆' | '╈'
                                | '╪' | '╽' => true,
                                _ => false,
                            },
                            '┃' | '┋' | '┇' | '╏' => match c {
                                '┗' | '┖' | '┛' | '┚' | '┸' | '┹' | '┺' | '┻'
                                | '╹' | '┞' | '┠' | '┡' | '┣' | '┦' | '┨' | '┩'
                                | '┫' | '╀' | '╂' | '╃' | '╄' | '╇' | '╉' | '╊'
                                | '╋' | '╿' => true,
                                _ => false,
                            },
                            '║' => match c {
                                '╚' | '╙' | '╝' | '╜' | '╨' | '╩' | '╟' | '╠'
                                | '╢' | '╣' | '╫' | '╬' => true,
                                _ => false,
                            },
                            _ => false,
                        },
                        _ => false,
                    }
                {
                    self.graph.add_edge(start, self.source_location, edge);
                    if let Some(brush) = match c {
                        '├' | '┝' | '┤' | '┥' | '╞' | '╡' | '┼' | '┽' | '┾'
                        | '┿' | '╪' | '┞' | '┡' | '┦' | '┩' | '╀' | '╃' | '╄'
                        | '╇' | '╿' => Some('│'),
                        '┟' | '┢' | '┧' | '┪' | '╁' | '╅' | '╆' | '╈' | '╽'
                        | '┠' | '┣' | '┨' | '┫' | '╂' | '╉' | '╊' | '╋' => {
                            Some('┃')
                        }
                        '╟' | '╠' | '╢' | '╣' | '╫' | '╬' => Some('║'),
                        _ => None,
                    } {
                        self.ports.push((
                            self.visible_location,
                            self.source_location,
                            self.source_location,
                            Edge(None, Brush::NorthSouth(brush), None),
                        ));
                    }
                } else {
                    self.ports.push((location, start, end, edge));
                }
            } else {
                self.graph.add_edge(start, end, edge);
            }
        }

        if pass {
            return;
        }

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
                        self.source_location,
                        Edge(None, Brush::NorthSouth(c), None),
                    ));
                    Tx::Initial
                }
                '┘' | '╜' | '╯' | '┚' | '┐' | '╖' | '╮' | '┒' | '┛' | '┙'
                | '┓' | '┑' | '╝' | '╛' | '╗' | '╕' | '┴' | '┵' | '┶' | '┷'
                | '╧' | '┤' | '┥' | '┧' | '┪' | '╡' | '┼' | '┽' | '┾' | '┿'
                | '╁' | '╅' | '╆' | '╈' | '╪' | '┸' | '┹' | '┺' | '┻' | '┦'
                | '┨' | '┩' | '┫' | '╀' | '╂' | '╃' | '╄' | '╇' | '╉' | '╊'
                | '╋' | '╨' | '╩' | '╢' | '╣' | '╫' | '╬' => {
                    let brush = match c {
                        '┛' | '┙' | '┓' | '┑' | '┵' | '┷' | '┥' | '┪' | '┽'
                        | '┿' | '╅' | '╈' | '┹' | '┻' | '┩' | '┫' | '╃' | '╇'
                        | '╉' | '╋' => '━',
                        '╝' | '╛' | '╗' | '╕' | '╧' | '╡' | '╪' | '╩' | '╣'
                        | '╬' => '═',
                        _ => '─',
                    };
                    self.graph.add_edge(
                        self.source_location,
                        self.source_location,
                        Edge(None, Brush::EastWest(brush), None),
                    );
                    if let Some(brush) = match c {
                        '┐' | '╮' | '┑' | '╕' | '┤' | '┥' | '╡' | '┼' | '┽'
                        | '┾' | '┿' | '╪' | '┦' | '┩' | '╀' | '╃' | '╄' | '╇' => {
                            Some('│')
                        }
                        '┓' | '┒' | '┧' | '┪' | '╁' | '╅' | '╆' | '╈' | '┨'
                        | '┫' | '╂' | '╉' | '╊' | '╋' => Some('┃'),
                        '╗' | '╖' | '╢' | '╣' | '╫' | '╬' => Some('║'),
                        _ => None,
                    } {
                        self.ports.push((
                            self.visible_location,
                            self.source_location,
                            self.source_location,
                            Edge(None, Brush::NorthSouth(brush), None),
                        ));
                    }
                    match match c {
                        '┼' | '┽' | '┴' | '┵' | '╁' | '╅' | '┸' | '┹' | '╀'
                        | '╂' | '╃' | '╉' | '╨' | '╫' => Some('─'),
                        '╋' | '┶' | '┷' | '┾' | '┿' | '╆' | '╈' | '┺' | '┻'
                        | '╄' | '╇' | '╊' => Some('━'),
                        '╬' | '╩' | '╧' | '╪' => Some('═'),
                        _ => None,
                    } {
                        Some(brush) => {
                            let start = self.graph.add_node(self.source_location.clone());
                            let edge = Edge(None, Brush::EastWest(brush), None);
                            Tx::Build { start, edge }
                        }
                        _ => Tx::Initial,
                    }
                }
                '┌' | '╔' | '┏' | '╒' | '╓' | '╭' | '┍' | '┎' => {
                    let start = self.graph.add_node(self.source_location.clone());
                    let brush = match c {
                        '╔' | '╓' => '║',
                        '┏' | '┎' => '┃',
                        _ => '│',
                    };
                    self.ports.push((
                        self.visible_location,
                        start,
                        self.source_location,
                        Edge(None, Brush::NorthSouth(brush), None),
                    ));
                    let brush = match c {
                        '╔' | '╒' => '═',
                        '┏' | '┍' => '━',
                        _ => '─',
                    };
                    let edge = Edge(None, Brush::EastWest(brush), None);
                    Tx::Build { start, edge }
                }
                '└' | '╚' | '┗' | '╘' | '╙' | '╰' | '┕' | '┖' => {
                    let start = self.graph.add_node(self.source_location.clone());
                    let brush = match c {
                        '╚' | '╘' => '═',
                        '┗' | '┕' => '━',
                        _ => '─',
                    };
                    let edge = Edge(None, Brush::EastWest(brush), None);
                    Tx::Build { start, edge }
                }
                _ => Tx::Initial,
            },
            Tx::Build { start, edge } => match edge.1 {
                Brush::EastWest(brush) if c == brush => self.tx,
                _ => {
                    let end = match edge.1 {
                        Brush::EastWest(brush)
                            if match brush {
                                '─' | '┈' | '┄' | '╌' => match c {
                                    '┘' | '╜' | '╯' | '┚' | '┐' | '╖' | '╮'
                                    | '┒' | '┴' | '┶' | '┤' | '┧' | '┼' | '┾'
                                    | '╁' | '╆' | '┸' | '┺' | '┦' | '┨' | '╀'
                                    | '╂' | '╄' | '╊' | '╨' | '╢' | '╫' => true,
                                    _ => false,
                                },
                                '━' | '┉' | '┅' | '╍' => match c {
                                    '┛' | '┙' | '┓' | '┑' | '┵' | '┷' | '┥'
                                    | '┪' | '┽' | '┿' | '╅' | '╈' | '┹' | '┻'
                                    | '┩' | '┫' | '╃' | '╇' | '╉' | '╋' => true,
                                    _ => false,
                                },
                                '═' => match c {
                                    '╝' | '╛' | '╗' | '╕' | '╧' | '╡' | '╪'
                                    | '╩' | '╣' | '╬' => true,
                                    _ => false,
                                },
                                _ => false,
                            } =>
                        {
                            let start = self.graph.add_node(self.source_location);
                            let brush = match c {
                                '┐' | '┑' | '╮' | '╕' | '┤' | '┥' | '╡' | '┼'
                                | '┽' | '┾' | '┿' | '╪' | '┦' | '┩' | '╀' | '╃'
                                | '╄' | '╇' => Some('│'),
                                '╗' | '╖' | '╢' | '╣' | '╫' | '╬' => Some('║'),
                                '┓' | '┒' | '┧' | '┪' | '╁' | '╅' | '╆' | '╈'
                                | '┨' | '┫' | '╂' | '╉' | '╊' | '╋' => Some('┃'),
                                _ => None,
                            };
                            if let Some(brush) = brush {
                                self.ports.push((
                                    self.visible_location,
                                    start,
                                    self.source_location,
                                    Edge(None, Brush::NorthSouth(brush), None),
                                ));
                            }
                            start
                        }
                        _ => {
                            let mut source_location = self.source_location.clone();
                            source_location.column -= 1;
                            self.graph.add_node(source_location)
                        }
                    };
                    self.graph.add_edge(start, end, edge);
                    match match c {
                        '┼' | '┴' | '┵' | '┽' | '╁' | '╅' | '┸' | '┹' | '╀'
                        | '╂' | '╃' | '╉' | '╨' | '╫' => Some('─'),
                        '╋' | '┶' | '┷' | '┾' | '┿' | '╆' | '╈' | '┺' | '┻'
                        | '╄' | '╇' | '╊' => Some('━'),
                        '╬' | '╧' | '╪' | '╩' => Some('═'),
                        _ => None,
                    } {
                        Some(brush) => Tx::Build {
                            start: self.source_location,
                            edge: Edge(None, Brush::EastWest(brush), None),
                        },
                        None => Tx::Initial,
                    }
                }
            },
        };
    }

    fn start() -> Self {
        State {
            graph: UnGraphMap::new(),
            ports: Vec::new(),
            source_location: LineColumn { line: 1, column: 0 },
            visible_location: LineColumn { line: 1, column: 0 },
            tx: Tx::Initial,
        }
    }

    fn finish(&mut self) {
        self.visible_location.line = std::usize::MAX;
        self.visible_location.column = 0;
        self.next('\n')
    }
}

impl FromStr for Graph {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut state = &mut State::start();

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
