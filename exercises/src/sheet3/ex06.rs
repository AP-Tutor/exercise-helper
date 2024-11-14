#![procedural::magic_macro]
//!How was that book called? Programming crust? Nevermind.
//!
//!Create BUP's library system. It should be able to store books, articles and magazines.
//!Each book, article and magazine should have a name, a code and a year of publication.
//!Books should also have an author and a publishing company.
//!Articles should have an orchid.
//!Magazines should have a number and a month.
//!Then implement a method to add a book, an article and a magazine to the library system.
//!Finally, implement a method to print the library system via the println! macro.

#[runtest(1.0)]
/// checks if all the struct have requested fields
fn test_enums() {
    let b = Book {
        name: "".to_string(),
        code: "".to_string(),
        year_publication: 0,
        author: "".to_string(),
        publishing_company: "".to_string(),
    };
    let a = Article {
        name: "".to_string(),
        code: "".to_string(),
        year_publication: 0,
        orcid: "".to_string(),
    };
    let m = Magazine {
        name: "".to_string(),
        code: "".to_string(),
        year_publication: 0,
        number: 0,
        month: "".to_string(),
    };
}

#[runtest(1.0)]
/// tests if the new function is implemented correctly
fn test_new() {
    let v = LibrarySystem::new();
}

#[runtest(1.0)]
#[overwrite(LibrarySystem::new)]
/// tests if the add_book function is implemented correctly
fn test_add_book() {
    let mut v = LibrarySystem::new();
    let b = Book {
        name: "".to_string(),
        code: "".to_string(),
        year_publication: 0,
        author: "".to_string(),
        publishing_company: "".to_string(),
    };
    v.add_book(b);
}

#[runtest(1.0, LibrarySystem::add_article)]
#[overwrite(LibrarySystem::new)]
/// tests if the add_article function is implemented correctly
fn test_add_article() {
    let mut v = LibrarySystem::new();
    let a = Article {
        name: "".to_string(),
        code: "".to_string(),
        year_publication: 0,
        orcid: "".to_string(),
    };
    v.add_article(a);
}

#[runtest(1.0, LibrarySystem::add_magazine)]
#[overwrite(LibrarySystem::new)]
/// tests if the add_magazine function is implemented correctly
fn test_add_magazine() {
    let mut v = LibrarySystem::new();
    let m = Magazine {
        name: "".to_string(),
        code: "".to_string(),
        year_publication: 0,
        number: 0,
        month: "".to_string(),
    };
    v.add_magazine(m);
}
#[runtest(1.0)]
#[overwrite(LibrarySystem::new)]
#[overwrite(LibrarySystem::add_book)]
#[overwrite(LibrarySystem::add_magazine)]
#[overwrite(LibrarySystem::add_article)]
/// tests if the function implements display
fn test_display() {
    let mut v = LibrarySystem::new();
    let b = Book {
        name: "".to_string(),
        code: "".to_string(),
        year_publication: 0,
        author: "".to_string(),
        publishing_company: "".to_string(),
    };
    let a = Article {
        name: "".to_string(),
        code: "".to_string(),
        year_publication: 0,
        orcid: "".to_string(),
    };
    let m = Magazine {
        name: "".to_string(),
        code: "".to_string(),
        year_publication: 0,
        number: 0,
        month: "".to_string(),
    };
    v.add_article(a);
    v.add_magazine(m);
    v.add_book(b);
    let _ = format!("{}", v);
}

use std::fmt::Display;

struct Book {
    name: String,
    code: String,
    year_publication: u16,
    author: String,
    publishing_company: String,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, code: {}, year: {}, author: {}, publisher: {}",
            self.name, self.code, self.year_publication, self.author, self.publishing_company
        )
    }
}

struct Article {
    name: String,
    code: String,
    year_publication: u16,
    orcid: String,
}

impl Display for Article {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, code: {}, year: {}, orchid: {}",
            self.name, self.code, self.year_publication, self.orcid
        )
    }
}

struct Magazine {
    name: String,
    code: String,
    year_publication: u16,
    number: u8,
    month: String,
}

impl Display for Magazine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, code: {}, year: {}, number: {}, month: {}",
            self.name, self.code, self.year_publication, self.number, self.month
        )
    }
}

struct LibrarySystem {
    books: Vec<Book>,
    articles: Vec<Article>,
    magazines: Vec<Magazine>,
}

impl Display for LibrarySystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("Books: \n");
        for book in &self.books {
            s.push_str(&format!("\t{}\n", book));
        }
        s.push_str("Articles: \n");
        for article in &self.articles {
            s.push_str(&format!("\t{}\n", article));
        }
        s.push_str("Magazines: \n");
        for magazine in &self.magazines {
            s.push_str(&format!("\t{}\n", magazine));
        }
        write!(f, "{}", s)
    }
}

impl LibrarySystem {
    fn new() -> LibrarySystem {
        LibrarySystem {
            books: Vec::new(),
            articles: Vec::new(),
            magazines: Vec::new(),
        }
    }

    fn add_book(&mut self, b: Book) {
        self.books.push(b);
    }

    fn add_article(&mut self, a: Article) {
        self.articles.push(a)
    }
    fn add_magazine(&mut self, m: Magazine) {
        self.magazines.push(m);
    }
}
