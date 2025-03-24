use std::collections::HashMap;

/// # Chapter 3 - Structs
///
/// Let us work on the menu of a library. Create a structure containing book information like accession number, name of
/// author, book title and flag to know whether book is issued or not.
/// Create a menu in which the following can be done.
/// 1 - Display book information
/// 2 - Add a new book
/// 3 - Display all the books in the library of a particular author
/// 4 - Display the number of books of a particular title
/// 5 - Display the total number of books in the library
/// 6 - Issue a book
/// (If we issue a book, then its number gets decreased by 1 and if we add a book, its number gets increased by 1)
fn main() {
    let mut lib = Library {
        books: HashMap::new(),
        curr_num_books: 0,
    };

    lib.add_book(Book::new(
        1,
        String::from("Pone Ding"),
        String::from("Hello"),
        false,
    ));
    lib.add_book(Book::new(
        2,
        String::from("Pone Ding"),
        String::from("Hey"),
        false,
    ));
    lib.add_book(Book::new(
        3,
        String::from("Pone Ding"),
        String::from("Hi"),
        false,
    ));
    lib.add_book(Book::new(
        4,
        String::from("Jay Chou"),
        String::from("Hello"),
        false,
    ));
    lib.add_book(Book::new(
        5,
        String::from("Pone Ding"),
        String::from("你好"),
        false,
    ));

    lib.issue_book(3);
    lib.issue_book(5);

    assert_eq!(lib.curr_num_books, 3);
    lib.show_num_books();

    lib.find_all_author_books("Pone Ding")
        .iter()
        .for_each(|book| book.display_info());

    assert_eq!(lib.find_num_title_books("Hello"), 2);
    let num_hello = lib.find_num_title_books("Hello");
    println!("Number of books with title 'Hello': {}", num_hello);
}

struct Library {
    books: HashMap<i32, Book>,
    curr_num_books: i32,
}

impl Library {
    fn add_book(&mut self, book: Book) {
        self.books.insert(book.accession_number, book);
        self.curr_num_books += 1;
    }

    fn issue_book(&mut self, accession_number: i32) {
        let book = self.books.get_mut(&accession_number).unwrap();
        book.is_issued = true;
        self.curr_num_books -= 1;
    }

    fn show_num_books(&self) {
        println!("books num: {}", self.curr_num_books);
    }

    fn find_all_author_books(&self, author: &str) -> Vec<Book> {
        self.books
            .iter()
            .filter(|(_, book)| book.author == author)
            .map(|(_, book)| book.clone())
            .collect()
    }

    fn find_num_title_books(&self, title: &str) -> i32 {
        self.books
            .iter()
            .filter(|(_, book)| book.title == title)
            .count() as i32
    }
}

#[derive(Clone)]
struct Book {
    accession_number: i32,
    author: String,
    title: String,
    is_issued: bool,
}

impl Book {
    fn new(accession_number: i32, author: String, title: String, is_issued: bool) -> Self {
        Book {
            accession_number,
            author,
            title,
            is_issued,
        }
    }

    fn display_info(&self) {
        println!("Accession Number: {}", self.accession_number);
        println!("Author: {}", self.author);
        println!("Title: {}", self.title);
        println!("Is Issued: {}", self.is_issued);
    }
}
