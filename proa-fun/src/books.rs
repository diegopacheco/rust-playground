pub struct Book {
    pub id: u32,
    pub title: &'static str,
    pub author: &'static str,
    pub year: u32,
    pub summary: &'static str,
}

pub const BOOKS: [Book; 3] = [
    Book {
        id: 1,
        title: "The Rust Programming Language",
        author: "Steve Klabnik and Carol Nichols",
        year: 2018,
        summary: "The official book on Rust, from ownership to fearless concurrency.",
    },
    Book {
        id: 2,
        title: "Designing Data-Intensive Applications",
        author: "Martin Kleppmann",
        year: 2017,
        summary: "Reliable, scalable and maintainable data systems explained.",
    },
    Book {
        id: 3,
        title: "Continuous Modernization",
        author: "Diego Pacheco",
        year: 2024,
        summary: "How to keep software healthy by modernizing it all the time.",
    },
];

pub fn find_book(id: u32) -> Option<&'static Book> {
    BOOKS.iter().find(|book| book.id == id)
}
