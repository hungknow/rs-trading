use std::{
    env,
    path::{Path, PathBuf},
};

pub const DATA_HISTORIC_XAUUSD_CANDLES_1M: &str = "candles/xauusd_1m.csv";

pub fn get_hk_trading_file_path(file_name: &str) -> Option<(PathBuf, PathBuf)> {
    let mut current_dir = env::current_dir().unwrap();

    let checking_paths = vec![
        [file_name, ""],
        ["hk-trading", file_name],
    ];

    for p in checking_paths {
        let mut path = current_dir.clone();
        for pp in p.iter() {
            if pp.is_empty() {
                continue;
            }
            path.push(pp);
        }

        if Path::new(&path).exists() {
            let mut dir = path.clone();
            dir.pop();
            return Some((path, dir));
        }
    }

    None
}
