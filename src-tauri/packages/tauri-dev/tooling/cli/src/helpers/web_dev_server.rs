use axum::{
  extract::{ws::WebSocket, WebSocketUpgrade},
  http::{header::CONTENT_TYPE, Request, StatusCode},
  response::IntoResponse,
  routing::get,
  Router, Server,
};
use html5ever::{namespace_url, ns, LocalName, QualName};
use kuchiki::{traits::TendrilSink, NodeRef};
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use std::{
  net::SocketAddr,
  path::{Path, PathBuf},
  str::FromStr,
  sync::{mpsc::sync_channel, Arc},
  thread,
  time::Duration,
};
use tauri_utils::mime_type::MimeType;
use tokio::sync::broadcast::{channel, Sender};

const AUTO_RELOAD_SCRIPT: &str = include_str!("./auto-reload.js");
pub const SERVER_URL: &str = "http://127.0.0.1:1430";

struct State {
  serve_dir: PathBuf,
  tx: Sender<()>,
}

pub fn start_dev_server<P: AsRef<Path>>(path: P) {
  let serve_dir = path.as_ref().to_path_buf();

  std::thread::spawn(move || {
    tokio::runtime::Builder::new_current_thread()
      .enable_io()
      .build()
      .unwrap()
      .block_on(async move {
        let (tx, _) = channel(1);

        let tokio_tx = tx.clone();
        let serve_dir_ = serve_dir.clone();
        thread::spawn(move || {
          let (tx, rx) = sync_channel(1);
          let mut watcher = new_debouncer(Duration::from_secs(1), None, move |r| {
            if let Ok(events) = r {
              tx.send(events).unwrap()
            }
          })
          .unwrap();

          watcher
            .watcher()
            .watch(&serve_dir_, RecursiveMode::Recursive)
            .unwrap();

          loop {
            if rx.recv().is_ok() {
              let _ = tokio_tx.send(());
            }
          }
        });

        let state = Arc::new(State { serve_dir, tx });
        let router = Router::new()
          .fallback(
            Router::new().nest(
              "/",
              get({
                let state = state.clone();
                move |req| handler(req, state)
              })
              .handle_error(|_error| async move { StatusCode::INTERNAL_SERVER_ERROR }),
            ),
          )
          .route(
            "/_tauri-cli/ws",
            get(move |ws: WebSocketUpgrade| async move {
              ws.on_upgrade(|socket| async move { ws_handler(socket, state).await })
            }),
          );
        Server::bind(&SocketAddr::from_str(SERVER_URL.split('/').nth(2).unwrap()).unwrap())
          .serve(router.into_make_service())
          .await
          .unwrap();
      })
  });
}

async fn handler<T>(req: Request<T>, state: Arc<State>) -> impl IntoResponse {
  let uri = req.uri().to_string();
  let uri = if uri == "/" {
    &uri
  } else {
    uri.strip_prefix('/').unwrap_or(&uri)
  };

  let file = std::fs::read(state.serve_dir.join(uri))
    .or_else(|_| std::fs::read(state.serve_dir.join(format!("{}.html", &uri))))
    .or_else(|_| std::fs::read(state.serve_dir.join(format!("{}/index.html", &uri))))
    .or_else(|_| std::fs::read(state.serve_dir.join("index.html")));

  file
    .map(|mut f| {
      let mime_type = MimeType::parse(&f, uri);
      if mime_type == MimeType::Html.to_string() {
        let mut document = kuchiki::parse_html().one(String::from_utf8_lossy(&f).into_owned());
        fn with_html_head<F: FnOnce(&NodeRef)>(document: &mut NodeRef, f: F) {
          if let Ok(ref node) = document.select_first("head") {
            f(node.as_node())
          } else {
            let node = NodeRef::new_element(
              QualName::new(None, ns!(html), LocalName::from("head")),
              None,
            );
            f(&node);
            document.prepend(node)
          }
        }

        with_html_head(&mut document, |head| {
          let script_el =
            NodeRef::new_element(QualName::new(None, ns!(html), "script".into()), None);
          script_el.append(NodeRef::new_text(AUTO_RELOAD_SCRIPT));
          head.prepend(script_el);
        });

        f = document.to_string().as_bytes().to_vec();
      }

      (StatusCode::OK, [(CONTENT_TYPE, mime_type)], f)
    })
    .unwrap_or_else(|_| {
      (
        StatusCode::NOT_FOUND,
        [(CONTENT_TYPE, "text/plain".into())],
        vec![],
      )
    })
}

async fn ws_handler(mut ws: WebSocket, state: Arc<State>) {
  let mut rx = state.tx.subscribe();
  while tokio::select! {
      _ = ws.recv() => return,
      fs_reload_event = rx.recv() => fs_reload_event.is_ok(),
  } {
    let ws_send = ws.send(axum::extract::ws::Message::Text(
      r#"{"reload": true}"#.to_owned(),
    ));
    if ws_send.await.is_err() {
      break;
    }
  }
}
