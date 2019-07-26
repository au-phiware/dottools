macro_rules! parse {
    ($name:ident, $input:expr, $graph:pat) => {
        #[test]
        fn $name() {
            let g = $input.parse::<Graph>().unwrap();
            println!("{:?}", g);
            let mut g = g
                .all_edges()
                .collect::<Vec<(LineColumn, LineColumn, &Edge)>>();
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
        use crate::{Brush, Edge, Graph, LineColumn};

        parse!(
            short,
            "-",
            [(
                LineColumn { line: 1, column: 0 },
                LineColumn { line: 1, column: 0 },
                Edge(None, Brush::EastWest('-'), None),
            )]
        );

        parse!(
            single_line,
            "─────",
            [(
                LineColumn { line: 1, column: 0 },
                LineColumn { line: 1, column: 4 },
                Edge(None, Brush::EastWest('─'), None),
            )]
        );

        parse!(
            multi_line,
            "──\n───",
            [
                (
                    LineColumn { line: 1, column: 0 },
                    LineColumn { line: 1, column: 1 },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn { line: 2, column: 0 },
                    LineColumn { line: 2, column: 2 },
                    Edge(None, Brush::EastWest('─'), None),
                )
            ]
        );
    }

    mod vertical {
        use crate::{Brush, Edge, Graph, LineColumn};

        parse!(
            short,
            "|\n\n",
            [(
                LineColumn { line: 1, column: 0 },
                LineColumn { line: 1, column: 0 },
                Edge(None, Brush::NorthSouth('|'), None),
            )]
        );

        parse!(
            single_line,
            "│\n│\n│\n",
            [(
                LineColumn { line: 1, column: 0 },
                LineColumn { line: 3, column: 0 },
                Edge(None, Brush::NorthSouth('│'), None),
            )]
        );

        parse!(
            multi_line,
            "││\n││\n│\n",
            [
                (
                    LineColumn { line: 1, column: 0 },
                    LineColumn { line: 3, column: 0 },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn { line: 1, column: 1 },
                    LineColumn { line: 2, column: 1 },
                    Edge(None, Brush::NorthSouth('│'), None),
                )
            ]
        );
    }

    mod node {
        use crate::{Brush, Edge, Graph, LineColumn};

        parse!(
            simple_cross,
            "
             │
            ─┼─
             │
            ",
            [
                (
                    LineColumn {
                        line: 2,
                        column: 13
                    },
                    LineColumn {
                        line: 3,
                        column: 13
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 12
                    },
                    LineColumn {
                        line: 3,
                        column: 13
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 13
                    },
                    LineColumn {
                        line: 3,
                        column: 14
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 13
                    },
                    LineColumn {
                        line: 4,
                        column: 13
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
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
                    LineColumn {
                        line: 2,
                        column: 12
                    },
                    LineColumn {
                        line: 2,
                        column: 14
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn {
                        line: 2,
                        column: 12
                    },
                    LineColumn {
                        line: 4,
                        column: 12
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 2,
                        column: 14
                    },
                    LineColumn {
                        line: 4,
                        column: 14
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 4,
                        column: 12
                    },
                    LineColumn {
                        line: 4,
                        column: 14
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
                    LineColumn {
                        line: 2,
                        column: 12
                    },
                    LineColumn {
                        line: 2,
                        column: 14
                    },
                    Edge(None, Brush::EastWest('━'), None),
                ),
                (
                    LineColumn {
                        line: 2,
                        column: 12
                    },
                    LineColumn {
                        line: 4,
                        column: 12
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 2,
                        column: 14
                    },
                    LineColumn {
                        line: 4,
                        column: 14
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 4,
                        column: 12
                    },
                    LineColumn {
                        line: 4,
                        column: 14
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
                    LineColumn {
                        line: 2,
                        column: 12
                    },
                    LineColumn {
                        line: 2,
                        column: 14
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn {
                        line: 2,
                        column: 12
                    },
                    LineColumn {
                        line: 4,
                        column: 12
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 2,
                        column: 14
                    },
                    LineColumn {
                        line: 4,
                        column: 14
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 4,
                        column: 12
                    },
                    LineColumn {
                        line: 4,
                        column: 14
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
                    LineColumn {
                        line: 2,
                        column: 13
                    },
                    LineColumn {
                        line: 3,
                        column: 13
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 13
                    },
                    LineColumn {
                        line: 3,
                        column: 15
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 13
                    },
                    LineColumn {
                        line: 5,
                        column: 13
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 15
                    },
                    LineColumn {
                        line: 3,
                        column: 16
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 15
                    },
                    LineColumn {
                        line: 5,
                        column: 15
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 5,
                        column: 12
                    },
                    LineColumn {
                        line: 5,
                        column: 13
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn {
                        line: 5,
                        column: 13
                    },
                    LineColumn {
                        line: 5,
                        column: 15
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn {
                        line: 5,
                        column: 15
                    },
                    LineColumn {
                        line: 6,
                        column: 15
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
                    LineColumn {
                        line: 2,
                        column: 13
                    },
                    LineColumn {
                        line: 3,
                        column: 13
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 13
                    },
                    LineColumn {
                        line: 3,
                        column: 15
                    },
                    Edge(None, Brush::EastWest('━'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 13
                    },
                    LineColumn {
                        line: 5,
                        column: 13
                    },
                    Edge(None, Brush::NorthSouth('┃'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 15
                    },
                    LineColumn {
                        line: 3,
                        column: 16
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn {
                        line: 3,
                        column: 15
                    },
                    LineColumn {
                        line: 5,
                        column: 15
                    },
                    Edge(None, Brush::NorthSouth('┃'), None),
                ),
                (
                    LineColumn {
                        line: 5,
                        column: 12
                    },
                    LineColumn {
                        line: 5,
                        column: 13
                    },
                    Edge(None, Brush::EastWest('─'), None),
                ),
                (
                    LineColumn {
                        line: 5,
                        column: 13
                    },
                    LineColumn {
                        line: 5,
                        column: 15
                    },
                    Edge(None, Brush::EastWest('━'), None),
                ),
                (
                    LineColumn {
                        line: 5,
                        column: 15
                    },
                    LineColumn {
                        line: 6,
                        column: 15
                    },
                    Edge(None, Brush::NorthSouth('│'), None),
                )
            ]
        );
    }
}
