#[macro_escape]
macro_rules! wayland_protocol(
    ($name: expr, [$(($import: ident, $interface: ident)),*]) => {
        #[cfg(feature = "client")]
        pub use self::generated::client;

        #[cfg(feature = "server")]
        pub use self::generated::server;

        #[cfg(all(feature = "native_lib", any(feature = "client", feature = "server")))]
        pub use self::generated::c_interfaces;

        #[cfg(all(not(feature = "native_lib"), any(feature = "client", feature = "server")))]
        mod generated {
            #![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
            #![allow(non_upper_case_globals,non_snake_case,unused_imports)]
            #![allow(missing_docs)]

            #[cfg(feature = "client")]
            pub mod client {
                //! Client-side API of this protocol
                pub(crate) use wayland_client::{NewProxy, Proxy};
                pub(crate) use wayland_commons::map::Object;
                pub(crate) use wayland_commons::{AnonymousObject, Interface, MessageGroup};
                pub(crate) use wayland_commons::wire::{MessageDesc, ArgumentType};
                pub(crate) use wayland_client::protocol::{$($import),*};
                include!(concat!(env!("OUT_DIR"), "/", $name, "_rust_client_api.rs"));
            }

            #[cfg(feature = "server")]
            pub mod server {
                //! Server-side API of this protocol
                pub(crate) use wayland_server::{NewResource, Resource};
                pub(crate) use wayland_commons::map::Object;
                pub(crate) use wayland_commons::{AnonymousObject, Interface, MessageGroup};
                pub(crate) use wayland_commons::wire::{MessageDesc, ArgumentType};
                pub(crate) use wayland_server::protocol::{$($import),*};
                include!(concat!(env!("OUT_DIR"), "/", $name, "_rust_server_api.rs"));
            }
        }

        #[cfg(all(feature = "native_lib", any(feature = "client", feature = "server")))]
        mod generated {
            #![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
            #![allow(non_upper_case_globals,non_snake_case,unused_imports)]
            #![allow(missing_docs)]

            pub mod c_interfaces {
                //! C interfaces for this protocol

                // import client or server, both are the same anyway
                #[cfg(feature = "client")]
                pub use wayland_client::sys::protocol_interfaces::{$($interface),*};
                #[cfg(all(not(feature = "client"), feature = "server"))]
                pub use wayland_server::sys::protocol_interfaces::{$($interface),*};

                include!(concat!(env!("OUT_DIR"), "/", $name, "_c_interfaces.rs"));
            }

            #[cfg(feature = "client")]
            pub mod client {
                //! Client-side API of this protocol
                pub(crate) use wayland_client::{NewProxy, Proxy};
                pub(crate) use wayland_commons::map::Object;
                pub(crate) use wayland_commons::{AnonymousObject, Interface, MessageGroup};
                pub(crate) use wayland_commons::wire::{MessageDesc, ArgumentType};
                pub(crate) use wayland_sys as sys;
                pub(crate) use wayland_client::protocol::{$($import),*};
                include!(concat!(env!("OUT_DIR"), "/", $name, "_c_client_api.rs"));
            }

            #[cfg(feature = "server")]
            pub mod server {
                //! Server-side API of this protocol
                pub(crate) use wayland_server::{NewResource, Resource};
                pub(crate) use wayland_commons::map::Object;
                pub(crate) use wayland_commons::{AnonymousObject, Interface, MessageGroup};
                pub(crate) use wayland_commons::wire::{MessageDesc, ArgumentType};
                pub(crate) use wayland_sys as sys;
                pub(crate) use wayland_server::protocol::{$($import),*};
                include!(concat!(env!("OUT_DIR"), "/", $name, "_c_server_api.rs"));
            }
        }
    }
);

#[cfg(feature = "unstable_protocols")]
#[macro_escape]
macro_rules! wayland_protocol_versioned(
    ($name: expr, [$($version: ident),*], $rest:tt) => {
        $(
            #[allow(missing_docs)]
            pub mod $version {
                wayland_protocol!(concat!($name, "-", stringify!($version)), $rest);
            }
        )*
    }
);
