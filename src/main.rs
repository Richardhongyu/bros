use std::env;
use serde::{Serialize, Deserialize};
use serde_yaml; 
use git2::{Repository, build};
use std::fs;

// fn build_command(config: &str) {
//     println!("Building kernel with config {}", config);

//     let repo = match Repository::open("./configs") {
//         Ok(repo) => repo,
//         Err(e) => panic!("failed to open: {}", e),
//     };
// }

// pub(crate) fn run_command(image: &str) {
//     println!("Running kernel image {}", image);

//     let repo = match Repository::init("./images") {
//         Ok(repo) => repo,
//         Err(e) => panic!("failed to init: {}", e),
//     };
// }

fn build_command(target: &str) {
    let (config, image, build, run) = parse_yaml_target(target);
    // run the build command
    println!("Building kernel with config {}", config);
    println!("The build command is {}", build);
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(build)
        .output()
        .expect("failed to execute process");
    println!("The build command output is {}", output.status);
}

pub(crate) fn run_command(target: &str) {
    let (config, image, build, run) = parse_yaml_target(target);
    println!("Running kernel image {}", image);
}

fn pull_config(config: &str) {
    println!("Pulling config {}", config);

    // cp the /etc/bros/configs/$config$ file to the current directory
    let source_config = format!("/etc/bros/configs/{}.config", config);
    let target_config = format!("{}/.config", std::env::current_dir().unwrap().as_os_str().to_str().unwrap());
    println!("The source config is {}", source_config);
    println!("The target config is {}", target_config);
    fs::copy(source_config, target_config).unwrap();
}

fn pull_image(image: &str) {
    println!("Pulling config {}", image);

    // use rust to copy the /etc/bros/configs/$config$ file to the current directory

}

fn use_command(target: &str) {
    println!("Using target {}", target);
    let (config, image, build, run) = parse_yaml_target(target);
    println!("The config is {}", config);
    println!("The image is {}", image);
    println!("The build is {}", build);
    println!("The run is {}", run);

    pull_config(&config);
    pull_image(&image);
}

fn parse_yaml_target(target: &str) -> (String, String, String, String) {
    let f = std::fs::File::open("./bros.yaml").unwrap();
    let d = serde_yaml::from_reader::<std::fs::File, BrosYaml>(f).unwrap();
    println!("The yaml file is {:?}", d);
    let config  = d.targets.iter().find(|&x| x.name == target).unwrap().config.clone();
    let image = d.targets.iter().find(|&x| x.name == target).unwrap().image.clone();
    let build = d.targets.iter().find(|&x| x.name == target).unwrap().build.clone();
    let run = d.targets.iter().find(|&x| x.name == target).unwrap().run.clone();
    (config, image, build, run)
}


#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct BrosYaml {
    tool: String,
    version: String,
    targets: Vec<BrosTarget>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct BrosTarget {
    name: String,
    project: String,
    branch: String,
    arch: String,
    config: String,
    image: String,
    build: String,
    run: String,
}

fn main() {
    println!("*********************************************************************************************************");
    println!("*******************Bros is a tool for building and running an operating system kernel!*******************");
    println!("*********************************************************************************************************\n\n");

    let args: Vec<String> = env::args().collect();

    let subcommand = &args[1];



    println!("The subcommand is {}", subcommand);
    if subcommand == "build" {
        // let config = &args[2];
        // println!("The config is {}", config);
        // build_command(config);
        let target = &args[2].clone();
        build_command(target);
    } else if subcommand == "init" {
        println!("Initializing bros");
        // init the current directory in the /etc/bros directory
        let current_dictory = std::env::current_dir().unwrap();
        println!("The current directory is {:?}", current_dictory);
        if !std::path::Path::new("/etc/bros").exists() {
            std::fs::create_dir_all("/etc/bros").unwrap();
        }
        //create the config directory
        if !std::path::Path::new("/etc/bros/configs").exists() {
            std::fs::create_dir_all("/etc/bros/configs").unwrap();
        }
        //create the images directory
        if !std::path::Path::new("/etc/bros/images").exists() {
            std::fs::create_dir_all("/etc/bros/images").unwrap();
        }
        //create the the bros.yaml file of the current directory
        let mut bros_config_yaml_filepath = String::new();
        bros_config_yaml_filepath.push_str("/etc/bros/configs/");
        bros_config_yaml_filepath.push_str(current_dictory.as_os_str().to_str().unwrap()[1..].replace("/", "_").as_str());
        bros_config_yaml_filepath.push_str("_bros.yaml");
        println!("The bros.yaml file path is {}", bros_config_yaml_filepath);
        if !std::path::Path::new(bros_config_yaml_filepath.as_str()).exists() {
            std::fs::File::create(bros_config_yaml_filepath.as_str()).unwrap();
        }
        //create the the bros.yaml file of the current directory
        let mut bros_img_yaml_filepath = String::new();
        bros_img_yaml_filepath.push_str("/etc/bros/images/");
        bros_img_yaml_filepath.push_str(current_dictory.as_os_str().to_str().unwrap()[1..].replace("/", "_").as_str());
        bros_img_yaml_filepath.push_str("_bros.yaml");
        println!("The bros.yaml file path is {}", bros_img_yaml_filepath);
        if !std::path::Path::new(bros_img_yaml_filepath.as_str()).exists() {
            std::fs::File::create(bros_img_yaml_filepath.as_str()).unwrap();
        }
    } else if subcommand == "list" {
        println!("Listing bros");
    } else if subcommand == "error" {
        println!("Erroring bros");
    } else if subcommand == "run" {
        // let image = &args[2];
        // println!("The image is {}", image);
        // run_command(image);
        let target = &args[2].clone();
        run_command(target);
    } else if subcommand == "help" {
        println!("Usage: bros [init|build|error|help|list|run] [config|image]");
        println!("bros add [config|image|conmbination]]");
        println!("bros init");
        println!("bros build");
        println!("bros delete [config|image|conmbination]]");
        println!("bros error");
        println!("bros help");
        println!("bros list [config|image|conmbination]]");
        println!("bros use");
        println!("bros run");
    } else if subcommand == "use" {
        let use_target = &args[2].clone();
        use_command(use_target);  
    } else {
        println!("Unknown subcommand {}", subcommand);
    }
    // use serde_yaml to deserial read from the yaml file bros.yaml


    // let yaml_str = include_str!("../bros.yaml");
    // // serde_yaml 解析字符串为 User 对象
    // let user: BrosYaml = serde_yaml::from_str(yaml_str)
    //     .expect("app.yaml read failed!");
    // println!("{:?}", user);
}
