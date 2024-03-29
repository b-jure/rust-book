//Using Message Passing to Transfer Data Between Threads

/*
To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels. 
A channel is a general programming concept by which data is sent from one thread to another.
A channel has two halves: a transmitter and a receiver. The transmitter half is the upstream location where you put rubber ducks into the river, 
and the receiver half is where the rubber duck ends up downstream.

First, in Listing 16-6, we’ll create a channel but not do anything with it. 
Note that this won’t compile yet because Rust can’t tell what type of values we want to send over the channel.

    use std::sync::mpsc;

    fn main() {
        let (tx, rx) = mpsc::channel();
    }
    Listing 16-6


We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer. 
In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that 
produce values but only one receiving end that consumes those values. 
Imagine multiple streams flowing together into one big river: everything sent down any of the streams will end up in one river at the end.


The mpsc::channel function returns a tuple, the first element of which is the sending end--the transmitter--and the second element is the receiving end--the receiver. 
The abbreviations tx and rx are traditionally used in many fields for transmitter and receiver respectively, so we name our variables as such to indicate each end.

We’re using a let statement with a pattern that destructures the tuples; we’ll discuss the use of patterns in let statements and destructuring in Chapter 18. 
For now, know that using a let statement this way is a convenient approach to extract the pieces of the tuple returned by mpsc::channel.

Let’s move the transmitting end into a spawned thread and have it send one string so the spawned thread is communicating with the main thread, as shown in Listing 16-7. 
This is like putting a rubber duck in the river upstream or sending a chat message from one thread to another.

    use std::sync::mpsc;
    use std::thread;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
    }
    Listing 16-7


Again, we’re using thread::spawn to create a new thread and then using move to move tx into the closure so the spawned thread owns tx. 
The spawned thread needs to own the transmitter to be able to send messages through the channel. 
The transmitter has a send method that takes the value we want to send. 
The send method returns a Result<T, E> type, so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error. 
In this example, we’re calling unwrap to panic in case of an error. But in a real application, we would handle it properly.

In Listing 16-8, we’ll get the value from the receiver in the main thread. 
This is like retrieving the rubber duck from the water at the end of the river or receiving a chat message.

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
    Listing 16-8


The receiver has two useful methods: recv and try_recv.

We’re using recv, short for receive, which will block the main thread’s execution and wait until a value is sent down the channel. 
Once a value is sent, recv will return it in a Result<T, E>. 
When the transmitter closes, recv will return an error to signal that no more values will be coming.

The try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available 
and an Err value if there aren’t any messages this time. 
Using try_recv is useful if this thread has other work to do while waiting for messages: we could write a loop that calls try_recv every so often, 
handles a message if one is available, and otherwise does other work for a little while until checking again.

We’ve used recv in this example for simplicity; we don’t have any other work for the main thread to do other than wait for messages, 
so blocking the main thread is appropriate.

When we run the code in Listing 16-8, we’ll see the value printed from the main thread:

    Got: hi
*/



//Channels and Ownership Transference

/*
The ownership rules play a vital role in message sending because they help you write safe, concurrent code. 
Preventing errors in concurrent programming is the advantage of thinking about ownership throughout your Rust programs. 
Let’s do an experiment to show how channels and ownership work together to prevent problems: 
we’ll try to use a val value in the spawned thread after we’ve sent it down the channel. 
Try compiling the code in Listing 16-9 to see why this code isn’t allowed:

    use std::sync::mpsc;
    use std::thread;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            println!("val is {}", val);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }
    Listing 16-9


Here, we try to print val after we’ve sent it down the channel via tx.send.
Allowing this would be a bad idea: once the value has been sent to another thread, that thread could modify or drop it before we try to use the value again. 
Potentially, the other thread’s modifications could cause errors or unexpected results due to inconsistent or nonexistent data. 
However, Rust gives us an error if we try to compile the code in Listing 16-9:

    $ cargo run
    Compiling message-passing v0.1.0 (file:///projects/message-passing)
    error[E0382]: borrow of moved value: `val`
    --> src/main.rs:10:31
    |
    8  |         let val = String::from("hi");
    |             --- move occurs because `val` has type `String`, which does not implement the `Copy` trait
    9  |         tx.send(val).unwrap();
    |                 --- value moved here
    10 |         println!("val is {}", val);
    |                               ^^^ value borrowed here after move
    |
    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

    For more information about this error, try `rustc --explain E0382`.
    error: could not compile `message-passing` due to previous error


Our concurrency mistake has caused a compile time error. 
The send function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it. 
This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.
*/



//Sending Multiple Values and Seeing the Receiver Waiting

/*
The code in Listing 16-8 compiled and ran, but it didn’t clearly show us that two separate threads were talking to each other over the channel. 
In Listing 16-10 we’ve made some modifications that will prove the code in Listing 16-8 is running concurrently: 
the spawned thread will now send multiple messages and pause for a second between each message.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
    Listing 16-10


This time, the spawned thread has a vector of strings that we want to send to the main thread. 
We iterate over them, sending each individually, and pause between each by calling the thread::sleep function with a Duration value of 1 second.

In the main thread, we’re not calling the recv function explicitly anymore: instead, we’re treating rx as an iterator. 
For each value received, we’re printing it. When the channel is closed, iteration will end.

When running the code in Listing 16-10, you should see the following output with a 1-second pause in between each line:

    Got: hi
    Got: from
    Got: the
    Got: thread


Because we don’t have any code that pauses or delays in the for loop in the main thread, 
we can tell that the main thread is waiting to receive values from the spawned thread.
*/



//Creating Multiple Producers by Cloning the Transmitter

/*
Earlier we mentioned that mpsc was an acronym for multiple producer, single consumer. 
Let’s put mpsc to use and expand the code in Listing 16-10 to create multiple threads that all send values to the same receiver. 
We can do so by cloning the transmitter, as shown in Listing 16-11:

    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    fn main() {
        // --snip--

        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }

        // --snip--
    }
    Listing 16-11


This time, before we create the first spawned thread, we call clone on the transmitter. 
This will give us a new transmitter we can pass to the first spawned thread. 
We pass the original transmitter to a second spawned thread. 
This gives us two threads, each sending different messages to the one receiver.

When you run the code, your output should look something like this:

    Got: hi
    Got: more
    Got: from
    Got: messages
    Got: for
    Got: the
    Got: thread
    Got: you

You might see the values in another order, depending on your system. 
This is what makes concurrency interesting as well as difficult. 
If you experiment with thread::sleep, giving it various values in the different threads, each run will be more nondeterministic and create different output each time.
*/

