mod components;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::middleware::Logger;
    use actix_web::*;
    use components::app::App;
    use leptos::{leptos_config::Env, LeptosOptions, *};
    use leptos_actix::{generate_route_list, LeptosRoutes};

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let conf = LeptosOptions {
        output_name: "movies_leptos".to_string(),
        site_root: ".".to_string(),
        site_pkg_dir: "pkg".to_string(),
        site_addr: std::net::SocketAddr::V4(("0.0.0.0:8000").parse().unwrap()),
        env: Env::DEV,
        reload_port: 3000,
    };

    let addr = conf.site_addr;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    // Explicit server function registration is no longer required
    // on the main branch. On 0.3.0 and earlier, uncomment the lines
    // below to register the server functions.
    // _ = GetPost::register();
    // _ = ListPostMetadata::register();

    HttpServer::new(move || {
        let leptos_options = &conf;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", format!("{site_root}/pkg")))
            .service(favicon)
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
            .wrap(Logger::new("%a %r %s"))
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
