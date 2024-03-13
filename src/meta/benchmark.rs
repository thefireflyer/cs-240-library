///////////////////////////////////////////////////////////////////////////////

use std::{
    collections::BTreeMap,
    fmt::Display,
    fs,
    time::{Duration, SystemTime},
};

use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    min: Duration,
    max: Duration,
    range: Duration,

    mean: Duration,
    median: Duration,
    mode: Duration,

    standard_deviation: Duration,

    passed: u32,
    total: u32,

    #[serde(skip)]
    logs: Vec<(Result<String, String>, Duration)>,

    version: u32,
    label: String,
}

//---------------------------------------------------------------------------//

impl Report {
    fn _save(&self) {
        let _ = fs::create_dir("./.benchmarks");

        fs::write(
            format!("./.benchmarks/{}.v{}.yaml", self.label, self.version),
            serde_yaml::to_string(self).unwrap(),
        )
        .unwrap();
    }

    fn _compare(&self) {
        match fs::read_to_string(format!(
            "./.benchmarks/{}.v{}.yaml",
            self.label,
            self.version - 1
        )) {
            Ok(other) => {
                let other: Self = serde_yaml::from_str(&other).unwrap();

                let c_min = self.min.as_nanos() / other.min.as_nanos() * 100;
                let c_max = self.max.as_nanos() / other.max.as_nanos() * 100;
                let c_mean = self.mean.as_nanos() / other.mean.as_nanos() * 100;
                let c_mode = self.mode.as_nanos() / other.mode.as_nanos() * 100;
                let c_median = self.median.as_nanos() / other.median.as_nanos() * 100;
                let c_stand_div =
                    self.standard_deviation.as_nanos() / other.standard_deviation.as_nanos() * 100;

                println!(
                    "{}

--- Comparison Report
Min: {:?}%
Max: {:?}%
Mean: {:?}%
Mode: {:?}%
Median: {:?}%
Standard deviation: {:?}%",
                    self, c_min, c_max, c_mean, c_mode, c_median, c_stand_div
                );
            }
            Err(_) => {}
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let failed: Vec<&(Result<String, String>, Duration)> =
            self.logs.iter().filter(|(res, _)| res.is_err()).collect();

        f.write_fmt(format_args!(
            "
==== {} v{} Report ====

--- Test Report
Passed: {}/{}
Failed: {:?}

--- Performance Report
{:?} < ... < {:?}
Mean: {:?}
Mode: {:?}
Median: {:?}
Standard deviation: {:?}",
            self.label,
            self.version,
            self.passed,
            self.total,
            failed,
            self.min,
            self.max,
            self.mean,
            self.mode,
            self.median,
            self.standard_deviation
        ))
    }
}

///////////////////////////////////////////////////////////////////////////////

fn _benchmark<T: Fn() -> Result<String, String>>(
    label: &str,
    version: u32,
    op: T,
    steps: u32,
) -> Report {
    let mut total = Duration::from_micros(0);
    let mut logs = Vec::new();
    let mut passed = 0;

    for _ in 1..steps + 1 {
        let time = SystemTime::now();

        let res = op();

        let dur = time.elapsed().unwrap();

        if res.is_ok() {
            passed += 1;
        }

        logs.push((res, dur));

        total += dur;
    }

    let mean;
    let median;
    let mode;
    let standard_deviation;

    mean = total / steps;

    logs.sort_by_key(|(_, dur)| dur.to_owned());

    let (_, min) = *logs.first().unwrap();
    let (_, max) = *logs.last().unwrap();

    let range = max - min;

    median = range / 2 + min;

    let mut freq = BTreeMap::new();

    for (_, dur) in &logs {
        match freq.get_mut(&dur) {
            Some(freq) => *freq += 1,
            None => {
                freq.insert(dur, 1);
            }
        }
    }

    mode = freq.first_key_value().unwrap().0.to_owned().to_owned();

    let mut total_error = Duration::from_micros(0);
    for (_, dur) in &logs {
        if *dur > mean {
            total_error += *dur - mean;
        } else {
            total_error += mean - *dur;
        }
    }
    standard_deviation = total_error / steps;

    Report {
        label: label.to_owned(),
        passed,
        total: steps,
        min,
        max,
        range,
        mean,
        median,
        mode,
        standard_deviation,
        logs,
        version,
    }
}

///////////////////////////////////////////////////////////////////////////////
/*
#[cfg(test)]
mod tests {
    use crate::meta::benchmark::benchmark;

    #[test]
    fn test() {
        let report = benchmark(
            "Meta.Benchmarking.Test.test01",
            2,
            || {
                let mut out = String::new();

                let len = 10;
                let res = (0..len).into_iter().fold(0, |acc, x| acc * x);

                for i in 0..len {
                    for j in 0..len {
                        let val = res + i * j;
                        out += &format!("{}, {} => {}", i, j, val);
                    }
                }

                Ok(out)
            },
            10_u32.pow(4),
        );

        report.compare();
        report.save();
    }
}
*/
///////////////////////////////////////////////////////////////////////////////
