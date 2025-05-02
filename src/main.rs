use uuid::Uuid;
use std::io;
#[derive(Debug)]
struct Task {
    id: String,
    text: String,
    is_completed: bool
}

impl Task {
    fn new(id: String, text: String, is_completed: bool) -> Self {
        Task {id, text, is_completed}
    }
}
fn main() {
    let mut tasks_list: Vec<Task> = Vec::new();

    println!("Введите задачи по одной. Для выхода введите 'exit'.");

    loop {
        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            println!("Error format");
            return;
        }
        let task_text = input.trim().to_string(); 

        if task_text.eq_ignore_ascii_case("exit") {
            break;
        }

        if task_text.is_empty() {
            println!("Task is empty");
            continue;
        }

        let task_id = Uuid::new_v4().to_string();
        let task = Task::new(task_id, task_text, false);
        tasks_list.push(task);
        
        println!("Задача добавлена!");
    }
    
    
    println!("{:?}", tasks_list);
    println!("\nВведите номер задачи, которую хотите отметить как выполненную (или 'нет' для выхода):");

    let mut complete_task = String::new();
    io::stdin().read_line(&mut complete_task).expect("Ошибка ввода");
    let complete_task = complete_task.trim();

    if let Ok(n) = complete_task.parse::<usize>() {
        if n > 0 && n <=tasks_list.len() {
            tasks_list[n - 1].is_completed = true;
            println!("Задача отмечена как выполненная!");
        } else {
            println!("Нет такой задачи.");
        }
    } else if complete_task.eq_ignore_ascii_case("нет") {
        println!("Выход без изменений.");
    } else {
        println!("Некорректный ввод.");
    }
    println!("\nОбновлённый список задач:");

    for (i, task) in tasks_list.iter().enumerate() {
        println!(
            "{}. {} [{}]",
            i + 1,
            task.text,
            if task.is_completed {"✓"} else {""}
        )
    }
}

