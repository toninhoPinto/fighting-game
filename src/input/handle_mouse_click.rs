use sdl2::{event::Event, mouse::MouseButton, rect::Rect};



pub fn rcv_mouse_input(event: &Event) -> Option<(bool, (i32, i32))>{
    return match *event {
        Event::MouseButtonDown { mouse_btn, clicks, x, y, ..} => {
            println!("mouse btn {:?} clicks {} x {} y {}", mouse_btn, clicks, x, y);
            if mouse_btn == MouseButton::Left {
                Some((true, (x, y)))
            } else {
                None
            }
        },
        Event::MouseMotion { x, y, ..} => {
            Some((false,(x, y)))
        }
        _ => None,
    };
}

pub fn check_mouse_within_rect(click: (i32, i32), rect: &Rect) -> bool {
    let inside_horizontal = click.0 > rect.x() && click.0 < (rect.x() + rect.width() as i32);  
    let inside_vertical = click.1 > rect.y() && click.1 < (rect.y() + rect.height() as i32);

    inside_horizontal && inside_vertical 
}