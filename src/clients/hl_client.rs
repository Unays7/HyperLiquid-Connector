use crate::{model::hl_msgs::TobMsg, utils::ws_utils::{ConnectionTimers, HypeStreamRequest, L2BookSubscription, Ping, SubscriptionType}};
use tokio::sync::mpsc;
use std::borrow::Cow;

use super::ws_client::WebsocketClient;


pub struct HypeClient<'a> {
    pub ws: WebsocketClient<'a>,
    pub msg_tx: mpsc::Sender<TobMsg>,
    pub timers: ConnectionTimers,
}

impl<'a> HypeClient<'a> {
    pub async fn new(url: &'a str, msg_tx: mpsc::Sender<TobMsg> ) -> anyhow::Result<Self>{
        let ws = WebsocketClient::new(url).await?;
        let timers = ConnectionTimers::default();
        Ok(Self {ws, msg_tx, timers})
    }

    pub fn subscribe_payload<'h>(type_field: &'h str, coin: &'h str) -> HypeStreamRequest<'h> {
        // could use pattern matching for subscription type to make it more extendable
        HypeStreamRequest {
            method: "subscribe",
            subscription: SubscriptionType::L2Book(L2BookSubscription {
                type_field: Cow::Borrowed(type_field), 
                coin: Cow::Borrowed(coin)
            })
        }
    }

    pub async fn subscribe(&mut self) -> anyhow::Result<()> {
        self.ws.send(HypeClient::subscribe_payload("l2Book", "BTC-USD")).await?;
        Ok(())
    }

    pub async fn send_ping(&mut self) -> anyhow::Result<()>{
        let ping = Ping::ping();
        self.ws.send_ping(ping).await?;
        Ok(())
    }

    pub async fn handle_msg(){}
    pub async fn consume() {}
    pub async fn reconnect(){}
    pub async fn run() {}
}