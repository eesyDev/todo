use uuid::Uuid;
use std::io;
use iced::{Element, Settings, Sandbox};
use iced::widget::{button, Button, Column, Container, text_input, TextInput, Text, Row};
use chrono::Local;

#[derive(Debug, Clone)]
enum Message {
    TaskInputChanged(String),
    AddTask,
    ToggleTask(usize)
}
#[derive(Debug)]
struct Task {
    id: String,
    text: String,
    is_completed: bool
}

struct TodoApp {
    tasks: Vec<Task>,
    input_value: String,
}

impl Default for TodoApp {
    fn default() -> Self {
        Self {
            tasks: vec![],
            input_value: String::new()
        }
    }
}

impl Task {
    fn new(id: String, text: String, is_completed: bool) -> Self {
        Task {id, text, is_completed}
    }

}

impl Sandbox for TodoApp {
    type Message = Message;
    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        let now = Local::now();
        format!("Todo today: {}", now.format("%Y-%m-%d"))
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TaskInputChanged(val) => {
                self.input_value = val
            },
            Message::AddTask => {
                if !self.input_value.trim().is_empty() {
                    self.tasks.push(
                        Task {
                            id: (self.tasks.len() + 1).to_string(),
                            text: self.input_value.trim().to_string(),
                            is_completed: false
                        }
                    )
                }
            },
            Message::ToggleTask(i) => {
                if let Some(task) = self.tasks.get_mut(i) {
                    task.is_completed = !task.is_completed;
                }
            } 
        }
    }

    fn view(&self) -> Element<Message> {
        let input = TextInput::new(
            "Input task...",
            &self.input_value
        ).on_input(Message::TaskInputChanged);

        let add_button = Button::new("Add").on_press(Message::AddTask);

        let mut col = Column::new().push(Row::new().push(input).push(add_button));

        for (i, task) in self.tasks.iter().enumerate() {
            let row = Row::new().push(
                Button::new(if task.is_completed { "✓" } else { " " }).on_press(Message::ToggleTask(i))
            )
            .push(Text::new(&task.text));

            col = col.push(row);
        }
        Container::new(col).into()
    }
}
fn main() {
    TodoApp::run(Settings::default());
    // let mut tasks_list: Vec<Task> = Vec::new();

    // println!("Введите задачи по одной. Для выхода введите 'exit'.");

    // loop {
    //     let mut input = String::new();

    //     if io::stdin().read_line(&mut input).is_err() {
    //         println!("Error format");
    //         return;
    //     }
    //     let task_text = input.trim().to_string(); 

    //     if task_text.eq_ignore_ascii_case("exit") {
    //         break;
    //     }

    //     if task_text.is_empty() {
    //         println!("Task is empty");
    //         continue;
    //     }

    //     let task_id = Uuid::new_v4().to_string();
    //     let task = Task::new(task_id, task_text, false);
    //     tasks_list.push(task);
        
    //     println!("Задача добавлена!");
    // }
    
    
    // println!("{:?}", tasks_list);
    // println!("\nВведите номер задачи, которую хотите отметить как выполненную (или 'нет' для выхода):");

    // let mut complete_task = String::new();
    // io::stdin().read_line(&mut complete_task).expect("Ошибка ввода");
    // let complete_task = complete_task.trim();

    // if let Ok(n) = complete_task.parse::<usize>() {
    //     if n > 0 && n <=tasks_list.len() {
    //         tasks_list[n - 1].is_completed = true;
    //         println!("Задача отмечена как выполненная!");
    //     } else {
    //         println!("Нет такой задачи.");
    //     }
    // } else if complete_task.eq_ignore_ascii_case("нет") {
    //     println!("Выход без изменений.");
    // } else {
    //     println!("Некорректный ввод.");
    // }
    // println!("\nОбновлённый список задач:");

    // for (i, task) in tasks_list.iter().enumerate() {
    //     println!(
    //         "{}. {} [{}]",
    //         i + 1,
    //         task.text,
    //         if task.is_completed {"✓"} else {""}
    //     )
    // }
}

