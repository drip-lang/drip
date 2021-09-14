use crate::event::Event;
use crate::parser::Parser;
use drip_syntax::SyntaxKind;
use drop_bomb::DropBomb;

pub struct Marker {
    pos: usize,
    completed: DropBomb,
}

impl Marker {
    pub(crate) fn new(pos: usize) -> Self {
        Self {
            pos,
            completed: DropBomb::new("Marker need to be completed"),
        }
    }

    pub(crate) fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.completed.defuse();

        let event_at_pos = &mut p.events[self.pos];
        assert_eq!(*event_at_pos, Event::Placeholder);

        *event_at_pos = Event::StartNode { kind, offset: None };

        p.events.push(Event::FinishNode);

        CompletedMarker { pos: self.pos }
    }

    pub(crate) fn abandon(mut self, p: &mut Parser) {
        self.completed.defuse();
        if self.pos == p.events.len() - 1 {
            match p.events.pop() {
                Some(Event::StartNode {
                    kind: SyntaxKind::Tombstone,
                    offset: None,
                }) => (),
                _ => unreachable!(),
            }
        }
    }
}

pub struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    pub(crate) fn precede(self, p: &mut Parser) -> Marker {
        let marker = p.start();

        if let Event::StartNode { ref mut offset, .. } = p.events[self.pos] {
            *offset = Some(marker.pos - self.pos)
        } else {
            unreachable!();
        }

        marker
    }
}
