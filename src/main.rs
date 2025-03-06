use std::io;

fn main() {
   let mut notes: Vec<String>= Vec::new();
   loop{
    println!("\nOptions:");
    println!("1. Add a note");
        println!("2. View all notes");
        println!("3. Exit");
        println!("4. delete a particular note");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to input");

        match choice.trim(){
            "1"=>{
                println!("enter ur note:");
                let mut note=String::new();
                io::stdin().read_line(&mut note).expect("failed");
                let note=note.trim().to_string();
                notes.push(note);
                println!("note added!");
            }
            "2"=>{
                if notes.is_empty(){
                    println!("no notes");
                }else{
                    println!("ur notes");
                    for(index,note) in notes.iter().enumerate(){
                        println!("{}.{}",index+1,note);
                    }
                }
            }
            "3"=>{
                println!("gudbye");
                break;
            }
            "4"=>{
                if notes.is_empty(){
                    println!("no notes to dlt");
                }else{
                    println!("these are all ur notes");
        
        for(index,note) in notes.iter().enumerate(){
            println!("{} => {}",index+1,note);
        }
        println!("Enter the number of the note to delete:");
                }

                let mut todelete=String::new();
                io::stdin().read_line(&mut todelete).expect("failed for id");
               match todelete.trim().parse::<usize>(){
                Ok(num)=>{
                    if num>0 && num<=notes.len(){
                        notes.remove(num-1);
                        println!("note deleted");
                    }else{
                        println!("invalid");
                    }
                }
                Err(_)=>println!("not valid number"),
               }
            }
            _=>println!("invalid")
        }
   }
}