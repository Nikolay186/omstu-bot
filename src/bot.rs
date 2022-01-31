use crate::Schedule;
use tbot::{markup::markdown_v2, types::parameters::Text as ParseMode, Bot};
use crate::request_handler;

pub async fn start() {
    let mut bot = Bot::from_env("BOT_TOKEN").event_loop();

    bot.text(|context| async move {
        let message_text = context.text.value.clone();

        let schedule = Schedule::new(message_text).await.to_message();

        let reply: ParseMode = markdown_v2(schedule).into();

        context
            .bot
            .send_message(context.chat.id, reply)
            .call()
            .await;
    });

    // bot.inline(|context| async move {
    //     let group = request_handler::search_group_id(context.query.to_string()).await;
    //     let title;
    //     let desc;
    //     let msg: ParseMode;

    //     if let Ok(answer) = schedule {
    //         title = answer.to_string();


    //     }
    // });

    bot.polling().start().await.unwrap();
}
