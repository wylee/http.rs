use std::from_str::FromStr;
use std::fmt;


macro_rules! make_meth (
    ( $( $ident:ident $name:expr )+ ) => (
        pub enum Method {
            $( $ident, )+
        }

        impl fmt::Show for Method {
            fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
                let s = match *self {
                    $( $ident => $name, )+
                };
                s.fmt(f)
            }
        }

        impl FromStr for Method {
            fn from_str (s: &str) -> Option<Method> {
                // XXX: Why doesn't a match work here? The following causes
                //      'error: unexpected token: `"CONNECT"`':
                // match s {
                //     $( $name => $ident, )+
                //     _ => None
                // }
                $( if s == $name { return Some($ident); } )+
                None
            }
        }
    );
)


make_meth!(
    CONNECT "CONNECT"
    DELETE "DELETE"
    HEAD "HEAD"
    GET "GET"
    OPTIONS "OPTIONS"
    POST "POST"
    PUT "PUT"
    TRACE "TRACE"
)
