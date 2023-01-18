use console::{self, Alignment, Term};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use metro_ticket::{metro::Metro, ticket::Ticket};

fn main() {
    let metro = Metro::new();

    let stations = &metro.all_stations;

    let term = Term::stdout();

    loop {
        let start = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your start")
            .default(0)
            .items(stations)
            .interact()
            .unwrap();

        let destination = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your destination")
            .default(0)
            .items(stations)
            .interact()
            .unwrap();

        term.clear_screen().unwrap();

        let (ticket, path, lines) =
            metro.calculate_ticket(&stations[start], &stations[destination]);

        let formatted_ticket = match ticket {
            Ticket::Yellow => console::style(format!("{:?} Ticket", ticket))
                .white()
                .on_yellow()
                .to_string(),
            Ticket::Green => console::style(format!("{:?} Ticket", ticket))
                .white()
                .on_green()
                .to_string(),

            Ticket::Purple => console::style(format!("{:?} Ticket", ticket))
                .white()
                .on_magenta()
                .to_string(),
        };

        println!(
            "\n\n\n{}",
            console::pad_str(
                &formatted_ticket,
                term.size().1.into(),
                Alignment::Center,
                None,
            )
        );

        println!(
            "{}",
            console::pad_str(
                &format!("{} Station(s)", path.len() - 1),
                term.size().1.into(),
                Alignment::Center,
                None
            )
        );
        println!(
            "{}",
            console::pad_str(
                &format!("{} Line(s)", lines),
                term.size().1.into(),
                Alignment::Center,
                None
            )
        );

        println!(
            "\n{}\n",
            console::pad_str("Path", term.size().1.into(), Alignment::Center, None)
        );

        let path = path.join(" -> ");
        println!("{}", path);

        println!("\n\nAgain? (y/n)");

        if term.read_char().unwrap().to_lowercase().next().unwrap() != 'y' {
            break;
        }
    }
}
