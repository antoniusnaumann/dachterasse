use super::database::LectureDatabase;
use dachterasse::{
    asynch::repository::LectureRepository,
    asynch::sources::{InMemoryDataSource, ScraperSource},
    StaticDegree,
};
use dachterasse::{Degrees, Lecture};
use rocket::{serde::json::Json, State};
use rocket::{Build, Rocket};
use sqlx::PgPool;

fn find_degree(id: &str) -> &'static StaticDegree {
    Degrees::all()
        .iter()
        .find(|d| d.id == id)
        .expect("No degree program with this ID found!")
}

#[route_module]
mod lectures {
    use super::*;

    #[get("/<degree>")]
    async fn all(state: &State<LectureRepository<'static>>, degree: &str) -> Json<Vec<Lecture>> {
        // TODO: Appropriate errors
        let degree = find_degree(degree);
        let lectures = state
            .load_and_update(degree)
            .await
            .expect("Could not load lectures...");
        Json(lectures)
    }
}

#[route_module]
mod degrees {
    use super::*;

    #[get("/")]
    async fn all() -> Json<&'static [StaticDegree]> {
        Json(Degrees::all())
    }

    #[get("/<degree>")]
    async fn with_id(degree: &str) -> Json<&'static StaticDegree> {
        Json(find_degree(degree))
    }
}

pub fn rocket(pool: PgPool) -> Rocket<Build> {
    let repository = LectureRepository::new()
        .source(InMemoryDataSource::new())
        .source(LectureDatabase::new(pool))
        .readonly_source(ScraperSource::new());

    rocket::build()
        .manage(repository)
        .mount("/lectures", module!(lectures))
        .mount("/degrees", module!(degrees))
}
