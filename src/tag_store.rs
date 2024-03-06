use crate::standard_values;
pub struct Tag {
    pub tag_name: String,
    pub default_value: String,
    pub exif_arg: String,
}

pub fn create_default_tag_store() -> Vec<Tag> {
    let mut tag_store: Vec<Tag> = vec![];

    tag_store.push(Tag{
        tag_name: "Manufacturer".to_string(),
        default_value: standard_values::MANUFACTURER_DEFAULT.to_string(),
        exif_arg: "-make=\"xxx\"".to_string(),
    });
    tag_store.push(Tag{
        tag_name: "Lens".to_string(),
        default_value: standard_values::MODEL_DEFAULT.to_string(),
        exif_arg: "-model=\"xxx\"".to_string(),
    });


    return tag_store;
}

pub fn add_tag_to_store(tag: Tag) {

}

pub fn remove_tag_from_store(tag: Tag) {
    
}