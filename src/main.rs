slint::include_modules!();

mod loading_and_manipulating_data;
mod type_conversion;
mod data_handler;
mod standard_values;
mod tag_store;
mod image_database;

use std::fs::{remove_dir, remove_file};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
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

    // ------ initializes datahandler and tag store
    let tag_store = Mutex::new(tag_store::create_default_tag_store());
    let data_handler = Mutex::new(image_database::DataHandler{
        currently_selected: 0,
        images: image_database::create_image_handler(),
    });
    //Removes the temporary entry that gets created upon creating the image_database
    data_handler.lock().unwrap().images.pop();

    let ts_handle = Arc::new(tag_store);
    let dh_handle = Arc::new(data_handler);
    //println!("{}", arg.replace("xxx", &data_handler.lock().unwrap().images[0][tag_name]));
   

    let ui = AppWindow::new()?;

    //------ setting up icons and standard values ------
    //TODO: emmbed the png into the exe, the program cant find it otherwise 

    //let icon = std::include_bytes!("../recources/ExifToolIcon.png");
    //ui.set_icon_(Image::load_from_path(&PathBuf::from("./recources/ExifToolIcon.png")).unwrap());
    //initialize_exif_tiles(&ui, &ts_handle);

    //------ handling callbacks ------

    ui.on_openFileSelector({ 
        let ui_handle = ui.as_weak();
        let data_handler = Arc::clone(&dh_handle);
        let tag_store = Arc::clone(&ts_handle);
        move || {
            let ui = ui_handle.unwrap();

            let mut new_images = loading_and_manipulating_data::open_file_selector();
            let mut new_previews = loading_and_manipulating_data::generate_previews(&new_images);
            data_handler.lock().unwrap().add_new_images(&mut new_images, &mut new_previews);
            data_handler.lock().unwrap().update_data_handler_tags(&tag_store);
            update_main_view(&ui, &data_handler);
            update_carousel(&ui, &data_handler);  
            update_exif_tiles(&ui, &data_handler, &tag_store);   
        }
    });


    ui.global::<Logic>().on_clickedCarouselTile({
        let ui_handle = ui.as_weak();
        let data_handler = Arc::clone(&dh_handle);
        let tag_store = Arc::clone(&ts_handle);
        move |id|{
            let ui = ui_handle.unwrap();

            data_handler.lock().unwrap().currently_selected = id as usize;
            //println!("clicked tile {id}");
            //println!("{}", DH.lock().unwrap().currently_selected);
            update_main_view(&ui, &data_handler);
            update_exif_tiles(&ui, &data_handler, &tag_store);
            ui.set_carousel_cur_selected(data_handler.lock().unwrap().currently_selected as i32);
        }
    });

    ui.global::<Logic>().on_updatedExifField({
        let ui_handle = ui.as_weak();
        let data_handler = Arc::clone(&dh_handle);
        let tag_store = Arc::clone(&ts_handle);
        move |id, entry|{
            let ui = ui_handle.unwrap();

            let cur = data_handler.lock().unwrap().currently_selected;
            let cur_tag = tag_store.lock().unwrap()[id as usize].tag_name.clone();
            let num_images = data_handler.lock().unwrap().get_noi();

            if num_images != 0 {
                data_handler.lock().unwrap().images[cur].insert(cur_tag, entry.to_string());
            }
            update_exif_tiles(&ui, &data_handler, &tag_store);
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
        let data_handler = Arc::clone(&dh_handle);
        move |key_event| {
            let ui = ui_handle.unwrap();
            let num_images = data_handler.lock().unwrap().get_noi();

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
                    if data_handler.lock().unwrap().currently_selected == 0 {
                        data_handler.lock().unwrap().currently_selected = num_images - 1;
                    } else {
                        data_handler.lock().unwrap().currently_selected -= 1;
                    }
                    ui.set_carousel_cur_selected(data_handler.lock().unwrap().currently_selected as i32);
                    update_main_view(&ui, &data_handler);
                }
                if key_event.text == SharedString::from(platform::Key::DownArrow) || 
                   key_event.text == SharedString::from(platform::Key::RightArrow) {
                    if data_handler.lock().unwrap().currently_selected == num_images - 1 {
                        data_handler.lock().unwrap().currently_selected = 0;
                    } else {
                        data_handler.lock().unwrap().currently_selected += 1;
                    }
                    ui.set_carousel_cur_selected(data_handler.lock().unwrap().currently_selected as i32);
                    update_main_view(&ui, &data_handler);
                }         
            }
        }
    });

    // ------ cleanup ------
    ui.window().on_close_requested({
        let data_handler = Arc::clone(&dh_handle);
        move || {
            println!("\nCleaning up!");
            let previews = data_handler.lock().unwrap().get_preview_paths();
            for preview in previews {
                let _res_file = remove_file(preview);
            }
            let _res_dir = remove_dir("exif_previews");
            return slint::CloseRequestResponse::HideWindow;
        } 
    });

    ui.run()
}

fn update_main_view(ui: &AppWindow, data_handler: &Mutex<image_database::DataHandler>){
    //Updates main Preview

    if data_handler.lock().unwrap().get_noi() == 0 {
        return;
    }
    
    let cur = data_handler.lock().unwrap().currently_selected;
    let cur_path = PathBuf::from(&data_handler.lock().unwrap().images[cur]["preview_path"]);
    let cur_selected_image = Image::load_from_path(&cur_path);
    ui.set_preview_image(
        cur_selected_image.unwrap()
    );  
}

fn update_carousel(ui: &AppWindow, data_handler: &Mutex<image_database::DataHandler>){

    ui.set_carousel_image_names(
        type_conversion::paths_to_model(data_handler.lock().unwrap().get_image_paths())    //dh.image_paths.to_vec()
    );

    ui.set_carousel_images(
        type_conversion::images_to_model(data_handler.lock().unwrap().get_preview_paths())
    );

    ui.set_carousel_viewport_height(data_handler.lock().unwrap().get_noi() as i32 * 150);
    ui.set_carousel_cur_selected(data_handler.lock().unwrap().currently_selected as i32);
}

fn update_exif_tiles(ui: &AppWindow, data_handler: &Mutex<image_database::DataHandler>, tag_store: &Mutex<Vec<tag_store::Tag>>){
    
    let mut tags = vec![];
    let mut values = vec![];
    for tag in tag_store.lock().unwrap().as_slice() {
        tags.push(tag.tag_name.clone());
    }
    let cur = data_handler.lock().unwrap().currently_selected;
    let cur_image = &data_handler.lock().unwrap().images[cur];
    for tag in &tags {
        values.push(cur_image[tag].clone());
    }
    
    
    ui.set_exif_viewport_height((&tag_store.lock().unwrap().len() * 120) as i32);
    ui.set_exif_tiles(type_conversion::string_to_model(tags));
    ui.set_exif_defaults(type_conversion::string_to_model(values));
}

fn initialize_exif_tiles(ui: &AppWindow,tag_store: &Mutex<Vec<tag_store::Tag>>) {
    let mut tags = vec![];
    let mut values = vec![];
    for tag in tag_store.lock().unwrap().as_slice() {
        tags.push(tag.tag_name.clone());
        values.push(tag.default_value.clone());
    }

    ui.set_exif_viewport_height((tag_store.lock().unwrap().len() * 120) as i32);
    ui.set_exif_tiles(type_conversion::string_to_model(tags));
    ui.set_exif_defaults(type_conversion::string_to_model(values));
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

