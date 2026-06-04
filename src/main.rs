use swayipc::{Connection, Event, EventType, Fallible, Rect, WindowChange};

/// Sway tick payload that triggers a pointer warp.
const TRIGGER: &str = "follow_focus";

fn center(rect: &Rect) -> (i32, i32) {
    (rect.x + rect.width / 2, rect.y + rect.height / 2)
}

fn main() -> Fallible<()> {
    // One connection issues commands, a second one streams events. The event
    // subscription consumes its connection, so it cannot be reused for commands.
    let mut commands = Connection::new()?;
    let events = Connection::new()?.subscribe([EventType::Window, EventType::Tick])?;

    // Remember the most recently focused window's geometry. The pointer is only
    // moved when a `follow_focus` tick arrives, never on focus changes alone.
    let mut current_rect: Option<Rect> = None;

    for event in events {
        match event? {
            Event::Window(e) if e.change == WindowChange::Focus => {
                current_rect = Some(e.container.rect);
            }

            Event::Tick(e) if e.payload == TRIGGER => {
                if let Some(rect) = &current_rect {
                    let (cx, cy) = center(rect);
                    commands.run_command(format!("seat - cursor set {cx} {cy}"))?;
                }
            }

            _ => {}
        }
    }

    Ok(())
}
