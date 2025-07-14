
slint::include_modules!();
use web::servo_start;

fn main() -> Result<(), slint::PlatformError> {
    // Create Slint AppWindow
    let app_window = AppWindow::new()?;

    //Create Browser (Servo wrapper)
    let mut browser = servo_start(800, 800);
    browser.load_page("https://servo.org");

    //Start a timer to update at ~60fps
    slint::Timer::default().start(
        slint::TimerMode::Repeated,
        std::time::Duration::from_millis(16),
        {
            let app_window_weak = app_window.as_weak();

            move || {
                if let Some(app_window) = app_window_weak.upgrade() {
                    
                    browser.render();

                    // Get the OpenGL texture ID
                    let texture_id = browser.get_texture_id();

                    // Update Slint with this texture
                    app_window.set_servo_texture_id(texture_id);

                    let servo_rendered_image = browser.render_view_as_image(texture_id);

                // Update Slint UI image
                    app_window.set_servo_image(servo_rendered_image);
                }
            }
        },
    );

    //Run the Slint app
    app_window.run()
}