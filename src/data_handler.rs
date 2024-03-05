use std::path::PathBuf;

#[path = "standard_values.rs"] mod standard_values;

#[derive(Clone, Debug)]
pub struct DataHandler {
    pub image_paths: Vec<PathBuf>,
    pub preview_paths: Vec<PathBuf>,
    pub currently_selected: usize,

    pub camera_names: Vec<String>,
    pub lens_names: Vec<String>,
    pub focal_length: Vec<String>,
    pub iso: Vec<String>,
    pub aperture: Vec<String>,
    pub shutter_speed: Vec<String>,
}

impl DataHandler {
    pub fn add_new_images(&mut self, new_image_paths: &mut Vec<PathBuf>, new_previews_paths: &mut Vec<PathBuf>) {
        let new_images_count: usize = new_image_paths.len();

        if new_images_count == 0 {
            return;
        }

        self.image_paths.append(new_image_paths); 
        self.preview_paths.append(new_previews_paths);
        
        for _i in  0..new_images_count{
            //println!("Appending Stuff");
            self.camera_names.append(&mut vec![standard_values::CAMERA_DEFAULT.to_string()]);
            self.lens_names.append(&mut vec![standard_values::LENS_DEFAULT.to_string()]);
            self.focal_length.append(&mut vec![standard_values::FOCAL_LENGTH.to_string()]);
            self.iso.append(&mut vec![standard_values::ISO_DEFAULT.to_string()]);
            self.aperture.append(&mut vec![standard_values::APERTURE_DEFAULT.to_string()]);
            self.shutter_speed.append(&mut vec![standard_values::SHUTTER_SPEED_DEFAULT.to_string()]);
        }
    }
    ///NOI: number of images 
    ///
    ///Returns the number of curently tracked images, by returning the length of the image_paths vector
    pub fn get_noi(&mut self) -> usize{
        return self.image_paths.len();
    }
}