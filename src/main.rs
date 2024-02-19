slint::include_modules!();

mod handling_images;
mod type_conversion;
mod data_handler;

use std::path::PathBuf;
use lazy_static::lazy_static;
use std::sync::Mutex;
use data_handler::DataHandler;
use slint::Image;

lazy_static! {
    static ref DH: Mutex<DataHandler> = Mutex::new(DataHandler{
        curretly_selected: 0,
        image_paths: vec![],
    });
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.global::<Logic>().on_clickedImageTile({
        let ui_handle = ui.as_weak();
        move |id|{
            let ui = ui_handle.unwrap();

            println!("clicked tile {id}");
            DH.lock().unwrap().set_currently_selected(id as usize);
            update(ui, false);
        }
    });

    ui.on_openFileSelector({ 
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            DH.lock().unwrap().add_image_paths(&mut handling_images::open_file_selector());
            update(ui, true);     
        }
    });

    ui.run()
}

fn update(ui: AppWindow, update_carousel: bool){
    let dh_handle = DH.lock().unwrap();
    //Updates image carousel
    if update_carousel {
        ui.set_carousel_image_names(
            type_conversion::paths_to_model(dh_handle.image_paths.to_vec())    //dh.image_paths.to_vec()
        );
    
        ui.set_carousel_images(
            type_conversion::images_to_model(dh_handle.image_paths.to_vec())
        );
    
        ui.set_carousel_viewport_height(dh_handle.image_paths.len() as i32 * 150);
        ui.set_carousel_cur_selected(dh_handle.curretly_selected as i32);
    }

    //Updates main Preview
    let cur_path = &dh_handle.image_paths[dh_handle.curretly_selected];
    let cur_selected = Image::load_from_path(&cur_path);
    ui.set_preview_image(
        Result::expect(cur_selected, "msg")
    );

    //updates exif editor
    
}

