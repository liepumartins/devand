use crate::auth::{self, AuthData};
use crate::PgDevandConn;
use rocket::http::Cookies;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket::Route;
use rocket_contrib::templates::Template;
use serde::Serialize;

// Handle authentication request
#[post("/login", data = "<credentials>")]
fn login(
    mut cookies: Cookies,
    credentials: Form<auth::Credentials>,
    conn: PgDevandConn,
) -> Result<Redirect, Flash<Redirect>> {
    auth::login(&mut cookies, credentials.0, &conn)
        .map(|_| Redirect::to(uri!(index)))
        .map_err(|_| {
            Flash::error(
                Redirect::to(uri!(login_page)),
                "Incorrect username or password",
            )
        })
}

// When user is authenticated, /login just redirect to index
#[get("/login")]
fn login_authenticated(_auth_data: auth::AuthData) -> Redirect {
    Redirect::to(uri!(index))
}

// When user is not authenticated, /login displays a form
#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage>) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        flash: Option<String>,
    }

    let context = Context {
        title: "Sign in to DevAndDev",
        flash: flash.map(|x| x.msg().to_string()),
    };

    Template::render("login", &context)
}

// /logout just remove the cookie
#[get("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    auth::logout(&mut cookies);
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

// Handle join request
#[post("/join", data = "<join_data>")]
fn join(
    mut cookies: Cookies,
    join_data: Form<auth::JoinData>,
    conn: PgDevandConn,
) -> Result<Redirect, Flash<Redirect>> {
    auth::join(&mut cookies, join_data.0, &conn)
        .map(|_| Redirect::to(uri!(index)))
        .map_err(|err| Flash::error(Redirect::to(uri!(join_page)), err.to_string()))
}

// When user is authenticated, /join just redirect to index
#[get("/join")]
fn join_authenticated(_auth_data: AuthData) -> Redirect {
    Redirect::to(uri!(index))
}

// When user is not authenticated, /join displays a form
#[get("/join", rank = 2)]
fn join_page(flash: Option<FlashMessage>, join_data: Option<auth::JoinData>) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        flash: Option<String>,
        username: Option<String>,
        email: Option<String>,
        password: Option<String>,
    }

    let context = Context {
        title: "Create your DevAndDev account",
        flash: flash.map(|x| x.msg().to_string()),
        username: join_data.as_ref().map(|x| x.username.to_string()),
        email: join_data.as_ref().map(|x| x.email.to_string()),
        password: join_data.as_ref().map(|x| x.password.to_string()),
    };

    Template::render("join", &context)
}

// When user is authenticated, home page shows user's dashboard
#[get("/")]
fn dashboard(auth_data: AuthData) -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        username: String,
    }

    let context = Context {
        title: "Your dashboard",
        username: auth_data.username,
    };

    Template::render("dashboard", &context)
}

// When user is not authenticated, home page just serve a static file
#[get("/", rank = 2)]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
    }

    let context = Context { title: "DevAndDev" };
    Template::render("index", &context)
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        join,
        join_authenticated,
        join_page,
        login,
        login_authenticated,
        login_page,
        logout,
        dashboard,
    ]
}
