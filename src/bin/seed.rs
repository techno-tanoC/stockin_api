use std::env;

use anyhow::Result;
use sqlx::PgPool;
use stockin_api::{domain::item::ItemParams, repo::item};

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    for (title, url, thumbnail) in items() {
        let params = ItemParams {
            title: title.to_string(),
            url: url.to_string(),
            thumbnail: thumbnail.to_string(),
        };
        item::insert(&pool, params).await?;
    }

    Ok(())
}

fn items() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "Android",
            "https://www.android.com/",
            "https://lh3.googleusercontent.com/GTmuiIZrppouc6hhdWiocybtRx1Tpbl52eYw4l-nAqHtHd4BpSMEqe-vGv7ZFiaHhG_l4v2m5Fdhapxw9aFLf28ErztHEv5WYIz5fA",
        ),
        (
            "TypeScript",
            "https://www.typescriptlang.org/",
            "",
        ),
        (
            "The Go Programming Language",
            "https://golang.org/",
            "",
        ),
        (
            "Docker",
            "https://www.docker.com/",
            "https://www.docker.com/wp-content/uploads/2022/12/Docker-Temporary-Image-Social-Thumbnail-1200x630-1.png",
        ),
        (
            "Haskell Language",
            "https://www.haskell.org/",
            "",
        ),
        (
            "Rustプログラミング言語",
            "https://www.rust-lang.org/ja",
            "https://www.rust-lang.org/static/images/rust-social-wide.jpg",
        ),
        (
            "GitLab",
            "https://gitlab.com/",
            "",
        ),
        (
            "GitHub",
            "https://github.com/",
            "https://github.githubassets.com/images/modules/site/social-cards/github-social.png",
        ),
        (
            "Twitter",
            "https://twitter.com/home",
            "",
        ),
        (
            "ニコニコ動画",
            "https://www.nicovideo.jp/",
            "https://nicovideo.cdn.nimg.jp/uni/images/ogp.png",
        ),
        (
            "YouTube",
            "https://www.youtube.com/",
            "https://www.youtube.com/img/desktop/yt_1200.png",
        ),
        (
            "Qiita",
            "https://qiita.com/",
            "https://cdn.qiita.com/assets/qiita-ogp-3b6fcfdd74755a85107071ffc3155898.png",
        ),
        (
            "CircleCIでイメージをビルドしてGCRにプッシュする",
            "https://qiita.com/techno-tanoC/items/845bb906156e66a24b7f",
            "",
        ),
        (
            "Zenn",
            "https://zenn.dev/",
            "https://zenn.dev/images/logo-only-dark.png",
        ),
        (
            "Rustの新しいWEBフレームワークaxumを触ってみた",
            "https://zenn.dev/techno_tanoc/articles/99e54c82cb049f",
            "",
        )
    ]
}
