//To avoid having common appear in the test output, instead of creating tests/common.rs, we’ll create tests/common/mod.rs.
//This is an alternate naming convention that Rust also understands. Naming the file this way tells Rust not to treat the common module as an integration test file. 
//When we move the setup function code into tests/common/mod.rs and delete the tests/common.rs file, the section in the test output will no longer appear.

pub fn setup() {
    // setup code specific to your library's tests would go here
}
//Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

//After we’ve created tests/common/mod.rs, we can use it from any of the integration test files as a module. 
//Here’s an example of calling the setup function from the it_adds_two test in tests/integration_test.rs:
//check - tests/integration_test.rs