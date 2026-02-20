use std::env::current_dir;
use std::fs;
use std::io;
use std::env;
use std::path::PathBuf;

struct Node {
    is_dir: bool,
    size: u32,
    parent_index: u32,
    child_start: u32,
    child_end: u32,
    name: String,
}

//prints out our data structure in tree-form
fn pretty_print(){}

//converts the given size to the largest denomination possible and returns it
fn convert_size(){}

//gets the size of the given file or dir
fn get_size(){}

//creates an instance of the node struct we use for organization
fn create_node(){}

//return the number of objects present in a given directory
fn dir_object_count(){}

//get the contents of the given directory
fn parse_dir(dir:PathBuf){
    let items = fs::read_dir(dir).unwrap();
    for item in items {
        //we can get the filename from the path, which of course we need to unwrap, but we can then display
        println!("Name: {}", item.unwrap().path().file_name().unwrap().display())
    }

}



//Need to support command line args, namely "-a" for getting all sub directories (Eventually)
fn main() -> Result<(), std::io::Error>{
    //initial thrust is getting the backend put together. 
    //We can use a vec of a custom struct
    //that holds the name, size in bits, type (file or directory, probably 0/1), index of parent,  
    //and a starting index in the vec itself if it is a directory.

    //[0,1,2,3...n]

    //get current directory of where the command is being invoked
    let cwd:PathBuf = current_dir()?;
    //We can get the current working directory using current_dir(), which returns a Result we need to unpack using "?"
    //In order to best do this, I recommend having main return a "Result<(), std::io::Error>"
    //This allows us to immediately exit if it runs into any issues with current dir
    //We need to end with Ok(()) to signal that the program has exited successfully

    //we can run this in the testing folder with "./../src/main"
    //Also, to ignore compiler warnings, we can compile with the -Awarnings flag
    //"rustc main.rs -Awarnings" in src

    //Following that, lets try getting the contents of the dir and printing it out
    println!("CWD: {}", cwd.display());
    parse_dir(cwd);

    Ok(())
}
