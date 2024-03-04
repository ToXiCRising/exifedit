use native_dialog::FileDialog;
use std::path::PathBuf;
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
    
    let preview_image_paths: Vec<PathBuf> = vec![];

    for image in original_image_paths.iter(){
        let img = image::open(image).unwrap();
        let out_path = "temp_".to_string() + image.file_name().unwrap().to_str().unwrap();
        println!("dimensions {:?}", img.dimensions());
        let img_smol = img.resize(512, 512, image::imageops::Nearest);
        img_smol.save("test.png").unwrap();
    }

    return preview_image_paths;
}
