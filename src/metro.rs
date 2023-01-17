use std::collections::{HashMap, HashSet};

use petgraph::{algo::astar, graph::NodeIndex, Graph, Undirected};

use crate::{
    manage_metro_map::{self, TRANSFER_STATIONS},
    ticket::Ticket,
};

pub struct Metro {
    metro_graph: Graph<String, (), Undirected>,
    station_node_map: HashMap<String, NodeIndex>,
    station_line_map: HashMap<String, u8>,
    transfer_stations: HashSet<String>,
    pub all_stations: Vec<String>,
}

impl Metro {
    pub fn new() -> Self {
        let (metro_graph, station_node_map) = manage_metro_map::get_metro_map();
        let station_line_map = manage_metro_map::get_station_line_map();
        let transfer_stations: HashSet<String> =
            TRANSFER_STATIONS.iter().map(|s| s.to_string()).collect();

        return Self {
            metro_graph,
            station_node_map,
            station_line_map,
            transfer_stations,
            all_stations: manage_metro_map::get_all_stations(),
        };
    }

    pub fn path_between(&self, start: &str, destination: &str) -> Vec<String> {
        let start = *self.station_node_map.get(start).unwrap();
        let destination = *self.station_node_map.get(destination).unwrap();

        let path = astar(
            &self.metro_graph,
            start,
            |goal| goal == destination,
            |_| 1,
            |_| 0,
        );

        return path
            .unwrap()
            .1
            .iter()
            .map(|n| self.metro_graph.node_weight(*n).unwrap().clone())
            .collect();
    }

    pub fn calculate_ticket(&self, start: &str, destination: &str) -> (Ticket, Vec<String>, u8) {
        let path = self.path_between(start, destination);

        let number_of_stations: u8 = (path.len() - 1).try_into().unwrap();

        let mut number_of_lines = 1;

        let mut prev_line: Option<&u8> = None;
        if !self.transfer_stations.contains(path.first().unwrap()) {
            prev_line = Some(self.station_line_map.get(path.first().unwrap()).unwrap());
        }

        let path_iter = path.iter().skip(1);

        for station in path_iter {
            if self.transfer_stations.contains(station) {
                continue;
            }

            let curr_line = self.station_line_map.get(station).unwrap();

            if prev_line.is_some() && curr_line != prev_line.unwrap() {
                number_of_lines += 1;
            }

            prev_line = Some(curr_line);
        }

        let ticket;

        match number_of_lines {
            1 => {
                if number_of_stations <= 9 {
                    ticket = Ticket::Yellow;
                } else if number_of_stations > 9 && number_of_stations <= 16 {
                    ticket = Ticket::Green;
                } else {
                    ticket = Ticket::Purple;
                }
            }
            2 => {
                if number_of_stations <= 16 {
                    ticket = Ticket::Green
                } else {
                    ticket = Ticket::Purple;
                }
            }
            3 => {
                ticket = Ticket::Purple;
            }
            x => panic!("`number_of_lines` was invalid = {x}"),
        }

        return (ticket, path, number_of_lines);
    }
}
