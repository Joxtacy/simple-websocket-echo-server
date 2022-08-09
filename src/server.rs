use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web_actors::ws;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response cause a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// WebSocket connection is long running connection, it is easier
/// to handle with an actor
pub struct MyWebSocket {
    /// Client must send ping at least once per `CLIENT_TIMEOUT`,
    /// otherwise we drop the connection
    heartbeat: Instant,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process WebSocket messages
        println!("WS: {msg:?}");

        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            },
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat = Instant::now();
            },
            Ok(ws::Message::Text(text)) => ctx.text(format!("echo: {}", text)),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    pub fn new() -> Self {
        Self { heartbeat: Instant::now() }
    }

    /// Helper method that sends ping to client every `HEARTBEAT_INTERVAL` seconds,
    ///
    /// also this method checks heatbeats from client
    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            // check client heartbeats
            if Instant::now().duration_since(actor.heartbeat) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("WebSocket Client heartbeat failed, disconnecting!");

                // stop actor
                context.stop();

                // don't try to send a ping
                return;
            }

            context.ping(b"");
        });
    }
}
