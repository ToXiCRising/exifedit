slint::include_modules!();

mod handling_images;
mod type_conversion;

use std::path::PathBuf;
use slint::Image;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_openFileSelector({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            let image_paths: Vec<PathBuf> = handling_images::open_file_selector();

            let cur_path = Option::expect(image_paths.last(), "msg");
            let cur_selected = Image::load_from_path(cur_path);
            ui.set_image(
                Result::expect(cur_selected, "msg")
            );

            ui.set_image_names(
                type_conversion::paths_to_model(image_paths)   
            )
            //the trait bound `ModelRc<slint::SharedString>: From<Vec<&str>>` is not
            
        }
    });

    ui.run()
}

