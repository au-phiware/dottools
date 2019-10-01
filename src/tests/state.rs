use crate::{Brush, Edge, LineColumn, Node, Port, Region, State};
use pretty_assertions::assert_eq;

#[test]
fn next() {
    let mut state = &mut State::start();
    state.location.character = '╢';
    state.next();
    state.previous_location = state.location;
    state.location.visual.line += 1;
    state.location.source.line += 1;
    state.next();
    assert_eq!(
        &state.ports[..],
        &[Port {
            start: Node {
                character: '╢',
                source: LineColumn { line: 2, column: 0 },
                visual: LineColumn { line: 2, column: 0 },
                region: (Region::Center, Region::Center),
            },
            end: Node {
                character: '╢',
                source: LineColumn { line: 2, column: 0 },
                visual: LineColumn { line: 2, column: 0 },
                region: (Region::South, Region::Center),
            },
            edge: Edge(None, Brush::NorthSouth('║'), None),
        }][..],
    );
}
