use cli_clipboard;
use colored::Colorize;
use rand::Rng;

fn main() {
    let photo_names = [
        "cat_bunk_beds.jpg",
        "cat_hens.jpg",
        "cat_snuggles_napping.jpg",
        "cat_car_sleep.jpg",
        "cat_lot_on_my_mlnd.jpg",
        "cat_star.jpg",
        "cat_cards.png",
        "cat_money.png",
        "cat_sunglasses.jpg",
        "cat_cartoon_bomb.jpg",
        "cat_pope.jpg",
        "crow.jpg",
        "cat_cowboy.jpg",
        "cat_punch.jpg",
        "dog.jpg",
        "cat_cowboy_scream.png",
        "cat_scream.png",
        "duck.png",
        "cat_eyes.png",
        "cat_scream_floor.jpg",
        "cat_gang.jpg",
        "cat_showering.png",
    ];

    let mut rng = rand::thread_rng();

    let photo_name = photo_names[rng.gen_range(0..photo_names.len())];
    let url = format!(
        "https://raw.githubusercontent.com/bryce-n-dev/gato/main/assets/cats/{}",
        photo_name
    );

    cli_clipboard::set_contents(url.to_owned()).expect("Oh no! Something went wrong.");
    println!(
        "🐈 {} Cat photo URL successfully copied to clipboard.",
        "Success!".green()
    );
}
