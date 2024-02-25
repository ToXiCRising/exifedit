use std::path::PathBuf;

#[path = "standard_values.rs"] mod standard_values;

#[derive(Clone, Debug)]
pub struct DataHandler {
    pub image_paths: Vec<PathBuf>,
    pub currently_selected: usize,

    pub camera_names: Vec<String>,
    pub lens_names: Vec<String>,
    pub focal_length: Vec<String>,
    pub iso: Vec<String>,
    pub aperture: Vec<String>,
    pub shutter_speed: Vec<String>,
}

impl DataHandler {
    pub fn add_new_images(&mut self, new_image_paths: &mut Vec<PathBuf>) {
        let new_images_count: usize = new_image_paths.len();
        self.image_paths.append(new_image_paths); 
        
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
}