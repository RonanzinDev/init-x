use std::env;
use std::fs;
use std::fs::File;
use std::io::{Error, Write};

#[derive(Debug)]
enum Languages {
    JavaScript,
    TypeScript,
    Golang,
}
impl Languages {
    fn match_input(input: &String) -> Option<Self> {
        match input.as_str() {
            "javascript" => Some(Self::JavaScript),
            "typeScript" => Some(Self::TypeScript),
            "golang" => Some(Self::Golang),
            _ => None,
        }
    }
}
fn find_prefix(lang: &Languages) -> &'static str {
    match lang {
        Languages::JavaScript => "js",
        Languages::TypeScript => "ts",
        Languages::Golang => "go",
    }
}
fn which_content(lang: &Languages) -> &'static str {
    match lang {
        Languages::JavaScript => "./examples/example.js",
        Languages::TypeScript => "./examples/example.ts",
        Languages::Golang => "./examples/example.go",
    }
}
fn main() -> Result<(), Error> {
    let mut input = String::new();
    let current_dir = env::current_dir().unwrap();
    println!("Type your language: ");

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed read the input");

    println!("type the file name: ");

    let mut filename = String::new();

    std::io::stdin()
        .read_line(&mut filename)
        .expect("Failed read the input");

    let language = Languages::match_input(&input.trim().to_string());

    if let Some(language) = language {
        let prefix = find_prefix(&language);
        let content = fs::read_to_string(which_content(&language)).expect("Some error occurred");

        //let file = format!("{:?}/{}.{}", current_dir, filename.trim(), prefix);
        let temp_file = current_dir.join(format!("{}.{}", filename.clone().trim(), prefix.clone()));

        let mut output = File::create(temp_file)?;
        output.write(content.as_bytes())?;
    } else {
        println!("This language is not supported yet!!!! Sorry :(")
    }

    Ok(())
}
