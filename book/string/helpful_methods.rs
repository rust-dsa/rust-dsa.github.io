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

    // ANCHOR: to_vector
    let my_str: &str = "123456789";
    let char_vec: Vec<char> = my_str.chars().collect();
    // ANCHOR_END: to_vector

    // ANCHOR: nth
    let my_str: &str = "Secret no: 4";
    let num = my_str.chars().nth(11).unwrap();
    println!("{num}");
    // ANCHOR_END: nth

    // ANCHOR: split
    let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);
    // ANCHOR_END: split
}