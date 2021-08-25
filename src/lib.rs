use regex::Regex;
use std::env;
use std::fs::File;
use std::io::Read;
#[cfg(test)]
mod tests {
    use crate::*;
    #[ignore]
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

    #[test]
    fn test_parse_date() {
        let case1 = "2019年8月31日星期六 上午12:55:51";
        // let dt = DateTime::parse_from_str("1983 Apr 13 12:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z");
        // let dt = DateTime::parse_from_str(case1, "%Y年%m月%d日星期 %H:%M:%S%.3f %z");

        let re =
            Regex::new(r"(\d{4})年(\d\d?)月(\d\d?)日星期(.*) (.*)(\d{2}):(\d{2}):(\d{2})").unwrap();

        let cap = re.captures(case1).unwrap();
        dbg!(&cap);
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

pub fn parse_date(input: &str) {}

pub fn parse_desc(input: &str) -> Result<Desc, ()> {
    let re = Regex::new(r"#(\d*)(-(\d*))?.*(20\d\d年.*)$").unwrap();
    let mut desc = Desc::default();
    if re.captures(input).is_none() {
        panic!(input.to_string());
    }
    let cap = re.captures(input).unwrap();
    if cap.len() != 5 || false {}
    dbg!(&cap);
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
    // let parts = &parts[0..3];

    // dbg!(&parts);
    let mut notes: Vec<Note> = vec![];
    for (i, part) in parts.iter().enumerate() {
        let lines: Vec<_> = part.split(NEWLINE).collect();
        // dbg!(i,&lines);
        if lines.len() != 5 || lines[2] != "" || lines[4] != "" {
            dbg!(i, &lines);
            panic!(123);
        } else {
            let mut note = Note::default();
            note.book = lines[0].to_string();
            note.desc = parse_desc(lines[1]).unwrap();
            note.content = lines[3].to_string();
            dbg!(&note);
        }
    }
}
