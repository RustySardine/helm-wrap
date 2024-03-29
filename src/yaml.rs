use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::io::{BufRead, BufReader,Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Releases {
    pub releases: Vec<Release>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Release {
    pub name: String,
    pub chart: String,
    pub namespace: String,
    pub needs: Vec<String>,
    pub values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repositories {
    pub repositories: Vec<Repository>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Repository {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Helmfile {
    repositories: Repositories,
    releases: Releases,
}

impl Helmfile {
    pub fn new() -> Self{
        Helmfile {
            releases: Releases::new(),
            repositories: Repositories::new(),
        }
    }
    fn cli_add_release(&mut self){
        self.releases.add_release(release_menu());
    }
    fn cli_add_repository(&mut self){
        self.repositories.add_repository(repository_menu());
    }
    fn print(&self){
        for repo in &self.repositories.repositories{
            repo.print();
        }
        for release in &self.releases.releases{
            release.print();
        }
    }
}
impl Release {
    fn print(&self){
        println!("Release -- Name: {} -- Chart: {} -- Namespace {}",self.name,self.chart,self.namespace);
    }
}
impl Repository{
    fn print(&self){
        println!("Repository -- Name: {} -- Url: {}",self.name,self.url);
    }
}
impl Releases {
    pub fn new() -> Self {
        Releases {
            releases: Vec::new(),
        }
    }
    
    pub fn add_release(&mut self, release: Release) {
        self.releases.push(release);
    }
}

impl Repositories {
    pub fn new() -> Self {
        Repositories {
            repositories: Vec::new(),
        }
    }
    pub fn add_repository(&mut self, repository: Repository) {
        self.repositories.push(repository);
    }
}
pub fn to_file(helmfile: &Helmfile){
    let separator = "---\n".as_bytes();
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("helmfile.yaml")
        .expect("Could not open new file");
    serde_yaml::to_writer(&file, &helmfile.repositories).unwrap();
    let _ = &file.write_all(separator);
    serde_yaml::to_writer(&file, &helmfile.releases).unwrap();
}
pub fn menu() -> i32 {
    println!("-----------------");
    println!("Helm-Wrap Menu\n1) Add a new release\n2) Add a Repository\n3) Print Releases\n4) Finalise choices");
    println!("-----------------");
    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    println!("{line}");
    line.truncate(line.len()-2);
    line.parse::<i32>().unwrap()
}
fn release_menu()-> Release{
    let mut release = Release::default();
    let mut need = String::new();
    println!("-----------------");
    println!("What is the release name?");
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut release.name).unwrap();
    
    println!("What is the chart? (Format bitnami/kafka for example");
    stdin.lock().read_line(&mut release.chart).unwrap();
    
    println!("And the namespace?");
    stdin.lock().read_line(&mut release.namespace).unwrap();

    println!("Does it have a need? (in namespace/name format)");
    stdin.lock().read_line(&mut need).unwrap();
    need.truncate(need.len()-2);


    release.name.truncate(release.name.len()-2);
    release.namespace.truncate(release.namespace.len()-2);
    release.chart.truncate(release.chart.len()-2);
    release.needs.push(need);
    release
    
}
fn repository_menu()-> Repository{
    let mut repo = Repository::default();
    println!("-----------------");
    println!("What is the repo name?");
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut repo.name).unwrap();
    repo.name.truncate(repo.name.len()-2);
    println!("What is the url?");
    stdin.lock().read_line(&mut repo.url).unwrap();
    repo.url.truncate(repo.url.len()-2);
    repo
    
}

pub fn input_to_helmfile()-> Helmfile {
    let mut helmfile = Helmfile::new();
    loop {
        match menu(){
            1 => helmfile.cli_add_release(),
            2 => helmfile.cli_add_repository(),
            3 => helmfile.print(),
            4 => break,
            _ => {
                println!("Not a valid input, try again");

            }
        }
    } 
    helmfile
}
pub fn file_to_helmfile()-> Helmfile{
    let file = std::fs::OpenOptions::new().read(true).open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut release = Release::default();
    let mut helmfile = Helmfile::new();
    for line in reader.lines() {
        let split = line.unwrap();
        let split = split.split_whitespace();
        let pair = split.collect::<Vec<&str>>();
        match pair.first() {
            Some(&"name") => release.name = (pair[1]).to_string(),
            Some(&"chart") => release.chart = (pair[1]).to_string(),
            Some(&"namespace") => release.namespace = (pair[1]).to_string(),
            Some(&"needs") => {
                for need in pair.iter().skip(1) {
                    release.needs.push((*need).to_string());
                }
            }
            Some(&"values") => {
                for value in pair.iter().skip(1) {
                    release.values.push((*value).to_string());
                }
            }
            Some(&"repository") => helmfile.repositories.add_repository(Repository {
                name: pair[1].to_string(),
                url: pair[2].to_string(),
            }),
            Some(&"---") => {
                helmfile.releases.add_release(release);
                release = Release::default();
            }
            None => println!("Pair not found"),
            _ => println!("Unidentified key in pair"),
        }
    }
    helmfile
}
