struct Movie {
    name: String,
    director: String,
    genre: String,
    price: f64,
    rating: f64,
}

fn main() {
    println!("Movie Recommendation System (Rust)");
    // the "!" stands for macro different from a regular function because it
    // can take a variable number of arguments and types and is expanded at compile time.
    
    //Rust is statically typed compiler looks at every function call to check arguments/types
    //before program runs

    // Initial dataset: a growable list (Vec) of Movie values.
    //20 movies
    let movies: Vec<Movie> = vec![
        Movie { name: String::from("Avengers: Infinity War"), director: String::from("Anthony and Joe Russo"), genre: String::from("Action"), price: 14.99, rating: 8.4 },
        Movie { name: String::from("The Shawshank Redemption"), director: String::from("Frank Darabont"), genre: String::from("Drama"), price: 9.99, rating: 9.3 },
        Movie { name: String::from("The Great Gatsby"), director: String::from("Baz Luhrmann"), genre: String::from("Drama"), price: 10.99, rating: 7.2 },
        Movie { name: String::from("Interstellar"), director: String::from("Christopher Nolan"), genre: String::from("Sci-Fi"), price: 13.99, rating: 8.7 },
        Movie { name: String::from("The Wizard of Oz"), director: String::from("Victor Fleming"), genre: String::from("Fantasy"), price: 7.99, rating: 8.1 },
        Movie { name: String::from("The Odyssey"), director: String::from("Christopher Nolan"), genre: String::from("Adventure"), price: 15.99, rating: 8.5 },
        Movie { name: String::from("Captain America: The Winter Soldier"), director: String::from("Anthony and Joe Russo"), genre: String::from("Action"), price: 12.99, rating: 7.7 },
        Movie { name: String::from("Captain America: Civil War"), director: String::from("Anthony and Joe Russo"), genre: String::from("Action"), price: 12.99, rating: 7.8 },
        Movie { name: String::from("Oppenheimer"), director: String::from("Christopher Nolan"), genre: String::from("Drama"), price: 13.99, rating: 8.3 },
        Movie { name: String::from("Young Washington"), director: String::from("Jon Erwin"), genre: String::from("Drama"), price: 9.99, rating: 7.0 },
        Movie { name: String::from("It"), director: String::from("Andy Muschietti"), genre: String::from("Horror"), price: 9.99, rating: 7.3 },
        Movie { name: String::from("The Conjuring"), director: String::from("James Wan"), genre: String::from("Horror"), price: 8.99, rating: 7.5 },
        Movie { name: String::from("Titanic"), director: String::from("James Cameron"), genre: String::from("Romance"), price: 11.99, rating: 7.9 },
        Movie { name: String::from("The Notebook"), director: String::from("Nick Cassavetes"), genre: String::from("Romance"), price: 8.49, rating: 7.8 },
        Movie { name: String::from("The Godfather"), director: String::from("Francis Ford Coppola"), genre: String::from("Crime"), price: 12.99, rating: 9.2 },
        Movie { name: String::from("Goodfellas"), director: String::from("Martin Scorsese"), genre: String::from("Crime"), price: 10.99, rating: 8.7 },
        Movie { name: String::from("Saving Private Ryan"), director: String::from("Steven Spielberg"), genre: String::from("War"), price: 11.49, rating: 8.6 },
        Movie { name: String::from("Jurassic Park"), director: String::from("Steven Spielberg"), genre: String::from("Adventure"), price: 8.99, rating: 8.2 },
        Movie { name: String::from("Jaws"), director: String::from("Steven Spielberg"), genre: String::from("Thriller"), price: 7.99, rating: 8.1 },
        Movie { name: String::from("Hoop Dreams"), director: String::from("Steve James"), genre: String::from("Sports Documentary"), price: 7.99, rating: 8.3 },
    ];

    println!("Loaded {} movies.", movies.len());
    // Loop through each movie and print it's five attributes.
    // & means we borrow the data instead of taking ownership of it by using references
    // 'movies' stays usable after the loop. 
    for movie in &movies {
        println!(
            "{} | {} | {} | ${:.2} | {:.1}",
            movie.name, movie.director, movie.genre, movie.price, movie.rating
        );
    }
}
