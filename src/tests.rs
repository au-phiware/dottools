extern crate base64;

macro_rules! parse {
    ($name:ident, $input:expr, $graph:expr) => {
        #[test]
        fn $name() {
            let input = $input;
            let g = input.parse::<Graph>().unwrap();
            println!("{:?}", g);
            let output = g.to_string();
            let mut v = g.all_edges().collect::<Vec<(Node, Node, &Edge)>>();
            let mut g = $graph;
            v.sort_by_key(|e| (e.0, e.1));
            g.sort_by_key(|e| (e.0, e.1));
            assert_eq!(&v[..], &g[..]);
            assert_eq!(clean_string(input.trim_end()), output);
        }
    };
}

mod parse {
    mod horizontal {
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
    }

    mod vertical {
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
    }

    mod diagonal {
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
    }

    mod node {
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
    }

    mod fuzzer {
        use crate::{clean_string, Brush, Edge, Graph, LineColumn, Node, Region};
        use pretty_assertions::assert_eq;

        parse!(
            crash_a2f152cf0c74db76441c5fed6ab26e4741ab0a71,
            String::from_utf8(base64::decode("PV0KPQ==").unwrap()).unwrap(),
            [
                (
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 0 },
                        visual: LineColumn { line: 1, column: 0 },
                        region: (Region::Center, Region::West),
                    },
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 0 },
                        visual: LineColumn { line: 1, column: 0 },
                        region: (Region::Center, Region::East),
                    },
                    &Edge(None, Brush::EastWest('='), None),
                ),
                (
                    Node {
                        character: '=',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::Center, Region::West),
                    },
                    Node {
                        character: '=',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::Center, Region::East),
                    },
                    &Edge(None, Brush::EastWest('='), None),
                )
            ]
        );

        parse!(
            crash_05ffccc067f6649815b9dc79753fcbea14c58ea4,
            String::from_utf8(base64::decode("Cz0/Cg==").unwrap()).unwrap(),
            [(
                Node {
                    character: '=',
                    source: LineColumn { line: 1, column: 1 },
                    visual: LineColumn { line: 1, column: 1 },
                    region: (Region::Center, Region::West),
                },
                Node {
                    character: '=',
                    source: LineColumn { line: 1, column: 1 },
                    visual: LineColumn { line: 1, column: 1 },
                    region: (Region::Center, Region::East),
                },
                &Edge(None, Brush::EastWest('='), None),
            )]
        );

        parse!(
            crash_85ec8783b4e6da3ccec8e7b3767bdf1ce176dc16,
            String::from_utf8(base64::decode("QS09").unwrap()).unwrap(),
            [
                (
                    Node {
                        character: '-',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 },
                        region: (Region::Center, Region::West),
                    },
                    Node {
                        character: '-',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 },
                        region: (Region::Center, Region::East),
                    },
                    &Edge(None, Brush::EastWest('-'), None),
                ),
                (
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 2 },
                        visual: LineColumn { line: 1, column: 2 },
                        region: (Region::Center, Region::West),
                    },
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 2 },
                        visual: LineColumn { line: 1, column: 2 },
                        region: (Region::Center, Region::East),
                    },
                    &Edge(None, Brush::EastWest('='), None),
                )
            ]
        );

        parse!(
            crash_67be3e563c0d4a75311b4a44ee096d7379796160,
            String::from_utf8(base64::decode("PS8ALw==").unwrap()).unwrap(),
            [
                (
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 0 },
                        visual: LineColumn { line: 1, column: 0 },
                        region: (Region::Center, Region::West),
                    },
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 0 },
                        visual: LineColumn { line: 1, column: 0 },
                        region: (Region::Center, Region::East),
                    },
                    &Edge(None, Brush::EastWest('='), None),
                ),
                (
                    Node {
                        character: '/',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 },
                        region: (Region::North, Region::East),
                    },
                    Node {
                        character: '/',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 },
                        region: (Region::South, Region::West),
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None),
                ),
                (
                    Node {
                        character: '/',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 },
                        region: (Region::North, Region::East),
                    },
                    Node {
                        character: '/',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 },
                        region: (Region::South, Region::West),
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None),
                )
            ]
        );

        parse!(
            crash_a0b68e002a1464a0fff1b2de7a18f1e345475483,
            String::from_utf8(base64::decode("ClxcXA==").unwrap()).unwrap(),
            [
                (
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::North, Region::West),
                    },
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::South, Region::East),
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None),
                ),
                (
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::North, Region::West),
                    },
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::South, Region::East),
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None),
                ),
                (
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 2 },
                        visual: LineColumn { line: 2, column: 2 },
                        region: (Region::North, Region::West),
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
            crash_89dee50b108f2e3632520f91ca3c87d8344dbcee,
            String::from_utf8(base64::decode("CgBYAA==").unwrap()).unwrap(),
            [
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::North, Region::West),
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::Center, Region::Center),
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None),
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::North, Region::East),
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::Center, Region::Center),
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None),
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::Center, Region::Center),
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::South, Region::East),
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None),
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::Center, Region::Center),
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::South, Region::West),
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None),
                ),
            ]
        );

        parse!(
            crash_d9b49648d44b6d2f434fb508baab728cab5528db,
            String::from_utf8(base64::decode("LgIJWA==").unwrap()).unwrap(),
            [
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 },
                        region: (Region::North, Region::West),
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 },
                        region: (Region::Center, Region::Center),
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None),
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 },
                        region: (Region::North, Region::East),
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 },
                        region: (Region::Center, Region::Center),
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None),
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 },
                        region: (Region::Center, Region::Center),
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 },
                        region: (Region::South, Region::East),
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None),
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 },
                        region: (Region::Center, Region::Center),
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 },
                        region: (Region::South, Region::West),
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None),
                ),
            ]
        );

        parse!(
            crash_8b3a3c0e7591d31a8bf0a30affe7d612d0cb2c4e,
            // *X
            // X\\X
            "*X\nX\\\\X",
            [
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 },
                        region: (Region::North, Region::West)
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 },
                        region: (Region::Center, Region::Center)
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None)
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 },
                        region: (Region::North, Region::East)
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 },
                        region: (Region::Center, Region::Center)
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None)
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 },
                        region: (Region::Center, Region::Center)
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::Center, Region::Center)
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None)
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 },
                        region: (Region::Center, Region::Center)
                    },
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 2 },
                        visual: LineColumn { line: 2, column: 2 },
                        region: (Region::South, Region::East)
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None)
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::North, Region::West)
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::Center, Region::Center)
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None)
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::Center, Region::Center)
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::South, Region::East)
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None)
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::Center, Region::Center)
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 },
                        region: (Region::South, Region::West)
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None)
                ),
                (
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::North, Region::West)
                    },
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 },
                        region: (Region::South, Region::East)
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None)
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 3 },
                        visual: LineColumn { line: 2, column: 3 },
                        region: (Region::North, Region::West)
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 3 },
                        visual: LineColumn { line: 2, column: 3 },
                        region: (Region::Center, Region::Center)
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None)
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 3 },
                        visual: LineColumn { line: 2, column: 3 },
                        region: (Region::North, Region::East)
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 3 },
                        visual: LineColumn { line: 2, column: 3 },
                        region: (Region::Center, Region::Center)
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None)
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 3 },
                        visual: LineColumn { line: 2, column: 3 },
                        region: (Region::Center, Region::Center)
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 3 },
                        visual: LineColumn { line: 2, column: 3 },
                        region: (Region::South, Region::East)
                    },
                    &Edge(None, Brush::NorthWestSouthEast('\\'), None)
                ),
                (
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 3 },
                        visual: LineColumn { line: 2, column: 3 },
                        region: (Region::Center, Region::Center)
                    },
                    Node {
                        character: 'X',
                        source: LineColumn { line: 2, column: 3 },
                        visual: LineColumn { line: 2, column: 3 },
                        region: (Region::South, Region::West)
                    },
                    &Edge(None, Brush::NorthEastSouthWest('/'), None)
                )
            ]
        );
    }
}
