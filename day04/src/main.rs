use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Default)]
struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

lazy_static! {
    static ref HGT_REGEX:Regex = Regex::new("^([0-9]+)(in|cm)$").unwrap();
    static ref HCL_REGEX:Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
    static ref ECL_REGEX:Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PID_REGEX:Regex = Regex::new("^[0-9]{9}$").unwrap();
}

impl Passport {
    /// Create a Passport from a string.
    pub fn from_str(s: &str) -> Result<Self, String> {
        let mut passport = Passport::default();

        for field in s.split_whitespace() {
            let mut parts = field.split(':');
            let key = parts.next()
                .ok_or_else(|| format!("missing key: {}", field))?;
            let val = parts.next()
                .ok_or_else(|| format!("missing val: {}", field))?;
            match key {
                "byr" =>
                    passport.byr = Some(val.parse::<usize>().map_err(|e| e.to_string())?),
                "iyr" =>
                    passport.iyr = Some(val.parse::<usize>().map_err(|e| e.to_string())?),
                "eyr" =>
                    passport.eyr = Some(val.parse::<usize>().map_err(|e| e.to_string())?),
                "hgt" => 
                    passport.hgt = Some(val.to_string()),
                "hcl" =>
                    passport.hcl = Some(val.to_string()),
                "ecl" =>
                    passport.ecl = Some(val.to_string()),
                "pid" =>
                    passport.pid = Some(val.to_string()),
                _ => {
                }
            }
        }

        Ok(passport)
    }

    pub fn is_valid(&self) -> Option<bool> {
        let res = self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some();
        return Some(res);
    }

    pub fn is_valid_ext(&self) -> Option<bool> {
        let res = (1920..=2020).contains(&self.byr?)
            && (2010..=2020).contains(&self.iyr?)
            && (2020..=2030).contains(&self.eyr?)
            && self.has_valid_hgt().unwrap_or_default()
            && HCL_REGEX.is_match(self.hcl.as_ref()?)
            && ECL_REGEX.is_match(self.ecl.as_ref()?)
            && PID_REGEX.is_match(self.pid.as_ref()?);

        return Some(res);
    }

    // TODO(sgosselin): this is a bit ugly.
    fn has_valid_hgt(&self) -> Result<bool, String> {
        if self.hgt.is_none() {
            return Err(format!("height is missing"));
        }

        let hgt = self.hgt.as_ref().unwrap();

        if !HGT_REGEX.is_match(&hgt) {
            return Err(format!("not a valid height"));
        }

        let caps_it = HGT_REGEX.captures(&hgt);
        let caps = caps_it.unwrap();
        let data = caps[1].parse::<usize>().map_err(|e| e.to_string())?;

        return match &caps[2] {
            "in" =>
                Ok((59..=76).contains(&data)),
            "cm" =>
                Ok((150..=193).contains(&data)),
            _ =>
                Err(format!("invalid height metric")),
        }
    }
}

fn process_passports(path: &str) -> usize {
    let f = File::open(path)
        .expect("could not open the input file");
    
    let mut buf = String::new();
    let mut res = 0;

    for data in BufReader::new(f).lines() {
        let line = data.unwrap();

        if line == "" {
            if let Ok(p) = Passport::from_str(&buf) {
                if let Some(true) = p.is_valid() {
                    res += 1;
                }
            }
            buf.clear();
        } else {
            buf.push_str(&line);
            buf.push_str(" ");
        }
    }

    if let Ok(p) = Passport::from_str(&buf) {
        if let Some(true) = p.is_valid() {
            res += 1;
        }
    }

    res
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} [path]", args[0]);
        return;
    }

    println!("num valid: {}", process_passports(&args[1]));
}
