use std::io;
use std::time::{Duration, Instant};

use fs::{save_to_file, read_from_file};
use iced::{Alignment, Color, Element, Length, Subscription, Task, Theme};
use iced::widget::{button, horizontal_space, text, text_input, vertical_space, Column, Container, Row, Scrollable};
use chrono::prelude::{DateTime, Local};

mod styles;
mod fs;
mod time;

// #[cfg(test)]
// mod tests;

use styles::buttons::*;
use styles::*;

// DONE: Update Timing System, to improve consistency
// TODO: Change Styling for file opening
// TODO: Migrate to new Time struct to reduce complexity
// TODO: Implement file system, saves, auto-load on start
// TODO: Implement auto-save functionality, on certain tick count
// TODO: Move auto save to separate struct
// TODO: Create Settings Menu, autosave on task completion
// TODO: Move to Serde JSON?
// TODO: Feature Request: Allow for dragging + reordering Tasks
// TODO: Allow sorting tasks, edit on click
// TODO: Create Tests?


#[derive(Debug, PartialEq, Clone)]
struct ToDo{
    time: DateTime<Local>,
    clock: String,
    tasks: Vec<String>,
    add: bool,
    complete: u32,
    removed: u32,
    start: Instant,
    old_dur: Duration,
    stopwatch: Duration,
    stop_string: String,
    last_time: Duration,
    last_string: String,
    rest:bool,
    pause_dur: Duration,
    breaks: Duration,
    break_start:Instant,
    break_string:String,
    sleep:bool,
    reset:bool,
    tick_count: i32,
    auto_save: bool,
}

impl Default for ToDo {
    fn default() -> Self {
        Self {
            time: Local::now(),
            clock: String::new(),
            tasks: Vec::new(),
            add: false,
            complete: 0,
            removed: 0,
            start:Instant::now(),
            old_dur: Duration::new(0,0),
            stopwatch:Duration::new(0,0),
            stop_string: String::new(),
            last_time:Duration::new(0,0),
            last_string: String::new(),
            rest: false,
            breaks: Duration::new(0,0),
            pause_dur:Duration::new(0,0),
            break_start:Instant::now(),
            break_string:String::new(),
            sleep: false,
            reset: false,
            tick_count: 0,
            auto_save: false,
        }
    }
}

#[derive(Debug, Clone)]
enum Message{
    Reset(bool),
    New,
    Cancel,
    End,
    AddTask(String, i32),
    RemoveTask(i32, bool),
    Tick,
    Break,
    Sleep,
    Save,
    Open,
    FileSave(Result<(), io::ErrorKind>),
    FileOpen(Result<ToDo, io::ErrorKind>),
}

impl ToDo {
    pub fn from(task_list: Vec<String>, completed: u32, removed_tasks: u32, task_time: Duration, prev_task_time: Duration, break_time: Duration) -> Self{
        ToDo {
            tasks:task_list,
            complete: completed,
            removed: removed_tasks,
            old_dur: task_time,
            last_time: prev_task_time,
            pause_dur: break_time,
            ..Default::default()
        }
    }

    fn replace(&mut self, todo: ToDo) {
        self.tasks = todo.tasks;
        self.complete = todo.complete;
        self.removed = todo.removed;
        self.old_dur = todo.old_dur;
        self.last_time = todo.last_time;
        self.pause_dur = todo.pause_dur;
    }

    pub fn view(&self) -> Element<Message>{
        // Sets the rounding radius for button elements with custom styles
        let radius = 2;

        // Stores all the contents on the screen
        let mut main: Column<'_, Message> = Column::new().align_x(Alignment::Center).width(Length::Fill).padding(20).spacing(10).into();


        main = main.push(
            // Top Row buttons
            Row::new().push(
                Row::new().push(if !self.rest && !self.reset {
                    button(
                        if !self.add {"New Task"} 
                        else {"Confirm"}).on_press(Message::New)
                    } 
                    else if self.reset {
                        button("Reset All").on_press(Message::Reset(false)).style(
                            move |_: &Theme, status| {
                                match status {
                                    button::Status::Hovered => {
                                        style_button(get_rgb_color(200, 0, 0), Color::WHITE, radius)
                                    },
                                    _ => {
                                        style_button(get_rgb_color(220, 8, 51), Color::WHITE, radius)
                                    },
                                }
                            }
                        )
                    }
                    else {
                        button(if !self.sleep {"Sleep"} else {"End Sleep"}).on_press(Message::Sleep).style(
                            move |_: &Theme, status| {
                                match status {
                                    _ => {
                                        if self.sleep{
                                            style_button(get_rgb_color(51, 89, 218), Color::WHITE, radius)
                                        } else {
                                            style_button(get_rgb_color(0, 155, 0), Color::WHITE, radius)
                                        }
                                    }
                                }
                        }
                    )
                    }
                )
                .push_maybe(
                    if self.add {
                        Some(button("Cancel").on_press(Message::Cancel))
                    } 
                    else {
                        None
                    }
                ).spacing(10)
            )
            .push_maybe(
                if !self.add && !self.reset{
                    Some(button(
                            if !self.rest {"Take a Break"} 
                            else {"End Break"}
                        ).on_press(Message::Break).style(
                            move |_: &Theme, status| {
                                match status {
                                    _ => {
                                        if self.rest{
                                            style_button(get_rgb_color(255, 0, 0), Color::WHITE, radius)
                                        } else {
                                            style_button(get_rgb_color(51, 89, 218), Color::WHITE, radius)
                                        }
                                    }
                                }
                            })
                        )} 
                        else if self.reset {Some(
                            button("Reset Time").on_press(Message::Reset(true)).style(
                                move |_: &Theme, status| {
                                    match status {
                                        button::Status::Hovered => {
                                            style_button(get_rgb_color(255, 0, 0), Color::WHITE, radius)
                                        },
                                        _ => {
                                            style_button(get_rgb_color(51, 89, 218), Color::WHITE, radius)
                                        },
                                    }
                                }
                            )
                        )}
                        else {
                            None
                        }).spacing(10)
                .push(horizontal_space())
                .push(button(if !self.reset {"Reset"} else {"Cancel"}).on_press(if !self.reset {Message::Reset(false)} else {Message::Cancel}).style(
                    move |_: &Theme, status| {
                        match status {
                            button::Status::Hovered => {
                                style_button(get_rgb_color(205, 0, 0), Color::WHITE, radius)
                            }
                            _ => {
                                if self.reset {
                                    style_button(get_rgb_color(155, 0, 0), Color::WHITE, radius)
                                } else {
                                    style_button(get_rgb_color(51, 89, 218), Color::WHITE, radius)
                                }
                            },
                        }
                    }
                )
            ).spacing(10)
            .push(
                button("Save").on_press(Message::Save).style(
                        move |_: &Theme, status| {
                            match status {
                                button::Status::Hovered => {
                                    style_button(get_rgb_color(0, 180, 0), Color::WHITE, radius)
                                }
                                _ => {
                                    style_button(get_rgb_color(51, 89, 218), Color::WHITE, radius)
                                }
                            }
                    }
                )
            )
            .push(
                button("Open").on_press(Message::Open).style(
                        move |_: &Theme, status| {
                            match status {
                                button::Status::Hovered => {
                                    style_button(get_rgb_color(0, 180, 0), Color::WHITE, radius)
                                }
                                _ => {
                                    style_button(get_rgb_color(51, 89, 218), Color::WHITE, radius)
                                }
                            }
                    }
                )
            )
        );

        // Text for ToDo List Including Task Count, Clock, and other text
        main = main.push(text("To Do List: ").size(20)).push(text(&self.clock).size(16)).push(text("").size(10));
        main = main.push(Row::with_children(vec![text(format!("Tasks Completed: {}", self.complete)).into(), text(format!("Tasks Removed: {}", self.removed)).into()]).spacing(20));

        // Section for all Tasks
        let mut tasks: Column<'_, Message> = Column::new().align_x(Alignment::Center).width(Length::Fill).padding(20).spacing(10).into();
        for (index, task) in self.tasks.clone().into_iter().enumerate(){
            if index != self.tasks.len() - 1 {
                tasks = tasks.push(
                    Container::new(
                        Row::new().push(button("Complete").on_press(Message::RemoveTask(index as i32, true)))
                        .push(text(task).size(16))
                        .push(horizontal_space())
                        .push(button("Remove").on_press(Message::RemoveTask(index as i32, false))
                            .style(
                                move |_: &Theme, status| {
                                    match status {
                                        button::Status::Active => {
                                            style_button(get_rgb_color(255, 0, 0), Color::WHITE, radius)
                                        }
                                        _ => {
                                            style_button(get_rgb_color(51, 89, 218), Color::WHITE, radius)
                                        },
                                    }
                                }
                            )
                        ).spacing(20).width(Length::Fill)
                    )
                );
            } else if !self.add && index == self.tasks.len() - 1 {
                // Last row, Special condition to make sure the new task being added doesn't show up until confirm button is clicked
                tasks = tasks.push(
                    Container::new(
                        Row::new().push(button("Complete").on_press(Message::RemoveTask(index as i32, true)))
                        .push(text(task).size(16))
                        .push(horizontal_space())
                        .push(button("Remove").on_press(Message::RemoveTask(index as i32, false))
                            .style(
                                move |_: &Theme, status| {
                                    match status {
                                        button::Status::Active => {
                                            style_button(get_rgb_color(255, 0, 0), Color::WHITE, radius)
                                        }
                                        _ => {
                                            style_button(get_rgb_color(51, 89, 218), Color::WHITE, radius)
                                        },
                                    }
                                }
                            )
                        ).spacing(20).width(Length::Fill)
                    )
                );
            }
        }
        main = main.push(text("").size(12));
        if self.add {
            main = main.push(Container::new(
                text_input("New Task ...", &self.tasks[self.tasks.len() - 1])
                    .on_input(|content:String | Message::AddTask(content, (self.tasks.len() - 1) as i32))
                    .on_submit(Message::End)));
        } else {
            main = main.push(text("").size(12))
        }
        main = main.push(Scrollable::new(tasks));
        main = main.push(vertical_space());
        // Times for tasks and time spent on breaks stored at the bottom row
        main = main.push(Row::with_children(vec![text(format!("Current Task: {}", self.stop_string)).into(), 
                        text(format!("Last Task: {}", self.last_string)).into(), 
                        text(format!("Break Time: {}", self.break_string)).color(if self.rest {Color::from_rgb(255.0, 0.0, 0.0)} else {Color::from_rgb(255.0, 255.0, 255.0)}).into()]).spacing(20));
        
       
        main.into()
    }

    pub fn update(&mut self, message:Message) -> Task<Message>{
        match message {
            Message::Reset(time_only) => {
                if self.reset {
                    if !time_only {
                        self.tasks =  Vec::new();
                        self.add = false;
                        self.complete = 0;
                        self.removed = 0;
                        self.rest = false;
                    }
                    self.start = Instant::now();
                    self.old_dur = Duration::new(0,0);
                    self.stopwatch = Duration::new(0,0);
                    self.stop_string = String::new();
                    self.last_time = Duration::new(0,0);
                    self.last_string = String::new();
                    self.pause_dur = Duration::new(0,0);
                    self.break_start = Instant::now();
                    self.break_string = String::new();
                    self.breaks = Duration::new(0,0);
                    self.reset = false;
                } else {
                    self.reset = true;
                }
                Task::none()
            },
            Message::New => {
                if self.add {
                    self.add = false; 
                } else {
                    self.add = true;
                    self.tasks.push(String::new());
                }
                Task::none()
            }
            Message::Cancel => {
                if self.add {
                    self.add = false;
                    self.tasks.remove(self.tasks.len() - 1);
                } else if self.reset {
                    self.reset = false;
                }
                Task::none()
            }
            Message::End => {
                self.add = false;
                Task::none()
            }
            Message::AddTask(task, index) => {
                self.tasks[index as usize] = task;
                Task::none()
            },
            Message::RemoveTask(task_num, completed) => {
                // Remove task from Vec
                self.tasks.remove(task_num as usize);

                // Caculate the total time it took for the task + move to last_time
                self.last_time = self.stopwatch + self.old_dur;
                self.last_string = format_duration(self.last_time);

                // Reset current task time
                self.stopwatch = Duration::new(0,0);
                self.start = Instant::now();
                self.old_dur = Duration::new(0,0);

                if completed {
                    self.complete += 1;
                } else {
                    self.removed += 1;
                }
                Task::none()
            },
            Message::Tick => {
                if Local::now() != self.time {
                    self.time = Local::now();
                    self.clock = self.time.format("%d/%m/%Y %H:%M:%S").to_string();
                    if !self.rest {
                        self.stopwatch = self.start.elapsed();
                        self.stop_string = format_duration(self.stopwatch + self.old_dur);
                    } else {
                        if !self.sleep {
                            self.breaks = self.break_start.elapsed();
                            self.break_string = format_duration(self.breaks + self.pause_dur);
                        }
                    }

                    self.tick_count += 1;
                    if self.tick_count == 120 && !self.sleep && self.auto_save{
                        // let _ = Task::perform(save_to_file(format!("{}/saves/{}", env!("CARGO_MANIFEST_DIR"), format!("auto{}_save.json", self.time.format("%H_%M").to_string())).into(), self.clone()), |result| Message::FileSave(result));
                        Task::perform(save_to_file(format!("{}/saves/", env!("CARGO_MANIFEST_DIR")).into(), "auto_save.json".to_owned(), self.clone()), |result| Message::FileSave(result))
                    } else {
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            },
            Message::Break => {
                if !self.rest{
                    // Start break + Add current task time to old_dur
                    self.rest = true;
                    self.old_dur += self.start.elapsed();
                    self.break_start = Instant::now();

                    self.stopwatch = Duration::new(0,0);
                } else {
                    self.rest = false;
                    if !self.sleep {
                        self.pause_dur += self.break_start.elapsed();
                    } 
                    self.breaks = Duration::new(0,0);
                    self.sleep = false;
                    self.start = Instant::now();
                }
                Task::none()
            },
            Message::Sleep => {
                if self.sleep {
                    if self.rest {
                        self.break_start = Instant::now();
                    }
                } else {
                    if self.rest {
                        self.pause_dur += self.break_start.elapsed();
                    }
                }
                self.sleep = !self.sleep;
                Task::none()
            },
            Message::Save => {
                // println!("Recieved Save Message");
                Task::perform(save_to_file(format!("{}/saves/", env!("CARGO_MANIFEST_DIR")).into(), "saves.json".to_owned(),self.clone()), |result| Message::FileSave(result))
            },
            Message::Open => {
                // println!("Recieved Save Message");
                Task::perform(read_from_file(format!("{}/saves/", env!("CARGO_MANIFEST_DIR")).into(), "saves.json".to_owned()), |result| Message::FileOpen(result))
            },
            Message::FileSave(result) => {
                match result {
                    Ok(_) => {},
                    Err(err) => {eprintln!("File Save failed {}", err)},
                }
                Task::none()
            }
            Message::FileOpen(result) => {
                match result {
                    Ok(new) => {self.replace(new)},
                    Err(err) => {eprintln!("File Load failed {}", err)}
                }
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message>{
        iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }



}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let hours = secs / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}


fn main() -> iced::Result<> {
    iced::application("To Do List", ToDo::update, ToDo::view).subscription(ToDo::subscription).run()
}
