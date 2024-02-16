use std::path::PathBuf;

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
