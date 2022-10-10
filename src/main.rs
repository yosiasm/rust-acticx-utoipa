use std::{error::Error, net::Ipv4Addr};

use actix_web::{middleware::Logger, web, App, HttpServer, Result};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};
use utoipa_swagger_ui::{SwaggerUi, Url};

// response model
#[derive(Serialize, Deserialize, ToSchema)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

// request body
#[derive(Serialize, Deserialize, ToSchema)]
struct PersonBirthDate {
    birth_date: String,
}
// param
#[derive(Serialize, Deserialize, IntoParams)]
struct PersonPhone {
    phone_numbers: String,
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    env_logger::init();

    // load api doc 1
    #[derive(OpenApi)]
    #[openapi(paths(api1::hello1))]
    struct ApiDoc1;

    // load api doc 2
    #[derive(OpenApi)]
    #[openapi(paths(api2::hello2), components(schemas(Person, PersonBirthDate)))]
    struct ApiDoc2;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(api1::hello1)
                    .service(api2::hello2),
            )
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (
                    Url::new("api1", "/api-doc/openapi1.json"),
                    ApiDoc1::openapi(),
                ),
                (
                    Url::with_primary("api2", "/api-doc/openapi2.json", true),
                    ApiDoc2::openapi(),
                ),
            ]))
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}

mod api1 {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api 1", body = String)
        )
    )]
    #[get("/api1/hello")]
    pub(super) async fn hello1() -> String {
        "hello from api 1".to_string()
    }
}

mod api2 {
    use actix_web::{post, web, Responder, Result};
    use chrono::{NaiveDate, Utc};

    use crate::{Person, PersonBirthDate, PersonPhone};

    #[utoipa::path(
        context_path = "/api",
        request_body(content=PersonBirthDate), // request body
        params(PersonPhone), // params
        responses(
            (status = 200, description = "Hello from api 2", body=Person ) // response
        )
    )]
    #[post("/api2/hello/{name}")]
    pub(super) async fn hello2(
        name: web::Path<String>,
        birth_date: web::Json<PersonBirthDate>,
        query: web::Query<PersonPhone>,
    ) -> Result<impl Responder> {
        // parse birthdate
        let parse_from_str = NaiveDate::parse_from_str;
        let bd = parse_from_str(&birth_date.birth_date.to_string(), "%Y-%m-%d").unwrap();
        let age_day = Utc::now().date_naive().signed_duration_since(bd);
        let age = (age_day.num_days() / 365) as u8;

        // parse phone numbers
        let phone_str = String::from(query.phone_numbers.to_string());
        let phones = phone_str.split(",").map(String::from).collect();

        // serialize response
        let resp = Person {
            name: name.to_string(),
            age: age,
            phones: phones,
        };
        Ok(web::Json(resp))
    }
}
