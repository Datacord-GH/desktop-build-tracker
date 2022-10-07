use serenity::prelude::SerenityError;
use serenity::{http::Http, model::channel::Embed, model::webhook::Webhook, utils::Colour};
use std::env;

use crate::models::Build;

fn embed_info(channel: &String) -> Result<(String, Colour), ()> {
    if channel == &"Stable".to_string() {
        return Ok((
            env::var("STABLE_ROLE_ID").expect("missing STABLE_ROLE_ID in .env"),
            Colour::from_rgb(7, 180, 255),
        ));
    } else if channel == &"PTB".to_string() {
        return Ok((
            env::var("PTB_ROLE_ID").expect("missing PTB_ROLE_ID in .env"),
            Colour::from_rgb(155, 89, 182),
        ));
    } else if channel == &"Canary".to_string() {
        return Ok((
            env::var("CANARY_ROLE_ID").expect("missing CANARY_ROLE_ID in .env"),
            Colour::from_rgb(230, 126, 34),
        ));
    }

    Err(())
}

pub async fn send_message(build: &Build) -> Result<(), SerenityError> {
    let http = Http::new("token");
    let token = env::var("WEBHOOK_URL").expect("missing WEBHOOK_URL in .env");
    let webhook = Webhook::from_url(&http, &token).await?;

    let (role_id, colour) = match embed_info(&build.channel) {
        Ok(data) => data,
        Err(_) => panic!("Invalid channel provided"),
    };

    let embed_title = format!("{} update", build.channel);
    let content = format!("<@&{}>", role_id);

    let build_number_value = format!("`{}`", build.build_number);
    let build_id_value = format!("`{}`", &build.build_id[..7]);
    let build_hash_value = format!("`{}`", build.build_hash);

    let build_embed = Embed::fake(|e| {
        e.title(embed_title)
            .colour(colour)
            .field("Build Number", build_number_value, true)
            .field("Build Id", build_id_value, true)
            .field("Build Hash", build_hash_value, false)
    });

    webhook
        .execute(&http, true, |w| {
            w.content(content)
                .username("Desktop Build Manager")
                .embeds(vec![build_embed])
        })
        .await?;

    Ok(())
}
