use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct CreateBeer {
    name: String,
    alcohol_content: f64,
    producent: String,
}

pub struct Beer {
    id: Uuid,
    name: String,
    alcohol_content: f64,
    producent: String,
}

impl From<CreateBeer> for Beer {
    fn from(value: CreateBeer) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: value.name,
            alcohol_content: value.alcohol_content,
            producent: value.producent,
        }
    }
}

#[tracing::instrument(name = "posting beer data", skip(pool))]
pub async fn post_beer(State(pool): State<PgPool>, Json(body): Json<CreateBeer>) -> Response {
    match insert_beer(&body, &pool).await {
        Ok(id) => (StatusCode::CREATED, format!("User created with id: {}", id)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[tracing::instrument(skip(pool))]
pub async fn insert_beer(new_beer: &CreateBeer, pool: &PgPool) -> Result<Uuid, sqlx::Error> {
    let beer_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO beers (id, name, alcohol_content, producent)
        VALUES ($1, $2, $3, $4)
        "#,
        beer_id,
        new_beer.name,
        new_beer.alcohol_content,
        new_beer.producent
    )
    .execute(pool)
    .await?;

    Ok(beer_id)
}
