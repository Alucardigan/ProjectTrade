use sqlx :: PgPool;
use uuid::Uuid;
//current implementation assumes no Errors
pub struct UserService{
    pub user_db : PgPool
}
impl UserService{
    pub fn new(db: PgPool) -> Self {
        Self { user_db: (db) }
    }
    pub async fn create_user(&self, user_id: Uuid, username: &str, email: &str, password_hash: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)",
            user_id,
            username,
            email,
            password_hash
        )
        .execute(&self.user_db)
        .await?;
        Ok(())
    }
    pub async fn register_user(&self, username: &str, email: &str, password: &str) -> Result<(), sqlx::Error> {
        let user_id: Uuid = Uuid::new_v4();
        let password_hash = password; // Replace with your actual hash function
        self.create_user(user_id, username, email, password_hash).await?;
        Ok(())
    }
}