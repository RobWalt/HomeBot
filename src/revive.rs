use std::time::Duration;

use anyhow::Error;
use telegram_bot::CanSendMessage;
use tokio::time::delay_for;

use crate::app_struct::App;

use anyhow::Result;

pub async fn try_restart_app(app: &mut App, error: Error) -> Result<()> {
    let error_text = error.to_string();
    let wait_secs = error_text
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .map_err(|_| error)?;

    delay_for(Duration::from_secs(wait_secs + 5)).await;

    let mut try_num = 0;
    while app
        .api
        .send(app.group.text(revive_message(error_text.as_str())))
        .await
        .is_err()
    {
        try_num += 1;
        delay_for(Duration::from_secs(wait_secs * 2_u64.pow(try_num))).await;
    }
    Ok(())
}

fn revive_message(error_text: &str) -> String {
    format!(
        "Ups, bot ist abgeschmiert wegen

=============
{error_text}
=============

und ist jetzt wieder online!
"
    )
}
