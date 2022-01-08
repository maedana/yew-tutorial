use yew::prelude::*;

mod components;
use components::video_details::*;
use components::videos_list::*;
use reqwasm::http::Request;

#[function_component(App)]
fn app() -> Html {
    // MEMO: ReactのHooksと同じだと思うがちゃんと理解してない
    let videos = use_state(|| vec![]);
    {
        // MEMO: cloneは仕方のないこと?
        let videos = videos.clone();
        // MEMO: ReactのHooksと同じだと思うがちゃんと理解してない
        use_effect_with_deps(
            // MEMO: moveって何だっけ?
            move |_| {
                // MEMO: cloneは仕方のないこと?
                let videos = videos.clone();
                // MEMO: ここも理解出来てない
                // spawn_localって何? moveって何だっけ?
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                        .send()
                        .await
                        // MEMO: Demoなのでunwrapしてるがホントはちゃんとエラーハンドリングするべき
                        .unwrap()
                        .json()
                        .await
                        // MEMO: Demoなのでunwrapしてるがホントはちゃんとエラーハンドリングするべき
                        .unwrap();
                    videos.set(fetched_videos);
                });
                // MEMO:  || () の部分理解出来てない
                || ()
            },
            (),
        );
    }

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
              // MEMO: Request::getで取得したデータをセットする仕組みにしたときにvideosを(*videos).clone()に変えたがこれってなんで必要なの? アスタリスクは参照外しってことでいいのかな?
              <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
          </div>
          // MEMO: Note the trick we pulled with { for details }. Option<_> implements Iterator so we can use it to display the only element returned by the Iterator with the { for ... } syntax. らしい
          { for details }
      </>
    }
}

fn main() {
    yew::start_app::<App>();
}
