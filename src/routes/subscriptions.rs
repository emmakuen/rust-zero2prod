use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormData>,
    connection_pool: web::Data<PgPool>,
) -> HttpResponse {
    let request_id = Uuid::new_v4();
    // create an info level span that represents the whole HTTP request
    // tracing allows us to associate structured information to our spans as a collection of key-value pairs.
    // % symbol is used to tell tracing to use its Display implementation for logging
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    // activate the span using enter method
    // the following guard will be dropped at the end of the current function, that's when we exit the span
    let _request_span_guard = request_span.enter();

    // we don't call `.enter` on query_span
    // `.instrument` takes care of it at the right moments in the query future lifetime
    let query_span = tracing::info_span!("Saving new subscriber details in the database");

    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    // 'get_ref' is used here to get an immutable reference to the 'PgConnection' wrapped by 'web::Data'
    .execute(connection_pool.as_ref())
    // first we attach the instrumentation to the query span, then we await it
    .instrument(query_span)
    .await;

    match result {
        Ok(_) => {
            tracing::info!("request_id {request_id} - New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // error log falls outside of query span - fix it later
            tracing::error!("request_id {request_id} - Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
