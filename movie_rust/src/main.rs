use std::io;

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

    //Ask the user for a director and read one line from the keyboard.
    println!("Enter a director (or None for no preference):");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        //pointer to input in stack frame which then points to the buffer
        //&mut says we can read and write whats in the buffer and
        //ownership stays with input so we just borrow ownership to modify input. 
        .expect("Failed to read line");
    let director = input.trim();

    //No filter if the user typed "none" (any case) or left the line blank (enter).
    let director_filter: Option<&str> =
    //Option is a type that means a value or nothing, in this case Option<&str> is either a reference
    //into input's text or None. 
        if director.is_empty() || director.eq_ignore_ascii_case("none") {
            None
        } else {
            Some(director)
        };

    match director_filter {
        Some(d) => println!("Filtering by director: {}", d),
        None => println!("No director filter."),
    }

    // Ask for a genre, using the same pattern as the director.
    println!("Enter a genre (or None for no preference):");
    let mut genre_input = String::new();
    io::stdin()
        .read_line(&mut genre_input)
        //.read_line(&mut genre_input) passes a pointer to genre_input in the stack frame.
        //That record's pointer leads to the heap buffer. &mut gives read_line permission to read
        //and write it, while genre_input keeps ownership the whole time, we just borrow access.
        .expect("Failed to read line");
    let genre = genre_input.trim();

    let genre_filter: Option<&str> =
    //Option holds either a reference to the text or it doesn't. 
        if genre.is_empty() || genre.eq_ignore_ascii_case("none") {
            None
        } else {
            Some(genre)
        };
    
    match genre_filter {
        Some(g) => println!("Filtering by genre: {}", g),
        None => println!("No genre filter."),
    }

    //Ask for a max price; keep asking until the input is valid.
    let max_price: Option<f64> = loop {
        println!("Enter a max price (or None for no preference):");
        let mut price_input = String::new();
        io::stdin()
            .read_line(&mut price_input)
            .expect("Failed to read line");
        let price_text = price_input.trim();

        //Blank or "none" means no price filter.
        if price_text.is_empty() || price_text.eq_ignore_ascii_case("none") {
            break None; //breaks loop and returns None to Option
        }

        //Try to turn the text into a non-negative number or loop again
        match price_text.parse::<f64>() {
            //::<_> is a turbofish
            Ok(value) if value >= 0.0 => break Some(value), //breaks loop and returns Some(value) to Option
            _ => println!("Invalid price. Enter a number like 12.50, or None."),
        }
    };

    match max_price {
        Some(p) => println!("Filtering by max price: ${:.2}", p),
        None => println!("No price filter."),
    }

    //Ask for a min rating: keep asking until the input is valid.
    let min_rating: Option<f64> = loop {
        println!("Enter a min rating from 0 to 10 (or None for no preference)");
        let mut rating_input = String::new();
        io::stdin()
            .read_line(&mut rating_input)
            .expect("Failed to read line");
        let rating_text = rating_input.trim();

        //Blank or "none" means no rating filter.
        if rating_text.is_empty() || rating_text.eq_ignore_ascii_case("none") {
            break None;
        }

        //Try to turn the text inot a number between 0 and 10.
        match rating_text.parse::<f64>() {
            Ok(value) if (0.0..=10.0).contains(&value) => break Some(value),
            _ => println!("Invalid rating. Enter a number from 0 to 10, or None."),
        }
    };
    
    match min_rating {
        Some(r) => println!("Filtering by min rating: {:.1}", r),
        None => println!("No rating filter."),
    }

    // Keep only the movies that pass every filter the user set.
    let mut results: Vec<&Movie> = Vec::new();
    // Empty vector that holds references to movie instances, not copies.
    // &Movie means each item points back into the original movies buffer
    // mut is needed because we push into it, changing it. 

    for movie in &movies {
        // Skip this movie if a director filter is set and it doesn't match.
        if let Some(d) = director_filter {
            if !movie.director.eq_ignore_ascii_case(d) {
                continue;
            }
        //Simple Example: If Christopher Nolan == Christopher Nolan we get true which is flipped to false
        //Which then causes the continue to be skipped, if its true the continue is ran
        }

        //Skip if a genre filter is set and it doesn't match
        if let Some(g) = genre_filter {
            if !movie.genre.eq_ignore_ascii_case(g) {
                continue;
            }
        }

        //Skip if a max price is set and the movie costs more.
        if let Some(p) = max_price {
            if movie.price > p {
                continue;
            }
        }

        //Skip if a min rating is set and the movie is rated lower.
        if let Some(r) = min_rating {
            if movie.rating < r {
                continue;
            }
        }

        // The movie passed every filter, so keep it.
        results.push(movie);
    }

    // Tell the user when nothing matched; otherwise list the matches.
    if results.is_empty() {
        println!("No movies matched your criteria.");
    } else {
        println!("{} movie(s) matched;", results.len());
        for movie in &results {
            println!(" {}", movie.name);
        }
    }

    // Ask which attribute to sort by; keep asking until the input is valid.
    let sort_by: &str = loop {
        println!("Sort by which attribute? (name, director, genre, price, rating):");
        let mut sort_input = String::new();
        io::stdin()
            .read_line(&mut sort_input)
            .expect("failed to read line");
        let sort_text = sort_input.trim().to_lowercase();

        match sort_text.as_str() {
            "name" => break "name",
            "director" => break "director", 
            "genre" => break "genre",
            "price" => break "price",
            "rating" => break "rating",
            _ => println!("Invalid choice. Type name, director, genre, price, or rating."),
        }
    };

    println!("Sorting by: {}", sort_by);
}
