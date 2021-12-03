use std::error::Error;
use std::fs::File;
use csv::StringRecord;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Pitcher {
    record_array: Vec<Vec<String>>,
    // 0 year: i32,  // 年
    // 1 age: i32,  // 年齢
    // 2 team: String,  // チーム
    // 3 league: String,  // リーグ
    // 4 wins: i32,  // 勝利数(W)
    // 5 losses: i32,  // 敗戦数(L)
    // 6 win_rate: f64, // 勝率(W.L)
    // 7 earned_run_average: f64,  // 防御率(ERA)
    // 8 games: i32,  // 試合数(G)
    // 9 games_started: i32,  // 先発数(GS)
    // 10 games_finished: i32,  // 先発を除き、ゲームの最後の投手になった試合数(GF)
    // 11 complete_games: i32,  // 完投数(CG)
    // 12 shutouts: i32, //完封数(SHO)
    // 13 game_saves_as_a_relief_pitcher: i32,  // セーブ数(SV)
    // 14 innings_pitched: f64, // 投球回数(IP)
    // 15 hits: i32,  // 被安打数(H)
    // 16 runs_allowed: i32,  // 失点(R)
    // 17 earned_runs: i32,  // 自責点(ER)
    // 18 home_runs: i32,  // 被本塁打数(HR)
    // 19 bases_on_balls: i32,  // 与四球数(BB)
    // 20 intentional_walks: i32,  // 敬遠による与四球数(IBB)
    // 21 strike_outs: i32, // 奪三振数(SO)
    // 22 hits_by_pitch: i32,  // 与死球数(HBP)
    // 23 balks: i32,  // ボーク数(BK)
    // 24 wild_pitches: i32,  // 暴投数(WP)
    // 25 batters_faced: i32,  // 対戦打者数(BF)
    // 26 adjusted_earned_run_average_plus: f64,  // 修正防御率(ERA+)
    // 27 walks_and_hits_per_innings_pitched: f64,  // 1イニング当たりに投手が許した走者の数(WHIP)
    // 28 hits_per_nine_innings: f64,  // 9イニングあたり被安打率(H/9)
    // 29 home_runs_per_nine_innings: f64,  // 9イニングあたり被本塁打率(HR/9)
    // 30 walks_per_nine_innings: f64,  // 9イニングあたり与四球率(BB/9)
    // 31 strikeouts_per_nine_innings: f64,  // 9イニングあたり奪三振率(SO/9)
    // 32 strikeout_walk_ratio: f64,  // 三振と四球の割合(SO/BB)
    // 33awards: String,  // 受賞
}

impl Pitcher {
    fn parse_int32(&self, year_index: usize, column_index: usize) -> Option<i32> {
        let value_str: &String = self.record_array.get(year_index)?.get(column_index)?;
        value_str.parse().ok()
    }

    pub fn year(&self) -> Vec<i32> {
        self.record_array.iter().map(|r|r.get(0).unwrap().parse().unwrap()).collect::<Vec<_>>()
    }

    pub fn age(&self) -> Vec<i32> {
        self.record_array.iter().map(|r|r.get(1).unwrap().parse().unwrap()).collect::<Vec<_>>()
    }

    pub fn team(&self) -> Vec<String> {
        self.record_array.iter().map(|r|r.get(2).unwrap().to_string()).collect::<Vec<_>>()
    }

    pub fn league(&self) -> Vec<String> {
        self.record_array.iter().map(|r| r.get(3).unwrap().to_string()).collect::<Vec<_>>()
    }

    pub fn wins(&self) -> Vec<i32> {
        self.record_array.iter().map(|r|r.get(4).unwrap().parse().unwrap()).collect::<Vec<_>>()
    }

    pub fn losses(&self) -> Vec<i32> {
        self.record_array.iter().map(|r| r.get(5).unwrap().parse().unwrap()).collect::<Vec<_>>()
    }

    pub fn win_rate(&self) -> Vec<Option<String>> {
        self.record_array.iter()
            .map(|r| r.get(6).map(|s| s.to_string()))
            .collect::<Vec<_>>()
    }
}

fn main() {
    let pitcher = make_pitcher();
    println!("{:?}", pitcher.year());
    println!("{:?}", pitcher.win_rate());
    ()
}

fn read_csv() -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let file = File::open("/home/ykanya/devel/tools/baseball/baseball_R/data/spahn.csv")?;
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(file);
    Ok(rdr.records().map(|r| r.unwrap()).collect::<Vec<_>>())
}


fn make_pitcher() -> Pitcher {
    let records = read_csv().unwrap();
    Pitcher{
        record_array: records
            .iter()
            .map(|r| r.iter().map(|item| item.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }
}
