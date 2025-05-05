use std::time::{Duration, Instant};

use iced::{Alignment, Color, Element, Length, Subscription, Theme};
use iced::widget::{button, horizontal_space, text, text_input, Column, Container, Row, Scrollable};
use chrono::prelude::{DateTime, Local};

#[derive(Debug, PartialEq)]
struct ToDo{
    time: DateTime<Local>,
    clock: String,
    tasks: Vec<String>,
    add: bool,
    complete: i32,
    removed: i32,
    start: Instant,
    stopwatch: Duration,
    stop_string: String,
    last_time: Duration,
    last_string: String,
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
            stopwatch:Duration::new(0,0),
            stop_string: String::new(),
            last_time:Duration::new(0,0),
            last_string: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message{
    Reset,
    New,
    Cancel,
    End,
    AddTask(String, i32),
    RemoveTask(i32, bool),
    Tick,

}

impl ToDo {
    pub fn view(&self) -> Element<Message>{
        let mut main: Column<'_, Message> = Column::with_children(vec![]).align_x(Alignment::Center).width(Length::Fill).padding(20).spacing(10).into();
        main = main.push(
            Row::new().push(Row::new().push(button(if !self.add {"New Task"} else {"Confirm"}).on_press(Message::New))
                .push_maybe(if self.add {Some(button("Cancel").on_press(Message::Cancel))} else {None}).spacing(10))
                .push(horizontal_space())
                .push(button("Reset").on_press(Message::Reset)).spacing(10));
        main = main.push(text("To Do List: ").size(20)).push(text(&self.clock).size(16)).push(text("").size(30));
        let mut tasks: Column<'_, Message> = Column::new().align_x(Alignment::Center).width(Length::Fill).padding(20).spacing(10).into();
        for (index, task) in self.tasks.clone().into_iter().enumerate(){
            if index != self.tasks.len() - 1 {
                tasks = tasks.push(
                    Container::new(
                        Row::new().push(button("Complete").on_press(Message::RemoveTask(index as i32, true)))
                        .push(text(task).size(16))
                        .push(horizontal_space())
                        .push(button("Remove").on_press(Message::RemoveTask(index as i32, false)).style(|_: &Theme, status| {
                            match status {
                                button::Status::Active => {
                                    button::Style::default()
                                       .with_background(Color::from_rgb(255.0, 0.0, 0.0),)
                                }
                                _ => {
                                    let mut style = button::Style::default()
                                       .with_background(Color::from_rgb(70.0, 0.0, 0.0),);
                                    style.text_color = Color::from_rgb(255.0, 255.0, 255.0);
                                    style
                                },
                            }
                        }))
                        .spacing(20).width(Length::Fill)
                    )
                );
            } else if !self.add && index == self.tasks.len() - 1 {
                tasks = tasks.push(
                    Container::new(
                        Row::new().push(button("Complete").on_press(Message::RemoveTask(index as i32, true)))
                        .push(text(task).size(16))
                        .push(horizontal_space())
                        .push(button("Remove").on_press(Message::RemoveTask(index as i32, false)).style(|_: &Theme, status| {
                            match status {
                                button::Status::Active => {
                                    let mut style = button::Style::default()
                                       .with_background(Color::from_rgb(255.0, 0.0, 0.0),);
                                    style.text_color = Color::from_rgb(255.0, 255.0, 255.0);
                                    style
                                }
                                _ => {
                                    let mut style = button::Style::default()
                                       .with_background(Color::from_rgb(70.0, 0.0, 0.0),);
                                    style.text_color = Color::from_rgb(255.0, 255.0, 255.0);
                                    style
                                },
                            }
                        }))
                        .spacing(20).width(Length::Fill)
                    )
                );
            }
        }
        
        main = main.push(Scrollable::new(tasks));
        main = main.push(Row::with_children(vec![text(format!("Current Task: {}", self.stop_string)).into(), text(format!("Last Task: {}", self.last_string)).into()]).spacing(20));
        if self.add {
            main = main.push(Container::new(
                text_input("New Task ...", &self.tasks[self.tasks.len() - 1])
                    .on_input(|content:String | Message::AddTask(content, (self.tasks.len() - 1) as i32))
                    .on_submit(Message::End)));
        }
        main.into()
    }

    pub fn update(&mut self, message:Message) {
        match message {
            Message::Reset => {
                self.tasks.clear();
            },
            Message::New => {
                if self.add {
                    self.add = false; 
                } else {
                    self.add = true;
                    self.tasks.push(String::new());
                }
            }
            Message::Cancel => {
                self.add = false;
                self.tasks.remove(self.tasks.len() - 1);
            }
            Message::End => {
                self.add = false;
            }
            Message::AddTask(task, index) => {
                self.tasks[index as usize] = task;
            },
            Message::RemoveTask(task_num, completed) => {
                self.tasks.remove(task_num as usize);
                self.last_time = self.stopwatch;
                self.last_string = format_duration(self.last_time);
                self.stopwatch = Duration::new(0,0);
                self.start = Instant::now();
            
                if completed {
                    self.complete += 1;
                } else {
                    self.removed += 1;
                }
            },
            Message::Tick => {
                if Local::now() != self.time {
                    self.time = Local::now();
                    self.clock = self.time.format("%d/%m/%Y %H:%M:%S").to_string();
                    self.stopwatch = self.start.elapsed();
                    self.stop_string = format_duration(self.stopwatch);

                }
            },
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
