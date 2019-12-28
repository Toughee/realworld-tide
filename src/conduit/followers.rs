use crate::db::models::NewFollower;
use crate::db::schema::followers;
use crate::Repo;
use diesel::expression::dsl::count;
use diesel::prelude::*;
use diesel::result::Error;
use uuid::Uuid;

pub fn follow(repo: &Repo, follower_id: Uuid, followed_id: Uuid) -> Result<(), Error> {
    let row = NewFollower {
        follower_id,
        followed_id,
    };
    repo.run(move |conn| {
        diesel::insert_into(followers::table)
            .values(&row)
            // If it already exists, ignore it and don't return an error
            .on_conflict_do_nothing()
            .execute(&conn)
            // Discard the number of inserted rows
            .map(|_| ())
    })
}

pub fn unfollow(
    repo: &Repo,
    follower_id_value: Uuid,
    followed_id_value: Uuid,
) -> Result<(), Error> {
    use crate::db::schema::followers::dsl::{followed_id, follower_id, followers};

    let to_be_deleted = followers.filter(
        followed_id
            .eq(followed_id_value)
            .and(follower_id.eq(follower_id_value)),
    );
    repo.run(move |conn| {
        diesel::delete(to_be_deleted)
            .execute(&conn)
            // Discard the number of deleted rows
            .map(|_| ())
    })
}

pub fn is_following(
    repo: &Repo,
    follower_id_value: Uuid,
    followed_id_value: Uuid,
) -> Result<bool, Error> {
    use crate::db::schema::followers::dsl::{followed_id, follower_id, followers};
    let n: i64 = repo.run(move |conn| {
        followers
            .filter(
                follower_id
                    .eq(follower_id_value)
                    .and(followed_id.eq(followed_id_value)),
            )
            .select(count(followed_id))
            .get_result(&conn)
    })?;
    Ok(n == 1)
}
