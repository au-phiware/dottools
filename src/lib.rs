#![feature(proc_macro_span)]

extern crate petgraph;
extern crate proc_macro;

#[cfg(test)]
mod tests;

use petgraph::graphmap::UnGraphMap;
use std::cmp::Ordering;
#[cfg(test)]
use std::env;
use std::fmt;
use std::iter::repeat;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use unicode_normalization::char::is_combining_mark;

macro_rules! tprintln {
    ($($_:tt)*) => {
        #[cfg(test)]
        {
            if env::var("DEBUG")
                .as_ref()
                .map(String::as_ref)
                .unwrap_or("0")
                == "1"
            {
                println!("{}", format!($($_)*));
            }
        }
    };
}

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
pub enum Region {
    Center,
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    character: char,
    source: LineColumn,
    visual: LineColumn,
    region: (Region, Region),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Brush {
    NorthSouth(char),
    EastWest(char),
    NorthEastSouthWest(char),
    NorthWestSouthEast(char),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(Option<char>, Brush, Option<char>);

#[derive(Debug)]
pub struct Graph(UnGraphMap<Node, Edge>);

/*

'─' | '━' | '│' | '┃' | '┄' | '┅' | '┆' | '┇' | '┈' | '┉' | '┊' | '┋' | '┌' | '┍' | '┎' | '┏' |
'┐' | '┑' | '┒' | '┓' | '└' | '┕' | '┖' | '┗' | '┘' | '┙' | '┚' | '┛' | '├' | '┝' | '┞' | '┟' |
'┠' | '┡' | '┢' | '┣' | '┤' | '┥' | '┦' | '┧' | '┨' | '┩' | '┪' | '┫' | '┬' | '┭' | '┮' | '┯' |
'┰' | '┱' | '┲' | '┳' | '┴' | '┵' | '┶' | '┷' | '┸' | '┹' | '┺' | '┻' | '┼' | '┽' | '┾' | '┿' |
'╀' | '╁' | '╂' | '╃' | '╄' | '╅' | '╆' | '╇' | '╈' | '╉' | '╊' | '╋' | '╌' | '╍' | '╎' | '╏' |
'═' | '║' | '╒' | '╓' | '╔' | '╕' | '╖' | '╗' | '╘' | '╙' | '╚' | '╛' | '╜' | '╝' | '╞' | '╟' |
'╠' | '╡' | '╢' | '╣' | '╤' | '╥' | '╦' | '╧' | '╨' | '╩' | '╪' | '╫' | '╬' | '╭' | '╮' | '╯' |
'╰' | '╱' | '╲' | '╳' | '╴' | '╵' | '╶' | '╷' | '╸' | '╹' | '╺' | '╻' | '╼' | '╽' | '╾' | '╿' |

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

macro_rules! add_edge {
    ($self:expr, $start:expr, $end:expr, $edge:expr) => {
        add_edge!($self, $start, $end, $edge,)
    };
    ($self:expr, $start:expr, $end:expr, $edge:expr,) => {
        let (start, end, edge) = ($start, $end, $edge);
        tprintln!(
            "add_edge(start: {:?}, end: {:?}, edge: {:?}) [{:?}]",
            start,
            end,
            edge,
            line!()
        );
        $self.graph.add_edge(start, end, edge)
    };
}

macro_rules! add_port {
    ($self:expr, $start:expr, $end:expr, $edge:expr) => {
        add_port!($self, $start, $end, $edge,)
    };
    ($self:expr, $start:expr, $end:expr, $edge:expr,) => {
        add_port!(
            $self,
            Port {
                start: $start,
                end: $end,
                edge: $edge
            },
        )
    };
    ($self:expr, $port:expr) => {
        add_port!($self, $port,)
    };
    ($self:expr, $port:expr,) => {
        let port = $port;
        tprintln!(
            "add_port(start: {:?}, end: {:?}, edge: {:?}) [{:?}]",
            port.start,
            port.end,
            port.edge,
            line!()
        );
        $self.ports.push(port);
    };
}

impl State {
    fn next(&mut self) {
        let c = self.location.character;
        tprintln!("tx: {:?};", self.tx);
        tprintln!("ports: {:?};", self.ports);
        let LineColumn { line, column } = self.location.visual;
        tprintln!(
            "next: {:?}; line: {:?}/{:?}; column: {:?}/{:?};",
            c,
            line,
            self.location.source.line,
            column,
            self.location.source.column
        );
        let ports = self.ports.to_owned();
        let mut pass = false;
        let mut complete_build = Tx::Initial;
        let mut built_north_south = false;
        let mut built_north_west_south_east = false;
        let mut built_north_east_south_west = false;
        self.ports = Vec::with_capacity(ports.len() + 1);
        for port in ports {
            let brush = port.edge.1;
            let location = port.end.visual;
            if location.line + 1 == line {
                if match brush {
                    Brush::NorthSouth(brush) => location.column == column && brush == c,
                    Brush::EastWest(_) => false,
                    Brush::NorthWestSouthEast(brush) => location.column + 1 == column && brush == c,
                    Brush::NorthEastSouthWest(brush) => location.column == column + 1 && brush == c,
                } {
                    tprintln!(
                        "...continuing port, line: {:?}; column: {:?};",
                        location.line,
                        location.column
                    );
                    let mut end = self.location.clone();
                    end.region = match brush {
                        Brush::NorthSouth(_) => (Region::South, Region::Center),
                        Brush::EastWest(_) => (Region::Center, Region::East),
                        Brush::NorthWestSouthEast(_) => (Region::South, Region::East),
                        Brush::NorthEastSouthWest(_) => (Region::South, Region::West),
                    };
                    add_port!(self, port.start, end, port.edge);
                    if !pass {
                        pass = true;
                        complete_build = self.tx;
                        self.tx = Tx::Initial;
                    }
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
                    let mut end = self.location.clone();
                    end.region = (Region::Center, Region::Center);
                    add_edge!(self, port.start, end, port.edge);
                    built_north_west_south_east = true;
                    tprintln!("built_north_west_south_east");
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
                    let mut end = self.location.clone();
                    end.region = (Region::Center, Region::Center);
                    add_edge!(self, port.start, end, port.edge);
                    built_north_east_south_west = true;
                    tprintln!("built_north_east_south_west");
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
                    let mut end = self.location.clone();
                    end.region = (Region::Center, Region::Center);
                    add_edge!(self, port.start, end, port.edge);
                    built_north_south = true;
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
                        let start = end.clone();
                        let mut end = self.location.clone();
                        end.region = (Region::South, Region::Center);
                        add_port!(self, start, end, Edge(None, Brush::NorthSouth(brush), None));
                    }
                } else {
                    add_port!(self, port);
                }
            } else if location.line + 1 >= line {
                add_port!(self, port);
            } else {
                add_edge!(self, port.start, port.end, port.edge);
            }
        }

        if !pass {
            if !built_north_south {
                if let Some(brush) = match c {
                    '└' | '┕' | '┘' | '┙' | '├' | '┝' | '┟' | '┢' | '┣' | '┤' | '┥' | '┧' | '┪'
                    | '┴' | '┵' | '┶' | '┷' | '┼' | '┽' | '┾' | '┿' | '╁' | '╅' | '╆' | '╈'
                    | '╘' | '╛' | '╞' | '╡' | '╧' | '╪' | '╯' | '╰' | '╵' | '╽' => {
                        Some('│')
                    }
                    '┖' | '┗' | '┚' | '┛' | '┞' | '┠' | '┡' | '┦' | '┨' | '┩' | '┫' | '┸' | '┹'
                    | '┺' | '┻' | '╀' | '╂' | '╃' | '╄' | '╇' | '╉' | '╊' | '╋' | '╹' | '╿' => {
                        Some('┃')
                    }
                    '╙' | '╚' | '╜' | '╝' | '╟' | '╠' | '╢' | '╣' | '╨' | '╩' | '╫' | '╬' => {
                        Some('║')
                    }
                    _ => None,
                } {
                    let mut start = self.location.clone();
                    start.region = (Region::North, Region::Center);
                    let mut end = self.location.clone();
                    end.region = (Region::Center, Region::Center);
                    add_edge!(self, start, end, Edge(None, Brush::NorthSouth(brush), None));
                }
            }

            self.tx = match (self.tx, c) {
                (_, '│')
                | (_, '║')
                | (_, '┃')
                | (_, '┊')
                | (_, '┋')
                | (_, '┆')
                | (_, '┇')
                | (_, '╎')
                | (_, '╏')
                | (_, '|')
                | (_, '\\')
                | (_, '╲')
                | (_, '/')
                | (_, '╱') => {
                    complete_build = self.tx;
                    let mut start = self.location.clone();
                    start.region = (Region::North, Region::Center);
                    let mut end = self.location.clone();
                    end.region = (Region::South, Region::Center);
                    let brush = match c {
                        '\\' | '╲' => {
                            start.region.1 = Region::West;
                            end.region.1 = Region::East;
                            Brush::NorthWestSouthEast(c)
                        }
                        '/' | '╱' => {
                            start.region.1 = Region::East;
                            end.region.1 = Region::West;
                            Brush::NorthEastSouthWest(c)
                        }
                        _ => Brush::NorthSouth(c),
                    };
                    add_port!(self, start, end, Edge(None, brush, None));
                    Tx::Initial
                }
                (_, '╳') | (_, 'X') => {
                    complete_build = self.tx;
                    let mut start = self.location.clone();
                    start.region = (Region::North, Region::West);
                    let mut end = self.location.clone();
                    end.region = (Region::Center, Region::Center);
                    let brush = Brush::NorthWestSouthEast(match c {
                        '╳' => '╲',
                        _ => '\\',
                    });
                    if !built_north_west_south_east {
                        add_edge!(self, start, end, Edge(None, brush, None));
                    }
                    start = end.clone();
                    end.region = (Region::South, Region::East);
                    add_port!(self, start, end, Edge(None, brush, None));
                    let brush = Brush::NorthEastSouthWest(match c {
                        '╳' => '╱',
                        _ => '/',
                    });
                    let mut start = self.location.clone();
                    start.region = (Region::North, Region::East);
                    let mut end = self.location.clone();
                    end.region = (Region::Center, Region::Center);
                    if !built_north_east_south_west {
                        add_edge!(self, start, end, Edge(None, brush, None));
                    }
                    start = end.clone();
                    end.region = (Region::South, Region::West);
                    add_port!(self, start, end, Edge(None, brush, None));
                    Tx::Initial
                }
                (_, '┌')
                | (_, '╔')
                | (_, '┏')
                | (_, '╒')
                | (_, '╓')
                | (_, '╭')
                | (_, '┍')
                | (_, '┎')
                | (_, '╷')
                | (_, '╻')
                | (_, '╿')
                | (_, '╽')
                | (_, '├')
                | (_, '┝')
                | (_, '┞')
                | (_, '┟')
                | (_, '┠')
                | (_, '┡')
                | (_, '┢')
                | (_, '┣')
                | (_, '╞')
                | (_, '╟')
                | (_, '╠') => {
                    complete_build = self.tx;
                    let mut start = self.location.clone();
                    start.region = (Region::Center, Region::Center);
                    let mut end = self.location.clone();
                    end.region = (Region::South, Region::Center);
                    let brush = match c {
                        '╔' | '╓' | '╟' | '╠' => '║',
                        '┏' | '┎' | '┟' | '┠' | '┢' | '┣' | '╻' | '╽' => '┃',
                        _ => '│',
                    };
                    add_port!(self, start, end, Edge(None, Brush::NorthSouth(brush), None));
                    if let Some(brush) = match c {
                        '╿' | '╽' | '╷' | '╻' => None,
                        '╔' | '╒' | '╞' | '╠' => Some('═'),
                        '┏' | '┍' | '┝' | '┡' | '┢' | '┣' => Some('━'),
                        _ => Some('─'),
                    } {
                        let edge = Edge(None, Brush::EastWest(brush), None);
                        Tx::Build { start, edge }
                    } else {
                        Tx::Initial
                    }
                }
                (_, '└')
                | (_, '╚')
                | (_, '┗')
                | (_, '╘')
                | (_, '╙')
                | (_, '╰')
                | (_, '┕')
                | (_, '┖')
                | (_, '╶')
                | (_, '╺') => {
                    complete_build = self.tx;
                    let mut start = self.location.clone();
                    start.region = (Region::Center, Region::Center);
                    let brush = match c {
                        '╚' | '╘' => '═',
                        '┗' | '┕' | '╺' => '━',
                        _ => '─',
                    };
                    let edge = Edge(None, Brush::EastWest(brush), None);
                    Tx::Build { start, edge }
                }
                (Tx::Initial, _) => match c {
                    '─' | '═' | '━' | '┈' | '┉' | '┄' | '┅' | '╌' | '╍' | '-' | '=' =>
                    {
                        let mut start = self.location.clone();
                        start.region = (Region::Center, Region::West);
                        let edge = Edge(None, Brush::EastWest(c), None);
                        Tx::Build { start, edge }
                    }
                    _ => {
                        let mut start = self.location.clone();
                        start.region = (Region::Center, Region::West);
                        let mut end = self.location.clone();
                        end.region = (Region::Center, Region::Center);
                        if let Some(brush) = match c {
                            '┐' | '┒' | '┘' | '┚' | '┤' | '┦' | '┧' | '┨' | '┬' | '┮' | '┰'
                            | '┲' | '┴' | '┶' | '┸' | '┺' | '┼' | '┾' | '╀' | '╁' | '╂' | '╄'
                            | '╆' | '╖' | '╜' | '╢' | '╥' | '╨' | '╫' | '╮' | '╯' | '╴' | '╼' => {
                                Some('─')
                            }
                            '┑' | '┓' | '┙' | '┛' | '┥' | '┩' | '┪' | '┫' | '┭' | '┯' | '┱'
                            | '┳' | '┵' | '┷' | '┹' | '┻' | '┽' | '┿' | '╃' | '╅' | '╇' | '╈'
                            | '╉' | '╊' | '╋' | '╸' | '╾' => Some('━'),
                            '╕' | '╗' | '╛' | '╝' | '╡' | '╣' | '╤' | '╦' | '╧' | '╩' | '╪'
                            | '╬' => Some('═'),
                            _ => None,
                        } {
                            let edge = Edge(None, Brush::EastWest(brush), None);
                            add_edge!(self, start, end, edge);
                        }
                        start.region = (Region::Center, Region::Center);
                        if let Some(brush) = match c {
                            '┌' | '┍' | '┐' | '┑' | '├' | '┝' | '┞' | '┡' | '┤' | '┥' | '┦'
                            | '┩' | '┬' | '┭' | '┮' | '┯' | '┼' | '┽' | '┾' | '┿' | '╀' | '╃'
                            | '╄' | '╇' | '╒' | '╕' | '╞' | '╡' | '╤' | '╪' | '╭' | '╮' | '╷'
                            | '╿' => Some('│'),
                            '┎' | '┏' | '┒' | '┓' | '┟' | '┠' | '┢' | '┣' | '┧' | '┨' | '┪'
                            | '┫' | '┰' | '┱' | '┲' | '┳' | '╁' | '╂' | '╅' | '╆' | '╈' | '╉'
                            | '╊' | '╋' | '╻' | '╽' => Some('┃'),
                            '╓' | '╔' | '╖' | '╗' | '╟' | '╠' | '╢' | '╣' | '╥' | '╦' | '╫'
                            | '╬' => Some('║'),
                            _ => None,
                        } {
                            let edge = Edge(None, Brush::NorthSouth(brush), None);
                            end.region = (Region::South, Region::Center);
                            add_port!(self, start, end, edge);
                        }
                        if let Some(brush) = match c {
                            '┌' | '┎' | '└' | '┖' | '├' | '┞' | '┟' | '┠' | '┬' | '┭' | '┰'
                            | '┱' | '┴' | '┵' | '┸' | '┹' | '┼' | '┽' | '╀' | '╁' | '╂' | '╃'
                            | '╅' | '╉' | '╓' | '╙' | '╟' | '╥' | '╨' | '╫' | '╭' | '╰' | '╶'
                            | '╾' => Some('─'),
                            '┍' | '┏' | '┕' | '┗' | '┝' | '┡' | '┢' | '┣' | '┮' | '┯' | '┲'
                            | '┳' | '┶' | '┷' | '┺' | '┻' | '┾' | '┿' | '╄' | '╆' | '╇' | '╈'
                            | '╊' | '╋' | '╺' | '╼' => Some('━'),
                            '╒' | '╔' | '╘' | '╚' | '╞' | '╠' | '╤' | '╦' | '╧' | '╩' | '╪'
                            | '╬' => Some('═'),
                            _ => None,
                        } {
                            let edge = Edge(None, Brush::EastWest(brush), None);
                            Tx::Build { start, edge }
                        } else {
                            Tx::Initial
                        }
                    }
                },
                (Tx::Build { start, edge }, _) => match edge.1 {
                    Brush::EastWest(brush) if c == brush => self.tx,
                    _ => {
                        let end = match edge.1 {
                            Brush::EastWest(brush)
                                if match brush {
                                    '─' | '┈' | '┄' | '╌' => match c {
                                        '┘' | '╜' | '╯' | '┚' | '┐' | '╖' | '╮' | '┒' | '┬'
                                        | '┮' | '┰' | '┲' | '┴' | '┶' | '┤' | '┧' | '┼' | '┾'
                                        | '╁' | '╆' | '┸' | '┺' | '┦' | '┨' | '╀' | '╂' | '╄'
                                        | '╊' | '╨' | '╢' | '╫' => true,
                                        _ => false,
                                    },
                                    '━' | '┉' | '┅' | '╍' => match c {
                                        '┛' | '┙' | '┓' | '┑' | '┭' | '┯' | '┱' | '┳' | '┵'
                                        | '┷' | '┥' | '┪' | '┽' | '┿' | '╅' | '╈' | '┹' | '┻'
                                        | '┩' | '┫' | '╃' | '╇' | '╉' | '╋' => true,
                                        _ => false,
                                    },
                                    '═' => match c {
                                        '╝' | '╛' | '╗' | '╕' | '╦' | '╤' | '╧' | '╡' | '╪'
                                        | '╩' | '╣' | '╬' => true,
                                        _ => false,
                                    },
                                    _ => false,
                                } =>
                            {
                                let mut start = self.location;
                                start.region = (Region::Center, Region::Center);
                                let mut end = self.location;
                                end.region = (Region::South, Region::Center);
                                let brush =
                                    match c {
                                        '┐' | '┑' | '╮' | '╕' | '├' | '┝' | '┞' | '┡' | '┬'
                                        | '┭' | '┮' | '┯' | '╞' | '╤' | '┤' | '┥' | '╡' | '┼'
                                        | '┽' | '┾' | '┿' | '╪' | '┦' | '┩' | '╀' | '╃' | '╄'
                                        | '╇' => Some('│'),
                                        '╗' | '╖' | '╟' | '╠' | '╥' | '╦' | '╢' | '╣' | '╫'
                                        | '╬' => Some('║'),
                                        '┓' | '┒' | '┟' | '┠' | '┢' | '┣' | '┧' | '┪' | '┨'
                                        | '┫' | '┰' | '┱' | '┲' | '┳' | '╁' | '╅' | '╆' | '╈'
                                        | '╂' | '╉' | '╊' | '╋' => Some('┃'),
                                        _ => None,
                                    };
                                if let Some(brush) = brush {
                                    let edge = Edge(None, Brush::NorthSouth(brush), None);
                                    add_port!(self, start, end, edge);
                                }
                                start
                            }
                            _ => {
                                if let Some(brush) = match c {
                                    '┐' | '┒' | '┘' | '┚' | '┤' | '┦' | '┧' | '┨' | '┬' | '┮'
                                    | '┰' | '┲' | '┴' | '┶' | '┸' | '┺' | '┼' | '┾' | '╀' | '╁'
                                    | '╂' | '╄' | '╆' | '╊' | '╖' | '╜' | '╢' | '╥' | '╨' | '╫'
                                    | '╮' | '╯' | '╴' | '╼' => Some('─'),
                                    '┑' | '┓' | '┙' | '┛' | '┥' | '┩' | '┪' | '┫' | '┭' | '┯'
                                    | '┱' | '┳' | '┵' | '┷' | '┹' | '┻' | '┽' | '┿' | '╃' | '╅'
                                    | '╇' | '╈' | '╉' | '╋' | '╸' | '╾' => Some('━'),
                                    '╕' | '╗' | '╛' | '╝' | '╡' | '╣' | '╤' | '╦' | '╧' | '╩'
                                    | '╪' | '╬' => Some('═'),
                                    _ => None,
                                } {
                                    let mut start = self.location.clone();
                                    start.region = (Region::Center, Region::West);
                                    let mut end = self.location.clone();
                                    end.region = (Region::Center, Region::Center);
                                    let edge = Edge(None, Brush::EastWest(brush), None);
                                    add_edge!(self, start, end, edge);
                                }
                                let mut end = self.previous_location.clone();
                                end.region = (Region::Center, Region::East);
                                end
                            }
                        };
                        add_edge!(self, start, end, edge);
                        let mut start = self.location;
                        start.region = (Region::Center, Region::Center);
                        if let Some(brush) = match c {
                            '┌' | '┍' | '┐' | '┑' | '├' | '┝' | '┞' | '┡' | '┤' | '┥' | '┦'
                            | '┩' | '┬' | '┭' | '┮' | '┯' | '┼' | '┽' | '┾' | '┿' | '╀' | '╃'
                            | '╄' | '╇' | '╒' | '╕' | '╞' | '╡' | '╤' | '╪' | '╭' | '╮' | '╷'
                            | '╿' => Some('│'),
                            '┎' | '┏' | '┒' | '┓' | '┟' | '┠' | '┢' | '┣' | '┧' | '┨' | '┪'
                            | '┫' | '┰' | '┱' | '┲' | '┳' | '╁' | '╂' | '╅' | '╆' | '╈' | '╉'
                            | '╊' | '╋' | '╻' | '╽' => Some('┃'),
                            '╓' | '╔' | '╖' | '╗' | '╟' | '╠' | '╢' | '╣' | '╥' | '╦' | '╫'
                            | '╬' => Some('║'),
                            _ => None,
                        } {
                            let edge = Edge(None, Brush::NorthSouth(brush), None);
                            let mut end = self.location;
                            end.region = (Region::South, Region::Center);
                            add_port!(self, start, end, edge);
                        }
                        let mut start = self.location;
                        start.region = (Region::Center, Region::Center);
                        if let Some(brush) = match c {
                            '┌' | '┎' | '└' | '┖' | '├' | '┞' | '┟' | '┠' | '┬' | '┭' | '┰'
                            | '┱' | '┴' | '┵' | '┸' | '┹' | '┼' | '┽' | '╀' | '╁' | '╂' | '╃'
                            | '╅' | '╉' | '╓' | '╙' | '╟' | '╥' | '╨' | '╫' | '╭' | '╰' | '╶'
                            | '╾' => Some('─'),
                            '┍' | '┏' | '┕' | '┗' | '┝' | '┡' | '┢' | '┣' | '┮' | '┯' | '┲'
                            | '┳' | '┶' | '┷' | '┺' | '┻' | '┾' | '┿' | '╄' | '╆' | '╇' | '╈'
                            | '╊' | '╋' | '╺' | '╼' => Some('━'),
                            '╒' | '╔' | '╘' | '╚' | '╞' | '╠' | '╤' | '╦' | '╧' | '╩' | '╪'
                            | '╬' => Some('═'),
                            '-' | '=' | '─' | '┈' | '┄' | '╌' | '━' | '┉' | '┅' | '╍' | '═' =>
                            {
                                start.region = (Region::Center, Region::West);
                                Some(c)
                            }
                            _ => None,
                        } {
                            Tx::Build {
                                start,
                                edge: Edge(None, Brush::EastWest(brush), None),
                            }
                        } else {
                            Tx::Initial
                        }
                    }
                },
            };
        }

        if let Tx::Build { start, edge } = complete_build {
            let mut end = self.previous_location.clone();
            end.region = (Region::Center, Region::East);
            add_edge!(self, start, end, edge);
        }
    }

    fn start() -> Self {
        State {
            graph: UnGraphMap::new(),
            ports: Vec::new(),
            location: Node {
                character: '\u{0000}',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::Center),
            },
            previous_location: Node {
                character: '\u{0000}',
                source: LineColumn { line: 0, column: 0 },
                visual: LineColumn { line: 0, column: 0 },
                region: (Region::Center, Region::Center),
            },
            tx: Tx::Initial,
        }
    }

    fn finish(&mut self) {
        self.next();
        tprintln!("finished");
        tprintln!("tx: {:?}", self.tx);
        tprintln!("ports: {:?}", self.ports);
    }
}

impl FromStr for Graph {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut state = &mut State::start();

        for c in input.chars() {
            let is_combining_mark = is_combining_mark(c);
            if !is_combining_mark {
                state.location.character = c;
                state.next();
                state.previous_location = state.location;
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
            region: (Region::Center, Region::Center),
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
                    (v.line + 1..u.line)
                        .zip((u.column + 1..v.column).rev())
                        .collect(),
                    brush,
                ),
                Brush::NorthWestSouthEast(brush) => (
                    (v.line + 1..u.line).zip(v.column + 1..u.column).collect(),
                    brush,
                ),
            };
            tprintln!("brush: {:?} line_columns: {:?}", brush, line_columns);
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

/// Removes combining marks and preserves box drawing characters (2500–257F) and whitespace,
/// whilst replacing other characters with spaces, unless they appear at the end of a line .
pub fn clean_string(s: &str) -> String {
    let mut t = String::with_capacity(s.len());
    for c in s[..].chars() {
        if '─' <= c && c <= '╿'
            || match c {
                '|' | '-' | '=' | '/' | '\\' | 'X' => true,
                '\n' => true,
                _ => false,
            }
        {
            t.push(c);
        } else if !is_combining_mark(c) {
            t.push(' ');
        }
    }
    t.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<&str>>()
        .join("\n")
}
