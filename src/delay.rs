#[cfg(target_arch = "wasm32")]
enum Global {
    Window(web_sys::Window),
    WorkerGlobalScope(web_sys::WorkerGlobalScope),
}

#[cfg(target_arch = "wasm32")]
fn global() -> Global {
    use web_sys::wasm_bindgen::JsCast;
    if let Ok(s) = web_sys::js_sys::global().dyn_into::<web_sys::Window>() {
        return Global::Window(s);
    }
    if let Ok(s) = web_sys::js_sys::global().dyn_into::<web_sys::WorkerGlobalScope>() {
        return Global::WorkerGlobalScope(s);
    }
    panic!("no global object!")
}

#[cfg(target_arch = "wasm32")]
impl Global {
    pub fn set_timeout_with_callback_and_timeout_and_arguments_0(
        &self,
        handler: &web_sys::js_sys::Function,
        timeout: i32,
    ) -> Result<i32, web_sys::wasm_bindgen::JsValue> {
        match self {
            Self::Window(s) => {
                s.set_timeout_with_callback_and_timeout_and_arguments_0(handler, timeout)
            }
            Self::WorkerGlobalScope(s) => {
                s.set_timeout_with_callback_and_timeout_and_arguments_0(handler, timeout)
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn sleep(duration: std::time::Duration) -> () {
    use std::convert::TryInto;
    let millis = duration.as_millis().try_into().unwrap_or(i32::MAX);

    let promise = web_sys::js_sys::Promise::new(&mut |resolve, _| {
        global()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, millis)
            .unwrap();
    });
    let js_fut = wasm_bindgen_futures::JsFuture::from(promise);
    let _ = js_fut.await;
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn sleep(duration: std::time::Duration) -> () {
    async_std::task::sleep(duration).await;
}

pub struct Delay;

impl Delay {
    pub async fn new(duration: std::time::Duration) -> () {
        sleep(duration).await;
    }
}
