use std::collections::HashMap;
use std::io::{self, Write};
use std::num::ParseIntError;

fn main() -> io::Result<()> {
    loop {
        print_menu()?;
        let choice = read_line("Choose a lesson: ")?;

        match choice.trim() {
            "1" => lesson_variables(),
            "2" => lesson_control_flow(),
            "3" => lesson_ownership(),
            "4" => lesson_structs_and_enums(),
            "5" => lesson_collections_and_iterators(),
            "6" => lesson_errors(),
            "7" => lesson_traits_and_generics(),
            "8" => show_challenges(),
            "q" | "Q" => {
                println!("Keep experimenting. See you next time!");
                break;
            }
            _ => println!("Please enter a number from 1 to 8, or q to quit."),
        }

        println!();
    }

    Ok(())
}

fn print_menu() -> io::Result<()> {
    println!("=== Rust Learning Lab ===");
    println!("1. Variables, mutability, and shadowing");
    println!("2. Control flow and loops");
    println!("3. Ownership and borrowing");
    println!("4. Structs, enums, and pattern matching");
    println!("5. Collections and iterators");
    println!("6. Result and error handling");
    println!("7. Traits and generics");
    println!("8. Your challenges");
    println!("q. Quit");
    io::stdout().flush()
}

fn read_line(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn lesson_variables() {
    let language = "Rust";
    let mut attempts = 1;
    attempts += 1;

    let language = language.to_uppercase();
    let version = 2021;
    let is_fun = true;

    println!("Shadowing made the language name {language}.");
    println!("Mutable attempts: {attempts}; edition: {version}; fun: {is_fun}");
    println!("A let binding is immutable by default. Use mut when a value must change.");
}

fn lesson_control_flow() {
    let scores = [8, 3, 10, 6];
    let mut total = 0;

    for score in scores {
        if score >= 6 {
            println!("{score}: passed");
        } else {
            println!("{score}: try again");
        }
        total += score;
    }

    let average = total as f64 / scores.len() as f64;
    println!("Average: {average:.1}");
    println!("for iterates over a collection; if and else choose a branch.");
}

fn lesson_ownership() {
    let message = String::from("ownership makes memory rules explicit");
    let length = byte_length(&message);
    println!("Borrowed the message to count its {length} bytes: {message}");

    let message = add_period(message);
    println!("The function took ownership and returned it: {message}");

    let mut editable = String::from("Borrowing");
    add_exclamation(&mut editable);
    println!("Mutably borrowed and changed it: {editable}");
}

fn byte_length(text: &str) -> usize {
    text.len()
}

fn add_period(mut text: String) -> String {
    text.push('.');
    text
}

fn add_exclamation(text: &mut String) {
    text.push('!');
}

#[derive(Debug)]
struct Task {
    title: String,
    state: TaskState,
}

#[derive(Debug)]
enum TaskState {
    Todo,
    InProgress,
    Done,
}

impl Task {
    fn describe(&self) -> String {
        format!("{} [{}]", self.title, self.state.label())
    }
}

impl TaskState {
    fn label(&self) -> &'static str {
        match self {
            TaskState::Todo => "todo",
            TaskState::InProgress => "in progress",
            TaskState::Done => "done",
        }
    }
}

fn lesson_structs_and_enums() {
    let task = Task {
        title: String::from("Learn pattern matching"),
        state: TaskState::InProgress,
    };

    println!("{}", task.describe());
    println!("A struct groups related data; an enum lists valid alternatives.");
    println!("match handles every TaskState variant, so the compiler checks coverage.");
}

fn lesson_collections_and_iterators() {
    let notes = vec!["borrow", "own", "borrow", "move", "borrow"];
    let mut counts = HashMap::new();

    for note in &notes {
        *counts.entry(*note).or_insert(0) += 1;
    }

    let borrowed_count = notes.iter().filter(|word| **word == "borrow").count();
    println!("Word counts: {counts:?}");
    println!("An iterator found {borrowed_count} uses of 'borrow'.");
    println!("Vec stores a growable list; HashMap stores key/value pairs.");
}

fn parse_score(input: &str) -> Result<u8, ParseIntError> {
    input.parse()
}

fn lesson_errors() {
    for input in ["42", "not a score"] {
        match parse_score(input) {
            Ok(score) => println!("Parsed score: {score}"),
            Err(error) => println!("Could not parse {input:?}: {error}"),
        }
    }

    println!("Result is either Ok(value) or Err(error); match forces you to handle both.");
}

trait Summary {
    fn summary(&self) -> String;
}

struct Article {
    headline: String,
    author: String,
}

struct Post {
    author: String,
    body: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("{} - {}", self.headline, self.author)
    }
}

impl Summary for Post {
    fn summary(&self) -> String {
        format!("{} posted: {}", self.author, self.body)
    }
}

fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}

fn lesson_traits_and_generics() {
    let article = Article {
        headline: String::from("Rust makes ownership visible"),
        author: String::from("Ada"),
    };
    let post = Post {
        author: String::from("Linus"),
        body: String::from("Traits describe shared behavior."),
    };

    print_summary(&article);
    print_summary(&post);
    println!("Both types implement Summary; the function accepts either implementation.");
}

fn show_challenges() {
    println!("=== Challenges ===");
    println!("1. Add a Priority enum to Task with Low, Normal, and High variants.");
    println!("   Show its priority in Task::describe and add a match arm for every variant.");
    println!("2. Write count_words(text: &str) -> HashMap<String, usize>.");
    println!("   Split on whitespace, normalize case, and count each word.");
    println!("3. Add Task::complete(&mut self) -> Result<(), String>.");
    println!("   Completing a Todo or InProgress task should work; completing Done should return Err.");
    println!("4. Add a menu option that reads a line from the user and runs your word counter on it.");
    println!("   Keep handling input errors with io::Result instead of panicking.");
    println!("Tip: start with challenge 3 and write a unit test for both success and error paths.");
}

#[cfg(test)]
mod tests {
    use super::{parse_score, TaskState};

    #[test]
    fn parses_valid_scores() {
        assert_eq!(parse_score("42").unwrap(), 42);
    }

    #[test]
    fn rejects_invalid_scores() {
        assert!(parse_score("hello").is_err());
    }

    #[test]
    fn task_states_have_readable_labels() {
        assert_eq!(TaskState::Todo.label(), "todo");
        assert_eq!(TaskState::Done.label(), "done");
    }
}