use std::env;
use std::fs::read_to_string;

fn parse_markdown(file_path: String) -> String {
    let contents = read_to_string(file_path).expect("file");
    contents
}

fn count_char(line: &String, character: char) -> usize {
    let mut count = 0;
    for char_ in line.chars(){
        if char_ == character{
            count += 1;
        }
    }
    count
}


fn format_string(heading: &String) -> String {
    let symbols = vec!["+","-> ", ",", ".", "/", ":", "(", ")", "$"];
    let mut heading = heading.trim().to_lowercase();
    
    for symbol in &symbols {
        heading = heading.replace(symbol, "");
    }
    
    heading.replace(" ", "-").to_string()
}

fn parse_line(line: String) -> Option<String> {
    let type_heading = count_char(&line, '#');
    if type_heading == 1 { 
        return None
    }

    let mut heading = line.replace("#", "");
    heading.remove(0);
    let indents = " ".repeat((type_heading-2)*2);
    
    let formated_string = format_string(&heading);

    Some(format!("{indents}- [{heading}](#{formated_string})"))
}

fn generate_header() {
    println!("<details>");
    println!("<summary><span style='font-weight:bold; font-size: 1.4rem;'>Contents</span></summary>");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Opening {file_path}");
    
    generate_header();

    let file_contents = parse_markdown(file_path.to_string());
    
    for line in file_contents.split("\n") {
        if let Some('#') = line.chars().next() {
            match parse_line(line.to_string()){
                Some(value) => println!("{}", value),
                _ => ()
            }
        }  
    }

    println!("</details>");
}
