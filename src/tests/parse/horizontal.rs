use crate::{clean_string, Brush, Edge, Graph, LineColumn, Node, Region};
use pretty_assertions::assert_eq;

parse!(
    short,
    "-",
    [(
        Node {
            character: '-',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::Center, Region::West),
        },
        Node {
            character: '-',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::Center, Region::East),
        },
        &Edge(None, Brush::EastWest('-'), None),
    )]
);

parse!(
    single_line,
    "─────",
    [(
        Node {
            character: '─',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::Center, Region::West),
        },
        Node {
            character: '─',
            source: LineColumn { line: 1, column: 4 },
            visual: LineColumn { line: 1, column: 4 },
            region: (Region::Center, Region::East),
        },
        &Edge(None, Brush::EastWest('─'), None),
    )]
);

parse!(
    multi_edge,
    "──  ───",
    [
        (
            Node {
                character: '─',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '─',
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '─',
                source: LineColumn { line: 1, column: 4 },
                visual: LineColumn { line: 1, column: 4 },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '─',
                source: LineColumn { line: 1, column: 6 },
                visual: LineColumn { line: 1, column: 6 },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('─'), None),
        )
    ]
);

parse!(
    multi_line,
    "──\n───",
    [
        (
            Node {
                character: '─',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '─',
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '─',
                source: LineColumn { line: 2, column: 0 },
                visual: LineColumn { line: 2, column: 0 },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '─',
                source: LineColumn { line: 2, column: 2 },
                visual: LineColumn { line: 2, column: 2 },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('─'), None),
        )
    ]
);
