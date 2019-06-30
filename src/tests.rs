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
            "││\n││\n││\n",
            [
                (
                    LineColumn { line: 1, column: 0 },
                    LineColumn { line: 3, column: 0 },
                    Edge(None, Brush::NorthSouth('│'), None),
                ),
                (
                    LineColumn { line: 1, column: 1 },
                    LineColumn { line: 3, column: 1 },
                    Edge(None, Brush::NorthSouth('│'), None),
                )
            ]
        );
    }
}
