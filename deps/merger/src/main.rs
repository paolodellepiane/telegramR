use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(0);
    });
    match cfg.cmd.as_ref() {
        "-m" => merge(cfg.dir_path, cfg.file_path, &cfg.opt_in_extensions),
        "-s" => split(cfg.file_path, cfg.dir_path),
        _ => {
            println!("unknown cmd {}", cfg.cmd);
            process::exit(1);
        }
    }
}

struct Config<'a> {
    cmd: String,
    file_path: String,
    dir_path: String,
    opt_in_extensions: Vec<&'a str>,
}

impl<'a> Config<'a> {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("usage: merger [cmd] [file] [dir] [comma separated opt-in extensions]");
        }
        let cmd = args[1].clone();
        let file_path = args[2].clone();
        let dir_path = args[3].clone();
        let mut opt_in_extensions = vec![];
        if args.len() == 5 {
            opt_in_extensions = args[4].split(',').collect();
        }
        Ok(Config {
            cmd,
            file_path,
            dir_path,
            opt_in_extensions,
        })
    }
}

fn merge<P: AsRef<Path>>(in_dir: P, out_file: P, extensions: &Vec<&str>) {
    let paths: Vec<_> = fs::read_dir(in_dir.as_ref())
        .unwrap()
        .filter_map(|p| p.map(|p| p.path()).ok())
        .filter(|p| extensions.contains(&p.extension().unwrap().to_str().unwrap()))
        .collect();
    out_file
        .as_ref()
        .parent()
        .map(|d| fs::create_dir_all(d).unwrap());
    let mut header = String::new();
    for path in paths.clone() {
        let info = path.metadata().unwrap();
        header.push_str(&format!(
            "{}|{}\t",
            path.file_name().unwrap().to_str().unwrap(),
            info.len()
        ));
    }
    // let paths = fs::read_dir(in_dir.as_ref()).unwrap();
    let mut file = File::create(out_file.as_ref()).unwrap();
    header.pop();
    file.write_all(&format!("{}\n", &header).as_bytes())
        .unwrap();
    for path in paths {
        let mut inp = File::open(path).unwrap();
        std::io::copy(&mut inp, &mut file).unwrap();
    }
}

fn split<P: AsRef<Path>>(in_file: P, out_dir: P) {
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
        println!("writing {}", out_dir.as_path().display());
        let mut f = File::create(out_dir).unwrap();
        f.write_all(&buffer).unwrap();
    }
    println!("{}", &line);
}
