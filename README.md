# unsupervised-learning-netflix

I intend to wrote this as the homework 1 of ECS271 Machine Learning.

However, it turns out that rust is not well suited for ML yet.

Over the 2 weeks I have learnt how to write (nice) doc and gained better understanding of trait system.
But there just isn't good enough 3rd party library that carries out fast, optimized linear algebra calculation.

You are more than welcome to continue my work as the framework is done but no algorithm is inside.

## READ THIS BEFORE YOU RUN

I renamed `movie_titles.txt` to `movie_titles.csv` and added `"` to all movie titles so it can be parsed by csv_parser. Notice that there is one movie with year `NULL`.

I also added a title line to `test.csv` or the first line will be ignored.

## Run, test, doc

`cargo` is really nice for rust.

``
cargo test
cargo run
cargo doc
```