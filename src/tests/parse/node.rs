use crate::{clean_string, Brush, Edge, Graph, LineColumn, Node, Region};
use pretty_assertions::assert_eq;

parse!(
    simple_cross,
    "
             │
            ─┼─
             │
            ",
    [
        (
            Node {
                character: '│',
                source: LineColumn {
                    line: 2,
                    column: 13
                },
                visual: LineColumn {
                    line: 2,
                    column: 13
                },
                region: (Region::North, Region::Center),
            },
            Node {
                character: '┼',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '─',
                source: LineColumn {
                    line: 3,
                    column: 12
                },
                visual: LineColumn {
                    line: 3,
                    column: 12
                },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '┼',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '┼',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '─',
                source: LineColumn {
                    line: 3,
                    column: 14
                },
                visual: LineColumn {
                    line: 3,
                    column: 14
                },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '┼',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '│',
                source: LineColumn {
                    line: 4,
                    column: 13
                },
                visual: LineColumn {
                    line: 4,
                    column: 13
                },
                region: (Region::South, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        )
    ]
);

parse!(
    diagonal_cross_bare,
    "╳",
    [
        (
            Node {
                character: '╳',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::North, Region::West),
            },
            Node {
                character: '╳',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthWestSouthEast('╲'), None),
        ),
        (
            Node {
                character: '╳',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::North, Region::East),
            },
            Node {
                character: '╳',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthEastSouthWest('╱'), None),
        ),
        (
            Node {
                character: '╳',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╳',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::South, Region::East),
            },
            &Edge(None, Brush::NorthWestSouthEast('╲'), None),
        ),
        (
            Node {
                character: '╳',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╳',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::South, Region::West),
            },
            &Edge(None, Brush::NorthEastSouthWest('╱'), None),
        ),
    ]
);

parse!(
    diagonal_cross,
    "
            ╲ ╱
             ╳
            ╱ ╲
            ",
    [
        (
            Node {
                character: '╲',
                source: LineColumn {
                    line: 2,
                    column: 12
                },
                visual: LineColumn {
                    line: 2,
                    column: 12
                },
                region: (Region::North, Region::West),
            },
            Node {
                character: '╳',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthWestSouthEast('╲'), None),
        ),
        (
            Node {
                character: '╱',
                source: LineColumn {
                    line: 2,
                    column: 14
                },
                visual: LineColumn {
                    line: 2,
                    column: 14
                },
                region: (Region::North, Region::East),
            },
            Node {
                character: '╳',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthEastSouthWest('╱'), None),
        ),
        (
            Node {
                character: '╳',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╱',
                source: LineColumn {
                    line: 4,
                    column: 12
                },
                visual: LineColumn {
                    line: 4,
                    column: 12
                },
                region: (Region::South, Region::West),
            },
            &Edge(None, Brush::NorthEastSouthWest('╱'), None),
        ),
        (
            Node {
                character: '╳',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╲',
                source: LineColumn {
                    line: 4,
                    column: 14
                },
                visual: LineColumn {
                    line: 4,
                    column: 14
                },
                region: (Region::South, Region::East),
            },
            &Edge(None, Brush::NorthWestSouthEast('╲'), None),
        )
    ]
);

parse!(
    diagonal_cross_truncated,
    " X\n/ \\",
    [
        (
            Node {
                character: 'X',
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::North, Region::West),
            },
            Node {
                character: 'X',
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthWestSouthEast('\\'), None),
        ),
        (
            Node {
                character: 'X',
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::North, Region::East),
            },
            Node {
                character: 'X',
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthEastSouthWest('/'), None),
        ),
        (
            Node {
                character: 'X',
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '/',
                source: LineColumn { line: 2, column: 0 },
                visual: LineColumn { line: 2, column: 0 },
                region: (Region::South, Region::West),
            },
            &Edge(None, Brush::NorthEastSouthWest('/'), None),
        ),
        (
            Node {
                character: 'X',
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '\\',
                source: LineColumn { line: 2, column: 2 },
                visual: LineColumn { line: 2, column: 2 },
                region: (Region::South, Region::East),
            },
            &Edge(None, Brush::NorthWestSouthEast('\\'), None),
        )
    ]
);

parse!(
    corners,
    "
            ┌─┐
            │ │
            └─┘
            ",
    [
        (
            Node {
                character: '┌',
                source: LineColumn {
                    line: 2,
                    column: 12
                },
                visual: LineColumn {
                    line: 2,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┐',
                source: LineColumn {
                    line: 2,
                    column: 14
                },
                visual: LineColumn {
                    line: 2,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '┌',
                source: LineColumn {
                    line: 2,
                    column: 12
                },
                visual: LineColumn {
                    line: 2,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '└',
                source: LineColumn {
                    line: 4,
                    column: 12
                },
                visual: LineColumn {
                    line: 4,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '┐',
                source: LineColumn {
                    line: 2,
                    column: 14
                },
                visual: LineColumn {
                    line: 2,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┘',
                source: LineColumn {
                    line: 4,
                    column: 14
                },
                visual: LineColumn {
                    line: 4,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '└',
                source: LineColumn {
                    line: 4,
                    column: 12
                },
                visual: LineColumn {
                    line: 4,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┘',
                source: LineColumn {
                    line: 4,
                    column: 14
                },
                visual: LineColumn {
                    line: 4,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        )
    ]
);

parse!(
    mixed_brush_corners,
    "
            ┍━┑
            │ │
            ╘═╛
            ",
    [
        (
            Node {
                character: '┍',
                source: LineColumn {
                    line: 2,
                    column: 12
                },
                visual: LineColumn {
                    line: 2,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┑',
                source: LineColumn {
                    line: 2,
                    column: 14
                },
                visual: LineColumn {
                    line: 2,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('━'), None),
        ),
        (
            Node {
                character: '┍',
                source: LineColumn {
                    line: 2,
                    column: 12
                },
                visual: LineColumn {
                    line: 2,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╘',
                source: LineColumn {
                    line: 4,
                    column: 12
                },
                visual: LineColumn {
                    line: 4,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '┑',
                source: LineColumn {
                    line: 2,
                    column: 14
                },
                visual: LineColumn {
                    line: 2,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╛',
                source: LineColumn {
                    line: 4,
                    column: 14
                },
                visual: LineColumn {
                    line: 4,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '╘',
                source: LineColumn {
                    line: 4,
                    column: 12
                },
                visual: LineColumn {
                    line: 4,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╛',
                source: LineColumn {
                    line: 4,
                    column: 14
                },
                visual: LineColumn {
                    line: 4,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('═'), None),
        )
    ]
);

parse!(
    rounded_corners,
    "
            ╭─╮
            │ │
            ╰─╯
            ",
    [
        (
            Node {
                character: '╭',
                source: LineColumn {
                    line: 2,
                    column: 12
                },
                visual: LineColumn {
                    line: 2,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╮',
                source: LineColumn {
                    line: 2,
                    column: 14
                },
                visual: LineColumn {
                    line: 2,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '╭',
                source: LineColumn {
                    line: 2,
                    column: 12
                },
                visual: LineColumn {
                    line: 2,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╰',
                source: LineColumn {
                    line: 4,
                    column: 12
                },
                visual: LineColumn {
                    line: 4,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '╮',
                source: LineColumn {
                    line: 2,
                    column: 14
                },
                visual: LineColumn {
                    line: 2,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╯',
                source: LineColumn {
                    line: 4,
                    column: 14
                },
                visual: LineColumn {
                    line: 4,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '╰',
                source: LineColumn {
                    line: 4,
                    column: 12
                },
                visual: LineColumn {
                    line: 4,
                    column: 12
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╯',
                source: LineColumn {
                    line: 4,
                    column: 14
                },
                visual: LineColumn {
                    line: 4,
                    column: 14
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        )
    ]
);

parse!(
    t_crossing,
    "
             │
             ├─┬─
             │ │
            ─┴─┤
               │
            ",
    [
        (
            Node {
                character: '│',
                source: LineColumn {
                    line: 2,
                    column: 13
                },
                visual: LineColumn {
                    line: 2,
                    column: 13
                },
                region: (Region::North, Region::Center),
            },
            Node {
                character: '├',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '├',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┬',
                source: LineColumn {
                    line: 3,
                    column: 15
                },
                visual: LineColumn {
                    line: 3,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '├',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┴',
                source: LineColumn {
                    line: 5,
                    column: 13
                },
                visual: LineColumn {
                    line: 5,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '┬',
                source: LineColumn {
                    line: 3,
                    column: 15
                },
                visual: LineColumn {
                    line: 3,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '─',
                source: LineColumn {
                    line: 3,
                    column: 16
                },
                visual: LineColumn {
                    line: 3,
                    column: 16
                },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '┬',
                source: LineColumn {
                    line: 3,
                    column: 15
                },
                visual: LineColumn {
                    line: 3,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┤',
                source: LineColumn {
                    line: 5,
                    column: 15
                },
                visual: LineColumn {
                    line: 5,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '─',
                source: LineColumn {
                    line: 5,
                    column: 12
                },
                visual: LineColumn {
                    line: 5,
                    column: 12
                },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '┴',
                source: LineColumn {
                    line: 5,
                    column: 13
                },
                visual: LineColumn {
                    line: 5,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '┴',
                source: LineColumn {
                    line: 5,
                    column: 13
                },
                visual: LineColumn {
                    line: 5,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┤',
                source: LineColumn {
                    line: 5,
                    column: 15
                },
                visual: LineColumn {
                    line: 5,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '┤',
                source: LineColumn {
                    line: 5,
                    column: 15
                },
                visual: LineColumn {
                    line: 5,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '│',
                source: LineColumn {
                    line: 6,
                    column: 15
                },
                visual: LineColumn {
                    line: 6,
                    column: 15
                },
                region: (Region::South, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        )
    ]
);

parse!(
    mixed_brush_t_crossing,
    "
             │
             ┢━┱─
             ┃ ┃
            ─┺━┩
               │
            ",
    [
        (
            Node {
                character: '│',
                source: LineColumn {
                    line: 2,
                    column: 13
                },
                visual: LineColumn {
                    line: 2,
                    column: 13
                },
                region: (Region::North, Region::Center),
            },
            Node {
                character: '┢',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        ),
        (
            Node {
                character: '┢',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┱',
                source: LineColumn {
                    line: 3,
                    column: 15
                },
                visual: LineColumn {
                    line: 3,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('━'), None),
        ),
        (
            Node {
                character: '┢',
                source: LineColumn {
                    line: 3,
                    column: 13
                },
                visual: LineColumn {
                    line: 3,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┺',
                source: LineColumn {
                    line: 5,
                    column: 13
                },
                visual: LineColumn {
                    line: 5,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('┃'), None),
        ),
        (
            Node {
                character: '┱',
                source: LineColumn {
                    line: 3,
                    column: 15
                },
                visual: LineColumn {
                    line: 3,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '─',
                source: LineColumn {
                    line: 3,
                    column: 16
                },
                visual: LineColumn {
                    line: 3,
                    column: 16
                },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '┱',
                source: LineColumn {
                    line: 3,
                    column: 15
                },
                visual: LineColumn {
                    line: 3,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┩',
                source: LineColumn {
                    line: 5,
                    column: 15
                },
                visual: LineColumn {
                    line: 5,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('┃'), None),
        ),
        (
            Node {
                character: '─',
                source: LineColumn {
                    line: 5,
                    column: 12
                },
                visual: LineColumn {
                    line: 5,
                    column: 12
                },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '┺',
                source: LineColumn {
                    line: 5,
                    column: 13
                },
                visual: LineColumn {
                    line: 5,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '┺',
                source: LineColumn {
                    line: 5,
                    column: 13
                },
                visual: LineColumn {
                    line: 5,
                    column: 13
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '┩',
                source: LineColumn {
                    line: 5,
                    column: 15
                },
                visual: LineColumn {
                    line: 5,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('━'), None),
        ),
        (
            Node {
                character: '┩',
                source: LineColumn {
                    line: 5,
                    column: 15
                },
                visual: LineColumn {
                    line: 5,
                    column: 15
                },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '│',
                source: LineColumn {
                    line: 6,
                    column: 15
                },
                visual: LineColumn {
                    line: 6,
                    column: 15
                },
                region: (Region::South, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        )
    ]
);
