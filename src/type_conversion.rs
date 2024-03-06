use std::path::PathBuf;

use slint::Image;

pub fn string_vec_to_sharedstring_vec(strings: Vec<&str>) -> Vec<slint::SharedString>{
    let mut out: Vec<slint::SharedString> = vec![];
    for s in strings {
        out.push(slint::SharedString::from(s));
    }
    return out;
}
/// Turns the vector of path buffers returned from the open_file_selector-function into
/// the wierd model-thingy slint expects for its own string-arrays.
/// The resulting string-array contains the names of the files
pub fn paths_to_model(paths: Vec<PathBuf>) -> slint::ModelRc<slint::SharedString>{
    
    let mut out: Vec<slint::SharedString> = vec![];
    for file_name in paths {
        let fn_oss = Option::expect(file_name.file_name(), "failed");
        let fn_str: &str = Option::expect(fn_oss.to_str(), "failed");
        
        out.push(slint::SharedString::from(fn_str));
    }
    return slint::ModelRc::new(slint::VecModel::from(out));
}

/// Same as paths_to_nmodel but returns the corresponding model for the images
pub fn images_to_model(paths: Vec<PathBuf>) -> slint::ModelRc<Image>{
    let mut out: Vec<Image> = vec![];
    for path in paths {
        let cur_image = Image::load_from_path(&path);
        let loaded_image = Result::expect(cur_image, "failed");
        out.push(loaded_image);
    }
    return slint::ModelRc::new(slint::VecModel::from(out));
}

pub fn string_to_model(strings: Vec<String>) -> slint::ModelRc<slint::SharedString> {
    let mut out: Vec<slint::SharedString> = vec![];
    for s in strings {
        out.push(slint::SharedString::from(s));
    }
    return slint::ModelRc::new(slint::VecModel::from(out));
}

pub fn split_camera_name(camera_name: String) -> (String, String){
    let it = camera_name.split_whitespace();

    let mut manufacturer: String = "".to_string();
    let mut model: String = "".to_string();


    for (i, part) in it.enumerate() {
        if i == 0 {
            manufacturer = part.to_string();
        } else {
            model = model + part;
        }

    }

    return (manufacturer, model);
}
