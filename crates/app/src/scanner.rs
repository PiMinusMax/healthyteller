//! Camera barcode scan in the web app. Frames are read on the device in Rust
//! (crate `barcode`); only the recognised number is used afterwards.

use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement, MediaStream};

use crate::i18n::tr;

const VIDEO_ID: &str = "scan-video";
/// Frames are scaled down to this width before decoding; enough for EAN-13.
const FRAME_WIDTH: u32 = 640;
const FRAME_INTERVAL_MS: i32 = 250;

#[derive(Clone, PartialEq)]
enum Status {
    Starting,
    Scanning,
    Failed(String),
}

async fn sleep(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        if let Some(w) = web_sys::window() {
            let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
        }
    });
    let _ = JsFuture::from(promise).await;
}

async fn open_camera() -> Result<MediaStream, String> {
    let window = web_sys::window().ok_or("no window")?;
    let devices = window
        .navigator()
        .media_devices()
        .map_err(|_| "no media devices".to_string())?;
    let constraints = web_sys::MediaStreamConstraints::new();
    // Prefer the back camera on phones.
    let video = js_sys::Object::new();
    let facing = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&facing, &"ideal".into(), &"environment".into());
    let _ = js_sys::Reflect::set(&video, &"facingMode".into(), &facing);
    constraints.set_video(&video);
    let promise = devices
        .get_user_media_with_constraints(&constraints)
        .map_err(|_| "getUserMedia failed".to_string())?;
    let stream = JsFuture::from(promise)
        .await
        .map_err(|_| "permission denied".to_string())?;
    stream
        .dyn_into::<MediaStream>()
        .map_err(|_| "no stream".to_string())
}

fn stop(stream: &MediaStream) {
    for track in stream.get_tracks().iter() {
        if let Ok(track) = track.dyn_into::<web_sys::MediaStreamTrack>() {
            track.stop();
        }
    }
}

/// Reads one frame of the video and tries to decode a barcode.
fn read_frame(video: &HtmlVideoElement, canvas: &HtmlCanvasElement) -> Option<String> {
    let (vw, vh) = (video.video_width(), video.video_height());
    if vw == 0 || vh == 0 {
        return None;
    }
    let width = FRAME_WIDTH.min(vw);
    let height = (f64::from(vh) * f64::from(width) / f64::from(vw)).round() as u32;
    canvas.set_width(width);
    canvas.set_height(height);
    let ctx = canvas
        .get_context("2d")
        .ok()??
        .dyn_into::<CanvasRenderingContext2d>()
        .ok()?;
    ctx.draw_image_with_html_video_element_and_dw_and_dh(
        video,
        0.0,
        0.0,
        f64::from(width),
        f64::from(height),
    )
    .ok()?;
    let data = ctx
        .get_image_data(0.0, 0.0, f64::from(width), f64::from(height))
        .ok()?
        .data();
    healthyteller_barcode::decode_luma(healthyteller_barcode::rgba_to_luma(&data), width, height)
}

#[component]
pub fn CameraScanner(on_code: EventHandler<String>, on_close: EventHandler<()>) -> Element {
    let mut status = use_signal(|| Status::Starting);
    let mut stream = use_signal(|| None::<MediaStream>);

    use_future(move || async move {
        let media = match open_camera().await {
            Ok(m) => m,
            Err(_) => {
                status.set(Status::Failed(tr("scan-no-camera")));
                return;
            }
        };
        stream.set(Some(media.clone()));
        let document = web_sys::window().and_then(|w| w.document());
        let video = document
            .as_ref()
            .and_then(|d| d.get_element_by_id(VIDEO_ID))
            .and_then(|e| e.dyn_into::<HtmlVideoElement>().ok());
        let canvas = document
            .as_ref()
            .and_then(|d| d.create_element("canvas").ok())
            .and_then(|e| e.dyn_into::<HtmlCanvasElement>().ok());
        let (Some(video), Some(canvas)) = (video, canvas) else {
            stop(&media);
            status.set(Status::Failed(tr("scan-no-camera")));
            return;
        };
        video.set_src_object(Some(&media));
        if let Ok(p) = video.play() {
            let _ = JsFuture::from(p).await;
        }
        status.set(Status::Scanning);
        loop {
            if stream.peek().is_none() {
                return;
            }
            if let Some(code) = read_frame(&video, &canvas) {
                stop(&media);
                stream.set(None);
                on_code.call(code);
                return;
            }
            sleep(FRAME_INTERVAL_MS).await;
        }
    });

    use_drop(move || {
        if let Some(s) = stream.peek().as_ref() {
            stop(s);
        }
    });

    rsx! {
        div { class: "scanner",
            video {
                id: VIDEO_ID,
                class: "scanner-video",
                autoplay: true,
                muted: true,
                "playsinline": "true",
                "aria-label": tr("scan-video-label"),
            }
            match status() {
                Status::Starting => rsx! { p { role: "status", {tr("scan-starting")} } },
                Status::Scanning => rsx! { p { role: "status", {tr("scan-hint")} } },
                Status::Failed(msg) => rsx! { p { class: "error", role: "alert", "{msg}" } },
            }
            button {
                class: "btn btn-quiet",
                onclick: move |_| {
                    if let Some(s) = stream.peek().as_ref() {
                        stop(s);
                    }
                    stream.set(None);
                    on_close.call(());
                },
                {tr("action-cancel")}
            }
        }
    }
}
