extern crate winres;

#[cfg(target_os = "windows")]
fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("icon.ico");
    res.compile().unwrap();
  }
}

#[cfg(not(target_os = "windows"))]

fn main() {

}