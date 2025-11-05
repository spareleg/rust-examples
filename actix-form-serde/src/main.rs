/// A slightly modified example from the best book on Rust: https://www.oreilly.com/library/view/programming-rust-3rd/9781098176228/
use actix_web::{App, HttpResponse, HttpServer, get, post, web};
use serde::Deserialize;

const PORT: u16 = 8080;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Serving on http://0.0.0.0:{PORT}");
    HttpServer::new(|| App::new().service(index).service(gcd_result))
        .bind(("0.0.0.0", PORT))?
        .run()
        .await
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <title>GCD Calculator</title>
            <form method="post">
            <input type="text" name="n"/>
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button>
            </form>
        "#,
    )
}

#[post("/")]
async fn gcd_result(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }
    HttpResponse::Ok().content_type("text/html").body(format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>",
        form.n,
        form.m,
        gcd(form.n, form.m)
    ))
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    while m != 0 {
        if m < n {
            (m, n) = (n, m);
        }
        m %= n;
    }
    n
}
