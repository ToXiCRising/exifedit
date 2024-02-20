slint::include_modules!();

mod handling_images;
mod type_conversion;
mod data_handler;
mod standard_values;

use std::path::PathBuf;
use lazy_static::lazy_static;
use std::sync::Mutex;
use data_handler::DataHandler;
use slint::Image;

lazy_static! {
    static ref DH: Mutex<DataHandler> = Mutex::new(DataHandler{
        curretly_selected: 0,
        image_paths: vec![],

        camera_names: vec![],
        lens_names: vec![],
        iso: vec![],
        aperture: vec![],
        shutter_speed: vec![]
    });
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    //------ setting up icons and standard values ------
    ui.set_icon_(Image::load_from_path(&PathBuf::from("./recources/ExifToolIcon.png")).unwrap());
    ui.set_exif_camera_default(slint::SharedString::from(standard_values::CAMERA_DEFAULT));
    ui.set_exif_lens_default(slint::SharedString::from(standard_values::LENS_DEFAULT));
    ui.set_exif_iso_default(slint::SharedString::from(standard_values::ISO_DEFAULT));
    ui.set_exif_aperture_default(slint::SharedString::from(standard_values::APERTURE_DEFAULT));
    ui.set_exif_shutter_speed_default(slint::SharedString::from(standard_values::SHUTTER_SPEED_DEFAULT));


    //------ handling callbacks ------
    ui.global::<Logic>().on_clickedImageTile({
        let ui_handle = ui.as_weak();
        move |id|{
            let ui = ui_handle.unwrap();

            println!("clicked tile {id}");
            DH.lock().unwrap().set_currently_selected(id as usize);
            update(ui, false);
        }
    });

    ui.global::<Logic>().on_updatedExifField({
        let ui_handle = ui.as_weak();
        move |id, entry|{
            let ui = ui_handle.unwrap();
            println!("Field with id: {id}, recieved value {entry}");
        }
    });

    ui.on_openFileSelector({ 
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            DH.lock().unwrap().add_new_images(&mut handling_images::open_file_selector());
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

