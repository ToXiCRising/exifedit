use native_dialog::FileDialog;
use std::path::PathBuf;

pub fn open_file_selector() -> PathBuf {
    let paths = FileDialog::new()
    .set_location("~/Desktop")
    .add_filter("PNG Image", &["png"])
    .add_filter("JPEG Image", &["jpg", "jpeg"])
    .show_open_multiple_file()
    .unwrap();
    

    for path in paths.iter(){
        println!("{}", path.display());
    }
    return Option::expect(paths.last().cloned(), "msg");
}

fn generate_previews() {
    
}