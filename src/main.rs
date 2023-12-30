use chrono::{DateTime, Datelike, Duration, Local, Weekday};
use dotenv::dotenv;
use slack_hook::{PayloadBuilder, Slack};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    match Local::now().weekday() {
        Weekday::Mon => post_paddle_msg(),
        Weekday::Tue => return,
        Weekday::Wed => return,
        Weekday::Thu => return,
        Weekday::Fri => return,
        Weekday::Sat => return,
        Weekday::Sun => return,
    }
}

fn post_paddle_msg() {
    let msg = get_slack_message();

    let slack_hook = env::var("SLACK_HOOK").unwrap();
    let slack_channel = env::var("SLACK_CHANNEL").unwrap();
    let slack = Slack::new(&*slack_hook).unwrap();

    let p = PayloadBuilder::new()
        .text(msg.clone())
        .channel(format!("#{}", slack_channel))
        .build()
        .unwrap();

    let res = slack.send(&p);
    match res {
        Ok(()) => print!("{}", msg),
        Err(err) => print!("error: {}", err),
    }
}

fn get_slack_message() -> String {
    let next_monday = next_weekday_next_week(Weekday::Mon);
    let next_wednesday = next_weekday_next_week(Weekday::Wed);

    format!(
        "*Padel* :padel:
Stem med emoji hvis du deltager
Mandag {man} klokken 16-17: :large_red_square:         
Mandag {man} klokken 17-18: :large_green_square:        
Onsdag {ons} klokken 16-17: :large_blue_square:",
        man = next_monday.format("%d/%m"),
        ons = next_wednesday.format("%d/%m")
    )
}

fn next_weekday_next_week(target: Weekday) -> DateTime<Local> {
    let mut date = Local::now() + Duration::days(1);
    while date.weekday() != Weekday::Sun {
        date = date + Duration::days(1);
    }

    while date.weekday() != target {
        date = date + Duration::days(1);
    }
    date
}
