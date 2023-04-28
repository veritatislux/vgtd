use std::time::Instant;


pub struct Task
{
    pub message: String,
    pub _details: String,
    contexts: Vec<String>,
    _creation_time: Instant,
}


impl Task
{
    pub fn new(message: String) -> Self
    {
        Task
        {
            message,
            _details: String::new(),
            contexts: Vec::<String>::new(),
            _creation_time: Instant::now(),
        }
    }

    pub fn contexts(&self) -> &Vec<String> { &self.contexts }

    pub fn add_context(&mut self, context: String) -> &mut Self
    {
        self.contexts.push(context);

        self
    }

    pub fn set_contexts(&mut self, contexts: Vec<String>)
    {
        self.contexts = contexts;
    }
}


pub struct List
{
    pub name: String,
    tasks: Vec<Task>,
}


impl List
{
    pub fn new(name: String) -> Self
    {
        List
        {
            name,
            tasks: Vec::<Task>::new(),
        }
    }


    pub fn len(&self) -> usize
    {
        self.tasks().len()
    }


    pub fn is_empty(&self) -> bool { self.tasks.is_empty() }


    pub fn sort<'a>(&mut self, selected_index: usize) -> usize
    {
        // FIXME: I dislike these `.clone()`s greatly.
        let selected_task_message = self.tasks()[selected_index].message.clone();
        let key_function = |task: &Task| task.message.clone();
        self.tasks_mut().sort_by_key(key_function);
        self.tasks().binary_search_by_key(
            &selected_task_message,
            key_function
        ).unwrap()
    }


    pub fn tasks(&self) -> &Vec<Task> { &self.tasks }


    pub fn tasks_mut(&mut self) -> &mut Vec<Task> { &mut self.tasks }


    pub fn push_task(&mut self, task: Task) -> &mut Self
    {
        self.tasks.push(task);

        self
    }


    pub fn remove_task(&mut self, index: usize)
    {
        self.tasks.remove(index);
    }


    pub fn move_task(&mut self, index: usize, target_list: &mut List)
    {
        target_list.tasks.push(self.tasks.remove(index));
    }
}
