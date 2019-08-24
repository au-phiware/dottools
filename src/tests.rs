macro_rules! parse {
    ($name:ident, $input:expr, $graph:pat) => {
        #[test]
        fn $name() {
            let g = $input.parse::<Graph>().unwrap();
            println!("{:?}", g);
            let mut g = g.all_edges().collect::<Vec<(Location, Location, &Edge)>>();
            g.sort_by_key(|e| (e.0, e.1));
            println!("{:?}", g);
            assert!(match &g[..] {
                $graph => true,
                _ => false,
            });
        }
    };
}

mod parse {
    mod horizontal {
        use crate::{Brush, Edge, Graph, LineColumn, Location};

        parse!(
            short,
            "-",
            [(
                Location {
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Location {
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
                Location {
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Location {
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
                    Location {
                        source: LineColumn { line: 1, column: 0 },
                        visual: LineColumn { line: 1, column: 0 }
                    },
                    Location {
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 }
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    Location {
                        source: LineColumn { line: 1, column: 4 },
                        visual: LineColumn { line: 1, column: 4 }
                    },
                    Location {
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
                    Location {
                        source: LineColumn { line: 1, column: 0 },
                        visual: LineColumn { line: 1, column: 0 }
                    },
                    Location {
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 }
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    Location {
                        source: LineColumn { line: 2, column: 0 },
                        visual: LineColumn { line: 2, column: 0 }
                    },
                    Location {
                        source: LineColumn { line: 2, column: 2 },
                        visual: LineColumn { line: 2, column: 2 }
                    },
                    Edge(None, Brush::EastWest('─'), None),
                )
            ]
        );
    }

    mod vertical {
        use crate::{Brush, Edge, Graph, LineColumn, Location};

        parse!(
            short,
            "|\n\n",
            [(
                Location {
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Location {
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
                Location {
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Location {
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
                Location {
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Location {
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
                Location {
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Location {
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
                    Location {
                        source: LineColumn { line: 1, column: 0 },
                        visual: LineColumn { line: 1, column: 0 }
                    },
                    Location {
                        source: LineColumn { line: 3, column: 0 },
                        visual: LineColumn { line: 3, column: 0 }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    Location {
                        source: LineColumn { line: 1, column: 1 },
                        visual: LineColumn { line: 1, column: 1 }
                    },
                    Location {
                        source: LineColumn { line: 2, column: 1 },
                        visual: LineColumn { line: 2, column: 1 }
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                )
            ]
        );
    }

    mod diagonal {
        use crate::{Brush, Edge, Graph, LineColumn, Location};

        parse!(
            backslash,
            "╲\n ╲\n  ╲",
            [(
                Location {
                    source: LineColumn { line: 1, column: 0 },
                    visual: LineColumn { line: 1, column: 0 }
                },
                Location {
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
                Location {
                    source: LineColumn { line: 1, column: 2 },
                    visual: LineColumn { line: 1, column: 2 }
                },
                Location {
                    source: LineColumn { line: 3, column: 0 },
                    visual: LineColumn { line: 3, column: 0 }
                },
                Edge(None, Brush::NorthEastSouthWest('/'), None),
            )]
        );
    }

    mod node {
        use crate::{Brush, Edge, Graph, LineColumn, Location};

        parse!(
            simple_cross,
            "
             │
            ─┼─
             │
            ",
            [
                (
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 14
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 14
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 14
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 14
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 4,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 4,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 14
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 14
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 4,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 4,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 14
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 14
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 4,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 4,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 15
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 15
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 15
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 15
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 5,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 5,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 5,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 5,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 5,
                            column: 15
                        },
                        visual: LineColumn {
                            line: 5,
                            column: 15
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 2,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 2,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 15
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 15
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 3,
                            column: 15
                        },
                        visual: LineColumn {
                            line: 3,
                            column: 15
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 5,
                            column: 12
                        },
                        visual: LineColumn {
                            line: 5,
                            column: 12
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 5,
                            column: 13
                        },
                        visual: LineColumn {
                            line: 5,
                            column: 13
                        }
                    },
                    Location {
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
                    Location {
                        source: LineColumn {
                            line: 5,
                            column: 15
                        },
                        visual: LineColumn {
                            line: 5,
                            column: 15
                        }
                    },
                    Location {
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
}
