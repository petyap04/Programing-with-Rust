// =========================================================
// ДАДЕНИ ТИПОВЕ (НЕ СЕ ПРОМЕНЯТ)
// =========================================================

#[derive(Debug)]
pub struct Request<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub body: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn ok(body: String) -> Self {
        Self { status: 200, body }
    }

    pub fn bad_request() -> Self {
        Self { status: 400, body: String::new() }
    }

    pub fn not_found() -> Self {
        Self { status: 404, body: String::new() }
    }
}

#[derive(Debug)]
pub enum ApiError {
    BadRequest,
    NotFound,
}

// =========================================================
// МАКРОС api_routes!
// =========================================================

macro_rules! api_routes {
    (
        $(
            $method:ident $path:literal => $handler:ident $( ( $param_ty:ty ) )?;
        )*
    ) => {
        pub fn route(req: Request<'_>) -> Response {
            $(
                if req.method == stringify!($method) {
                    let pattern = $path;
                    
                    api_routes!(@handle_route req pattern $handler $( ( $param_ty ) )?);
                }
            )*

            Response::not_found()
        }
    };

    (@handle_route $req:ident $pattern:ident $handler:ident ( $param_ty:ty )) => {
        if $pattern.contains(':') {
            let prefix = $pattern.split(':').next().unwrap();
            if $req.path.starts_with(prefix) {
                let param_str = &$req.path[prefix.len()..];
                if let Ok(val) = param_str.parse::<$param_ty>() {
                    match $handler(val) {
                        Ok(body) => return Response::ok(body),
                        Err(ApiError::BadRequest) => return Response::bad_request(),
                        Err(ApiError::NotFound) => return Response::not_found(),
                    }
                } else {
                    return Response::bad_request();
                }
            }
        }
    };

    (@handle_route $req:ident $pattern:ident $handler:ident) => {
        if $req.path == $pattern {
            match $handler() {
                Ok(body) => return Response::ok(body),
                Err(ApiError::BadRequest) => return Response::bad_request(),
                Err(ApiError::NotFound) => return Response::not_found(),
            }
        }
    };
}

// =========================================================
// HANDLER ФУНКЦИИ 
// =========================================================

fn hello() -> Result<String, ApiError> {
    Ok("Hello, world!".to_string())
}

fn square(x: i32) -> Result<String, ApiError> {
    Ok((x * x).to_string())
}

// =========================================================
// ИЗВИКВАНЕ НА МАКРОСА
// =========================================================

api_routes! {
    GET "/hello" => hello;
    GET "/square/:" => square(i32); 
}

fn main() {
    let req = Request {
        method: "GET",
        path: "/square/5",
        body: None,
    };
    
    let res = route(req);
    println!("{:?}", res); // Трябва да изведе Response { status: 200, body: "25" }
}