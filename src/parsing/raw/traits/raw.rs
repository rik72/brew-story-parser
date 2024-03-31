use super::inner_raws::InnerRaws;

pub trait Raw: InnerRaws {
    fn set_file_name(&mut self, file_name: &String);
    fn get_file_name(&self) -> String;
}
