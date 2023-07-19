fn main() {
    let mut library = Library::new();
    println!("Library is empty: {}", library.is_empty());
    library.add_book(Book::new("The Name of the Wind", 2007));
    library.add_book(Book::new("A Game of Thrones", 1996));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    println!("Library is empty: {}", library.is_empty());
    match library.oldest_book() {
        Some(book) => println!("The oldest book is {}", book.title),
        None => println!("The library is empty")
    }
    println!("Library has {} books", library.len());
    library.print_books();
}


struct Library {
    books: Vec<Book>
}


struct Book {
    title: String,
    year: u16
}


impl Book {
    // This is a constructor
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year
        }
    }
}


impl Library {
    fn new() -> Library {
        Library {
            books: Vec::new()
        }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
        //self.books.len() == 0
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{} ({})", book.title, book.year)
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut oldest = None;
        for book in &self.books {
            match oldest {
                None => oldest = Some(book),
                Some(old) => {
                    if book.year < old.year {
                        oldest = Some(book);
                    }
                }
            }
        }
        oldest
    }
}

