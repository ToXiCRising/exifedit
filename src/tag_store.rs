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
        exif_arg: "-make=xxx".to_string(),
    });
    tag_store.push(Tag{
        tag_name: "Model".to_string(),
        default_value: standard_values::MODEL_DEFAULT.to_string(),
        exif_arg: "-model=xxx".to_string(),
    });
    tag_store.push(Tag{
        tag_name: "Lens".to_string(),
        default_value: standard_values::LENS_DEFAULT.to_string(),
        exif_arg: "-lens=xxx".to_string(),
    });
    tag_store.push(Tag{
        tag_name: "Focal Length".to_string(),
        default_value: standard_values::FOCAL_LENGTH.to_string(),
        exif_arg: "-focallength=xxx".to_string(),
    });
    tag_store.push(Tag{
        tag_name: "ISO".to_string(),
        default_value: standard_values::ISO_DEFAULT.to_string(),
        exif_arg: "-iso=xxx".to_string(),
    });
    tag_store.push(Tag{
        tag_name: "Aperture".to_string(),
        default_value: standard_values::APERTURE_DEFAULT.to_string(),
        exif_arg: "-aperturevalue=xxx".to_string(), // -Fnumber=xxx
    });
    tag_store.push(Tag{
        tag_name: "Shutter Speed".to_string(),
        default_value: standard_values::SHUTTER_SPEED_DEFAULT.to_string(),
        exif_arg: format!("-exposuretime={}", "xxx").to_string(),
    });



    return tag_store;
}

pub fn add_tag_to_store(tag: Tag) {

}

pub fn remove_tag_from_store(tag: Tag) {
    
}