pub struct ListItem
{
    pub message: String,
    contexts: Vec<String>
}


impl ListItem
{
    pub fn new(message: String) -> Self
    {
        ListItem
        {
            message,
            contexts: Vec::<String>::new()
        }
    }


    pub fn contexts(&self) -> &Vec<String> { &self.contexts }


    pub fn add_context(&mut self, context: String) -> &mut Self
    {
        self.contexts.push(context);

        self
    }
}


pub struct List
{
    pub name: String,
    items: Vec<ListItem>,
}


impl List
{
    pub fn new(name: String) -> Self
    {
        List
        {
            name,
            items: Vec::<ListItem>::new(),
        }
    }


    pub fn len(&self) -> usize
    {
        self.items().len()
    }


    pub fn items(&self) -> &Vec<ListItem> { &self.items }


    pub fn mut_items(&mut self) -> &mut Vec<ListItem> { &mut self.items }


    pub fn push_item(&mut self, item: ListItem) -> &mut Self
    {
        self.items.push(item);

        self
    }


    pub fn move_item(&mut self, index: usize, target_list: &mut List)
    {
        target_list.items.push(self.items.remove(index));
    }
}
