struct Book {
    title: String,      // 标题
    author: String,     // 作者
    is_available: bool, // 是否可借
}

impl Book {
    // 创建一个新的 Book 实例
    fn new(title: String, author: String) -> Book {
        Book {
            title,
            author,
            is_available: true,
        }
    }

    // 检查书籍是否可借
    fn is_available(&self) -> bool {
        self.is_available
    }

    // 借出书籍，如果书籍可借，则返回 true，否则返回 false
    fn borrow(&mut self) -> bool {
        if self.is_available {
            self.is_available = false;
            println!(
                "You have borrowed {}!, 作者是: {} 先生",
                self.title, self.author
            );
            true
        } else {
            println!("Sorry, {} is currently on loan.", self.title);
            false
        }
    }

    // 归还书籍
    fn return_book(&mut self) {
        self.is_available = true;
        println!("Thank you for returning {}!", self.title);
    }
}

// 书籍管理员
struct Librarian {
    name: String,
}

impl Librarian {
    // 创建一个新的 Librarian 实例
    fn new(name: String) -> Librarian {
        Librarian { name }
    }

    // 借出书籍
    fn borrow_book(&self, book: &mut Book) {
        println!("管理员【{}】，借出书籍【{}】", self.name, book.title);
        book.borrow();
    }

    // 归还书籍
    fn return_book(&self, book: &mut Book) {
        println!("管理员【{}】，归还书籍【{}】", self.name, book.title);
        book.return_book();
    }
}

fn main() {
    test_book();
    test_librarian();
}

fn test_librarian() {
    let mut book = Book::new("Rust 语言圣经".to_string(), "Homer".to_string());
    let librarian = Librarian::new("John".to_string());
    librarian.borrow_book(&mut book);
    librarian.return_book(&mut book);
    println!("Is {} available? {}", book.title, book.is_available());
}

fn test_book() {
    let mut book = Book::new("Rust 语言圣经".to_string(), "Homer".to_string());
    book.borrow();
    println!("Is {} available? {}", book.title, book.is_available());
    book.borrow();
    book.return_book();
    println!("Is {} available? {}", book.title, book.is_available());
    println!("[{}]这本书的作者是: {}", book.title, book.author);
}