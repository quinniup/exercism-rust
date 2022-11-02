use std::collections::HashMap;

struct Result {
    pub name: String,
    pub points: i8,
    pub won: i8,
    pub lost: i8,
    pub tied: i8,
    pub played: i8,
}

impl Result {
    pub fn new(name: &str) -> Self {
        Result {
            name: String::from(name),
            points: 0,
            won: 0,
            lost: 0,
            tied: 0,
            played: 0,
        }
    }

    pub fn win(&mut self) {
        self.won += 1;
        self.points += 3;
        self.played += 1;
    }

    pub fn loss(&mut self) {
        self.played += 1;
        self.lost += 1;
    }

    pub fn tied(&mut self) {
        self.played += 1;
        self.tied += 1;
        self.points += 1;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut result: HashMap<String, Result> = HashMap::new();

    for line in match_results.lines() {
        let ele = line.split(";").collect::<Vec<&str>>();
        if ele.len() != 3 {
            continue;
        }
        // play result: won, loss, draw
        let r = ele[2];


        let flag = match r {
            "win" => (1, -1),
            "loss" => (-1, 1),
            "draw" => (0, 0),
            _ => continue
        };

        record(&mut result, String::from(ele[0]), flag.0);
        record(&mut result, String::from(ele[1]), flag.1);
    }

    let mut tally = result.values().collect::<Vec<&Result>>();
    // sort points
    tally.sort_by(|x,y| {
        if x.points == y.points {
            x.name.partial_cmp(&y.name).unwrap()
        } else {
            y.points.partial_cmp(&x.points).unwrap()
        }
    });
    let mut result_str = format("Team", "MP", "W", "D", "L", "P");

    for each in tally {
        result_str = result_str + "\n" + &*format(&*each.name, &*each.played.to_string(),
                                                  &*each.won.to_string(), &*each.tied.to_string(),
                                                  &*each.lost.to_string(), &*each.points.to_string()
        );
    }

    result_str
}

fn record(result: &mut HashMap<String, Result>, name: String, flag: i8) {
    let current = result.entry(name.clone()).or_insert(Result::new(&*name));
    match flag {
        1 => current.win(),
        -1 => current.loss(),
        0 => current.tied(),
        _ => {}
    }
}


fn format(title: &str, mp: &str, w: &str, d: &str, l: &str, points: &str) -> String {
    format!("{:31}| {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", title, mp, w, d, l, points)
}
