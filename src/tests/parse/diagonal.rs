use crate::{clean_string, Brush, Edge, Graph, LineColumn, Node, Region};
use pretty_assertions::assert_eq;

parse!(
    backslash,
    "╲\n ╲\n  ╲",
    [(
        Node {
            character: '╲',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::North, Region::West),
        },
        Node {
            character: '╲',
            source: LineColumn { line: 3, column: 2 },
            visual: LineColumn { line: 3, column: 2 },
            region: (Region::South, Region::East),
        },
        &Edge(None, Brush::NorthWestSouthEast('╲'), None),
    )]
);
parse!(
    forwardslash,
    "  /\n /\n/",
    [(
        Node {
            character: '/',
            source: LineColumn { line: 1, column: 2 },
            visual: LineColumn { line: 1, column: 2 },
            region: (Region::North, Region::East),
        },
        Node {
            character: '/',
            source: LineColumn { line: 3, column: 0 },
            visual: LineColumn { line: 3, column: 0 },
            region: (Region::South, Region::West),
        },
        &Edge(None, Brush::NorthEastSouthWest('/'), None),
    )]
);
