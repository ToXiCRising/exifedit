use std::collections::HashMap;
use std::sync::Mutex;
use std::path::PathBuf;

use crate::tag_store;

#[derive(Clone, Debug)]
pub struct DataHandler {
    pub currently_selcted: usize,
    pub images: Vec<HashMap<String, String>>
}

impl DataHandler {

    /// Iterates through all
    pub fn update_data_handler_tags(&mut self, tag_store: &Mutex<Vec<tag_store::Tag>>) {
        let cur_tags = tag_store.lock().unwrap();
        for image in &mut self.images{
            for tag in cur_tags.iter() {
                if !image.contains_key(&tag.tag_name){
                    image.insert(tag.tag_name.clone(), tag.default_value.clone());
                }
            }
        }
    }

    pub fn add_new_images(&mut self, new_image_paths: &mut Vec<PathBuf>, new_previews_paths: &mut Vec<PathBuf>) {
        let new_images_count: usize = new_image_paths.len();

        if new_images_count == 0 {
            return;
        }

        for i in 0..new_image_paths.len(){
            let mut ni = create_image();
            //so ne id zu vergegbn funktioniert nicht wenn jemals bilder entfernt werden sollen
            ni.insert("ID".to_string(), (self.images.len() + i).to_string());
            ni.insert("original_path".to_string(),  new_image_paths[i].to_str().unwrap().to_string());
            ni.insert("preview_path".to_string(), new_previews_paths[i].to_str().unwrap().to_string());
            self.images.push(ni);
        }
    }

    pub fn get_noi(&mut self) -> usize{
        return self.images.len();
    }
    
}


pub fn create_image_handler() -> Vec<HashMap<String, String>> {
    let foo: HashMap<String, String> = HashMap::new();
    //foo.insert("".to_string(), "".to_string());
    let mut f: Vec<HashMap<String, String>> = Vec::new();
    f.push(foo);
    return f;
}

pub fn create_image() -> HashMap<String, String> {
    let mut image: HashMap<String, String> = HashMap::new();
    image.insert("ID".to_string(), "".to_string());
    image.insert("original_path".to_string(), "".to_string());
    image.insert("preview_path".to_string(), "".to_string());

    return image;
}

pub fn remove_image(id: String) {
    //TODO
    todo!("Implement this sucker")
}

fn updated_image_tags(image: &mut HashMap<String, String>, tag_store: &Vec<tag_store::Tag>) {
    for tag in tag_store.iter() {
        if !image.contains_key(&tag.tag_name){
            image.insert(tag.tag_name.clone(), tag.default_value.clone());
        }
    }
    //TODO: dont just add but also remove tags
}

pub fn print_image_tags(image: &HashMap<String, String>){
    for key in image.keys(){ 
        println!("{key}");
    }
}
