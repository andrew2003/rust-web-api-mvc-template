use crate::models::resume::{ResumeInfo, Resume, ResumeId};
use crate::models::store::Store;

use sqlx::{
    postgres::{PgPool, PgPoolOptions, PgRow},
    Row,
};
use handle_errors::Error;
use crate::models::user::UserId;

pub trait ResumeStoreMethods {
    async fn create_resume(self, new_resume: ResumeInfo)
                           -> Result<Resume, Error>;
    async fn get_resume_by_id(self, resume_id: ResumeId)
                              -> Result<Resume, Error>;
    async fn get_resume_by_user_id(self, user_id: UserId)
                                   -> Result<Resume, Error>;
    async fn get_list_resumes(self)
                              -> Result<Vec<Resume>, Error>;
    async fn update_resume(self, resume_info: ResumeInfo)
                           -> Result<Resume, Error>;
    async fn delete_resume(self, resume_id: ResumeId)
                                      -> Result<bool, Error>;
}
impl ResumeStoreMethods for Store {
    async fn create_resume(self, new_resume: ResumeInfo)
                               -> Result<Resume, Error>
    {
        match sqlx::query("INSERT INTO resumes (user_id, email, url, is_delete) \
                            VALUES ($1, $2, $3, $4)\
                            RETURNING id, user_id, email, url, is_delete")
            .bind(new_resume.user_id.0)
            .bind(new_resume.email)
            .bind(new_resume.url)
            .bind(false)
            .map(|row: PgRow| Resume {
                id: Some(ResumeId(row.get("id"))),
                user_id:UserId(row.get("user_id")),
                email:row.get("email"),
                url: row.get("url"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(resume) => Ok(resume),
            Err(error) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = error
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message =
                        error.as_database_error().unwrap().message(),
                    constraint = error
                        .as_database_error()
                        .unwrap()
                        .constraint()
                        .unwrap()
                );
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    async fn get_resume_by_id(self, resume_id: ResumeId)
                                  -> Result<Resume, Error>
    {
        match sqlx::query("SELECT * FROM RESUMES WHERE id = $1")
            .bind(resume_id.0)
            .map(|row: PgRow| Resume {
                id: Some(ResumeId(row.get("id"))),
                user_id:UserId(row.get("user_id")),
                email:row.get("email"),
                url: row.get("url"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(resume) => Ok(resume),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_resume_by_user_id(self, user_id: UserId)
                                       -> Result<Resume, Error>
    {
        match sqlx::query("SELECT * FROM RESUMES WHERE user_id = $1")
            .bind(user_id.0)
            .map(|row: PgRow| Resume {
                id: Some(ResumeId(row.get("id"))),
                user_id:UserId(row.get("user_id")),
                email:row.get("email"),
                url: row.get("url"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(resume) => Ok(resume),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn get_list_resumes(self)
                                  -> Result<Vec<Resume>, Error>
    {
        match sqlx::query("SELECT * FROM RESUMES")
            .map(|row: PgRow| Resume {
                id: Some(ResumeId(row.get("id"))),
                user_id:UserId(row.get("user_id")),
                email:row.get("email"),
                url: row.get("url"),
                is_delete: row.get("is_delete")
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(resumes) => Ok(resumes),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn update_resume(self, resume_info: ResumeInfo)
                               -> Result<Resume, Error>
    {
        match sqlx::query(
            "Update resumes SET (user_id, email, url) \
                            VALUES ($1, $2, $3)\
                            RETURNING id, user_id, email, url, is_delete")
            .bind(resume_info.user_id.0)
            .bind(resume_info.email)
            .bind(resume_info.url)
            .map(|row: PgRow| Resume {
                id: Some(ResumeId(row.get("id"))),
                user_id:UserId(row.get("user_id")),
                email:row.get("email"),
                url: row.get("url"),
                is_delete: row.get("is_delete")
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(resume) => Ok(resume),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }

    async fn delete_resume(self, resume_id: ResumeId)
                                          -> Result<bool, Error>
    {
        match sqlx::query("Update resumes set is_delete = $1 where id = $2")
            .bind(true)
            .bind(resume_id.0)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::DatabaseQueryError(e))
            }
        }
    }
}