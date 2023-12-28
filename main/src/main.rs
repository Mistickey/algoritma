use std::io;

// Definisikan struktur data untuk buku
struct Book {
    title: String,
    author: String,
    year_published: u32,
}

fn main() {
    // Inisialisasi vektor untuk menyimpan daftar buku
    let mut book_list: Vec<Book> = Vec::new();

    loop {
        // Tampilkan menu
        println!("Pendataan Buku");
        println!("1. Tambah Buku");
        println!("2. Tampilkan Daftar Buku");
        println!("3. Keluar");
        println!("Pilih menu:");

        // Baca pilihan pengguna
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Gagal membaca baris");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Masukkan angka yang valid!");
                continue;
            }
        };

        match choice {
            1 => {
                // Tambah buku
                let new_book = add_book();
                book_list.push(new_book);
                println!("Buku berhasil ditambahkan!");
            }
            2 => {
                // Tampilkan daftar buku
                display_books(&book_list);
            }
            3 => {
                // Keluar dari program
                println!("Terima kasih!");
                break;
            }
            _ => {
                println!("Pilihan tidak valid. Silakan pilih lagi.");
            }
        }
    }
}

// Fungsi untuk menambahkan buku baru
fn add_book() -> Book {
    println!("Masukkan judul buku:");
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Gagal membaca baris");

    println!("Masukkan nama pengarang:");
    let mut author = String::new();
    io::stdin().read_line(&mut author).expect("Gagal membaca baris");

    println!("Masukkan tahun terbit:");
    let mut year = String::new();
    io::stdin().read_line(&mut year).expect("Gagal membaca baris");
    let year: u32 = match year.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Masukkan tahun yang valid!");
            0
        }
    };

    // Membuat dan mengembalikan struktur buku baru
    Book {
        title: title.trim().to_string(),
        author: author.trim().to_string(),
        year_published: year,
    }
}

// Fungsi untuk menampilkan daftar buku
fn display_books(books: &Vec<Book>) {
    println!("Daftar Buku:");
    for book in books {
        println!("Judul: {}", book.title);
        println!("Pengarang: {}", book.author);
        println!("Tahun Terbit: {}", book.year_published);
        println!("-----------------------");
    }
}
