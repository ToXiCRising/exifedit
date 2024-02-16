slint::include_modules!();

mod handling_images;
use handling_images::*;


use slint::Image;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_openFileSelector({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let cur_selected = Image::load_from_path(&open_file_selector());
            
            ui.set_image(
                Result::expect(cur_selected, "msg")
            );
        }
    });

    ui.run()
}

