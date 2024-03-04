use native_dialog::FileDialog;
use std::fs::create_dir;
use std::path::{PathBuf, Path};
use image;
use image::GenericImageView;

pub fn open_file_selector() -> Vec<PathBuf> {
    let paths = FileDialog::new()
    .set_location("~/Desktop")
    .add_filter("JPEG Image", &["jpg", "jpeg"])
    .add_filter("PNG Image", &["png"])
    .show_open_multiple_file()
    .unwrap_or_else(|error| {
        panic!("{:?}", error);
    });
    

    for path in paths.iter(){
        println!("{}", path.display());
    }

    return paths;
}

pub fn generate_previews(original_image_paths: &Vec<PathBuf>) -> Vec<PathBuf>{
    //todo!("Generation of previews is not implemented yet");
    
    let mut preview_image_paths: Vec<PathBuf> = vec![];
    let preview_dir = "exif_previews";

    let _res = create_dir(preview_dir).unwrap_or_else(|error| {
        println!("{}", error);
    });

    for image in original_image_paths.iter(){
        let img = image::open(image).unwrap();

        let temp_name = preview_dir.to_string() + "/temp_" + image.file_name().unwrap().to_str().unwrap();
        let out_path = Path::new(&temp_name);
        
        println!("{:?}", out_path);
        println!("dimensions {:?}", img.dimensions());
        let img_smol = img.resize(512, 512, image::imageops::Nearest);
        
        img_smol.save(out_path).unwrap();
        preview_image_paths.append(&mut vec![out_path.to_path_buf()]);
    }

    return preview_image_paths;
}

