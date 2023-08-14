use isahc::AsyncReadResponseExt;
use ori_graphics::ImageData;

use crate::{
    views::{Image, Suspense},
    Node,
};

async fn get_image_data(url: &str) -> Result<ImageData, isahc::Error> {
    let bytes = isahc::get_async(url).await?.bytes().await?;
    let data = ImageData::from_bytes(&bytes);
    Ok(data)
}

pub fn network_image(url: &str) -> Suspense {
    let url = String::from(url);

    Suspense::new(async move {
        match get_image_data(&url).await {
            Ok(data) => Node::new(Image::new(data)),
            Err(err) => Node::new(format!("{:?}", err)),
        }
    })
    .fallback(Node::new("Loading..."))
}
