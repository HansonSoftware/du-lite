use std::fs;
use std::io;

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
fn parse_dir(){}

//Need to support command line args, namely "-a" for getting all sub directories 
fn main() {
    println!("Hello, world!");

    //initial thrust is getting the backend put together. 
    //We can use a vec of a custom struct
    //that holds the name, size in bits, type (file or directory, probably 0/1), index of parent,  
    //and a starting index in the vec itself if it is a directory.

    //[0,1,2,3...n]
}
