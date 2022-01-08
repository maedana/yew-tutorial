use yew::prelude::*;

// MEMO: Clone, PartialEqの意味わかってない
// MEMO: 外から値をセットしたいので構造体、構造体メンバともにpubに
#[derive(Clone, PartialEq)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}

// MEMO: Properties, PartialEqの意味わかってない
// MEMO: 外部コンポーネントから利用したいので構造体、構造体メンバともにpubに
#[derive(Clone, PartialEq, Properties)]
pub struct VideosListProps {
    pub videos: Vec<Video>,
}

#[function_component(VideosList)]
pub fn videos_list(VideosListProps { videos }: &VideosListProps) -> Html {
    videos
        .iter()
        .map(|video| {
            html! {
                <p>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}
