use cli_clipboard;


fn main() {
    let the_string = "";
    cli_clipboard::set_contents(the_string.to_owned()).unwrap();
    println!("🐈 Success! Cat photo URL successfully copied to clipboard.");
}
