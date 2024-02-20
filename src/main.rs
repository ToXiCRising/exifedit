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
        currently_selected: 0,
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

    ui.on_openFileSelector({ 
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            DH.lock().unwrap().add_new_images(&mut handling_images::open_file_selector());
            update(ui, true);     
        }
    });


    ui.global::<Logic>().on_clickedImageTile({
        let ui_handle = ui.as_weak();
        move |id|{
            let ui = ui_handle.unwrap();

            println!("clicked tile {id}");
            DH.lock().unwrap().set_currently_selected(id as usize);
            println!("{}", DH.lock().unwrap().currently_selected);
            update(ui, false);
        }
    });

    ui.global::<Logic>().on_updatedExifField({
        move |id, entry|{
            let mut dh_handle = DH.lock().unwrap();
            let cur = dh_handle.currently_selected;
            println!("{cur}");
            if dh_handle.image_paths.len() != 0{
                match id {
                    0=>dh_handle.camera_names[cur] = entry.to_string(),
                    1=>dh_handle.lens_names[cur] = entry.to_string(),
                    2=>dh_handle.iso[cur] = entry.to_string(),
                    3=>dh_handle.aperture[cur] = entry.to_string(),
                    4=>dh_handle.shutter_speed[cur] = entry.to_string(),
    
                    i32::MIN..=-1_i32 | 1_i32..=i32::MAX => unimplemented!(),  
                }
            }
        }
    });

    ui.on_writeExifData({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let dh_handle = DH.lock().unwrap();
            if dh_handle.image_paths.is_empty(){
                println!("No images loaded yet!")
            } else {
                
                println!("{}", dh_handle.camera_names.len());
                for i in 0..dh_handle.image_paths.len() {
                    println!("Camera: {}", dh_handle.camera_names[i]);
                    println!("Lens: {}", dh_handle.lens_names[i]);
                    println!("ISO: {}", dh_handle.iso[i]);
                    println!("Aperture: {}", dh_handle.aperture[i]);
                    println!("Shutter Speed: {} \n", dh_handle.shutter_speed[i]);
                }

            }
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
        ui.set_carousel_cur_selected(dh_handle.currently_selected as i32);
    }

    //Updates main Preview
    let cur_path = &dh_handle.image_paths[dh_handle.currently_selected];
    let cur_selected = Image::load_from_path(&cur_path);
    ui.set_preview_image(
        cur_selected.unwrap()
    );

    //updates exif editor
    
}

