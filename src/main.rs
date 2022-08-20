use std::cell::RefCell;
use std::rc::Rc;

use fltk::{*, enums::*, group::*, prelude::*};
use fltk::button::Button;
use fltk::draw::{draw_point, draw_rect_fill, draw_rect_with_color, get_color, LineStyle, set_draw_color, set_line_style};
use fltk::frame::Frame;
use fltk::surface::ImageSurface;

mod nimage;
mod ndb;

fn main() {
    let h = 800;
    let w = 600;
    let legends = ndb::get_colored_legend(&ndb::connect()).expect("failed to query legend");

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let mut wind = window::Window::default().with_size(w, h);

    let mut pos = 0;
    let mut hpack = Pack::new(0, 0, 16, 16, "");
    let mut frame = frame::Frame::default().with_size(680, 580).center_of_parent();
    hpack.set_type(PackType::Vertical);
    let (s, r) = app::channel::<i64>();
    let mut but_op_vec: Vec<Button> = Vec::new();
    for item in legends {
        let mut but = Button::new(60 + 16 * pos, 210, 15, 15, "");
        let bcolor = Color::from_rgb(item.r as u8, item.g as u8, item.b as u8);
        but.set_color(bcolor);
        but.emit(s, item.ind_value);
        but_op_vec.push(but);
        pos = pos + 1;
    }
    hpack.end();

    frame.set_color(Color::Black);
    frame.set_frame(FrameType::DownBox);

    let surf = ImageSurface::new(frame.width(), frame.height(), false);
    ImageSurface::push_current(&surf);
    draw_rect_fill(0, 433, 300, 123, Color::DarkCyan);
    ImageSurface::pop_current();
    let surf = Rc::from(RefCell::from(surf));

    frame.draw({
        let surf = surf.clone();
        move |f| {
            let surf = surf.borrow();
            let mut img = surf.image().unwrap();
            img.draw(f.x(), f.y(), f.w(), f.h());
        }
    });

    wind.make_resizable(true);
    wind.show();
    wind.end();

    app::get_system_colors();
    while app.wait() {
        if let Some(val) = r.recv() {
            println!("--- {}", val);
            frame.draw({
                let surf = surf.clone();
                move |f| {
                    let surf = surf.borrow();
                    ImageSurface::push_current(&surf);
                    set_draw_color(Color::Red);
                    set_line_style(LineStyle::Solid, 3);
                    let coords = (32, 425);
                    let x = coords.0;
                    let y = coords.1;
                    draw_point(x, y);
                    draw_rect_with_color(x, y, x + 95, y + 43, Color::Green);
                    ImageSurface::pop_current();
                    f.redraw();
                }
            });
        }
    }
    app.run().unwrap();
}