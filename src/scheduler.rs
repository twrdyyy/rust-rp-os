
type Task = fn() -> ();

pub struct Scheduler {
    tasks_list: [Option<Task>; 10]
}

impl Scheduler {
    
    pub fn take_task(&mut self) -> Option<Task> {
        let mut i: usize = 0;
        for task_option in &self.tasks_list
        {
            if *task_option!=None
            {   
                let result: Option<Task> = self.tasks_list[i];
                self.tasks_list[i]=None;
                return result;
            }
            i+=1;
        }
        return None;
    }

    pub fn add_task(&mut self, task: &Task) {
        let mut i: usize = 0;
        for task_option in &self.tasks_list
        {
            if *task_option==None
            {
                self.tasks_list[i]=Some(*task);
                return;
            }
            i+=1;
        }
        
    }
}
pub static mut SCHEDULER: Scheduler = Scheduler {
    tasks_list: [None; 10]
};