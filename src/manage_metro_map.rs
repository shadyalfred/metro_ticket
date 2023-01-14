use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::{fs::File, io::BufWriter};

use petgraph::{graph::NodeIndex, Graph, Undirected};

pub const LINE1: [&str; 35] = [
    "New El-Marg",
    "El-Marg",
    "Ezbet El-Nakhl",
    "Ain Shams",
    "El-Matareyya",
    "Helmeyet El-Zaitoun",
    "Hadayeq El-Zaitoun",
    "Saray El-Qobba",
    "Hammamat El-Qobba",
    "Kobri El-Qobba",
    "Manshiet El-Sadr",
    "El-Demerdash",
    "Ghamra",
    "Al Shohadaa",
    "Urabi",
    "Naser",
    "Sadat",
    "Saad Zaghloul",
    "AlSayyeda Zeinab",
    "El-Malek El-Saleh",
    "Mar Girgis",
    "El-Zahraa'",
    "Dar El-Salam",
    "Hadayeq El-Maadi",
    "Maadi",
    "Thakanat El-Maad",
    "Tora El-Balad",
    "Kozzika",
    "Tora El-Asmant",
    "El-Maasara",
    "Hadayeq Helwan",
    "Wadi Hof",
    "Helwan University",
    "Ain Helwan",
    "Helwan",
];

pub const LINE2: [&str; 20] = [
    "Shobra El Kheima",
    "Koliet El-Zeraa",
    "Mezallat",
    "Khalafawy",
    "Sainte Teresa",
    "Road El-Farag",
    "Massara",
    "Al Shohadaa",
    "Ataba",
    "Naguib",
    "Sadat",
    "Opera",
    "Dokki",
    "El Behoos",
    "Cairo University",
    "Faisal",
    "Giza",
    "Omm el Misryeen",
    "Sakiat Mekki",
    "El Mounib",
];

pub const LINE3: [&str; 28] = [
    "Adly Mansour",
    "Hikestep",
    "Omar ibn Al-khattab",
    "Kebaa",
    "Hisham Barakat",
    "El-Nozha",
    "El-Shams Club",
    "Alf Masken",
    "Heliopolis Square",
    "Haroun",
    "Al Ahram",
    "Koleyet El Banat",
    "Cairo Stadium",
    "Fair Zone",
    "Abbassia",
    "Abdou Pasha",
    "El Geish",
    "Bab El Shaaria",
    "Ataba",
    "Naser",
    "Maspero",
    "Zamalek",
    "Kit Kat",
    "Al-Tawfikya",
    "Wadi Al-Nile",
    "Gamaet Al-Dowal",
    "Bolak Al-Dakror",
    "Cairo University",
];

pub const LINE3_HAROUN_EXTENSION: [&str; 5] = [
    "Al-Hegaz Square",
    "Al-Hegaz 2",
    "Military Academy",
    "Sheraton",
    "Airport",
];

pub const LINE3_KIT_KAT_EXTENSION: [&str; 6] = [
    "Sudan",
    "Imbaba",
    "Al-Bohy",
    "Al-Kawmeiah",
    "Ring Road",
    "Rod Al-Farag Corridor",
];

pub const TRANSFER_STATIONS: [&str; 5] =
    ["Al Shohadaa", "Sadat", "Ataba", "Naser", "Cairo University"];

pub fn get_metro_map() -> (Graph<String, (), Undirected>, HashMap<String, NodeIndex>) {
    let graph_bin_file = include_bytes!("../data/graph.bin");
    let graph: Graph<String, (), Undirected> = bincode::deserialize(graph_bin_file).unwrap();

    let hashmap_bin_file = include_bytes!("../data/station_node_map.bin");
    let station_node_map: HashMap<String, NodeIndex> =
        bincode::deserialize(hashmap_bin_file).unwrap();

    return (graph, station_node_map);
}

pub fn get_station_line_map() -> HashMap<String, u8> {
    return bincode::deserialize(include_bytes!("../data/station_line_map.bin")).unwrap();
}

#[allow(dead_code)]
pub fn generate_metro_map() {
    let mut graph = Graph::<String, (), Undirected>::new_undirected();

    let mut station_node_map = HashMap::<String, NodeIndex>::new();

    populate_line(&mut graph, &LINE1, &mut station_node_map);
    populate_line(&mut graph, &LINE2, &mut station_node_map);
    populate_line(&mut graph, &LINE3, &mut station_node_map);

    populate_line_from(
        &mut graph,
        &LINE3_HAROUN_EXTENSION,
        &mut station_node_map,
        "Haroun".to_string(),
    );
    populate_line_from(
        &mut graph,
        &LINE3_KIT_KAT_EXTENSION,
        &mut station_node_map,
        "Kit Kat".to_string(),
    );

    let dir = Path::new("data");

    bincode::serialize_into(
        BufWriter::new(File::create(dir.join("graph.bin")).unwrap()),
        &graph,
    )
    .unwrap();

    bincode::serialize_into(
        BufWriter::new(File::create(dir.join("station_node_map.bin")).unwrap()),
        &station_node_map,
    )
    .unwrap();

    let mut station_line_map: HashMap<String, u8> = HashMap::new();

    populate_station_to_line(&mut station_line_map, &LINE1, 1);
    populate_station_to_line(&mut station_line_map, &LINE2, 2);
    populate_station_to_line(&mut station_line_map, &LINE3, 3);
    populate_station_to_line(&mut station_line_map, &LINE3_HAROUN_EXTENSION, 3);
    populate_station_to_line(&mut station_line_map, &LINE3_KIT_KAT_EXTENSION, 3);

    bincode::serialize_into(
        BufWriter::new(File::create(dir.join("station_line_map.bin")).unwrap()),
        &station_line_map,
    )
    .unwrap();
}

fn populate_station_to_line(
    station_to_line: &mut HashMap<String, u8>,
    line: &[&'static str],
    n: u8,
) {
    let transfer_stations: HashSet<&str> = TRANSFER_STATIONS.iter().copied().collect();

    for station in line {
        if transfer_stations.contains(station) {
            continue;
        }

        station_to_line.insert(station.to_string(), n);
    }
}

fn populate_line(
    graph: &mut Graph<String, (), Undirected>,
    line: &[&'static str],
    station_node_map: &mut HashMap<String, NodeIndex>,
) {
    let mut prev_station: Option<String> = None;

    for curr_station in line {
        let curr_station = curr_station.to_string();

        if !station_node_map.contains_key(&curr_station.to_string()) {
            let node = graph.add_node(curr_station.clone());

            station_node_map.insert(curr_station.clone(), node);
        }

        if let Some(prev_station) = prev_station {
            graph.add_edge(
                *station_node_map.get(&prev_station).unwrap(),
                *station_node_map.get(&curr_station).unwrap(),
                (),
            );
        }

        prev_station = Some(curr_station.clone());
    }
}

fn populate_line_from(
    graph: &mut Graph<String, (), Undirected>,
    line: &[&'static str],
    station_node_map: &mut HashMap<String, NodeIndex>,
    from_station: String,
) {
    let mut prev_station = from_station;

    for curr_station in line {
        let curr_station = curr_station.to_string();

        if !station_node_map.contains_key(&curr_station.to_string()) {
            let node = graph.add_node(curr_station.clone());

            station_node_map.insert(curr_station.clone(), node);
        }

        graph.add_edge(
            *station_node_map.get(&prev_station).unwrap(),
            *station_node_map.get(&curr_station).unwrap(),
            (),
        );

        prev_station = curr_station.to_string();
    }
}
