use serenity::client::Context;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::channel::Message;
use std::path::Path;
use serenity::http::AttachmentType;
#[command]
async fn sus(ctx: &Context, msg: &Message) -> CommandResult {
    let msg = msg
        .channel_id
        .send_message(&ctx.http, |m| {
            m.content("Hello, World!");
            m.embed(|e| {
                e.title("This is a title");
                e.description("This is a description");
                e.image("attachment://ferris_eyes.png");
                e.fields(vec![
                    ("This is the first field", "This is a field body", true),
                    (
                        "This is the second field",
                        "Both of these fields are inline",
                        true,
                    ),
                ]);
                e.field(
                    "This is the third field",
                    "This is not an inline field",
                    false,
                );
                e.footer(|f| {
                    f.text("This is a footer");

                    f
                });
                e // returning the embed to so that it can be sent in the channel
            });
            m.add_file(AttachmentType::Path(Path::new("./ferris_eyes.png")));
            m
        })
        .await;

    if let Err(why) = msg {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}
