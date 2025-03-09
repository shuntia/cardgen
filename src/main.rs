use base64::{self, Engine, engine::GeneralPurposeConfig};
use rfd;
use std::fs::OpenOptions;
use std::io::Write;
use std::{io::Read, io::stdin, path::PathBuf, process::exit};

fn main() {
    println!("ようこそ。カードジェネレーターへ。");
    let mut card = Card {
        title: String::new(),
        description: Vec::new(),
        caption: String::new(),
        image: PathBuf::new(),
        ctype: CardType::Song,
        category: CardCategory::Notes,
        extra: None,
        genre: Vec::new(),
    };
    println!("カードのタイトルを入力してください。");
    stdin().read_line(&mut card.title).unwrap();
    card.title = card.title.trim().to_string();
    println!("カードの説明を入力してください。終わったらqと入力してください。");
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        if line.trim() == "q" {
            break;
        }
        card.description.push(line);
    }
    card.description = card
        .description
        .iter()
        .map(|x| x.trim().to_string())
        .collect();
    println!("カードの画像を選択してください。");
    let image;
    loop {
        let tmp = rfd::FileDialog::new()
            .add_filter("Image", &["jpg", "png"])
            .pick_file();
        match tmp {
            Some(path) => {
                image = path;
                break;
            }
            None => {
                println!("画像が選択されていません。もう一度選択してください。");
            }
        }
    }
    card.image = image;
    println!("カードの種類を選択してください。");
    println!("1: Song, 2: Skill, 3: Character");
    let ctype;
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        match line.trim() {
            "1" => {
                ctype = CardType::Song;
                break;
            }
            "2" => {
                ctype = CardType::Skill;
                break;
            }
            "3" => {
                ctype = CardType::Character;
                break;
            }
            _ => {
                println!("無効な入力です。もう一度入力してください。");
            }
        }
    }
    card.ctype = ctype;
    println!("カードのカテゴリーを選択してください。");
    println!("1: Notes, 2: Peak, 3: Technique, 4: Stamina, 5: Tricky, 6: Special");
    let category;
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        match line.trim() {
            "1" => {
                category = CardCategory::Notes;
                break;
            }
            "2" => {
                category = CardCategory::Peak;
                break;
            }
            "3" => {
                category = CardCategory::Technique;
                break;
            }
            "4" => {
                category = CardCategory::Stamina;
                break;
            }
            "5" => {
                category = CardCategory::Tricky;
                break;
            }
            "6" => {
                category = CardCategory::Special;
                break;
            }
            _ => {
                println!("無効な入力です。もう一度入力してください。");
            }
        }
    }
    card.category = category;
    println!("カードのジャンルを入力してください。終わったらqと入力してください。");
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        if line.trim() == "q" {
            break;
        }
        card.genre.push(line);
    }
    println!("画像のサブタイトルを入力してください。");
    stdin().read_line(&mut card.caption).unwrap();
    card.caption = card.caption.trim().to_string();
    println!("画像の追加情報を入力してください。必要ない場合はEnterを押してください。");
    let mut extra = String::new();
    stdin().read_line(&mut extra).unwrap();
    if extra.trim() != "" {
        card.extra = Some(extra);
    }
    println!("生成しますか？(y/n)");
    loop {
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        match line.trim() {
            "y" => {
                println!("はわわー！生成、スタートです！！！");
                break;
            }
            "n" => {
                println!("生成をキャンセルしました。");
                stdin().read_line(&mut line).unwrap();
                exit(0);
            }
            _ => {
                println!("無効な入力です。もう一度入力してください。");
            }
        }
    }
    let mut html = HTML.to_string();
    html = html.replace("NAME", &card.title);
    let mut description = String::new();
    for line in card.description {
        description.push_str(&format!("<li>{}</li>", line));
    }
    html = html.replace("DESCRIPTION", &description);
    println!("現在カテゴリと種類の画像は未実装です。");
    let mut f = std::fs::File::open(card.image).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let base64engine = base64::engine::GeneralPurpose::new(
        &base64::alphabet::STANDARD,
        GeneralPurposeConfig::new(),
    );
    let base64 = base64engine.encode(buf);
    html = html.replace("MAINIMG", &format!("data:image/png;base64,{}", base64));
    match card.extra {
        Some(extra) => {
            html = html.replace("EXTRA", &extra);
            html = html.replace("display: none;", "");
        }
        None => {
            html = html.replace("EXTRA", "");
        }
    }
    html = html.replace("SUBTITLE", &card.caption);
    let mut genre = String::new();
    for line in card.genre {
        genre.push_str(&format!("<div>{}</div>", line));
    }
    html = html.replace("GENRE", &genre);
    loop {
        let savefile = rfd::FileDialog::new()
            .add_filter("HTML", &["html"])
            .set_file_name(format!("{}.html", card.title))
            .save_file();
        match savefile {
            Some(path) => {
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path)
                    .unwrap();
                file.write_all(html.as_bytes()).unwrap();
                println!("保存しました。");
                break;
            }
            None => {
                println!("保存されていません。もう一度選択してください。");
            }
        }
    }
}

const HTML: &str = r#"
<style>
    .header{
        padding:10px;
        height:150px;
        width:1480;
        background-color: #f1f1f1;
        display:flex;
        flex-direction:row;
        justify-content:space-between;
    }
    #type{
        border-radius: 75px;
        border-width: 5px;
        border-color:#000000;
        height:150px;
        width:150px;
        background-color: #bebebe;
    }
    #category{
        border-radius: 75px;
        border-width: 5px;
        height:150px;
        width:150px;
        background-color: #bebebe;
    }
    #name{
        display: flex;
        justify-content: center;
        align-items: center;
        height:100%;
        background-color: #f1f1f1;
        text-align: center;
        font-size: 100px;
    }
    #image{
        overflow: hidden;
        height:900px;
        width:1400px;
        align-self: center;
        background-image: url(MAINIMG);
        background-repeat: no-repeat;
        background-size: cover;
        border-radius: 50px;
        position: relative;
    }
    .card{
        font-family: "oswald", sans-serif;
        display: flex;
        flex-direction: column;
        overflow: hidden;
        height:2100px;
        width:1500px;
        border:5px;
        border-style: solid;
        border-radius: 25px;
        border-color:#000000;
        background-color: antiquewhite;
        align-self:center;
    }
    #extra{
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        position:absolute;
        bottom:0;
        padding-right: 10px;
        height:100px;
        width:auto;
        background-color: #f1f1f1;
        border-top-right-radius: 25px;
        font-size:50px;
        display: none;
    }
    #subtitle{
        line-height: 75px;
        font-size: 30px;
        height:75px;
        width:1300px;
        background-color: #f1f1f1;
        text-align: center;
    }
    #description{
        font-size: 50px;
        border-style: solid;
        border-radius: 25px;
        border-width: 2px;
        height:750px;
        width:1200;
        background-color: #f1f1f1;
        text-align: left;
    }
    .separator{
        height:50px;
        visibility: hidden;
    }
    #genre{
        border-style: solid;
        border-width:0px;
        border-top-width: 2px;
        display: flex;
        flex-direction: column;
        justify-content: center;
        font-size:50px;
        bottom:0;
        height:150px;
        width:1500;
        background-color: #f1f1f1;
        text-align: center;
        justify-self: flex-end;
    }
    .genre-container{
        display: flex;
        flex-direction: row;
        justify-content:space-around;
    }
    image{
        height:100%;
        width:100%;
        object-fit: cover;
    }
    .body{
        padding:10px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-between;
        height:100%;
    }
</style>
<link rel="stylesheet" href="https://use.typekit.net/ops8lqp.css">
<div class="card">
    <div class="header">
        <div id="type">
        </div>
        <div id="name">
            NAME
        </div>
        <div id="category">
        </div>
    </div>
    <div class="body">
        <div id="image">
            <div id="extra">
                EXTRA
            </div>
        </div>
        <div id="subtitle">
            SUBTITLE
        </div>
        <div id="description">
            <ul>
                DESCRIPTION
            </ul>
        </div>
    </div>
    <div id="genre">
        <div class="genre-container">
            GENRE
        </div>
    </div>
</div>
"#;
struct Card {
    pub title: String,
    pub caption: String,
    pub description: Vec<String>,
    pub image: PathBuf,
    pub ctype: CardType,
    pub category: CardCategory,
    pub extra: Option<String>,
    pub genre: Vec<String>,
}
#[derive(Debug)]
pub enum CardType {
    Song,
    Skill,
    Character,
}
#[derive(Debug)]
pub enum CardCategory {
    Notes,
    Peak,
    Technique,
    Stamina,
    Tricky,
    Special,
}
