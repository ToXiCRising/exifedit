slint::include_modules!();

mod loading_and_manipulating_data;
mod type_conversion;
mod standard_values;
mod tag_store;
mod image_database;

use std::fs::{remove_dir, remove_file};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use slint::{Image, SharedString, platform};

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


    let ui = AppWindow::new()?;

    //------ setting up icons and standard values ------
    ui.set_icon_(loading_and_manipulating_data::load_slint_icon());
    //initialize_exif_tiles(&ui, &ts_handle);

    //------ handling callbacks ------

    ui.on_openFileSelector({ 
        let ui_handle = ui.as_weak();
        let data_handler = Arc::clone(&dh_handle);
        let tag_store = Arc::clone(&ts_handle);
        move || {
            let ui = ui_handle.unwrap();

            let mut new_images = loading_and_manipulating_data::open_file_selector();
            let mut new_previews = loading_and_manipulating_data::generate_preview_paths(&new_images);
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
        let data_handler = Arc::clone(&dh_handle);
        let tag_store = Arc::clone(&ts_handle);
        move || {
            if data_handler.lock().unwrap().images.is_empty(){
                println!("No images loaded yet!")
            } else {
                let num_images = data_handler.lock().unwrap().get_noi();
                
                for i in 0..num_images {
                    let image_to_process = &data_handler.lock().unwrap().images[i];
                    //TODO: Handle the exit codes of exiftool!
                    let _exit_code = call_exiftool(image_to_process, &tag_store);
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

fn call_exiftool(image: &HashMap<String, String>, tag_store: &Mutex<Vec<tag_store::Tag>>) -> i32{
    
    //let (manufacturer, model) = type_conversion::split_camera_name(DH.lock().unwrap().camera_names[i].clone());
    println!("");
    for tag in image.iter(){
        println!("{:?}", tag);
    }
    println!("");

    let mut argument: Vec<String> = vec![];
    for tag in image.iter(){
        if !tag.0.contains("ID") && 
           !tag.0.contains("path") {
            //println!("test: {:?}", tag);
            for tag_from_store in tag_store.lock().unwrap().as_slice(){
                if tag_from_store.tag_name.contains(tag.0) {
                    argument.push(tag_from_store.exif_arg.replace("xxx", tag.1));
                }
            }
        }
    }
    //println!("{:?}", argument);
    
    let output = Command::new("exiftool").args(argument).arg(image["original_path"].clone()).status().expect("exiftool failed");
    return output.code().unwrap();
}

