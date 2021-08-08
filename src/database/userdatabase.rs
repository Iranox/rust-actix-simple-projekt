use sqlx::{self, PgPool};


pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) {
    // Prepare SQL statement
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 where tutor_id =
    $1",
        tutor_id
    )
    .fetch_all(pool)
    .await;
}
