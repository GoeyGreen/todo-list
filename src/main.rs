use iced::{Alignment, Element, Length, Subscription};
use iced::widget::{button, horizontal_space, text, text_input, Column, Container, Row};
use chrono::prelude::{DateTime, Local};

#[derive(Debug, Default, PartialEq)]
struct ToDo{
    time: DateTime<Local>,
    clock: String,
    tasks: Vec<String>,
    add: bool,
}

#[derive(Debug, Clone)]
enum Message{
    Reset,
    New,
    Cancel,
    End,
    AddTask(String, i32),
    RemoveTask(i32),
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
        for (index, task) in self.tasks.clone().into_iter().enumerate(){
            if index != self.tasks.len() - 1 {
                main = main.push(
                    Container::new(
                        Row::new().push(button("Complete").on_press(Message::RemoveTask(index as i32)))
                        .push(text(task).size(16))
                        .push(horizontal_space())
                        .push(button("Remove").on_press(Message::RemoveTask(index as i32))).spacing(20).width(Length::Fill)
                    )
                );
            } else if !self.add && index == self.tasks.len() - 1 {
                main = main.push(
                    Container::new(
                        Row::new().push(button("Complete").on_press(Message::RemoveTask(index as i32)))
                        .push(text(task).size(16))
                        .push(horizontal_space())
                        .push(button("Remove").on_press(Message::RemoveTask(index as i32))).spacing(20).width(Length::Fill)
                    )
                );
            }
        }
        
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
            Message::RemoveTask(task_num) => {
                self.tasks.remove(task_num as usize);
            },
            Message::Tick => {
                if Local::now() != self.time {
                    self.time = Local::now();
                    self.clock = self.time.format("%d/%m/%Y %H:%M:%S").to_string()
                }
            },
        }
    }

    pub fn subscription(&self) -> Subscription<Message>{
        iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick)
    }



}



fn main() -> iced::Result<> {
    iced::application("To Do List", ToDo::update, ToDo::view).subscription(ToDo::subscription).run()
}
