use rand::{thread_rng, Rng};
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
#[cfg(test)]
mod tests {
    use crate::*;
    // #[ignore]
    #[test]
    fn it_works() {
        let file_path = env::current_dir().unwrap().join("My Clippings.txt");
        let mut db_file = File::open(file_path).unwrap();
        let mut input: String = "".to_string();
        db_file.read_to_string(&mut input).unwrap();

        parse_input(&input);
    }

    #[ignore]
    #[test]
    fn test_parse_desc() {
        let case1 = "- 您在位置 #2484-2487的标注 | 添加于 2021年4月7日星期三 下午10:55:58";
        let case2 = "- 您在第 47 页（位置 #708-710）的标注 | 添加于 2021年3月5日星期五 上午1:26:56";
        let case3 = "- 您在位置 #205 的笔记 | 添加于 2019年8月31日星期六 上午12:55:51";

        let desc = parse_desc(case1);
        dbg!(&desc);
        let desc = parse_desc(case3);
        dbg!(&desc);
    }

    #[ignore]
    #[test]
    fn test_parse_date() {
        let case1 = "2019年8月31日星期六 上午12:55:51";
        // let dt = DateTime::parse_from_str("1983 Apr 13 12:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z");
        // let dt = DateTime::parse_from_str(case1, "%Y年%m月%d日星期 %H:%M:%S%.3f %z");

        // let re =
        //     Regex::new(r"(\d{4})年(\d\d?)月(\d\d?)日星期(.*) (.*)(\d{2}):(\d{2}):(\d{2})").unwrap();

        // let cap = re.captures(case1).unwrap();
        // dbg!(&cap);
        let cap = parse_date(case1);
        dbg!(&cap);

        Weekday::from_str("二");
        Weekday::from_str("xx");
        Weekday::from_str("x");
    }
}

#[derive(Debug, Clone)]
pub enum Weekday {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl Weekday {
    pub fn from_str(wd: &str) -> Option<Self> {
        let weekday = match wd {
            "一" => Weekday::Mon,
            "二" => Weekday::Tue,
            "三" => Weekday::Wed,
            "四" => Weekday::Thu,
            "五" => Weekday::Fri,
            "六" => Weekday::Sat,
            "日" => Weekday::Sun,
            _ => return None,
        };
        Some(weekday)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Datetime {
    year: u16,
    month: u8,
    day: u8,
    weekday: String,
    am_or_pm: String,
    hour: u8,
    minute: u8,
    second: u8,
}

pub fn parse_date(input: &str) -> Result<Datetime, ()> {
    let re =
        Regex::new(r"(\d{4})年(\d\d?)月(\d\d?)日星期(.*) (.*)(\d{2}):(\d{2}):(\d{2})").unwrap();

    if let Some(caps) = re.captures(input) {
        let internal_parse = |idx| -> usize {
            caps.get(idx)
                .map(|x| x.as_str())
                .unwrap_or("")
                .parse()
                .unwrap_or_default()
        };
        let internal_parse_str =
            |idx| -> String { caps.get(idx).map(|x| x.as_str()).unwrap_or("").to_string() };
        assert_eq!(caps.len(), 9);
        let mut datetime = Datetime::default();
        // datetime.year = caps.get(1).map(|x|x.as_str()).unwrap_or("").parse().unwrap_or_default();
        datetime.year = internal_parse(1) as _;
        datetime.month = internal_parse(2) as _;
        datetime.day = internal_parse(3) as _;
        datetime.weekday = internal_parse_str(4) as _;
        datetime.am_or_pm = internal_parse_str(5) as _;
        datetime.hour = internal_parse(6) as _;
        datetime.minute = internal_parse(7) as _;
        datetime.second = internal_parse(8) as _;
        return Ok(datetime);
    }
    return Err(());
}

pub fn parse_desc(input: &str) -> Result<Desc, ()> {
    let re = Regex::new(r"#(\d*)(-(\d*))?.*(20\d\d年.*)$").unwrap();
    let mut desc = Desc::default();
    if re.captures(input).is_none() {
        panic!(input.to_string());
    }
    let cap = re.captures(input).unwrap();
    if cap.len() != 5 || false {}
    // dbg!(&cap);
    if cap.len() == 5 {
        desc.pos_start = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
        desc.pos_end = match cap.get(3) {
            Some(i) => i.as_str().parse::<u32>().ok(),
            None => None,
        };
    } else {
        panic!(input.to_string());
    }
    Ok(desc)
}
const SEP: &str = "==========\r\n";
const NEWLINE: &str = "\r\n";
const OPTION_META_DATA: bool = false;

#[derive(Debug, Default, Clone)]
pub struct Desc {
    pos_start: u32,
    pos_end: Option<u32>,
    // timestamp: String
}
#[derive(Debug, Default, Clone)]
pub struct Note {
    book: String,
    desc: Desc, //
    content: String,
}

fn parse_input(input: &str) {
    let parts: Vec<_> = input.split(SEP).collect();
    dbg!(parts.len());
    // let parts = &parts[0..300];

    // dbg!(&parts);
    let mut notes_map: HashMap<String, Vec<Note>> = HashMap::new();
    let mut notes: Vec<Note> = vec![];
    for (i, part) in parts.iter().enumerate() {
        let lines: Vec<_> = part.split(NEWLINE).collect();
        // dbg!(i,&lines);
        if lines.len() != 5 || lines[2] != "" || lines[4] != "" {
            dbg!(i, &lines);
            // panic!(123);
            break;
        } else {
            let mut note = Note::default();
            note.book = lines[0].to_string();
            note.desc = parse_desc(lines[1]).unwrap();
            note.content = lines[3].to_string();
            // dbg!(&note);
            let map_entry = notes_map.entry(note.book.clone()).or_insert(vec![]);
            map_entry.push(note);
        }
    }
    let key_len = notes_map.keys().len();
    for (k, v) in notes_map {
        // println!("{}: \n=> {:?}, {:?}", k, v.len(), v[0]);
        // println!("{}: \n=> {:?}", k, v.len());
        // println!("=============");
        //
        if k.contains("Digital Minimalism") {
            let mut rng = thread_rng();
            let today_clip = &v[rng.gen_range(0..v.len())];
            println!("=====================");
            println!("Today's Clipping:\n\n  {}", today_clip.content);
            println!("=====================");

            let mut file = File::create(format!("{}.org", k)).unwrap();
            write!(file, "* Highlights in \"{}\"\n", k);
            for line in v {
                write!(file, "** {}\n", line.content);

                // Optional: write metadata
                if OPTION_META_DATA {
                    write!(file, "\t:PROPERTIES:\n");
                    write!(file, "\t:POS_START: {}\n", line.desc.pos_start);
                    if line.desc.pos_end.is_some() {
                        write!(file, "\t:POS_END: {}\n", line.desc.pos_end.unwrap());
                    }
                    write!(file, "\t:END:\n");
                }
            }
        }
    }
    println!("TOTAL: {} books", key_len);
}
