# Sriram Maganti 
Final Project CS 377 <br>
Rust Implementation of Project 1


# Design
## Notes
Moving on to the project, learning about Rust and implementing it within this project was interesting because I learned about all the ways Rust was different from C/C++. One of the ways is that Rust Syntax is typically written using snake_case naming conventions. This is when variable names are separated by underscores and in lowercase. This is different from C++ and its CamelCase naming conventions. Also as you can see, you can make functions public by adding the keyword “pub” in front of fn  so you can call it from other files. I’ll show this in the demo.

Another way Rust differs is that for the fileList Streams in the C++ implementation, you have to close them each time by typing in fileList.close();. However, this is different from Rust because the File Type automatically implements the Drop trait. This means that when the File goes out of scope, it will close by itself and you don’t need to manually close the current file.

In the original implementation, we read the files using a built-in method called getline(). It reads a line of text from an input stream and stores it in a string object. Comparing this to rust, I used a BufReader, which is a type of Reader that provides additional functionality by using an internal buffer to optimize certain reading operations. Without a buffer, operations such as reading a file line-by-line can be inefficient, but BufReader's read_line method and lines iterator make it easy to read files in this way. In addition to line-by-line reading, BufReader also supports other useful methods such as read_until and read_exact. By using BufReader, you can improve the performance of your file reading operations and take advantage of its convenient methods for processing text.

In C++, we used a map of sets to implement the inverted index. In my rust implementation, I decided to use a BTreeMap of sets. A BTreeMap is a map implementation that uses a balance Binary Search tree to store the elements. The keys are stored in the tree in sorted order, which allows for efficient lookup, insertion, and deletion operations while maintaining the sorted order of the keys. This is because the BTreeMap has a characteristic called “Ord”. According to the rust documentation, Ord is a trait for types that form a total order. Total Order In mathematics, a total or linear order is a partial order in which any two elements are comparable. A set equipped with a total order is a totally ordered set.


Finally, we have the `mut` keyword, which is short for "mutable". In Rust, variables are immutable by default, meaning that their values cannot be changed once they have been assigned but can be changed it we add mut in front of let. Ex: let mut monkey;

### Link to Slide-show and Video Demo
https://docs.google.com/presentation/d/1JCkXV8rMzSmWvxcRApW29Ae_8BCGpbQ8wGNQGXQLVCM/edit?usp=sharing

https://drive.google.com/file/d/1UHXFG16L-3neLASwU9xGgYoV_n52Cpe3/view?usp=sharing
