use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    // The idea behind a server, is that when it receives some request on some port, we do something based on the information it carries
    // The most common way to allow a server to handle different kinds of requests is to partition requests based on "routes"
    // "Routes" are the section of the URL beyond the root URL (e.g. "www.google.com" os a root URL)
    // We then filter the request by the route to decide which function is called if it matches the route provided.
    // The default response is "not found" when no route matches any of our filters.
    // The below example creates a route that listens to http://localhost:[port]/api/HttpExample
    
    // warp::get() initialises the route as get only filter
    let example_route = warp::get()
        // we build the route for the request by adding paths to the filter
        .and(warp::path("api"))
        .and(warp::path("HttpExample"))
        // we take all query parameters as a hashmap of parameter name to value
        .and(warp::query::<HashMap<String, String>>())
        // we take the hashmap of parameters and try to get the parameter "name", which returns an option
        .map(|p: HashMap<String, String>| match p.get("name") {
            // if the option returns some value, we handle it via lambda that returns a customised response
            Some(name) => Response::builder().body(format!("Hello, {}. This HTTP triggered function executed successfully.", name)),
            // if the option returns none, we handle it by returing a standard response
            None => Response::builder().body(String::from("This HTTP triggered function executed successfully. Pass a name in the query string for a personalized response.")),
        });

    // This key is for an environment variable
    // When run via azure functions, it will inject this environment variable based on what the runtime is configured to do
    // It is also possible to set this environment variable globally anyway when running as a standalone executable, but that's not advisable
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    // we match on env::var, which returns an option for the environment variable that we defined the key for
    let port: u16 = match env::var(port_key) {
        // If the environment variable was found, we parse the string as a u16 and return it as port
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        // else we return the default of 3000 for our port
        Err(_) => 3000,
    };

    warp::serve(example_route).run((Ipv4Addr::LOCALHOST, port)).await
}