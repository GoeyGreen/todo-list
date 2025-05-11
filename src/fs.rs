use json::{self, JsonValue};
use std::{io, path::PathBuf};

pub fn save_to_json(contents: &crate::ToDo) -> JsonValue{
    json::object!{
        completed: contents.complete,
        removed: contents.removed,
        

    }   
}

pub async fn save_to_file(path: PathBuf, content: &crate::ToDo) -> Result<(), io::Error>{
    tokio::fs::write(path, json::stringify(save_to_json(content))).await.map_err(|error| error)
}