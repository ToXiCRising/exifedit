use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct DataHandler {
    pub image_paths: Vec<PathBuf>,
    pub curretly_selected: usize,
}
impl DataHandler {
    pub fn add_image_paths(&mut self, new_image_paths: &mut Vec<PathBuf>) {
        self.image_paths.append(new_image_paths); 
    }

    pub fn set_currently_selected(&mut self, cur: usize){
        self.curretly_selected = cur;
    }
}