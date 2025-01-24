use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use reqwest::blocking::get;
use anyhow::Result;

#[derive(Debug, Deserialize)]
struct Data {
    repositories: HashMap<String, String>,
}

// Function to check if working directory user chrome folder

// Confirmation if user wants to overwrite the current userChrome

//Opens the url path and gets the content
fn fetch_website_content(url: &str) -> Result<String, reqwest::Error> {
    let response = get(url)?;
    let content = response.text()?;
    Ok(content)
}

fn create_user_chrome_file(chrome_content: &str) {
    match fs::write("userChrome.css", chrome_content) {
        Ok(_) => println!("Created userChrome.css"),
        Err(_) => eprintln!("Error writing userChrome.css")
    }
}

// Reads the repositories in the json file and for each repositorie to then use it later to put into a userChrome.css
fn read_repositories(repositories_json: &str)  -> String{
    // Turns the json file into a string
    let repositories = fs::read_to_string(repositories_json)
        .expect("content from the repositories json as a string");
     
    // Parse the json to a Struct
    let data: Data = serde_json::from_str(&repositories).expect("Error parsing JSON");

    // Stores all of the repositories css into a single variable
    let mut user_chrome_content = String::new();

    // For each repository copy the content and put it into the user_chrome_content
    for (_name, url) in &data.repositories {
        match fetch_website_content(url){
            Ok(css_content ) =>{
                user_chrome_content.push_str(&css_content);
            }
            Err(e) => {
                eprintln!("Error fetching website content: {}", e);
            }
        }        
    }
    user_chrome_content
}

fn main() {
    // The location of the repositories json file
    let repositories_json = "/Users/mstui/Code/zen-browser-theme-importer/src/userChrome.json"; // userChrome.json
    // For debugging: Use the absolute location of json file
    // Otherwise use the relative location. 

    // For each repository put the content of the chrome into one variable
    let chrome_content = read_repositories(repositories_json);
    
    // Creates the final userChrome.css
    create_user_chrome_file(&chrome_content);
}