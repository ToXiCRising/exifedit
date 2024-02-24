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
    ui.set_exif_camera((standard_values::CAMERA_DEFAULT).into());
    ui.set_exif_lens((standard_values::LENS_DEFAULT).into());
    ui.set_exif_iso((standard_values::ISO_DEFAULT).into());
    ui.set_exif_aperture((standard_values::APERTURE_DEFAULT).into());
    ui.set_exif_shutter_speed((standard_values::SHUTTER_SPEED_DEFAULT).into());


    //------ handling callbacks ------

    ui.on_openFileSelector({ 
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            DH.lock().unwrap().add_new_images(&mut handling_images::open_file_selector());
            update_main_view(&ui);
            update_carousel(&ui);     
        }
    });


    ui.global::<Logic>().on_clickedImageTile({
        let ui_handle = ui.as_weak();
        move |id|{
            let ui = ui_handle.unwrap();

            //println!("clicked tile {id}");
            DH.lock().unwrap().set_currently_selected(id as usize);
            println!("{}", DH.lock().unwrap().currently_selected);
            update_main_view(&ui);
            update_exif_tiles(&ui);
        }
    });

    ui.global::<Logic>().on_updatedExifField({
        let ui_handle = ui.as_weak();
        move |id, entry|{
            let ui = ui_handle.unwrap();
            //let mut DH.lock().unwrap() = DH.lock().unwrap();
            let cur = DH.lock().unwrap().currently_selected;
            let no_images = DH.lock().unwrap().image_paths.len() == 0;

            //println!("{cur}");
            if  !no_images {
                match id {
                    0=>DH.lock().unwrap().camera_names[cur] = entry.to_string(),
                    1=>DH.lock().unwrap().lens_names[cur] = entry.to_string(),
                    2=>DH.lock().unwrap().iso[cur] = entry.to_string(),
                    3=>DH.lock().unwrap().aperture[cur] = entry.to_string(),
                    4=>DH.lock().unwrap().shutter_speed[cur] = entry.to_string(),
            
                    i32::MIN..=-1_i32 | 1_i32..=i32::MAX => unimplemented!(),  
                }
                update_exif_tiles(&ui);
            }
        }
    });

    ui.on_writeExifData({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            if DH.lock().unwrap().image_paths.is_empty(){
                println!("No images loaded yet!")
            } else {
                
                println!("{}", DH.lock().unwrap().camera_names.len());
                for i in 0..DH.lock().unwrap().image_paths.len() {
                    println!("Camera: {}", DH.lock().unwrap().camera_names[i]);
                    println!("Lens: {}", DH.lock().unwrap().lens_names[i]);
                    println!("ISO: {}", DH.lock().unwrap().iso[i]);
                    println!("Aperture: {}", DH.lock().unwrap().aperture[i]);
                    println!("Shutter Speed: {} \n", DH.lock().unwrap().shutter_speed[i]);
                }
                //call_exiftool();
            }
        }
    });



    ui.run()
}

fn update_main_view(ui: &AppWindow){
    //Updates main Preview

    let cur = DH.lock().unwrap().currently_selected;
    let cur_path = &DH.lock().unwrap().image_paths[cur];
    let cur_selected = Image::load_from_path(&cur_path);
    ui.set_preview_image(
        cur_selected.unwrap()
    );  
}

fn update_carousel(ui: &AppWindow){
    ui.set_carousel_image_names(
        type_conversion::paths_to_model(DH.lock().unwrap().image_paths.to_vec())    //dh.image_paths.to_vec()
    );

    ui.set_carousel_images(
        type_conversion::images_to_model(DH.lock().unwrap().image_paths.to_vec())
    );

    ui.set_carousel_viewport_height(DH.lock().unwrap().image_paths.len() as i32 * 150);
    ui.set_carousel_cur_selected(DH.lock().unwrap().currently_selected as i32);
}

fn update_exif_tiles(ui: &AppWindow){

    //updates exif editor
    let cur = DH.lock().unwrap().currently_selected;

    ui.set_exif_camera((&DH.lock().unwrap().camera_names[cur]).into());
    ui.set_exif_lens((&DH.lock().unwrap().lens_names[cur]).into());
    ui.set_exif_iso((&DH.lock().unwrap().iso[cur]).into());
    ui.set_exif_aperture((&DH.lock().unwrap().aperture[cur]).into());
    ui.set_exif_shutter_speed((&DH.lock().unwrap().shutter_speed[cur]).into());
}

fn call_exiftool(){
    use std::process::Command;
    let cur = DH.lock().unwrap().currently_selected;
    
    //let output = Command::new("echo").arg("hi").output().expect("echo command failed to start");

    //println!("status: {}", output.status);


    //TODO: Fix This!
    //Command::new("echo")
    //                    .arg(format!("-make=\"{}\"", DH.lock().unwrap().camera_names[cur]))
    //                    .arg(format!("-model=\"{}\"", DH.lock().unwrap().camera_names[cur]))
    //                    .arg(format!("-lens=\"{}\"", DH.lock().unwrap().lens_names[cur]))
    //                    .arg("-focallength=\"127\"")
    //                    .arg(format!("-iso=\"{}\"", DH.lock().unwrap().iso[cur]))
    //                    .arg(format!("-aperturevalue=\"{}\"", DH.lock().unwrap().aperture[cur]))
    //                    .arg(format!("-Fnumber=\"{}\"", DH.lock().unwrap().aperture[cur]))
    //                    .arg(format!("-exposuretime=\"{}\"", DH.lock().unwrap().shutter_speed[cur]))
    //                    .spawn().expect("test");
}

