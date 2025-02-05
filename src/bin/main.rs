use std::process::ExitCode;

use todos::app::App;

#[tokio::main]
async fn main() -> Result<ExitCode, ExitCode> {
    let run = App::run().await;

    if let Err(e) = run {
        tracing::error!("Error {:?}", e);
        Err(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
