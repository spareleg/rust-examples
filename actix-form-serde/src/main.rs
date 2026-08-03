//! A slightly modified example from the book:
//! <https://www.oreilly.com/library/view/programming-rust-3rd/9781098176228/>

use actix_web::{App, HttpResponse, HttpServer, get, post, web::Form};
use serde::Deserialize;

const PORT: u16 = 8080;

#[derive(Deserialize)]
struct Gcd {
    n: u64,
    m: u64,
}

impl Gcd {
    fn valid(&self) -> bool {
        self.n != 0 && self.m != 0
    }

    fn gcd(&self) -> u64 {
        let &Gcd { mut n, mut m } = self;
        while m != 0 {
            if m < n {
                std::mem::swap(&mut m, &mut n);
            }
            m %= n;
        }
        n
    }

    fn ratio(&self) -> (u64, u64) {
        let gcd = self.gcd();
        (self.n / gcd, self.m / gcd)
    }
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
async fn gcd_result(gcd: Form<Gcd>) -> HttpResponse {
    if !gcd.valid() {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is a snooze-fest");
    }

    let ratio = gcd.ratio();
    let resp = format!(
        "<p>The GCD of {} and {} is <b>{}</b></p><p>Ratio is <b>{}:{}</b></p>",
        gcd.n,
        gcd.m,
        gcd.gcd(),
        ratio.0,
        ratio.1,
    );

    HttpResponse::Ok().content_type("text/html").body(resp)
}
