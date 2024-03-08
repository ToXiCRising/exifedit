use std::fs::create_dir;
use std::path::{PathBuf, Path};
use std::sync::{Arc, Mutex};
use std::thread;
use native_dialog::FileDialog;
use image;
use image::GenericImageView;

use crate::standard_values;

pub fn open_file_selector() -> Vec<PathBuf> {
    let paths = FileDialog::new()
    .set_location("~/Desktop")
    .add_filter("JPEG Image", &["jpg", "jpeg"])
    .add_filter("PNG Image", &["png"])
    .show_open_multiple_file()
    .unwrap_or_else(|error| {
        panic!("{:?}", error);
    });
    

    println!("Selected files:");
    for path in paths.iter(){
        println!("\t- {}", path.display());
    }

    return paths;
}

pub fn generate_previews(original_image_paths: &Vec<PathBuf>) -> Vec<PathBuf>{
    //TODO: Multithread that sucker?
    
    println!("\nGenerating Previews!");

    let mut preview_image_paths: Vec<PathBuf> = vec![];
    //let preview_dir = "exif_previews";

    let _res = create_dir(standard_values::PREVIEW_DIR).unwrap_or_else(|error| {
        println!("{}", error);
    });

    for image in original_image_paths.iter(){
        let img = image::open(image).unwrap();

        let temp_name = standard_values::PREVIEW_DIR.to_string() + "/temp_" + image.file_name().unwrap().to_str().unwrap();
        let out_path = Path::new(&temp_name);
        
        println!("\toriginal dimensions: {:?}", img.dimensions());
        println!("\tpreview file: {:?}", out_path);
        let img_smol = img.resize(512, 512, image::imageops::Nearest);
        
        img_smol.save(out_path).unwrap();
        preview_image_paths.append(&mut vec![out_path.to_path_buf()]);
    }

    return preview_image_paths;
}

pub fn generate_preview_paths(original_image_paths: &Vec<PathBuf>) -> Vec<PathBuf> {

    println!("\nGenerating Previews!");

    let mut preview_image_paths: Vec<PathBuf> = vec![];
    //let preview_dir = "exif_previews";

    for image in original_image_paths.iter(){

        let temp_name = standard_values::PREVIEW_DIR.to_string() + "/temp_" + image.file_name().unwrap().to_str().unwrap();
        let out_path = Path::new(&temp_name);
        //println!("\tpreview file: {:?}", out_path);
        
        preview_image_paths.append(&mut vec![out_path.to_path_buf()]);
    }

    gp_mt(&original_image_paths, &preview_image_paths);

    return preview_image_paths;
}

fn gp_mt(original_image_paths: &Vec<PathBuf>, preview_image_paths: &Vec<PathBuf>){

    let _res = create_dir(standard_values::PREVIEW_DIR).unwrap_or_else(|error| {
        println!("{}", error);
    });

    let oip = Arc::new(Mutex::new(original_image_paths.clone()));
    let pip = Arc::new(Mutex::new(preview_image_paths.clone()));
    let mut handles = vec![];

    for i in 0..original_image_paths.len(){
        let oip_handle = Arc::clone(&oip);
        let pip_handle = Arc::clone(&pip);
        let handle = thread::spawn(move || {
            let p = oip_handle.lock().unwrap()[i].clone();
            let img = image::open(&p).unwrap();
            //println!("\toriginal dimensions: {:?}", img.dimensions());
            let img_smol = img.resize(512, 512, image::imageops::Nearest);
            img_smol.save(&pip_handle.lock().unwrap()[i]).unwrap();
            println!("\tgenerated preview file at: {:?}", p);
        });
        handles.push(handle);
    }

    for h in handles{
        h.join().unwrap();
    }
}