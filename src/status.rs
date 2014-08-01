use std::fmt;


macro_rules! make_status(
    ( $( $name:ident $code:expr $reason:expr )+ ) => (
        pub enum Status {
            $( $name = $code, )+
        }

        impl fmt::Show for Status {
            fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
                let s = match *self {
                    $( $name => format!("{} {}", $code, $reason), )+
                };
                s.fmt(f)
            }
        }
    );
)


make_status!(
    OK 200i "OK"

    BadRequest 400i "Bad Request"
    Forbidden 403i "Forbidden"
    NotFound 404i "Not Found"
    MethodNotAllowed 405i "Method Not Allowed"

    InternalServerError 500i "Internal Server Error"
)
