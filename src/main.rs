use chrono::{NaiveDate,FixedOffset,TimeZone,Local,Datelike};
use std::time::{SystemTime,UNIX_EPOCH};
use std::{thread,time,env};
use std::io::{stdout,Write};
use crossterm::{QueueableCommand, cursor};
use soloud::*;

fn play_sound() {
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load_mem(include_bytes!("valve_-_cp_violation_cbr_128k.mp3")).unwrap();
    sl.play(&wav);
    while sl.voice_count() > 0{
        thread::sleep(time::Duration::from_millis(100))
    }
}

fn seconds_to_hms(mut seconds: f64) -> Vec<i64>{
    let hours = seconds/3600.0;
    seconds %= 3600.0;
    let minutes = seconds/60.0;
    seconds %= 60.0;
    let miliseconds = seconds*1000.0-(seconds.floor()*1000.0);
    vec![hours as i64,minutes as i64,seconds as i64,miliseconds as i64]
}

fn get_timestamp_from_today(hour: u32,minute: u32,second: u32) -> f64 {
    (FixedOffset::west(3*3600).from_local_datetime(&NaiveDate::from_ymd(Local::now().year(),Local::now().month(),Local::now().day()).and_hms(hour,minute,second)).unwrap()).timestamp() as f64
}

fn get_current_timestamp() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let hms: Vec<u32> = args[1].split(":").map(|num|num.parse::<u32>().unwrap()).collect();
    let mut future = get_timestamp_from_today(hms[0],hms[1],hms[2]);

    if get_current_timestamp() > future {
        let today_mn = get_timestamp_from_today(00,00,00);
        future = (today_mn+86400.0)-(today_mn-future);
    }
    
    loop {
        let tminus = future-get_current_timestamp();
        if tminus < 0.0 {
            stdout().write("Chegou a hora !                                     \r".as_bytes()).unwrap();
            stdout().flush().unwrap();
            play_sound();
            return
        }
        let hms = seconds_to_hms(tminus);
        stdout().queue(cursor::SavePosition).unwrap();
        stdout().write(format!("Faltam: {} Horas {} Minutos {}.{} Segundos ",hms[0],hms[1],hms[2],hms[3]/100).as_bytes()).unwrap();
        stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(1));
        stdout().queue(cursor::RestorePosition).unwrap();
    }
}