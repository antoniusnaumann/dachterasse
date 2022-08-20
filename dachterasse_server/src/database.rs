use dachterasse::asynch::datasource::{
    LoadResult, ReadOnlyDataSource, ReadWriteDataSource, SaveResult,
};
use dachterasse::StaticDegree;
use serde::Serialize;
use sqlx::{FromRow, PgPool, Postgres, QueryBuilder};

#[derive(Serialize, FromRow)]
struct Lecture {
    title: String,
    url: String,
    degree: String,
}

pub struct LectureDatabase {
    pool: PgPool,
}

impl LectureDatabase {
    pub fn new(pool: PgPool) -> Self {
        LectureDatabase { pool }
    }
}

#[async_trait]
impl ReadOnlyDataSource for LectureDatabase {
    async fn load_lectures(&self, degree: &'static StaticDegree) -> LoadResult {
        let database_lectures: Vec<Lecture> =
            sqlx::query_as("SELECT * FROM lectures WHERE id = $1")
                .bind(degree.id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

        let lectures = database_lectures
            .iter()
            .map(|lecture| dachterasse::Lecture {
                title: lecture.title.to_owned(),
                url: lecture.url.to_owned(),
                categories: None,
            })
            .collect::<Vec<_>>();

        if lectures.is_empty() {
            Err(format!(
                "No lectures in database for degree {}",
                degree.name
            ))
        } else {
            Ok(lectures)
        }
    }
}

#[async_trait]
impl ReadWriteDataSource for LectureDatabase {
    async fn save_lectures(
        &self,
        degree: &'static StaticDegree,
        lectures: &[dachterasse::Lecture],
    ) -> SaveResult {
        let database_lectures = lectures.iter().map(|lecture| Lecture {
            title: lecture.title.clone(),
            url: lecture.url.clone(),
            degree: degree.id.to_owned(),
        });

        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO lectures(title, url, degree_id)");

        query_builder.push_values(database_lectures, |mut b, lecture| {
            b.push_bind(lecture.title)
                .push_bind(lecture.url)
                .push_bind(degree.id);
        });

        query_builder
            .build()
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
