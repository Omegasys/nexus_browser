// Nexus Browser Task Scheduler
// GPL-3.0 License


#[derive(Debug)]
pub enum TaskPriority {


    Critical,

    High,

    Normal,

    Background,


}



pub struct BrowserTask {


    pub name: String,

    pub priority: TaskPriority,


}



pub struct Scheduler {


    tasks:
        Vec<BrowserTask>,


}



impl Scheduler {


    pub fn new() -> Self {


        Self {

            tasks:
                Vec::new(),

        }


    }



    pub fn schedule(
        &mut self,
        task: BrowserTask
    ) {


        println!(
            "Scheduling task {}",
            task.name
        );


        self.tasks.push(
            task
        );


    }



    pub fn execute_next(
        &mut self
    ) {


        if let Some(task) =
            self.tasks.pop()
        {


            println!(
                "Executing {}",
                task.name
            );


        }


    }



}
