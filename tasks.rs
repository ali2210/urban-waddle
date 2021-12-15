

// struct objects
#[derive(Debug)]
struct Todo{
    name : String,
    description : String,
}

#[derive(Debug)]
struct Author<'a>{
    task : &'a Todo,
    name : String,
}

// traits called special functions. Todo have one special function this task created by whom
trait Features{
    fn view(&self , name : String) ->  Author;
}

// traits definition 
impl Features for Todo{
    fn view(&self, name : String) -> Author{
        // set whom field 
        let creator : Author = Author{task : self, name: name };
        creator
    }
}

fn main(){
    // create a new task
    let task = new(String::from("rust program"), String::from("simple program"));
    println!("task created ! ({:#?})", task.view(String::from("Ali")));
}


// create new task object
fn new(name : String, description : String) -> Todo{
    
    let todo = Todo{name, description};
    todo
}
