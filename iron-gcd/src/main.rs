extern crate iron;
extern crate mime;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("serving on http://localhost:3000");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut("text/html; charset=utf-8");
    response.set_mut(
        r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="n"/>
        <button type="submit">Compute GCD </button>
    </form>
    "#,
    );

    Ok(response)
}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map,
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums,
    };

    IronResult::Ok(response)
}
