fn main() {
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Golden Rings",
        "six Geese a-Laying",
        "seven Swans a-Swimming",
        "eight Maids a-Milking",
        "nine Ladies Dancing",
        "ten Lords a-Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let mut day = 0;
    while day < 12 {
        println!(
            "On the {} day of Christmas, my true love sent to me:",
            days[day]
        );

        // Affichage des cadeaux en ordre inverse
        let mut gift = day;
        while gift >= 0 {
            if day != 0 && gift == 0 {
                print!("and ");
            }
            println!("{}", gifts[gift]);

            if gift == 0 {
                break; // Sortir de la boucle quand gift est à 0 pour éviter un underflow
            }
            gift -= 1;
        }

        println!(); // Ligne vide entre les strophes
        day += 1;
    }
}
