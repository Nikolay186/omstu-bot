mod request_handler;
mod schedule;
mod schedule_day;
mod subject;

use schedule::Schedule;
use tokio;
#[tokio::main]
async fn main() {
    let str = String::from("ИВТ-191");
    let schedule: Schedule = Schedule::new(str).await;
    println!("{}", schedule);
}
