use crate::{clean_string, Brush, Edge, Graph, LineColumn, Node, Region};
use pretty_assertions::assert_eq;

parse!(
    short,
    "|\n\n",
    [(
        Node {
            character: '|',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::North, Region::Center),
        },
        Node {
            character: '|',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::South, Region::Center),
        },
        &Edge(None, Brush::NorthSouth('|'), None),
    )]
);

parse!(
    short_single_line,
    "|\n",
    [(
        Node {
            character: '|',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::North, Region::Center),
        },
        Node {
            character: '|',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::South, Region::Center),
        },
        &Edge(None, Brush::NorthSouth('|'), None),
    )]
);

parse!(
    short_no_newline,
    "|",
    [(
        Node {
            character: '|',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::North, Region::Center),
        },
        Node {
            character: '|',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::South, Region::Center),
        },
        &Edge(None, Brush::NorthSouth('|'), None),
    )]
);

parse!(
    single_line,
    "│\n│\n│\n",
    [(
        Node {
            character: '│',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::North, Region::Center),
        },
        Node {
            character: '│',
            source: LineColumn { line: 3, column: 0 },
            visual: LineColumn { line: 3, column: 0 },
            region: (Region::South, Region::Center),
        },
        &Edge(None, Brush::NorthSouth('│'), None),
    )]
);

parse!(
    multi_line,
    "││\n││\n│\n",
    [
        (
            Node {
                character: '│',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::North, Region::Center),
            },
            Node {
                character: '│',
                source: LineColumn { line: 3, column: 0 },
                visual: LineColumn { line: 3, column: 0 },
                region: (Region::South, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '│',
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::North, Region::Center),
            },
            Node {
                character: '│',
                source: LineColumn { line: 2, column: 1 },
                visual: LineColumn { line: 2, column: 1 },
                region: (Region::South, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        )
    ]
);
