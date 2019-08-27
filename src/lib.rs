#![feature(proc_macro_span)]

extern crate petgraph;
extern crate proc_macro;

#[cfg(test)]
mod tests;

use petgraph::graphmap::UnGraphMap;
use std::cmp::Ordering;
use std::fmt;
use std::iter::repeat;
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    character: char,
    source: LineColumn,
    visual: LineColumn,
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
pub struct Graph(UnGraphMap<Node, Edge>);

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

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source.cmp(&other.source)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.source.partial_cmp(&other.source)
    }
}

impl Deref for Graph {
    type Target = UnGraphMap<Node, Edge>;

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
    Build { start: Node, edge: Edge },
}

#[derive(Debug, Copy, Clone)]
struct Port {
    start: Node,
    end: Node,
    edge: Edge,
}

struct State {
    graph: UnGraphMap<Node, Edge>,
    ports: Vec<Port>,
    location: Node,
    previous_location: Node,
    tx: Tx,
}

impl State {
    fn next(&mut self) {
        let c = self.location.character;
        let LineColumn { line, column } = self.location.visual;
        let ports = self.ports.to_owned();
        let mut pass = false;
        let mut built_north_west_south_east = false;
        let mut built_north_east_south_west = false;
        self.ports = Vec::with_capacity(ports.len() + 1);
        for port in ports {
            let brush = port.edge.1;
            let location = port.end.visual;
            if location.line + 1 >= line {
                if match brush {
                    Brush::NorthSouth(brush) => location.column == column && brush == c,
                    Brush::EastWest(_) => false,
                    Brush::NorthWestSouthEast(brush) => location.column + 1 == column && brush == c,
                    Brush::NorthEastSouthWest(brush) => location.column == column + 1 && brush == c,
                } {
                    self.ports.push(Port {
                        start: port.start,
                        end: self.location,
                        edge: port.edge,
                    });
                    pass = true;
                } else if location.column + 1 == column
                    && match brush {
                        Brush::NorthWestSouthEast(brush) => match brush {
                            '╲' => match c {
                                '╳' => true,
                                _ => false,
                            },
                            '\\' => match c {
                                'X' => true,
                                _ => false,
                            },
                            _ => false,
                        },
                        _ => false,
                    }
                {
                    self.graph.add_edge(port.start, self.location, port.edge);
                    built_north_west_south_east = true;
                } else if location.column == column + 1
                    && match brush {
                        Brush::NorthEastSouthWest(brush) => match brush {
                            '╱' => match c {
                                '╳' => true,
                                _ => false,
                            },
                            '/' => match c {
                                'X' => true,
                                _ => false,
                            },
                            _ => false,
                        },
                        _ => false,
                    }
                {
                    self.graph.add_edge(port.start, self.location, port.edge);
                    built_north_east_south_west = true;
                } else if location.column == column
                    && match brush {
                        Brush::NorthSouth(brush) => match brush {
                            '│' | '┊' | '┆' | '╎' => match c {
                                '└' | '╘' | '╰' | '┕' | '┘' | '╛' | '╯' | '┙' | '┴' | '┵' | '┶'
                                | '┷' | '╧' | '╵' | '├' | '┝' | '┟' | '┢' | '┤' | '┥' | '┧'
                                | '┪' | '╞' | '╡' | '┼' | '┽' | '┾' | '┿' | '╁' | '╅' | '╆'
                                | '╈' | '╪' | '╽' => true,
                                _ => false,
                            },
                            '┃' | '┋' | '┇' | '╏' => match c {
                                '┗' | '┖' | '┛' | '┚' | '┸' | '┹' | '┺' | '┻' | '╹' | '┞' | '┠'
                                | '┡' | '┣' | '┦' | '┨' | '┩' | '┫' | '╀' | '╂' | '╃' | '╄'
                                | '╇' | '╉' | '╊' | '╋' | '╿' => true,
                                _ => false,
                            },
                            '║' => match c {
                                '╚' | '╙' | '╝' | '╜' | '╨' | '╩' | '╟' | '╠' | '╢' | '╣' | '╫'
                                | '╬' => true,
                                _ => false,
                            },
                            _ => false,
                        },
                        _ => false,
                    }
                {
                    self.graph.add_edge(port.start, self.location, port.edge);
                    if let Some(brush) = match c {
                        '├' | '┝' | '┤' | '┥' | '╞' | '╡' | '┼' | '┽' | '┾' | '┿' | '╪' | '┞'
                        | '┡' | '┦' | '┩' | '╀' | '╃' | '╄' | '╇' | '╿' => {
                            Some('│')
                        }
                        '┟' | '┢' | '┧' | '┪' | '╁' | '╅' | '╆' | '╈' | '╽' | '┠' | '┣' | '┨'
                        | '┫' | '╂' | '╉' | '╊' | '╋' => Some('┃'),
                        '╟' | '╠' | '╢' | '╣' | '╫' | '╬' => Some('║'),
                        _ => None,
                    } {
                        self.ports.push(Port {
                            start: self.location,
                            end: self.location,
                            edge: Edge(None, Brush::NorthSouth(brush), None),
                        });
                    }
                } else {
                    self.ports.push(port);
                }
            } else {
                self.graph.add_edge(port.start, port.end, port.edge);
            }
        }

        if pass {
            return;
        }

        self.tx = match self.tx {
            Tx::Initial => match c {
                '─' | '═' | '━' | '┈' | '┉' | '┄' | '┅' | '╌' | '╍' | '-' | '=' =>
                {
                    let start = self.graph.add_node(self.location.clone());
                    let edge = Edge(None, Brush::EastWest(c), None);
                    Tx::Build { start, edge }
                }
                '│' | '║' | '┃' | '┊' | '┋' | '┆' | '┇' | '╎' | '╏' | '|' | '\\' | '╲' | '/'
                | '╱' => {
                    let start = self.graph.add_node(self.location.clone());
                    let brush = match c {
                        '\\' | '╲' => Brush::NorthWestSouthEast(c),
                        '/' | '╱' => Brush::NorthEastSouthWest(c),
                        _ => Brush::NorthSouth(c),
                    };
                    self.ports.push(Port {
                        start,
                        end: self.location,
                        edge: Edge(None, brush, None),
                    });
                    Tx::Initial
                }
                '╳' | 'X' => {
                    let start = self.graph.add_node(self.location.clone());
                    let brush = Brush::NorthWestSouthEast(match c {
                        '╳' => '╲',
                        _ => '\\',
                    });
                    if !built_north_west_south_east {
                        self.graph.add_edge(start, start, Edge(None, brush, None));
                    }
                    self.ports.push(Port {
                        start,
                        end: self.location,
                        edge: Edge(None, brush, None),
                    });
                    let brush = Brush::NorthEastSouthWest(match c {
                        '╳' => '╱',
                        _ => '/',
                    });
                    if !built_north_east_south_west {
                        self.graph.add_edge(start, start, Edge(None, brush, None));
                    }
                    self.ports.push(Port {
                        start,
                        end: self.location,
                        edge: Edge(None, brush, None),
                    });
                    Tx::Initial
                }
                '┘' | '╜' | '╯' | '┚' | '┐' | '╖' | '╮' | '┒' | '┛' | '┙' | '┓' | '┑' | '╝'
                | '╛' | '╗' | '╕' | '┴' | '┵' | '┶' | '┷' | '╧' | '┤' | '┥' | '┧' | '┪' | '╡'
                | '┼' | '┽' | '┾' | '┿' | '╁' | '╅' | '╆' | '╈' | '╪' | '┸' | '┹' | '┺' | '┻'
                | '┦' | '┨' | '┩' | '┫' | '╀' | '╂' | '╃' | '╄' | '╇' | '╉' | '╊' | '╋' | '╨'
                | '╩' | '╢' | '╣' | '╫' | '╬' => {
                    let brush = match c {
                        '┛' | '┙' | '┓' | '┑' | '┵' | '┷' | '┥' | '┪' | '┽' | '┿' | '╅' | '╈'
                        | '┹' | '┻' | '┩' | '┫' | '╃' | '╇' | '╉' | '╋' => '━',
                        '╝' | '╛' | '╗' | '╕' | '╧' | '╡' | '╪' | '╩' | '╣' | '╬' => {
                            '═'
                        }
                        _ => '─',
                    };
                    self.graph.add_edge(
                        self.location,
                        self.location,
                        Edge(None, Brush::EastWest(brush), None),
                    );
                    if let Some(brush) = match c {
                        '┐' | '╮' | '┑' | '╕' | '├' | '┝' | '┞' | '┡' | '╞' | '┤' | '┥' | '╡'
                        | '┼' | '┽' | '┾' | '┿' | '╪' | '┦' | '┩' | '╀' | '╃' | '╄' | '╇' => {
                            Some('│')
                        }
                        '┓' | '┒' | '┟' | '┠' | '┢' | '┣' | '┧' | '┪' | '╁' | '╅' | '╆' | '╈'
                        | '┨' | '┫' | '╂' | '╉' | '╊' | '╋' => Some('┃'),
                        '╗' | '╖' | '╟' | '╠' | '╢' | '╣' | '╫' | '╬' => Some('║'),
                        _ => None,
                    } {
                        self.ports.push(Port {
                            start: self.location,
                            end: self.location,
                            edge: Edge(None, Brush::NorthSouth(brush), None),
                        });
                    }
                    if let Some(brush) = match c {
                        '┼' | '┽' | '┴' | '┵' | '╁' | '╅' | '┸' | '┹' | '╀' | '├' | '┞' | '┟'
                        | '┠' | '╟' | '╂' | '╃' | '╉' | '╨' | '╫' => Some('─'),
                        '╋' | '┶' | '┷' | '┾' | '┿' | '╆' | '╈' | '┺' | '┻' | '┝' | '┡' | '┢'
                        | '┣' | '╄' | '╇' | '╊' => Some('━'),
                        '╬' | '╩' | '╧' | '╪' | '╞' | '╠' => Some('═'),
                        _ => None,
                    } {
                        let start = self.graph.add_node(self.location.clone());
                        let edge = Edge(None, Brush::EastWest(brush), None);
                        Tx::Build { start, edge }
                    } else {
                        Tx::Initial
                    }
                }
                '┌' | '╔' | '┏' | '╒' | '╓' | '╭' | '┍' | '┎' | '├' | '┝' | '┞' | '┟' | '┠'
                | '┡' | '┢' | '┣' | '╞' | '╟' | '╠' => {
                    let start = self.graph.add_node(self.location.clone());
                    let brush = match c {
                        '╔' | '╓' | '╟' | '╠' => '║',
                        '┏' | '┎' | '┟' | '┠' | '┢' | '┣' => '┃',
                        _ => '│',
                    };
                    self.ports.push(Port {
                        start,
                        end: self.location,
                        edge: Edge(None, Brush::NorthSouth(brush), None),
                    });
                    let brush = match c {
                        '╔' | '╒' | '╞' | '╠' => '═',
                        '┏' | '┍' | '┝' | '┡' | '┢' | '┣' => '━',
                        _ => '─',
                    };
                    let edge = Edge(None, Brush::EastWest(brush), None);
                    Tx::Build { start, edge }
                }
                '└' | '╚' | '┗' | '╘' | '╙' | '╰' | '┕' | '┖' => {
                    let start = self.graph.add_node(self.location.clone());
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
                                    '┘' | '╜' | '╯' | '┚' | '┐' | '╖' | '╮' | '┒' | '┬' | '┮'
                                    | '┰' | '┲' | '┴' | '┶' | '┤' | '┧' | '┼' | '┾' | '╁' | '╆'
                                    | '┸' | '┺' | '┦' | '┨' | '╀' | '╂' | '╄' | '╊' | '╨' | '╢'
                                    | '╫' => true,
                                    _ => false,
                                },
                                '━' | '┉' | '┅' | '╍' => match c {
                                    '┛' | '┙' | '┓' | '┑' | '┭' | '┯' | '┱' | '┳' | '┵' | '┷'
                                    | '┥' | '┪' | '┽' | '┿' | '╅' | '╈' | '┹' | '┻' | '┩' | '┫'
                                    | '╃' | '╇' | '╉' | '╋' => true,
                                    _ => false,
                                },
                                '═' => match c {
                                    '╝' | '╛' | '╗' | '╕' | '╦' | '╤' | '╧' | '╡' | '╪' | '╩'
                                    | '╣' | '╬' => true,
                                    _ => false,
                                },
                                _ => false,
                            } =>
                        {
                            let start = self.graph.add_node(self.location);
                            let brush = match c {
                                '┐' | '┑' | '╮' | '╕' | '├' | '┝' | '┞' | '┡' | '┬' | '┭' | '┮'
                                | '┯' | '╞' | '╤' | '┤' | '┥' | '╡' | '┼' | '┽' | '┾' | '┿'
                                | '╪' | '┦' | '┩' | '╀' | '╃' | '╄' | '╇' => {
                                    Some('│')
                                }
                                '╗' | '╖' | '╟' | '╠' | '╥' | '╦' | '╢' | '╣' | '╫' | '╬' => {
                                    Some('║')
                                }
                                '┓' | '┒' | '┟' | '┠' | '┢' | '┣' | '┧' | '┪' | '┨' | '┫' | '┰'
                                | '┱' | '┲' | '┳' | '╁' | '╅' | '╆' | '╈' | '╂' | '╉' | '╊'
                                | '╋' => Some('┃'),
                                _ => None,
                            };
                            if let Some(brush) = brush {
                                self.ports.push(Port {
                                    start,
                                    end: self.location,
                                    edge: Edge(None, Brush::NorthSouth(brush), None),
                                });
                            }
                            start
                        }
                        _ => self.graph.add_node(self.previous_location.clone()),
                    };
                    self.graph.add_edge(start, end, edge);
                    match match c {
                        '┼' | '┴' | '┵' | '├' | '┞' | '┟' | '┠' | '┬' | '┭' | '┰' | '┱' | '┽'
                        | '╁' | '╅' | '┸' | '┹' | '╀' | '╂' | '╃' | '╉' | '╨' | '╫' => {
                            Some('─')
                        }
                        '╋' | '┶' | '┷' | '┾' | '┿' | '╆' | '╈' | '┺' | '┻' | '┮' | '┯' | '┲'
                        | '┳' | '┝' | '┡' | '┢' | '┣' | '╄' | '╇' | '╊' => {
                            Some('━')
                        }
                        '╬' | '╧' | '╪' | '╩' | '╤' | '╦' | '╞' | '╠' => Some('═'),
                        _ => None,
                    } {
                        Some(brush) => Tx::Build {
                            start: self.location,
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
            location: Node {
                character: '\u{0000}',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
            },
            previous_location: Node {
                character: '\u{0000}',
                source: LineColumn { line: 0, column: 0 },
                visual: LineColumn { line: 0, column: 0 },
            },
            tx: Tx::Initial,
        }
    }

    fn finish(&mut self) {
        self.next();
    }
}

impl FromStr for Graph {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut state = &mut State::start();

        for c in input.chars() {
            state.location.character = c;
            let is_combining_mark = is_combining_mark(c);
            if !is_combining_mark {
                state.next();
            }
            state.previous_location = state.location;
            if !is_combining_mark {
                state.location.visual.column += 1;
            }

            // TODO drop unreachable ports

            if c == '\n' {
                state.location.visual.line += 1;
                state.location.visual.column = 0;
                state.location.source.line += 1;
                state.location.source.column = 0;
            } else {
                state.location.source.column += 1;
            }
        }
        state.location = Node {
            character: '\n',
            visual: LineColumn {
                line: std::usize::MAX,
                column: 0,
            },
            source: LineColumn {
                line: std::usize::MAX,
                column: 0,
            },
        };

        state.finish();

        Ok(Graph(state.graph.to_owned()))
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines: Vec<Vec<char>> = Vec::new();
        let mut draw = |LineColumn { line, column }, brush| {
            if lines.len() < line {
                lines.resize_with(line, Default::default);
            }
            let line = &mut lines[line - 1];

            if line.len() < column + 1 {
                line.resize(column + 1, ' ');
            }
            line[column] = brush;
        };

        for location in self.0.nodes() {
            draw(location.visual, location.character);
        }

        for (v, u, &Edge(_, brush, _)) in self.0.all_edges() {
            let (v, u) = if v < u { (v, u) } else { (u, v) };
            let (v, u) = (v.visual, u.visual);

            let (line_columns, brush): (Vec<(usize, usize)>, _) = match brush {
                Brush::NorthSouth(brush) => {
                    ((v.line + 1..u.line).zip(repeat(v.column)).collect(), brush)
                }
                Brush::EastWest(brush) => {
                    (repeat(v.line).zip(v.column + 1..u.column).collect(), brush)
                }
                Brush::NorthEastSouthWest(brush) => (
                    (v.line + 1..u.line).zip(u.column + 1..v.column).collect(),
                    brush,
                ),
                Brush::NorthWestSouthEast(brush) => (
                    (v.line + 1..u.line).zip(v.column + 1..u.column).collect(),
                    brush,
                ),
            };
            for (line, column) in line_columns {
                draw(LineColumn { line, column }, brush);
            }
        }

        let lines: String = lines
            .into_iter()
            .flat_map(|mut line| {
                line.push('\n');
                line
            })
            .into_iter()
            .collect();

        f.write_str(lines.as_str().trim_end())
    }
}
