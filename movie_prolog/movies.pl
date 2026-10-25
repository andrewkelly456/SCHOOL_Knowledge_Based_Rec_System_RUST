% Movie Recommendation System (Prolog)

:- initialization(main, main). % second main added so the program exits when it finished instead of the ?- prompt

% Each line is a fact, all movie(...) facts is the predicate movie/5 meaning 5 arguments
% movie(Name, Director, Genre, Price, Rating).
movie('Avengers: Infinity War', 'Anthony and Joe Russo', 'Action', 14.99, 8.4).
movie('The Shawshank Redemption', 'Frank Darabont', 'Drama', 9.99, 9.3).
movie('The Great Gatsby', 'Baz Luhrmann', 'Drama', 10.99, 7.2).
movie('Interstellar', 'Christopher Nolan', 'Sci-Fi', 13.99, 8.7).
movie('The Wizard of Oz', 'Victor Fleming', 'Fantasy', 7.99, 8.1).
movie('The Odyssey', 'Christopher Nolan', 'Adventure', 15.99, 8.5).
movie('Captain America: The Winter Soldier', 'Anthony and Joe Russo', 'Action', 12.99, 7.7).
movie('Captain America: Civil War', 'Anthony and Joe Russo', 'Action', 12.99, 7.8).
movie('Oppenheimer', 'Christopher Nolan', 'Drama', 13.99, 8.3).
movie('Young Washington', 'Jon Erwin', 'Drama', 9.99, 7.0).
movie('It', 'Andy Muschietti', 'Horror', 9.99, 7.3).
movie('The Conjuring', 'James Wan', 'Horror', 8.99, 7.5).
movie('Titanic', 'James Cameron', 'Romance', 11.99, 7.9).
movie('The Notebook', 'Nick Cassavetes', 'Romance', 8.49, 7.8).
movie('The Godfather', 'Francis Ford Coppola', 'Crime', 12.99, 9.2).
movie('Goodfellas', 'Martin Scorsese', 'Crime', 10.99, 8.7).
movie('Saving Private Ryan', 'Steven Spielberg', 'War', 11.49, 8.6).
movie('Jurassic Park', 'Steven Spielberg', 'Adventure', 8.99, 8.2).
movie('Jaws', 'Steven Spielberg', 'Thriller', 7.99, 8.1).
movie('Hoop Dreams', 'Steve James', 'Sports Documentary', 7.99, 8.3).

% Print every movie: for all movies found, print their five attributes.
print_all :-
    forall(movie(Name, Director, Genre, Price, Rating),
        format('~w | ~w | ~w | $~2f | ~1f~n',
            [Name, Director, Genre, Price, Rating])).
% forall(Condition, Action) for every way Condition can be true, Action must succeed
% No counter, index and no for keyword. Prolog automatically finds every matching fact through
% backtracking

% Turn typed text into a filter: blank or "none" (any case) means no filter.
% Otherwise, the filter is the text in lowercase, so matching ignores case.
make_filter(Text, Filter) :-
    downcase_atom(Text, Lower),
    (   ( Lower == '' ; Lower == none )
    ->  Filter = none
    ;   Filter = Lower
    ).

ask_filter(Prompt, Filter) :-
    write(Prompt),
    nl,
    read_line_to_string(user_input, Line),
    normalize_space(string(Text), Line),
    make_filter(Text, Filter).

% Ask for a number from Low to High; none means no filter.
% If the answer is invalid, ask again by calling this rule again (recursion).
ask_number(Prompt, Low, High, Filter) :-
    write(Prompt),
    nl,
    read_line_to_string(user_input, Line),
    normalize_space(string(Text), Line),
    make_filter(Text, Candidate),
    (   Candidate == none
    ->  Filter = none
    ;   atom_number(Candidate, Number),
        Number >= Low,
        Number =< High
    ->  Filter = Number
    ;   write('Invalid number. Try again, or type None.'),
        nl,
        ask_number(Prompt, Low, High, Filter)
    ).

% The attributes a user may sort by.
valid_sort(name).
valid_sort(director).
valid_sort(genre).
valid_sort(price).
valid_sort(rating).

% Ask which attribute to sort by; "default" means name.
% If the answer is invalid, ask again by calling this rule again (recursion).
ask_sort(SortBy) :-
    write('Sort by which attribute? (name, director, genre, price, rating, or default for name):'),
    nl,
    read_line_to_string(user_input, Line),
    normalize_space(string(Text), Line),
    downcase_atom(Text, Lower),
    (   Lower == default
    ->  SortBy = name
    ;   valid_sort(Lower)
    ->  SortBy = Lower
    ;   write('Invalid choice. Type name, director, genre, price, rating, or default.'),
        nl,
        ask_sort(SortBy)
    ).

% A text filter passes if it is none, or equals the movies value (ignoring case).
% DirectorFilter or GenreFilter is passed into the first argument, value is the second argument from
% the matches loop
text_ok(none, _).
text_ok(Filter, Value) :-
    Filter \== none,
    downcase_atom(Value, Lower),
    Lower == Filter.

% A max price passes if it is none, or the movies price is not above it.
price_ok(none, _).
price_ok(Max, Price) :-
    Max \== none,
    Price =< Max.

% A min rating passes if it is none, or the movies rating is not below it.
rating_ok(none, _).
rating_ok(Min, Rating) :-
    Min \== none,
    Rating >= Min.

% Collect every movie that passes all four filters into a list.
find_matches(DirectorFilter, GenreFilter, MaxPrice, MinRating, Matches) :-
    findall(movie(Name, Director, Genre, Price, Rating), % Picks one movie at a time and works through all of them by backtracking
            (   movie(Name, Director, Genre, Price, Rating),
                text_ok(DirectorFilter, Director),
                text_ok(GenreFilter, Genre),
                price_ok(MaxPrice, Price),
                rating_ok(MinRating, Rating)
            ),
            Matches).

% Print the tables header row and a divider line.
print_header :-
    format('~w~t~5|~w~t~42|~w~t~67|~w~t~88|~t~w~96| ~t~w~104|~n',
            ['#', 'Name', 'Director', 'Genre', 'Price', 'Rating']),
    format('~`-t~104|~n').

% Print one movie as a row of the table.
print_row(Index, Name, Director, Genre, Price, Rating) :-
    format('~w~t~5|~w~t~42|~w~t~67|~w~t~88|~t$~2f~96| ~t~1f~104|~n',
            [Index, Name, Director, Genre, Price, Rating]).

% Show the results; an empty list means nothing matched.
show_results([]) :-
    write('No movies matched your criteria.'),
    nl.
show_results(Matches) :-
    Matches \== [],
    length(Matches, Count),
    format('~w movie(s) matched:~n', [Count]),
    print_header,
    %nth1(Index, List, Item)
    % True when the index item (item 1) matches the pattern movie(Name, Director, Genre, Price, Rating)
    forall(nth1(Index, Matches, movie(Name, Director, Genre, Price, Rating)),
           print_row(Index, Name, Director, Genre, Price, Rating)).

% For each sort attribute: which argument of movie/5 it is, and which direction.
% @=< is ascending (low to hight); @>= is descending (high to low).
sort_position(name, 1, @=<).
sort_position(director, 2, @=<).
sort_position(genre, 3, @=<).
sort_position(price, 4, @=<).
sort_position(rating, 5, @>=).

% Sort the list of movies by the chosen attribute.
sort_matches(SortBy, Matches, Sorted) :-
    sort_position(SortBy, Position, Order),
    sort(Position, Order, Matches, Sorted).

% Ask whether to search again; yes or y means yes, anything else means no.
ask_again(Answer) :-
    write('Search again? (yes/no):'),
    nl,
    read_line_to_string(user_input, Line),
    normalize_space(string(Text), Line),
    downcase_atom(Text, Lower),
    (   memberchk(Lower, [yes, y])
    ->  Answer = yes
    ;   Answer = no
    ).

% Run one search from start to finish
run_search :-
    ask_filter('Enter a director (or None for no preference);', DirectorFilter),
    ask_filter('Enter a genre (or None for no preference):', GenreFilter),
    ask_number('Enter a max price (or None for no preference):', 0, 1000000, MaxPrice),
    ask_number('Enter a min rating from 0 to 10 (or None for no preference):', 0, 10, MinRating),
    ask_sort(SortBy),
    find_matches(DirectorFilter, GenreFilter, MaxPrice, MinRating, Matches),
    sort_matches(SortBy, Matches, Sorted),
    show_results(Sorted).

% Repeat searches until the user quits. The rule calls itself to repeat (recursion).
search_loop :-
    run_search,
    ask_again(Again),
    (   Again == yes
    ->  search_loop
    ;   write('Goodbye!'),
        nl 
    ).

% main:- is a rule :- is read as "if", it says main is true if these steps succeed. 
main :- 
    write('Movie Recommendation System (Prolog)'),
    nl,
    print_all,
    search_loop.
  

