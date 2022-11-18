//Rc<T>, the Reference Counted Smart Pointer

/*
In the majority of cases, ownership is clear: you know exactly which variable owns a given value. 
However, there are cases when a single value might have multiple owners. 
For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it. 
A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it.

To enable multiple ownership, Rust has a type called Rc<T>, which is an abbreviation for reference counting. 
The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use. 
If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

Imagine Rc<T> as a TV in a family room. When one person enters to watch TV, they turn it on. 
Others can come into the room and watch the TV. When the last person leaves the room, they turn off the TV because it’s no longer being used. 
If someone turns off the TV while others are still watching it, there would be uproar from the remaining TV watchers!

We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read 
and we can’t determine at compile time which part will finish using the data last.
If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

Note that Rc<T> is only for use in single-threaded scenarios. When we discuss concurrency in Chapter 16, we’ll cover how to do reference counting in multithreaded programs.
*/



//Using Rc<T> to Share Data
/*
Let’s return to our cons list example in Listing 15-5. Recall that we defined it using Box<T>. This time, we’ll create two lists that both share ownership of a third list.

We’ll create list a that contains 5 and then 10. 
Then we’ll make two more lists: b that starts with 3 and c that starts with 4. Both b and c lists will then continue on to the first a list containing 5 and 10. 
In other words, both lists will share the first list containing 5 and 10.

Trying to implement this scenario using our definition of List with Box<T> won’t work, as shown in Listing 15-17:

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use crate::List::{Cons, Nil};

    fn main() {
        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let b = Cons(3, Box::new(a));
        let c = Cons(4, Box::new(a));
    }

    Listing 15-17

When we compile this code, we get this error:

    $ cargo run
    Compiling cons-list v0.1.0 (file:///projects/cons-list)
    error[E0382]: use of moved value: `a`
    --> src/main.rs:11:30
    |
    9  |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
    10 |     let b = Cons(3, Box::new(a));
    |                              - value moved here
    11 |     let c = Cons(4, Box::new(a));
    |                              ^ value used here after move

    For more information about this error, try `rustc --explain E0382`.
    error: could not compile `cons-list` due to previous error


The Cons variants own the data they hold, so when we create the b list, a is moved into b and b owns a. 
Then, when we try to use a again when creating c, we’re not allowed to because a has been moved.

We could change the definition of Cons to hold references instead, but then we would have to specify lifetime parameters. 
By specifying lifetime parameters, we would be specifying that every element in the list will live at least as long as the entire list. 
This is the case for the elements and lists in Listing 15-17, but not in every scenario.

Instead, we’ll change our definition of List to use Rc<T> in place of Box<T>, as shown in Listing 15-18. 
Each Cons variant will now hold a value and an Rc<T> pointing to a List. When we create b, instead of taking ownership of a, 
we’ll clone the Rc<List> that a is holding, thereby increasing the number of references from one to two and letting a and b share ownership of the data in that Rc<List>. 
We’ll also clone a when creating c, increasing the number of references from two to three. 
Every time we call Rc::clone, the reference count to the data within the Rc<List> will increase, and the data won’t be cleaned up unless there are zero references to it.

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use crate::List::{Cons, Nil};
    use std::rc::Rc;

    fn main() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));
    }

    Listing 15-18


We need to add a use statement to bring Rc<T> into scope because it’s not in the prelude. 
In main, we create the list holding 5 and 10 and store it in a new Rc<List> in a. 
Then when we create b and c, we call the Rc::clone function and pass a reference to the Rc<List> in a as an argument.

We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case. 
The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do. 
The call to Rc::clone only increments the reference count, which doesn’t take much time. 
Deep copies of data can take a lot of time. By using Rc::clone for reference counting, 
we can visually distinguish between the deep-copy kinds of clones and the kinds of clones that increase the reference count. 
When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to Rc::clone.
*/


//Cloning an Rc<T> Increases the Reference Count

/*
Let’s change our working example in Listing 15-18 so we can see the reference counts changing as we create and drop references to the Rc<List> in a.

In Listing 15-19, we’ll change main so it has an inner scope around list c; then we can see how the reference count changes when c goes out of scope.

    fn main() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    Listing 15-19


At each point in the program where the reference count changes, we print the reference count, which we can get by calling the Rc::strong_count function. 
This function is named strong_count rather than count because the Rc<T> type also has a weak_count.

    $ cargo run
    Compiling cons-list v0.1.0 (file:///projects/cons-list)
        Finished dev [unoptimized + debuginfo] target(s) in 0.45s
        Running `target/debug/cons-list`
    count after creating a = 1
    count after creating b = 2
    count after creating c = 3
    count after c goes out of scope = 2


We can see that the Rc<List> in a has an initial reference count of 1; then each time we call clone, the count goes up by 1. 
When c goes out of scope, the count goes down by 1. 
We don’t have to call a function to decrease the reference count like we have to call Rc::clone to increase the reference count: 
the implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.

What we can’t see in this example is that when b and then a go out of scope at the end of main, the count is then 0, and the Rc<List> is cleaned up completely at that point. 
Using Rc<T> allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners still exist.

Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only. 
If Rc<T> allowed you to have multiple mutable references too, you might violate one of the borrowing rules discussed in Chapter 4: 
multiple mutable borrows to the same place can cause data races and inconsistencies.

But being able to mutate data is very useful! In the next section, we’ll discuss the interior mutability pattern and the RefCell<T> type 
that you can use in conjunction with an Rc<T> to work with this immutability restriction.
*/