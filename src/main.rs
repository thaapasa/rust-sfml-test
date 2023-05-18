use sfml::graphics::{Color, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable};

fn main() {
    let mut window = RenderWindow::new((800, 600), "SFML Window", Default::default(), &Default::default());
    window.set_vertical_sync_enabled(true);

    let mut rectangle = RectangleShape::new();
    rectangle.set_fill_color(Color::GREEN);
    rectangle.set_size((100., 100.));
    rectangle.set_position((100., 100.));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if let sfml::window::Event::Closed = event {
                window.close();
            }
        }

        window.clear(Color::BLACK);
        window.draw(&rectangle);
        window.display();
    }
}
