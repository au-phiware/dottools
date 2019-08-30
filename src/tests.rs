extern crate base64;

macro_rules! parse {
    ($name:ident, $input:expr, $graph:pat) => {
        #[test]
        fn $name() {
            let input = $input;
            let g = input.parse::<Graph>().unwrap();
            println!("{:?}", g);
            let mut v = g.all_edges().collect::<Vec<(Node, Node, &Edge)>>();
            v.sort_by_key(|e| (e.0, e.1));
            println!("{:?}", v);
            assert!(match &v[..] {
                $graph => true,
                _ => false,
            });
            assert_eq!(clean_string(input.trim_end()), g.to_string());
        }
    };
}

mod parse {
    mod horizontal {
        use crate::{clean_string, Brush, Edge, Graph, LineColumn, Node};

        parse!(
            short,
            "-",
            [(
                Node {
                    character: '-',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Node {
                    character: '-',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Edge(None, Brush::EastWest('-'), None),
            )]
        );

        parse!(
            single_line,
            "─────",
            [(
                Node {
                    character: '─',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Node {
                    character: '─',
                    source: LineColumn { line: 1, column: 4 },
                    visual: LineColumn { line: 1, column: 4 }
                },
                Edge(None, Brush::EastWest('─'), None),
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
                        visual: LineColumn { line: 1, column: 0 }
                    },
                    Node {
                        character: '─',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 }
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    Node {
                        character: '─',
                        source: LineColumn { line: 1, column: 4 },
                        visual: LineColumn { line: 1, column: 4 }
                    },
                    Node {
                        character: '─',
                        source: LineColumn { line: 1, column: 6 },
                        visual: LineColumn { line: 1, column: 6 }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        visual: LineColumn { line: 1, column: 0 }
                    },
                    Node {
                        character: '─',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 }
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    Node {
                        character: '─',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 }
                    },
                    Node {
                        character: '─',
                        source: LineColumn { line: 2, column: 2 },
                        visual: LineColumn { line: 2, column: 2 }
                    },
                    Edge(None, Brush::EastWest('─'), None),
                )
            ]
        );
    }

    mod vertical {
        use crate::{clean_string, Brush, Edge, Graph, LineColumn, Node};

        parse!(
            short,
            "|\n\n",
            [(
                Node {
                    character: '|',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Node {
                    character: '|',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Edge(None, Brush::NorthSouth('|'), None),
            )]
        );

        parse!(
            short_single_line,
            "|\n",
            [(
                Node {
                    character: '|',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Node {
                    character: '|',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Edge(None, Brush::NorthSouth('|'), None),
            )]
        );

        parse!(
            short_no_newline,
            "|",
            [(
                Node {
                    character: '|',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Node {
                    character: '|',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Edge(None, Brush::NorthSouth('|'), None),
            )]
        );

        parse!(
            single_line,
            "│\n│\n│\n",
            [(
                Node {
                    character: '│',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Node {
                    character: '│',
                    source: LineColumn { line: 3, column: 0 },
                    visual: LineColumn { line: 3, column: 0 }
                },
                Edge(None, Brush::NorthSouth('│'), None),
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
                        visual: LineColumn { line: 1, column: 0 }
                    },
                    Node {
                        character: '│',
                        source: LineColumn { line: 3, column: 0 },
                        visual: LineColumn { line: 3, column: 0 }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    Node {
                        character: '│',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 }
                    },
                    Node {
                        character: '│',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                )
            ]
        );
    }

    mod diagonal {
        use crate::{clean_string, Brush, Edge, Graph, LineColumn, Node};

        parse!(
            backslash,
            "╲\n ╲\n  ╲",
            [(
                Node {
                    character: '╲',
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Node {
                    character: '╲',
                    source: LineColumn { line: 3, column: 2 },
                    visual: LineColumn { line: 3, column: 2 }
                },
                Edge(None, Brush::NorthWestSouthEast('╲'), None),
            )]
        );
        parse!(
            forwardslash,
            "  /\n /\n/",
            [(
                Node {
                    character: '/',
                    source: LineColumn { line: 1, column: 2 },
                    visual: LineColumn { line: 1, column: 2 }
                },
                Node {
                    character: '/',
                    source: LineColumn { line: 3, column: 0 },
                    visual: LineColumn { line: 3, column: 0 }
                },
                Edge(None, Brush::NorthEastSouthWest('/'), None),
            )]
        );
    }

    mod node {
        use crate::{clean_string, Brush, Edge, Graph, LineColumn, Node};

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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                )
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthWestSouthEast('╲'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthEastSouthWest('╱'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthEastSouthWest('╱'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthWestSouthEast('╲'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('━'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('═'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('━'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('┃'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('┃'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('─'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::EastWest('━'), None),
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
                        }
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
                        }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                )
            ]
        );
    }

    mod fuzzer {
        use crate::{clean_string, Brush, Edge, Graph, LineColumn, Node};

        parse!(
            crash_a2f152cf0c74db76441c5fed6ab26e4741ab0a71,
            String::from_utf8(base64::decode("PV0KPQ==").unwrap()).unwrap(),
            [
                (
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 0 },
                        visual: LineColumn { line: 1, column: 0 }
                    },
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 0 },
                        visual: LineColumn { line: 1, column: 0 }
                    },
                    Edge(None, Brush::EastWest('='), None),
                ),
                (
                    Node {
                        character: '=',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 }
                    },
                    Node {
                        character: '=',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 }
                    },
                    Edge(None, Brush::EastWest('='), None),
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
                    visual: LineColumn { line: 1, column: 1 }
                },
                Node {
                    character: '=',
                    source: LineColumn { line: 1, column: 1 },
                    visual: LineColumn { line: 1, column: 1 }
                },
                Edge(None, Brush::EastWest('='), None),
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
                        visual: LineColumn { line: 1, column: 1 }
                    },
                    Node {
                        character: '-',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 }
                    },
                    Edge(None, Brush::EastWest('-'), None),
                ),
                (
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 2 },
                        visual: LineColumn { line: 1, column: 2 }
                    },
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 2 },
                        visual: LineColumn { line: 1, column: 2 }
                    },
                    Edge(None, Brush::EastWest('='), None),
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
                        visual: LineColumn { line: 1, column: 0 }
                    },
                    Node {
                        character: '=',
                        source: LineColumn { line: 1, column: 0 },
                        visual: LineColumn { line: 1, column: 0 }
                    },
                    Edge(None, Brush::EastWest('='), None),
                ),
                (
                    Node {
                        character: '/',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 }
                    },
                    Node {
                        character: '/',
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 }
                    },
                    Edge(None, Brush::NorthEastSouthWest('/'), None),
                ),
                (
                    Node {
                        character: '/',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 }
                    },
                    Node {
                        character: '/',
                        source: LineColumn { line: 1, column: 3 },
                        visual: LineColumn { line: 1, column: 3 }
                    },
                    Edge(None, Brush::NorthEastSouthWest('/'), None),
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
                        visual: LineColumn { line: 2, column: 0 }
                    },
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 }
                    },
                    Edge(None, Brush::NorthWestSouthEast('\\'), None),
                ),
                (
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 }
                    },
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 }
                    },
                    Edge(None, Brush::NorthWestSouthEast('\\'), None),
                ),
                (
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 2 },
                        visual: LineColumn { line: 2, column: 2 }
                    },
                    Node {
                        character: '\\',
                        source: LineColumn { line: 2, column: 2 },
                        visual: LineColumn { line: 2, column: 2 }
                    },
                    Edge(None, Brush::NorthWestSouthEast('\\'), None),
                )
            ]
        );
    }
}
