use crate::{configuration, data::movie::Movies};

use leptos::{html::Input, *};
use log::{error, info};

#[component]
pub fn SearchBar(cx: Scope) -> impl IntoView {
    //    let movies = create_resource(cx, )

    //let (movies, set_movies) = create_signal(cx, Movies::new());

    let (query, set_query) = create_signal(cx, String::new());

    let movies_resource = create_resource(cx, move || query.get(), search_movie);

    let input = create_node_ref::<Input>(cx);

    let submit = move |_ev| {
        let value = input.get().unwrap().value();
        set_query.update(move |q| {
            *q = value;
        });
    };

    view! {
        cx,
        <form>
            <label for="default-search" class="mb-2 text-sm font-medium text-gray-900 sr-only dark:text-white">Search</label>
            <div class="relative">
                <div class="absolute inset-y-0 left-0 flex items-center pl-3 pointer-events-none">
                    <svg class="w-4 h-4 text-gray-500 dark:text-gray-400" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z" />
                    </svg>
                </div>
                <input _ref={input} type="search" id="default-search" class="block w-full p-4 pl-10 text-sm text-gray-900 border border-gray-300 rounded-lg bg-gray-50 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" placeholder="Search Movies..." required />
                <button type="submit" on:click=submit class="text-white absolute right-2.5 bottom-2.5 bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-4 py-2 dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800">Search</button>
            </div>
        </form>

        <Suspense fallback = || view!{cx, "Loading..."}>
        {
            move || movies_resource.read(cx).map(|movies| match movies {
                Ok(movies) => view! {cx, <div>
                    <For
                    each=move || movies.clone()
                    key=|m| m.id
                    view=move |cx, m| view! {cx, <div>m.id - m.title</div>}
                    /> </div>
                },
                Err(e) => {
                    error!("Error loading resource {}", e);
                    view! {cx, <div>"Could not load resource"</div>}
                },
            })
        }
        </Suspense>

    }
}

#[server(SearchMovie)]
async fn search_movie(query: String) -> Result<Movies, ServerFnError> {
    let configuration = configuration::get_configuration().expect("failed to read configuration");

    let query = format!("query={}&include_adult=false&language=en-US&page=1", query);

    let mut url = "https://api.themoviedb.org/3/search/movie/".to_string();

    url_escape::encode_query_to_string(query, &mut url);

    info!("Got query {}", url);

    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .header(
            "Authorization",
            format!("Bearer: {}", configuration.themoviedb.api_secret),
        )
        .header("accept", "application/json")
        .send()
        .await?;

    let movies: Movies = body.json().await?;

    Ok(movies)
}
