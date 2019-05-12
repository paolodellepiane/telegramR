extern crate serde_json;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Deserialize)]
#[serde(tag = "action")]
pub enum Action {
    getFile,
    info { text: String },
}

//fn process(_: &str, _: &mut View) -> Result<String, Box<Error>> {
//    Ok(String::from(""))
//}
