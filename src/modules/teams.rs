use serde::Deserialize;
use super::csv::{read_csv};
use std::path::Path;
use crate::modules::csv::takeColumns;

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
    println!("{:?}", teamTable.year_array());
    let teamColumns = takeColumns(teamTable.record_array, vec![0, 2, 1, 6, 7, 12, 13]);
    let teamsScores =
        teamColumns
            .iter()
            .filter(|line| (*line).get(0).unwrap().parse::<i32>().unwrap() > 2000)
            .collect::<Vec<_>>();
    println!("{:?}", teamsScores);
    ()
}

impl TeamTable {
    pub fn year_array(&self) -> Vec<i32> {
        self.record_array.iter().map(|r| r.get(0).unwrap().parse().unwrap()).collect::<Vec<_>>()
    }

    pub fn team_array(&self) -> Vec<String> {
        self.record_array.iter().map(|r| r.get(2).unwrap().to_string()).collect::<Vec<_>>()
    }

    pub fn league_array(&self) -> Vec<String> {
        self.record_array.iter().map(|r| r.get(1).unwrap().to_string()).collect::<Vec<_>>()
    }

    pub fn games_array(&self) -> Vec<i32> {
        self.record_array.iter().map(|r| r.get(6).unwrap().parse().unwrap()).collect::<Vec<_>>()
    }

    pub fn wins_array(&self) -> Vec<i32> {
        self.record_array.iter().map(|r| r.get(6).unwrap().parse().unwrap()).collect::<Vec<_>>()
    }

    pub fn loses_array(&self) -> Vec<i32> {
        self.record_array.iter().map(|r| r.get(7).unwrap().parse().unwrap()).collect::<Vec<_>>()
    }

    pub fn runs_array(&self) -> Vec<i32> {
        self.record_array.iter().map(|r| r.get(12).unwrap().parse().unwrap()).collect::<Vec<_>>()
    }

    pub fn runs_allowed_array(&self) -> Vec<i32> {
        self.record_array.iter().map(|r| r.get(13).unwrap().parse().unwrap()).collect::<Vec<_>>()
    }
}
