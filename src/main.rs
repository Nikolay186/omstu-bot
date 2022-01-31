mod bot;
mod request_handler;
mod schedule;
mod schedule_day;
mod subject;
// mod group;

use bot::start;
use schedule::Schedule;
use tokio;

#[tokio::main]
async fn main() {
    start().await;
}
