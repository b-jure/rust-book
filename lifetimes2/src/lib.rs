//If we try to implement the longest function as shown in Listing 10-21, it won’t compile.
//When we’re defining this function, we don’t know the concrete values that will be passed into this function, 
//so we don’t know whether the if case or the else case will execute.
//We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes as we did in 
//Listings 10-18 and 10-19 to determine whether the reference we return will always be valid. 
//The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value.
//To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

//fn longest(slice1: &str, slice2: &str) -> &str {
//    if slice1.len() => slice2.len() {
//        slice1
//    }else {
//        slice2
//    }
//}
//Listing 10-21



//Lifetime Annotation Syntax

//Lifetime annotations don’t change how long any of the references live.
//Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
//Just as functions can accept any type when the signature specifies a generic type parameter, 
//functions can accept references with any lifetime by specifying a generic lifetime parameter.

//Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters 
//must start with an apostrophe (') and are usually all lowercase and very short, like generic types.
//Most people use the name 'a for the first lifetime annotation.
//We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s type.
//a reference to an i32 without a lifetime parameter, a reference to an i32 that has a lifetime parameter named 'a, 
//and a mutable reference to an i32 that also has the lifetime 'a.
// &i32         a reference
// &'a i32      a reference with an explicit lifetime
// &'a mut i32  a mutable reference with an explicit lifetime

//let’s say we have a function with the parameter first that is a reference to an i32 with lifetime 'a.
//The function also has another parameter named second that is another reference to an i32 that also has the lifetime 'a.
//The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.





//Lifetime Annotations in Function Signatures

//As with generic type parameters, we need to declare generic lifetime parameters inside angle brackets between the function name and the parameter list.
//We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid.
//This is the relationship between lifetimes of the parameters and the return value.
//We’ll name the lifetime 'a and then add it to each reference
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}//The longest function definition specifying that all the references in the signature must have the same lifetime 'a
//The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a.
//The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.
//In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in.
//These relationships are what we want Rust to use when analyzing this code.

//Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. 
//Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints.
//Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.
//When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.
//Having function signatures contain the lifetime contract means the analysis the Rust compiler does can be simpler.
//If there’s a problem with the way a function is annotated or the way it is called, the compiler errors can point to the part of our code and the constraints more precisely.

//When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y.
//In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y.
//Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned 
//reference will also be valid for the length of the smaller of the lifetimes of x and y.

//Let’s look at how the lifetime annotations restrict the longest function by passing in references that have different concrete lifetimes. (main.rs)


