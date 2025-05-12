use json::{self, JsonValue};
use std::{io, path::PathBuf};

pub fn save_to_json(contents: &crate::ToDo) -> JsonValue{
    let cur_task;
    let rest;
    if contents.rest {
        // ! ERROR: Not calculating / saving correct times depending on current status (break vs work times)
        cur_task = ((contents.stopwatch.as_secs() + contents.old_dur.as_secs()),
        (contents.stopwatch.as_millis()%1000) as u32 + (contents.old_dur.as_millis()%1000) as u32);

        rest = ((contents.pause_dur.as_secs() + contents.breaks.as_secs()),
        (contents.pause_dur.as_millis()%1000) as u32 + (contents.breaks.as_millis()%1000) as u32);
    } else {
        cur_task = ((contents.stopwatch.as_secs() + contents.old_dur.as_secs()),
        (contents.stopwatch.as_millis()%1000) as u32 + (contents.old_dur.as_millis()%1000) as u32);

        rest = ((contents.pause_dur.as_secs() + contents.breaks.as_secs()),
        (contents.pause_dur.as_millis()%1000) as u32 + (contents.breaks.as_millis()%1000) as u32);
    }
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
    println!("{}", json::stringify(save.clone()));
    save
}

pub async fn save_to_file(path: PathBuf, content: &crate::ToDo) -> Result<(), io::Error>{
    tokio::fs::write(path, json::stringify(save_to_json(content))).await.map_err(|error| error)
}