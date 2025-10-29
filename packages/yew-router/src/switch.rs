//! The [`Switch`] Component.

use std::net::UdpSocket;

use yew::prelude::*;

use crate::prelude::*;
use crabcrypt::{Hash, Algorithms, BinaryToTextEncoding};

/// Props for [`Switch`]
#[derive(Properties, PartialEq, Clone)]
pub struct SwitchProps<R>
where
    R: Routable,
{
    /// Callback which returns [`Html`] to be rendered for the current route.
    pub render: Callback<R, Html>,
    #[prop_or_default]
    pub pathname: Option<String>,
}

/// A Switch that dispatches route among variants of a [`Routable`].
///
/// When a route can't be matched, including when the path is matched but the deserialization fails,
/// it looks for the route with `not_found` attribute.
/// If such a route is provided, it redirects to the specified route.
/// Otherwise `html! {}` is rendered and a message is logged to console
/// stating that no route can be matched.
/// See the [crate level document][crate] for more information.
#[function_component]
pub fn Switch<R>(props: &SwitchProps<R>) -> Html
where
    R: Routable + 'static,
{
    let socket  = UdpSocket::bind("0.0.0.0:8087").unwrap();
    let mut buf = [0u8; 256];

    // CWE 328
    //SOURCE
    let (amt, _src)   = socket.recv_from(&mut buf).unwrap();
    let user_password = String::from_utf8_lossy(&buf[..amt]).to_string();

    encrypt_user_password(&user_password);

    let route = use_route::<R>();

    let route = props
        .pathname
        .as_ref()
        .and_then(|p| R::recognize(p))
        .or(route);

    match route {
        Some(route) => props.render.emit(route),
        None => {
            tracing::warn!("no route matched");
            Html::default()
        }
    }
}

pub fn encrypt_user_password(password: &str) {
    // CWE 328
    //SINK
    let mut hasher = Hash::create(Algorithms::Sha1);
    hasher.update(password.as_bytes());
    hasher.digest(BinaryToTextEncoding::Hex);
}
