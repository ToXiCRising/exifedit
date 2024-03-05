slint::include_modules!();

mod loading_and_manipulating_data;
mod type_conversion;
mod data_handler;
mod standard_values;

use std::{fs::remove_dir, fs::remove_file, process::Command};
use std::sync::Mutex;
use lazy_static::lazy_static;
use slint::{Image, SharedString, platform};
use data_handler::DataHandler;


lazy_static! {
    static ref DH: Mutex<DataHandler> = Mutex::new(DataHandler{
        currently_selected: 0,
        image_paths: vec![],
        preview_paths: vec![],

        camera_names: vec![],
        lens_names: vec![],
        focal_length: vec![],
        iso: vec![],
        aperture: vec![],
        shutter_speed: vec![]
    });
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    //------ setting up icons and standard values ------
    //TODO: emmbed the png into the exe, the program cant find it otherwise 

    //let icon = std::include_bytes!("../recources/ExifToolIcon.png");

    //ui.set_icon_(Image::load_from_path(&PathBuf::from("./recources/ExifToolIcon.png")).unwrap());
    ui.set_exif_camera((standard_values::CAMERA_DEFAULT).into());
    ui.set_exif_lens((standard_values::LENS_DEFAULT).into());
    ui.set_exif_focal_length((standard_values::FOCAL_LENGTH).into());
    ui.set_exif_iso((standard_values::ISO_DEFAULT).into());
    ui.set_exif_aperture((standard_values::APERTURE_DEFAULT).into());
    ui.set_exif_shutter_speed((standard_values::SHUTTER_SPEED_DEFAULT).into());


    //------ handling callbacks ------

    ui.on_openFileSelector({ 
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();

            let mut new_images = loading_and_manipulating_data::open_file_selector();
            let mut new_previews = loading_and_manipulating_data::generate_previews(&new_images);
            DH.lock().unwrap().add_new_images(&mut new_images, &mut new_previews);
            update_main_view(&ui);
            update_carousel(&ui);     
        }
    });


    ui.global::<Logic>().on_clickedCarouselTile({
        let ui_handle = ui.as_weak();
        move |id|{
            let ui = ui_handle.unwrap();

            DH.lock().unwrap().currently_selected = id as usize;
            //println!("clicked tile {id}");
            //println!("{}", DH.lock().unwrap().currently_selected);
            update_main_view(&ui);
            update_exif_tiles(&ui);
            ui.set_carousel_cur_selected(DH.lock().unwrap().currently_selected as i32);
        }
    });

    ui.global::<Logic>().on_updatedExifField({
        let ui_handle = ui.as_weak();
        move |id, entry|{
            let ui = ui_handle.unwrap();

            let cur = DH.lock().unwrap().currently_selected;
            let num_images = DH.lock().unwrap().get_noi();

            if  num_images != 0 {
                match id {
                    0=>DH.lock().unwrap().camera_names[cur] = entry.to_string(),
                    1=>DH.lock().unwrap().lens_names[cur] = entry.to_string(),
                    2=>DH.lock().unwrap().focal_length[cur] = entry.to_string(),
                    3=>DH.lock().unwrap().iso[cur] = entry.to_string(),
                    4=>DH.lock().unwrap().aperture[cur] = entry.to_string(),
                    5=>DH.lock().unwrap().shutter_speed[cur] = entry.to_string(),
            
                    i32::MIN..=-1_i32 | 1_i32..=i32::MAX => unimplemented!(),  
                }
                update_exif_tiles(&ui);
            }
        }
    });

    ui.on_writeExifData({
        move || {
            if DH.lock().unwrap().image_paths.is_empty(){
                println!("No images loaded yet!")
            } else {
                let num_images = DH.lock().unwrap().get_noi();
                
                for i in 0..num_images {
                    //TODO: Handle the exit codes of exiftool!
                    let _exit_code = call_exiftool(i);
                }  
            }
        }
    });

    // ------ handles key-based navigation ------
    ui.on_keyPressed({
        let ui_handle = ui.as_weak();
        move |key_event| {
            let ui = ui_handle.unwrap();
            let num_images = DH.lock().unwrap().get_noi();

            if  num_images != 0 {
                // Tabbing through exif-tiles
                if key_event.text == SharedString::from(platform::Key::Tab) &&
                   !key_event.modifiers.shift{
                    println!("Tabbed");
                }
                //NOTE: Backtab would be the right key, but doesnt seem to work 
                if key_event.text == SharedString::from(platform::Key::Tab) &&
                   key_event.modifiers.shift {
                    println!("Backtabbed");
                }

                // Navigation through the carousel
                if key_event.text == SharedString::from(platform::Key::UpArrow) || 
                   key_event.text == SharedString::from(platform::Key::LeftArrow) {
                    if DH.lock().unwrap().currently_selected == 0 {
                        DH.lock().unwrap().currently_selected = num_images - 1;
                    } else {
                        DH.lock().unwrap().currently_selected -= 1;
                    }
                    ui.set_carousel_cur_selected(DH.lock().unwrap().currently_selected as i32);
                    update_main_view(&ui);
                }
                if key_event.text == SharedString::from(platform::Key::DownArrow) || 
                   key_event.text == SharedString::from(platform::Key::RightArrow) {
                    if DH.lock().unwrap().currently_selected == num_images - 1 {
                        DH.lock().unwrap().currently_selected = 0;
                    } else {
                        DH.lock().unwrap().currently_selected += 1;
                    }
                    ui.set_carousel_cur_selected(DH.lock().unwrap().currently_selected as i32);
                    update_main_view(&ui);
                }         
            }
        }
    });

    // ------ cleanup ------
    ui.window().on_close_requested(move || {
        println!("Cleaning up!");
        //EVIIIILLLLL!!!!! aber ich weis auch momentan net wie besser.....
        let previews = &DH.lock().unwrap().preview_paths;
        for preview in previews {
            let _res_file = remove_file(preview);
        }
        let _res_dir = remove_dir("exif_previews");
        return slint::CloseRequestResponse::HideWindow;
    });

    ui.run()
}

fn update_main_view(ui: &AppWindow){
    //Updates main Preview

    if DH.lock().unwrap().get_noi() == 0 {
        return;
    }

    let cur = DH.lock().unwrap().currently_selected;
    let cur_path = &DH.lock().unwrap().preview_paths[cur];
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
        type_conversion::images_to_model(DH.lock().unwrap().preview_paths.to_vec())
    );

    ui.set_carousel_viewport_height(DH.lock().unwrap().image_paths.len() as i32 * 150);
    ui.set_carousel_cur_selected(DH.lock().unwrap().currently_selected as i32);
}

fn update_exif_tiles(ui: &AppWindow){

    //updates exif editor
    let cur = DH.lock().unwrap().currently_selected;

    ui.set_exif_camera((&DH.lock().unwrap().camera_names[cur]).into());
    ui.set_exif_lens((&DH.lock().unwrap().lens_names[cur]).into());
    ui.set_exif_focal_length((&DH.lock().unwrap().focal_length[cur]).into());
    ui.set_exif_iso((&DH.lock().unwrap().iso[cur]).into());
    ui.set_exif_aperture((&DH.lock().unwrap().aperture[cur]).into());
    ui.set_exif_shutter_speed((&DH.lock().unwrap().shutter_speed[cur]).into());
}

fn call_exiftool(i: usize) -> i32{
    
    let (manufacturer, model) = type_conversion::split_camera_name(DH.lock().unwrap().camera_names[i].clone());
    
    println!("\nCamera: {}", DH.lock().unwrap().camera_names[i]);
    println!("Lens: {}", DH.lock().unwrap().lens_names[i]);
    println!("Focal Length: {}", DH.lock().unwrap().focal_length[i]);
    println!("ISO: {}", DH.lock().unwrap().iso[i]);
    println!("Aperture: {}", DH.lock().unwrap().aperture[i]);
    println!("Shutter Speed: {} \n", DH.lock().unwrap().shutter_speed[i]);
    
    let output = Command::new("exiftool")
                    .arg(format!("-make=\"{}\"", manufacturer))
                    .arg(format!("-model=\"{}\"", model))
                    .arg(format!("-lens=\"{}\"", DH.lock().unwrap().lens_names[i]))
                    .arg(format!("-focallength={}", DH.lock().unwrap().focal_length[i]))
                    .arg(format!("-iso={}", DH.lock().unwrap().iso[i]))
                    .arg(format!("-aperturevalue={}", DH.lock().unwrap().aperture[i]))
                    .arg(format!("-Fnumber={}", DH.lock().unwrap().aperture[i]))
                    .arg(format!("-exposuretime={}", DH.lock().unwrap().shutter_speed[i]))
                    .arg(&DH.lock().unwrap().image_paths[i])
                    .status().expect("exiftool failed!");
    println!("{output}\n");
    return output.code().unwrap();
}

