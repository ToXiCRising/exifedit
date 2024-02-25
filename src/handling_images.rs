use native_dialog::FileDialog;
use std::path::PathBuf;


pub fn open_file_selector() -> Vec<PathBuf> {
    let paths = FileDialog::new()
    .set_location("~/Desktop")
    .add_filter("PNG Image", &["png"])
    .add_filter("JPEG Image", &["jpg", "jpeg"])
    .show_open_multiple_file()
    .unwrap();
    

    for path in paths.iter(){
        println!("{}", path.display());
    }

    return paths;
}

fn generate_previews(original_image_paths: Vec<PathBuf>) -> (Vec<PathBuf>, Vec<PathBuf>){
    todo!("Generation of previews is not implemented yet");
    
    let preview_image_med_paths: Vec<PathBuf> = vec![];
    let preview_image_smol_paths: Vec<PathBuf> = vec![];

    for image in original_image_paths.iter(){

    }

    return (preview_image_med_paths, preview_image_smol_paths);
}
