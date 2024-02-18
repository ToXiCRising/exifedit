slint::include_modules!();

mod handling_images;
mod type_conversion;
mod data_handler;

use std::path::PathBuf;
use data_handler::DataHandler;
use slint::Image;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let mut dh: DataHandler = DataHandler{
        curretly_selected: 0,
        image_paths: vec![],
    };

    ui.global::<Logic>().on_clickedImageTile({
        let ui_handle = ui.as_weak();
        move |id|{
            let ui = ui_handle.unwrap();
            dh.set_currently_selected(id as usize);
            //update(ui, &dh_handle);
        }
    });

    ui.on_openFileSelector({ 
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            dh.add_image_paths(&mut handling_images::open_file_selector());

            update(ui, &dh);     
        }
    });

    ui.run()
}

fn update(ui: AppWindow, dh: &DataHandler){
    //Updates image carousel
    ui.set_carousel_image_names(
        type_conversion::paths_to_model(dh.image_paths.to_vec())   
    );

    ui.set_carousel_images(
        type_conversion::images_to_model(dh.image_paths.to_vec())
    );

    ui.set_carousel_viewport_height(dh.image_paths.len() as i32 * 150);
    ui.set_carousel_cur_selected(dh.curretly_selected as i32);

    //Updates main Preview
    let cur_path = &dh.image_paths[dh.curretly_selected];
    let cur_selected = Image::load_from_path(&cur_path);
    ui.set_preview_image(
        Result::expect(cur_selected, "msg")
    );

    //updates exif editor
    
}

