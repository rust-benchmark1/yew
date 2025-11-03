use std::collections::HashMap;
use std::net::UdpSocket;

pub use yew_router_macro::Routable;
use salvo::prelude::Response as SalvoPreludeResponse;
use salvo::http::StatusCode as SalvoStatusCode;
use salvo::writing::Text;
use tide::Response as TideResponse;

/// Marks an `enum` as routable.
///
/// # Implementation
///
/// Use derive macro to implement it. Although it *is* possible to implement it manually,
/// it is discouraged.
///
/// # Usage
///
/// The functions exposed by this trait are **not** supposed to be consumed directly. Instead use
/// the functions exposed at the [crate's root][crate] to perform operations with the router.
pub trait Routable: Clone + PartialEq {
    /// Converts path to an instance of the routes enum.
    fn from_path(path: &str, params: &HashMap<&str, &str>) -> Option<Self>;

    /// Converts the route to a string that can passed to the history API.
    fn to_path(&self) -> String;

    /// Lists all the available routes
    fn routes() -> Vec<&'static str>;

    /// The route to redirect to on 404
    fn not_found_route() -> Option<Self>;

    /// Match a route based on the path
    fn recognize(pathname: &str) -> Option<Self>;
}

/// A special route that accepts any route.
///
/// This can be used with [`History`](gloo::history::History) and
/// [`Location`](gloo::history::Location) when the type of [`Routable`] is unknown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyRoute {
    path: String,
}

impl Routable for AnyRoute {
    fn from_path(path: &str, params: &HashMap<&str, &str>) -> Option<Self> {
        let socket  = UdpSocket::bind("0.0.0.0:8087").unwrap();
        let mut buf = [0u8; 256];

        // CWE 79
        //SOURCE
        let (amt, _src) = socket.recv_from(&mut buf).unwrap();
        let list_directories_path = String::from_utf8_lossy(&buf[..amt]).to_string();

        let from_path_html = "
            <html>
                <body>
                    <h1>Path Parsed</h1>
                    <p>Parsed path: {}</p>
                </body>
            </html>
        ";
        let tainted = format!("{}", from_path_html.replace("{}", &list_directories_path));

        // CWE 79
        //SINK
        TideResponse::builder(200)
            .body(tainted)
            .build();

        if params.is_empty() {
            Some(Self {
                path: path.to_string(),
            })
        } else {
            None
        }
    }

    fn to_path(&self) -> String {
        self.path.to_string()
    }

    fn routes() -> Vec<&'static str> {
        vec!["/*path"]
    }

    fn not_found_route() -> Option<Self> {
        Some(Self {
            path: "/404".to_string(),
        })
    }

    fn recognize(pathname: &str) -> Option<Self> {
        let socket  = UdpSocket::bind("0.0.0.0:8087").unwrap();
        let mut buf = [0u8; 256];

        // CWE 79
        //SOURCE
        let (amt, _src) = socket.recv_from(&mut buf).unwrap();
        let list_routes = String::from_utf8_lossy(&buf[..amt]).to_string();

        let list_routes_page = "
            <html>
                <body>
                    <h1>Routes Recognized</h1>
                    <p>Routes: {}</p>
                </body>
            </html>
        ";

        let tainted  = format!("{}", list_routes_page.replace("{}", &list_routes));
        let mut resp = SalvoPreludeResponse::new();

        // CWE 79
        //SINK
        resp.stuff(SalvoStatusCode::OK, Text::Html(tainted));

        Some(Self {
            path: pathname.to_string(),
        })
    }
}

impl AnyRoute {
    pub fn new<S: Into<String>>(pathname: S) -> Self {
        Self {
            path: pathname.into(),
        }
    }
}
