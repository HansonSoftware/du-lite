use std::env::current_dir;
use std::fs;
use std::io;
use std::env;
use std::path::PathBuf;

struct Node {
    is_dir: bool,
    size: u64,
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
// fn get_size(){}
//No longer needed, we can get this from metadata

//creates an instance of the node struct we use for organization
fn create_node(p: &PathBuf) ->Result<Node, std::io::Error>{

    let f_name = p.file_name().unwrap();
    let metadata = fs::metadata(p)?;

    let new_node: Node = Node {
        is_dir: p.is_dir(),
        size: metadata.len(), //file's size in bytes
        parent_index: 0, //placeholder
        child_start: 0, //placeholder
        child_end: 0, //placeholder
        name: f_name.to_str().unwrap().to_string(),
    };
    Ok(new_node)
}

//return the number of objects present in a given directory
fn dir_object_count(){}

//get the contents of the given directory
fn parse_dir(dir:PathBuf) -> Result<(), std::io::Error> {
    let items = fs::read_dir(dir).unwrap();
    for item in items {
        //we can get the filename from the path, which of course we need to unwrap, but we can then display
        //Have to pass in the PathBuf item as a pointer so that the create_node function can properly use it to create a metadata obj
        let new_node: Node = create_node(&item.unwrap().path())?;
        println!("Name: {}, Size: {}, is_dir {}", new_node.name, new_node.size, new_node.is_dir)
        
    }
    Ok(())
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
    parse_dir(cwd)?;

    Ok(())
}
