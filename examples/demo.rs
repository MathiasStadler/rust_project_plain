fn main() {
    println!("Creating demo project...");
    
    match rust_project_plain::create_project("demo_project") {
        Ok(_) => println!("Demo project created successfully in /tmp/demo_project"),
        Err(e) => eprintln!("Error creating demo project: {}", e),
    }
}