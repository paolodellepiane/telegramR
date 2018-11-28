use std::{
    env,
    fs::{self, File},
    io::{self, prelude::*, BufReader},
    path::{Path, PathBuf},
};

fn split<P1: AsRef<Path>, P2: AsRef<Path>>(in_file: P1, out_dir: P2) {
    let merged = File::open(in_file.as_ref()).unwrap();
    fs::create_dir_all(out_dir.as_ref()).unwrap();
    let mut reader = BufReader::new(&merged);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.pop();
    let v: Vec<&str> = line.split('\t').collect();
    let out_dir = out_dir.as_ref();
    for file_info in v {
        let namelen: Vec<&str> = file_info.split('|').collect();
        let bytes_to_read: usize = namelen[1].parse().unwrap();
        let mut buffer = vec![0u8; bytes_to_read];
        reader.read_exact(&mut buffer).unwrap();
        let out_dir = out_dir.join(namelen[0]);
        let mut f = File::create(out_dir).unwrap();
        f.write_all(&buffer).unwrap();
    }
}

pub fn unzip<R: Read, P2: AsRef<Path>>(reader: R, out_dir: P2) {
    let mut rdr = lzma::read(reader).unwrap();
    let tmp_file_path = out_dir.as_ref().join("__tmp__");
    let mut wtr = File::create(tmp_file_path.clone()).expect(&format!("can't create {:?}", tmp_file_path));
    io::copy(&mut rdr, &mut wtr).expect("can't decompress view");
    split(tmp_file_path.clone(), out_dir);
    fs::remove_file(tmp_file_path).expect("can't remove tmp file");
}

pub fn unzip_to_tmp<R: Read>(reader: R, dir_name_in_tmp: &str) -> Result<PathBuf, &'static str> {
    let tmp_dir = env::temp_dir().join(dir_name_in_tmp);
    fs::create_dir_all(tmp_dir.as_path()).expect(&format!("can't create tmp dir {:?}", tmp_dir));
    unzip(reader, tmp_dir.clone());
    Ok(tmp_dir)
}
