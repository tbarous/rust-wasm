use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

struct Coordinates {
    x: f64,
    y: f64,
}

struct Triangle<'a> {
    context: &'a web_sys::CanvasRenderingContext2d,
    top: Coordinates,
    height: f64,
}

impl Triangle<'_> {
    fn new(top: Coordinates, height: f64, context: &web_sys::CanvasRenderingContext2d) -> Triangle {
        Triangle {
            top,
            height,
            context,
        }
    }

    fn get_base_width(&self) -> f64 {
        self.height / 2.0
    }

    fn get_half_base_width(&self) -> f64 {
        self.get_base_width() / 2.0
    }

    fn get_left_coordinates(&self) -> Coordinates {
        Coordinates {
            x: self.top.x - self.get_base_width(),
            y: self.top.y + self.height,
        }
    }

    fn get_right_coordinates(&self) -> Coordinates {
        Coordinates {
            x: self.top.x + self.get_base_width(),
            y: self.top.y + self.height,
        }
    }

    fn draw(&self) -> &Self {
        let left_coordinates = self.get_left_coordinates();
        let right_coordinates = self.get_right_coordinates();

        self.context.move_to(self.top.x, self.top.y);
        self.context.begin_path();
        self.context.line_to(left_coordinates.x, left_coordinates.y);
        self.context
            .line_to(right_coordinates.x, right_coordinates.y);
        self.context.line_to(self.top.x, self.top.y);
        self.context.close_path();
        self.context.stroke();

        &self
    }

    fn fill(&self) {
        self.context.fill();
    }

    fn get_half_height(&self) -> f64 {
        self.height / 2.0
    }

    fn get_top_sub_triangle(&self) -> Triangle {
        Triangle::new(
            Coordinates {
                x: self.top.x,
                y: self.top.y,
            },
            self.get_half_height(),
            self.context,
        )
    }

    fn get_left_sub_triangle(&self) -> Triangle {
        Triangle::new(
            Coordinates {
                x: self.top.x - self.get_half_base_width(),
                y: self.top.y + self.get_half_height(),
            },
            self.get_half_height(),
            self.context,
        )
    }

    fn get_right_sub_triangle(&self) -> Triangle {
        Triangle::new(
            Coordinates {
                x: self.top.x + self.get_half_base_width(),
                y: self.top.y + self.get_half_height(),
            },
            self.get_half_height(),
            self.context,
        )
    }

    fn split(&self, times: u64) {
        if times == 0 {
            return;
        }

        self.get_top_sub_triangle().draw().split(times - 1);
        self.get_left_sub_triangle().draw().split(times - 1);
        self.get_right_sub_triangle().draw().split(times - 1);
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    Triangle::new(Coordinates { x: 300.0, y: 0.0 }, 600.0, &context)
        .draw()
        .split(6);

    Ok(())
}
