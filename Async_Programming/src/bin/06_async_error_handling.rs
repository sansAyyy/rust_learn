use std::error::Error;
use std::fmt;
use std::time::Duration;
use tokio::task::JoinError;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("error: {error}");
        let mut source = error.source();
        while let Some(cause) = source {
            eprintln!("caused by: {cause}");
            source = cause.source();
        }
    }
}

async fn run() -> Result<(), AppError> {
    let user = fetch_user(1).await?;
    println!("user = {user}");

    let missing = fetch_user(0).await;
    println!("missing user result = {:?}", missing);

    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        Ok::<_, AppError>(String::from("background value"))
    });

    // 第一个 ? 处理 JoinError，第二个 ? 处理任务内部的 AppError。
    let value = handle.await??;
    println!("spawned task returned = {value}");

    Ok(())
}

async fn fetch_user(id: u64) -> Result<String, AppError> {
    sleep(Duration::from_millis(50)).await;

    if id == 0 {
        Err(AppError::NotFound(String::from("user id 0")))
    } else {
        Ok(format!("user-{id}"))
    }
}

#[derive(Debug)]
enum AppError {
    NotFound(String),
    Join(JoinError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(resource) => write!(f, "{resource} was not found"),
            AppError::Join(error) => write!(f, "task failed to join: {error}"),
        }
    }
}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Join(error) => Some(error),
            _ => None,
        }
    }
}

impl From<JoinError> for AppError {
    fn from(value: JoinError) -> Self {
        AppError::Join(value)
    }
}
