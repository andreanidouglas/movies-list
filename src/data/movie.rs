use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Movie {
    pub backdrop_path: String,
    pub genre_ids: Vec<u32>,
    pub id: u64,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: f32,
    pub poster_path: String,
    pub release_date: String,
    pub title: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Movies {
    page: u64,
    results: Vec<Movie>,
    total_pages: u64,
    total_results: u64,
}

impl Movies {
    pub fn new() -> Self {
        Self {
            page: 0,
            results: Vec::new(),
            total_pages: 0,
            total_results: 0,
        }
    }
}

pub struct MovieIntoIterator {
    movies: Movies,
}

impl IntoIterator for Movies {
    type Item = Movie;
    type IntoIter = MovieIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        MovieIntoIterator { movies: self }
    }
}

impl Iterator for MovieIntoIterator {
    type Item = Movie;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.movies.results.iter().next();
        let movie = match result {
            Some(res) => Some(res.clone()),
            None => None,
        };

        movie
    }
}
