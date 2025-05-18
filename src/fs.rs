use json::{self, JsonValue};
use std::{io, path::PathBuf, time::Duration};

use crate::ToDo;

fn save_to_json(contents: &crate::ToDo) -> JsonValue{
    let cur_task;
    let rest;
    cur_task = ((contents.stopwatch.as_secs() + contents.old_dur.as_secs()),
    (contents.stopwatch.as_millis()%1000) as u32 + (contents.old_dur.as_millis()%1000) as u32);

    rest = ((contents.pause_dur.as_secs() + contents.breaks.as_secs()),
    (contents.pause_dur.as_millis()%1000) as u32 + (contents.breaks.as_millis()%1000) as u32);
    let save = json::object!{
        completed: contents.complete,
        removed: contents.removed,
        tasks: contents.tasks.clone(),
        break_time: [
            rest.0,
            rest.1,
        ],
        cur_task: [
            cur_task.0,
            cur_task.1,
        ],
        prev_task: [
            contents.last_time.as_secs(),
            (contents.last_time.as_millis() % 1000) as u32
        ]
    };
    println!("{}", json::stringify_pretty(save.clone(), 4));
    save
}

pub async fn save_to_file(path: PathBuf, filename: String, content: crate::ToDo) -> Result<(), io::ErrorKind>{
    // println!("Recieved Save");
    if tokio::fs::metadata(&path).await.is_err() {
        let _ = tokio::fs::create_dir(&path).await.map_err(|error| eprintln!("Failed to create directory {}", error.kind()));
    }
    let full_path = if let Some(dir) = path.to_str() {dir.to_owned() + &filename} else {return Err(io::ErrorKind::InvalidInput);};
    tokio::fs::write(full_path, json::stringify_pretty(save_to_json(&content), 4)).await.map_err(|error| error.kind())
}

pub async fn read_from_file(path: PathBuf, filename: String) -> Result<ToDo, io::ErrorKind> {
    let full_path = if let Some(dir) = path.to_str() {dir.to_owned() + &filename} else {return Err(io::ErrorKind::InvalidInput);};
    let values = tokio::fs::read_to_string(full_path).await.map_err(|error| error.kind())?;
    Ok(get_todo(&mut parse_json(values)))
}

fn parse_json(read_in: String) -> JsonValue {
    let parsed = json::parse(&read_in);
    match (parsed){
        Ok(values) => values,
        Err(_) => {panic!("Failed to parse Json")},
    }
        
}

pub fn get_todo(json: &mut JsonValue) -> ToDo {
    let completed = json["completed"].as_u32().unwrap_or(0);
    let removed = json["removed"].as_u32().unwrap_or(0);
    let tasks: Vec<String> = json["tasks"].members().filter_map(|v| v.as_str().map(String::from)).collect();
    let break_time:Vec<u64> = json["break_time"].members().filter_map(|x| x.as_u64()).collect();
    let cur_task: Vec<u64> = json["cur_task"].members().filter_map(|x| x.as_u64()).collect();
    let prev_task: Vec<u64> = json["prev_task"].members().filter_map(|x| x.as_u64()).collect();
    ToDo::from(tasks, completed, removed, Duration::new(cur_task[0], cur_task[1] as u32), Duration::new(prev_task[0], prev_task[1] as u32), Duration::new(break_time[0], break_time[1] as u32))


}