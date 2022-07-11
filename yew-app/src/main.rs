mod structs;

use yew::prelude::*;
use structs::*;
use reqwasm::http::Request;



#[derive(Properties, PartialEq)]
struct VideoProps {
	videos: Vec<Video>,
	on_click: Callback<Video>,
}

#[function_component(VideoComponent)]
fn video_component(VideoProps { videos, on_click } : &VideoProps) -> Html {
	html! {
		<div class={classes!("video")}>
			{videos.iter().map(|v| {

				let select_video = {
					let on_click = on_click.clone();
					let v = v.clone();
					Callback::from(move |_| {
						on_click.emit(v.clone())
					})
				};

				html! {
					<button class={classes!("button")} onclick={select_video} >
					{ format!("{}: {}", v.speaker, v.title) }</button>
				}
			}).collect::<Html>()}
		</div>
	}
}


#[derive(Properties, PartialEq)]
struct VideoDetailProps {
	video: Video,
}

#[function_component(VideoDetail)]
fn video_detail(VideoDetailProps { video } : &VideoDetailProps) -> Html{
	html! {
		<div>
			<h3>{ video.title.clone() }</h3>
			<img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
		</div>
	}
}

#[function_component(App)]
fn app() -> Html {

	
	let videos = use_state(|| vec![]);
	{
		let videos = videos.clone();
		use_effect_with_deps(move |_| {
			let videos = videos.clone();
			wasm_bindgen_futures::spawn_local(async move {
				let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
				.send()
				.await
				.unwrap()
				.json()
				.await
				.unwrap();
				videos.set(fetched_videos);
			});
			|| ()
		}, ());
	}
	let selected_video = use_state(|| None);

	let select_video = {
		let selected_video = selected_video.clone();
		Callback::from(move |video: Video| {
			selected_video.set(Some(video))
		})
	};
	let details = selected_video.as_ref().map(|v| html!{
		<VideoDetail video={v.clone()}/>
	});
	html! {
		<>
			<h1>{ "RustConf Explorer" }</h1>
			<div>
				<h3>{"Videos to watch"}</h3>
				<VideoComponent videos={(*videos).clone()} on_click={select_video}/>
			</div>
			{ for details }
		</>
	}
}

fn main() {
    yew::start_app::<App>();
}
