use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if the supplied arguments match a version number
    let regex = Regex::new(r"^[\d.]+$").unwrap();
    for arg in args.iter().skip(1) {
        if !regex.is_match(arg) {
            println!("Invalid version number found: {}. Only integers and periods are allowed. Ex. 5.3.14", arg);
            return
        }
    }

    // Ensure correct number of arguments provided
    if args.len() != 3 {
        println!("Usage: versionbump SEARCH_STRING REPLACE_STRING");
        return;
    }

    let search_string = &args[1];
    let replace_string = &args[2];

    // Define file paths
    let files = [
        "plugins/bringatrailer/bringatrailer.php", 
        "plugins/bringatrailer/components/action/ajax/rendering/chart/chart.page.php", 
        "plugins/bringatrailer/constants/configuration/site.php", 
        "themes/bringatrailer/style.css", 
        "themes/bringatrailer/pcss/utilities/variables/_misc.pcss"]
    ;

    // Iterate over files and replace search string with replace string
    for file in files.iter() {
        let mut f = File::open(file).expect("Failed to open file");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect("Failed to read file");

        let modified_contents = contents.replace(search_string, replace_string);

        let mut f = File::create(file).expect("Failed to create file");
        f.write_all(modified_contents.as_bytes()).expect("Failed to write to file");
    }

    // Run git diff command
    let output = Command::new("git")
        .arg("diff")
        .output()
        .expect("Failed to run git diff command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
