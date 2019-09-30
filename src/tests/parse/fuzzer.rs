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

parse!(
    crash_98b62147253b4356e07e2128988583c491bb77df,
    // |||-|| ||
    // -|||||
    String::from_utf8(base64::decode("fHx8LXx8MXx8AAotfHx8fHw=").unwrap()).unwrap(),
    [
        (
            Node {
                character: '|',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::North, Region::Center)
            },
            Node {
                character: '|',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::South, Region::Center)
            },
            &Edge(None, Brush::NorthSouth('|'), None)
        ),
        (
            Node {
                character: '|',
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::North, Region::Center)
            },
            Node {
                character: '|',
                source: LineColumn { line: 2, column: 1 },
                visual: LineColumn { line: 2, column: 1 },
                region: (Region::South, Region::Center)
            },
            &Edge(None, Brush::NorthSouth('|'), None)
        ),
        (
            Node {
                character: '|',
                source: LineColumn { line: 1, column: 2 },
                visual: LineColumn { line: 1, column: 2 },
                region: (Region::North, Region::Center)
            },
            Node {
                character: '|',
                source: LineColumn { line: 2, column: 2 },
                visual: LineColumn { line: 2, column: 2 },
                region: (Region::South, Region::Center)
            },
            &Edge(None, Brush::NorthSouth('|'), None)
        ),
        (
            Node {
                character: '-',
                source: LineColumn { line: 1, column: 3 },
                visual: LineColumn { line: 1, column: 3 },
                region: (Region::Center, Region::West)
            },
            Node {
                character: '-',
                source: LineColumn { line: 1, column: 3 },
                visual: LineColumn { line: 1, column: 3 },
                region: (Region::Center, Region::East)
            },
            &Edge(None, Brush::EastWest('-'), None)
        ),
        (
            Node {
                character: '|',
                source: LineColumn { line: 1, column: 4 },
                visual: LineColumn { line: 1, column: 4 },
                region: (Region::North, Region::Center)
            },
            Node {
                character: '|',
                source: LineColumn { line: 2, column: 4 },
                visual: LineColumn { line: 2, column: 4 },
                region: (Region::South, Region::Center)
            },
            &Edge(None, Brush::NorthSouth('|'), None)
        ),
        (
            Node {
                character: '|',
                source: LineColumn { line: 1, column: 5 },
                visual: LineColumn { line: 1, column: 5 },
                region: (Region::North, Region::Center)
            },
            Node {
                character: '|',
                source: LineColumn { line: 2, column: 5 },
                visual: LineColumn { line: 2, column: 5 },
                region: (Region::South, Region::Center)
            },
            &Edge(None, Brush::NorthSouth('|'), None)
        ),
        (
            Node {
                character: '|',
                source: LineColumn { line: 1, column: 7 },
                visual: LineColumn { line: 1, column: 7 },
                region: (Region::North, Region::Center)
            },
            Node {
                character: '|',
                source: LineColumn { line: 1, column: 7 },
                visual: LineColumn { line: 1, column: 7 },
                region: (Region::South, Region::Center)
            },
            &Edge(None, Brush::NorthSouth('|'), None)
        ),
        (
            Node {
                character: '|',
                source: LineColumn { line: 1, column: 8 },
                visual: LineColumn { line: 1, column: 8 },
                region: (Region::North, Region::Center)
            },
            Node {
                character: '|',
                source: LineColumn { line: 1, column: 8 },
                visual: LineColumn { line: 1, column: 8 },
                region: (Region::South, Region::Center)
            },
            &Edge(None, Brush::NorthSouth('|'), None)
        ),
        (
            Node {
                character: '-',
                source: LineColumn { line: 2, column: 0 },
                visual: LineColumn { line: 2, column: 0 },
                region: (Region::Center, Region::West)
            },
            Node {
                character: '-',
                source: LineColumn { line: 2, column: 0 },
                visual: LineColumn { line: 2, column: 0 },
                region: (Region::Center, Region::East)
            },
            &Edge(None, Brush::EastWest('-'), None)
        ),
        (
            Node {
                character: '|',
                source: LineColumn { line: 2, column: 3 },
                visual: LineColumn { line: 2, column: 3 },
                region: (Region::North, Region::Center)
            },
            Node {
                character: '|',
                source: LineColumn { line: 2, column: 3 },
                visual: LineColumn { line: 2, column: 3 },
                region: (Region::South, Region::Center)
            },
            &Edge(None, Brush::NorthSouth('|'), None)
        )
    ]
);

parse!(
    crash_ee047d9ed0f8e590f9381417b5d1c26a210a9627,
    // ╿$
    String::from_utf8(base64::decode("4pW/JA==").unwrap()).unwrap(),
    [
        (
            Node {
                character: '╿',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::North, Region::Center),
            },
            Node {
                character: '╿',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('┃'), None),
        ),
        (
            Node {
                character: '╿',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╿',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::South, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('│'), None),
        )
    ]
);

parse!(
    crash_0840baa337e4477c28876adfce0bb7517f68ddc9,
    //   -/
    //  -/
    //  //
    // /
    String::from_utf8(base64::decode("OTItLwo5LS8KOS8vCi8=").unwrap()).unwrap(),
    [
        (
            Node {
                character: '-',
                source: LineColumn { line: 1, column: 2 },
                visual: LineColumn { line: 1, column: 2 },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '-',
                source: LineColumn { line: 1, column: 2 },
                visual: LineColumn { line: 1, column: 2 },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('-'), None),
        ),
        (
            Node {
                character: '-',
                source: LineColumn { line: 2, column: 1 },
                visual: LineColumn { line: 2, column: 1 },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '-',
                source: LineColumn { line: 2, column: 1 },
                visual: LineColumn { line: 2, column: 1 },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('-'), None),
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
                source: LineColumn { line: 4, column: 0 },
                visual: LineColumn { line: 4, column: 0 },
                region: (Region::South, Region::West),
            },
            &Edge(None, Brush::NorthEastSouthWest('/'), None),
        ),
        (
            Node {
                character: '/',
                source: LineColumn { line: 3, column: 2 },
                visual: LineColumn { line: 3, column: 2 },
                region: (Region::North, Region::East),
            },
            Node {
                character: '/',
                source: LineColumn { line: 3, column: 2 },
                visual: LineColumn { line: 3, column: 2 },
                region: (Region::South, Region::West),
            },
            &Edge(None, Brush::NorthEastSouthWest('/'), None),
        )
    ]
);

parse!(
    crash_684fc0c85b8895ad0165fec4cfad85581b7c8360,
    String::from_utf8(base64::decode("4pW+").unwrap()).unwrap(),
    [
        (
            Node {
                character: '╾',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '╾',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('━'), None),
        ),
        (
            Node {
                character: '╾',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╾',
                source: LineColumn { line: 1, column: 0 },
                visual: LineColumn { line: 1, column: 0 },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('─'), None),
        )
    ]
);

parse!(
    crash_e8724022d4d356a55083c9a2766944ce750c0c40,
    String::from_utf8(base64::decode("4pW325jboCI=").unwrap()).unwrap(),
    [(
        Node {
            character: '╷',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::Center, Region::Center),
        },
        Node {
            character: '╷',
            source: LineColumn { line: 1, column: 0 },
            visual: LineColumn { line: 1, column: 0 },
            region: (Region::South, Region::Center),
        },
        &Edge(None, Brush::NorthSouth('│'), None),
    )]
);

parse!(
    crash_e2b9fd646af8daa1e3d76831d1d3898156cfc91e,
    // ==╢ ╥
    String::from_utf8(base64::decode("PT3ilaId4pWl").unwrap()).unwrap(),
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
                source: LineColumn { line: 1, column: 1 },
                visual: LineColumn { line: 1, column: 1 },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('='), None),
        ),
        (
            Node {
                character: '╢',
                source: LineColumn { line: 1, column: 2 },
                visual: LineColumn { line: 1, column: 2 },
                region: (Region::North, Region::Center),
            },
            Node {
                character: '╢',
                source: LineColumn { line: 1, column: 2 },
                visual: LineColumn { line: 1, column: 2 },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('║'), None),
        ),
        (
            Node {
                character: '╢',
                source: LineColumn { line: 1, column: 2 },
                visual: LineColumn { line: 1, column: 2 },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '╢',
                source: LineColumn { line: 1, column: 2 },
                visual: LineColumn { line: 1, column: 2 },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '╢',
                source: LineColumn { line: 1, column: 2 },
                visual: LineColumn { line: 1, column: 2 },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╢',
                source: LineColumn { line: 1, column: 2 },
                visual: LineColumn { line: 1, column: 2 },
                region: (Region::South, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('║'), None),
        ),
        (
            Node {
                character: '╥',
                source: LineColumn { line: 1, column: 4 },
                visual: LineColumn { line: 1, column: 4 },
                region: (Region::Center, Region::West),
            },
            Node {
                character: '╥',
                source: LineColumn { line: 1, column: 4 },
                visual: LineColumn { line: 1, column: 4 },
                region: (Region::Center, Region::Center),
            },
            &Edge(None, Brush::EastWest('─'), None),
        ),
        (
            Node {
                character: '╥',
                source: LineColumn { line: 1, column: 4 },
                visual: LineColumn { line: 1, column: 4 },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╥',
                source: LineColumn { line: 1, column: 4 },
                visual: LineColumn { line: 1, column: 4 },
                region: (Region::South, Region::Center),
            },
            &Edge(None, Brush::NorthSouth('║'), None),
        ),
        (
            Node {
                character: '╥',
                source: LineColumn { line: 1, column: 4 },
                visual: LineColumn { line: 1, column: 4 },
                region: (Region::Center, Region::Center),
            },
            Node {
                character: '╥',
                source: LineColumn { line: 1, column: 4 },
                visual: LineColumn { line: 1, column: 4 },
                region: (Region::Center, Region::East),
            },
            &Edge(None, Brush::EastWest('─'), None),
        )
    ]
);
