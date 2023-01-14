use metro_ticket::metro::Metro;

fn main() {
    let metro = Metro::new();

    let (ticket, number_of_stations, number_of_lines) =
        metro.calculate_ticket("Rod Al-Farag Corridor", "Helwan");

    dbg!(&ticket);
    dbg!(&number_of_stations);
    dbg!(&number_of_lines);
}
