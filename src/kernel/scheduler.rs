use crate::task::Task;

pub struct Scheduler {
    tasks: Vec<Task>,
    current_task: usize,
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            tasks: Vec::new(),
            current_task: 0,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn run(&mut self) {
        while !self.tasks.is_empty() {
            let task = &mut self.tasks[self.current_task];
            task.run();
            if task.is_done() {
                self.tasks.remove(self.current_task);
            } else {
                self.current_task = (self.current_task + 1) % self.tasks.len();
            }
        }
    }
}
