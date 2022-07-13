use yew::prelude::*;
use web_sys::{HtmlElement, window};
use wasm_bindgen::{JsCast, prelude::*};

#[function_component(App)]
fn app() -> Html {


	let onclick = {
		Callback::from(move |e: MouseEvent| {

			let ripple = window()
				.unwrap_throw()
				.document()
				.unwrap_throw()
				.create_element("span")
				.unwrap_throw()
				.dyn_into::<HtmlElement>()
				.unwrap_throw();

			let button = e.target()
				.unwrap_throw()
				.dyn_into::<HtmlElement>()
				.unwrap_throw();

			let x = &format!("{}px",
				e.client_x() - button.offset_left());

			let y = &format!("{}px",
				e.client_y() - button.offset_top());


			ripple.set_id("ripple");

			ripple.style()
				.set_property("left", x)
				.unwrap_throw();


			ripple.style()
				.set_property("top", y)
				.unwrap_throw();

			button.append_child(&ripple).unwrap_throw();
			
			let func = Closure::wrap(Box::new(move || ripple.remove()) as Box<dyn FnMut()>);

			window()
				.unwrap_throw()
				.set_timeout_with_callback_and_timeout_and_arguments_0(
					func.as_ref().unchecked_ref(), 500
				)
				.unwrap_throw();

			// memory leak moment
			func.forget();
			
		})
	};

	html! {
		<button id={"ripple-button"} {onclick}>
			{"CLICK ME"}
		</button>
	}
}

fn main() {
	wasm_logger::init(wasm_logger::Config::default());
	yew::start_app::<App>();
}
