use dialoguer::Input;
use reqwest::Client;
use tokio::runtime::Runtime;

use errors::SoundCloudError;
use types::PagedResponse;

use crate::types::Song;

mod errors;
mod types;

async fn get_likes(client_id: String, username: String) -> Result<Vec<Song>, SoundCloudError> {
    let client = Client::new();
    let default_params = vec![
        ("client_id", client_id),
    ];

    let res = client.get("https://api.soundcloud.com/resolve")
                    .query(&default_params)
                    .query(&[("url", format!("https://soundcloud.com/{}/likes", username))])
                    .send()
                    .await?;
    match res.status().as_u16() {
        200 => (),
        401 => return Err(SoundCloudError {
            message: "Unauthorized: Is your Client ID valid ?".to_string()
        }),
        404 => return Err(SoundCloudError {
            message: "NotFound: Make sure the Username is correct.".to_string()
        }),
        _ => return Err(SoundCloudError {
            message: format!("Error: The API returned a unknown error with code `{}`", res.status())
        })
    }
    let mut next_href: Option<String> = Some(res.url().to_string());
    let mut songs: Vec<Song> = vec![];
    loop {
        match next_href {
            Some(url) => {
                let res = client.get(&url)
                                .query(&default_params)
                                .query(&[
                                    ("limit", "200".to_string()),
                                    ("linked_partitioning", "true".to_string())
                                ])
                                .send()
                                .await?;
                let favourites: PagedResponse = res.json::<PagedResponse>().await?;

                songs.extend(favourites.collection);
                next_href = favourites.next_href;
            }
            None => break
        }
    }
    Ok(songs)
}

fn main() -> Result<(), SoundCloudError> {
    let client_id = Input::<String>::new().with_prompt("Client ID").interact()?;
    let username = Input::<String>::new().with_prompt("Username").interact()?;

    let future = get_likes(client_id, username);
    match Runtime::new()?.block_on(future) {
        Ok(songs) => println!("{}", serde_json::to_string_pretty(&serde_json::to_value(songs)?)?),
        Err(e) => eprintln!("{}", e),
    };
    Ok(())
}
