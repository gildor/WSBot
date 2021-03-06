extern crate discord;
extern crate regex;
extern crate reqwest;
//extern crate mysql;
//VH14.spaceweb.ru
use discord::Discord;
use discord::model::Event;
use regex::Regex;
use std::io::Read;
use regex::SplitN;

// Элемент очереди
struct Player {
    name: String,
    btag: String,
    rdy: bool,
    h: bool, // Хил?
    d: bool, // Дамагер?
    t: bool, // Танк?
    inv: String,
}
impl Player {
    fn new(name :&str, btag :&str, rdy :bool, h :bool, d :bool, t :bool, inv :&str) -> Player {
        Player {
            name: name.to_string(),
            btag: btag.to_string(),
            rdy: false,
            h: true,
            d: true,
            t: true,
            inv: inv.to_string(),
        }
    }
}

fn startmix(name: &str) {
    loop {

    }
    println!("{} заехал в очередь. В очереди людей", name);
}

fn stopmix(name: &str) {
    println!("{} вышел из очереди", name);
}

fn load_overwatch_rating(name: &str, id: &str) -> String {
    let url = &format!("https://playoverwatch.com/en-us/career/pc/eu/{}-{}", name, id);
    let mut resp = reqwest::get(url).expect("Wrong url");
    let regex = Regex::new("<div class=\"u-align-center h6\">(\\d+)</div>").unwrap();
    let mut content = String::new();
    resp.read_to_string(&mut content).expect("Rating downloading error");
    let rating = regex.captures(&content).unwrap().get(1).expect("Rating not found").as_str();
    return rating.to_string();
}
//fn actualrating(discord: &Discord, name :&str , id :&str,) {
//    let rating = load_overwatch_rating(name, id);
//    let acrat = ("{}#{} ваш актуальный рейтинг: {}", name, id, rating);
//    let _ = discord.send_message(message.channel_id, &acrat, "", false);
//    println!("{}#{} actual rating: {}", name, id, rating);
//}

//let mut stack = Vec::new();
//stack.push(1);
//stack.push(2);
//stack.push(3);
//while let Some(top) = stack.pop() {
//println!("{}", top);
//}

fn main() {
    let btag_reg = Regex::new(r"^!wsreg\s+([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}$").unwrap(); //форма среза текста "!wsreg battletag#123"
    let re = Regex::new(r"([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}").unwrap();//форма среза текста "battletag#123"
    // Log in to Discord using a bot token from the environment
    let discord = Discord::from_bot_token("MzA4MDQ4NzQ0NzgyMzY0Njcy.C-qiOw.YG8nGwogD7FTRxNTotFhLWCr3Dg").expect("толи сервер толи токен");
    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    let mut list: Vec<Player>;
    let newplayer = Player::new("", "", false, true, true, true, "");
    list.push(newplayer);

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                match message.content.as_str() {

                    "!wshelp" => {
                        let wshelp = "Отчаяние =). Введите !wscmd";
                        let _ = discord.send_message(message.channel_id, wshelp, "", false);
                    },
                    "!wsreg" => {
                        let wsreg = "Введите команду !wsreg вместе с батлтагом. Например: !wsreg Valera#228";
                        let _ = discord.send_message(message.channel_id, wsreg, "", false);
                    },
                    "!wscmd" => {
                        let wscmd = include_str!("cmd.ws");
                        let _ = discord.send_message(message.channel_id, wscmd, "", false);
                        },
                    "!wsmix" => {
                        let wsmix = "Вы собрались поиграть миксы?";
                        let _ = discord.send_message(message.channel_id, wsmix, "", false);
                    },
                    "!wsmixgo" => {
                        for i in list {
                            if i.name == message.author.name {
                                // do stuff
                                let _ = discord.send_message(message.channel_id, "Вы встали в очередь для поиска миксов", "", false);
                                for i in list {
                                    if i.rdy == true {
                                        let listmsg = format!("Игрок {} тоже ищет миксы", i.name);
                                        let _ = discord.send_message(message.channel_id, &listmsg, "", false);
                                    } else { return; }
                                }
                            }
                            else {
                                let _ = discord.send_message(message.channel_id, "Введите команду !wsreg вместе с батлтагом. Например: !wsreg Valera#228", "", false);
                            };
                        };
                    },
                    //_ => {
                    //    let btag_reg = Regex::new(r"^!wsad\s+([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}$").unwrap();
                    //    if let Some(caps) = btag_reg.captures(&message.content) {
                    //        let re = Regex::new(r"([0-9\p{Cyrillic}]|[0-9\p{Latin}]){2,16}#[0-9]{2,6}").unwrap();
                    //        for cap in re.captures_iter(&message.content) {
                    //            let fullbtag = cap;
                    //            let mut s = fullbtag.get(0).unwrap().as_str().splitn(2, '#');
                    //            let (name, id) = (s.next().unwrap(), s.next().unwrap());
                    //            println!("foo");
                    //            let rating = load_overwatch_rating(name, id);
                    //            let acrat = format!("Актуальный рейтинг игрока {}#{}: {}",name, id, rating);
                    //            let _ = discord.send_message(message.channel_id, &acrat, "", false);
                    //            };
                    //    };
                    //},
                    _ => {
                        if let Some(caps) = btag_reg.captures(&message.content) {
                            println!("Определен");
                            for cap in re.captures_iter(&message.content) {
                                println!("Привязан - {}", &cap[0]);
                                let btmsg = format!("К вам привязан батлтаг - {}", &cap[0]);
                                let _ = discord.send_message(message.channel_id, &btmsg, "", false);
                                let fullbtag = cap;
                                let mut s = fullbtag.get(0).unwrap().as_str().splitn(2, '#');
                                let (name, id) = (s.next().unwrap(), s.next().unwrap());
                                println!("Разбит - {} - {}", name, id);
                                let rating = load_overwatch_rating(name, id);
                                let acrat = format!("Ваш актуальный рейтинг: {}", rating);
                                let _ = discord.send_message(message.channel_id, &acrat, "", false);
                                println!("Рейтинг - {}", rating);
                                let mixname = message.author.name;
                                let fbtag = format!("{}#{}", name, id);
                                let newplayer = Player::new(&mixname, &fbtag, false, true, true, true, "");
                                list.push(newplayer);
                                };
                        };

                    },
                    _ => {},
                };

            },
            Ok(Event::ServerMemberAdd(serverid, member)) => {
                let welcome = "Добропожаловать на сервер уважаемый";
                println!("{:?} {:?} - вы на планете № {:?}", &welcome, &member.nick, &serverid);
                break
            },
            Ok(_) => {},
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            },
            Err(err) => println!("Receive error: {:?}", err),
        };
    };
}

