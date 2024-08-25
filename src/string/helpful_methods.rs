fn main() {
    // ANCHOR: as_str
    let string = String::from("Hello World");
    let slice = string.as_str();
    // ANCHOR_END: as_str

    // ANCHOR: to_string
    let slice = "Hello World";
    let string = slice.to_string();
    // ANCHOR_END: to_string

    // ANCHOR: format
    let name = "Hashirama";
    let sentence = format!("I've been waiting for you, {}!", name);
    println!("{sentence}");
    // ANCHOR_END: format

    // ANCHOR: replace
    let mut sentence = "mife is movemy";
    let new_sentence =  sentence.replace("m", "l");
    println!("{new_sentence}");
    // ANCHOR_END: replace
}