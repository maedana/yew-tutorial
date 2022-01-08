use yew::prelude::*;

mod components;
use components::video_details::*;
use components::videos_list::*;

#[function_component(App)]
fn app() -> Html {
    let videos = vec![
        Video {
            id: 1,
            title: "Building and breaking things".to_string(),
            speaker: "John Doe".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 2,
            title: "The development process".to_string(),
            speaker: "Jane Smith".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 3,
            title: "The Web 7.0".to_string(),
            speaker: "Matt Miller".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
        Video {
            id: 4,
            title: "Mouseless development".to_string(),
            speaker: "Tom Jerry".to_string(),
            url: "https://youtu.be/PsaFVLr8t4E".to_string(),
        },
    ];

    // MEMO: use_stateは後から説明があるらしい。
    // MEMO: Don't worry about the use_state right now, we will come back to that later.  とのこと
    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
      <>
          <h1>{ "RustConf Explorer" }</h1>
          <div>
              <h3>{"Videos to watch"}</h3>
              <VideosList videos={videos} on_click={on_video_select.clone()} />
          </div>
          // MEMO: Note the trick we pulled with { for details }. Option<_> implements Iterator so we can use it to display the only element returned by the Iterator with the { for ... } syntax. らしい
          { for details }
      </>
    }
}

fn main() {
    yew::start_app::<App>();
}
