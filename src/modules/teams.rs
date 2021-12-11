use serde::Deserialize;
use super::csv::{read_csv};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct TeamTable {
    record_array: Vec<Vec<String>>,
}

fn make_team_table() -> TeamTable {
    let records =
        read_csv(Path::new("/home/ykanya/devel/tools/baseball/work/baseballdatabank-master/core/Teams.csv")).unwrap();
    TeamTable {
        record_array: records
            .iter()
            .map(|r| r.iter().map(|item| item.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }
}

pub fn process_teams() {
    let teamTable = make_team_table();


    ()
}

impl TeamTable {}
