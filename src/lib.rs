//! Rust client for BOINC RPC protocol.
//!
//! # Example
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! let transport = boinc_rpc::Transport::new("127.0.0.1:31416", Some("my-pass-in-gui_rpc_auth.cfg"));
//! let mut client = boinc_rpc::Client::new(transport);
//!
//! println!("{:?}\n", client.get_messages(0).await.unwrap());
//! println!("{:?}\n", client.get_projects().await.unwrap());
//! println!("{:?}\n", client.get_account_manager_info().await.unwrap());
//! println!("{:?}\n", client.exchange_versions(&boinc_rpc::models::VersionInfo::default()).await.unwrap());
//! println!("{:?}\n", client.get_results(false).await.unwrap());
//! # })
//! ```

#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::enum_variant_names, clippy::type_complexity)]

pub mod errors;
pub mod messages;
pub mod models;
pub mod rpc;
mod util;

use crate::{errors::Error, rpc::DaemonStream};
use std::{
    fmt::Display,
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::{net::TcpStream, sync::Mutex};
use tower::ServiceExt;

fn verify_rpc_reply_contents(data: &[treexml::Element]) -> Result<bool, Error> {
    let mut success = false;
    for node in data {
        match &*node.name {
            "success" => success = true,
            "status" => {
                return Err(Error::Status(
                    util::eval_node_contents(node).unwrap_or(9999),
                ));
            }
            "unauthorized" => {
                return Err(Error::Auth(String::new()));
            }
            "error" => {
                let error_msg = node
                    .text
                    .clone()
                    .ok_or_else(|| Error::Daemon("Unknown error".into()))?;

                return match &*error_msg {
                    "unauthorized" | "Missing authenticator" => Err(Error::Auth(error_msg)),
                    "Missing URL" => Err(Error::InvalidURL(error_msg)),
                    "Already attached to project" => Err(Error::AlreadyAttached(error_msg)),
                    _ => Err(Error::DataParse(error_msg)),
                };
            }
            _ => {}
        }
    }
    Ok(success)
}

type DaemonStreamFuture =
    Pin<Box<dyn Future<Output = Result<DaemonStream<TcpStream>, Error>> + Send + Sync + 'static>>;

enum ConnState {
    Connecting(DaemonStreamFuture),
    Ready(DaemonStream<TcpStream>),
    Error(Error),
}

pub struct Transport {
    state: Arc<Mutex<Option<ConnState>>>,
}

impl Transport {
    pub fn new<A: Display, P: Display>(addr: A, password: Option<P>) -> Self {
        let addr = addr.to_string();
        let password = password.map(|p| p.to_string());
        Self {
            state: Arc::new(Mutex::new(Some(ConnState::Connecting(Box::pin(
                DaemonStream::connect(addr, password),
            ))))),
        }
    }
}

impl tower::Service<Vec<treexml::Element>> for Transport {
    type Response = Vec<treexml::Element>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let Ok(mut g) = self.state.try_lock() else {
            return Poll::Pending;
        };

        let (state, out) = match g.take() {
            Some(ConnState::Connecting(mut future)) => {
                let res = future.as_mut().poll(cx);
                match res {
                    Poll::Pending => (Some(ConnState::Connecting(future)), Poll::Pending),
                    Poll::Ready(Ok(conn)) => (Some(ConnState::Ready(conn)), Poll::Ready(Ok(()))),
                    Poll::Ready(Err(e)) => (None, Poll::Ready(Err(e))),
                }
            }
            Some(ConnState::Ready(conn)) => (Some(ConnState::Ready(conn)), Poll::Ready(Ok(()))),
            Some(ConnState::Error(error)) => (
                Some(ConnState::Error(error.clone())),
                Poll::Ready(Err(error)),
            ),
            None => (
                None,
                Poll::Ready(Err(Error::Null("Null state".to_string()))),
            ),
        };

        *g = state;
        out
    }

    fn call(&mut self, req: Vec<treexml::Element>) -> Self::Future {
        let state = self.state.clone();
        Box::pin(async move {
            let mut state = state.lock().await;

            let Some(ConnState::Ready(mut conn)) = state.take() else {
                unreachable!()
            };

            let query_res = conn.query(req).await;

            if let Err(e) = &query_res {
                *state = Some(ConnState::Error(e.clone()));
            }

            query_res
        })
    }
}

pub struct Client<S> {
    transport: S,
}

impl<S> Client<S>
where
    S: tower::Service<Vec<treexml::Element>, Response = Vec<treexml::Element>, Error = Error>,
{
    pub const fn new(transport: S) -> Self {
        Self { transport }
    }

    async fn get_object<T: for<'a> From<&'a treexml::Element>>(
        &mut self,
        req_data: Vec<treexml::Element>,
        object_tag: &str,
    ) -> Result<T, Error> {
        self.transport.ready().await?;
        let data = self.transport.call(req_data).await?;
        verify_rpc_reply_contents(&data)?;
        for child in &data {
            if child.name == object_tag {
                return Ok(T::from(child));
            }
        }
        Err(Error::DataParse("Object not found.".to_string()))
    }

    async fn get_object_by_req_tag<T: for<'a> From<&'a treexml::Element>>(
        &mut self,
        req_tag: &str,
        object_tag: &str,
    ) -> Result<T, Error> {
        self.get_object(vec![treexml::Element::new(req_tag)], object_tag)
            .await
    }

    async fn get_vec<T: for<'a> From<&'a treexml::Element>>(
        &mut self,
        req_data: Vec<treexml::Element>,
        vec_tag: &str,
        object_tag: &str,
    ) -> Result<Vec<T>, Error> {
        let mut v = Vec::new();
        {
            self.transport.ready().await?;
            let data = self.transport.call(req_data).await?;
            verify_rpc_reply_contents(&data)?;
            let mut success = false;
            for child in data {
                if child.name == vec_tag {
                    success = true;
                    for vec_child in &child.children {
                        if vec_child.name == object_tag {
                            v.push(T::from(vec_child));
                        }
                    }
                }
            }
            if !success {
                return Err(Error::DataParse("Objects not found.".to_string()));
            }
        }
        Ok(v)
    }

    async fn get_vec_by_req_tag<T: for<'a> From<&'a treexml::Element>>(
        &mut self,
        req_tag: &str,
        vec_tag: &str,
        object_tag: &str,
    ) -> Result<Vec<T>, Error> {
        self.get_vec(vec![treexml::Element::new(req_tag)], vec_tag, object_tag)
            .await
    }

    pub async fn get_messages(&mut self, seqno: i64) -> Result<Vec<models::Message>, Error> {
        self.get_vec(
            vec![{
                let mut node = treexml::Element::new("get_messages");
                node.text = Some(format!("{seqno}"));
                node
            }],
            "msgs",
            "msg",
        )
        .await
    }

    pub async fn get_projects(&mut self) -> Result<Vec<models::ProjectInfo>, Error> {
        self.get_vec_by_req_tag("get_all_projects_list", "projects", "project")
            .await
    }

    pub async fn get_account_manager_info(&mut self) -> Result<models::AccountManagerInfo, Error> {
        self.get_object_by_req_tag("acct_mgr_info", "acct_mgr_info")
            .await
    }

    pub async fn get_account_manager_rpc_status(&mut self) -> Result<i32, Error> {
        self.transport.ready().await?;
        let data = self
            .transport
            .call(vec![treexml::Element::new("acct_mgr_rpc_poll")])
            .await?;
        verify_rpc_reply_contents(&data)?;

        let mut v: Option<i32> = None;
        for child in &data {
            if &*child.name == "acct_mgr_rpc_reply" {
                for c in &child.children {
                    if &*c.name == "error_num" {
                        v = util::eval_node_contents(c);
                    }
                }
            }
        }
        v.ok_or_else(|| Error::DataParse("acct_mgr_rpc_reply node not found".into()))
    }

    pub async fn connect_to_account_manager(
        &mut self,
        url: &str,
        name: &str,
        password: &str,
    ) -> Result<bool, Error> {
        let mut req_node = treexml::Element::new("acct_mgr_rpc");
        req_node.children = vec![
            {
                let mut node = treexml::Element::new("url");
                node.text = Some(url.into());
                node
            },
            {
                let mut node = treexml::Element::new("name");
                node.text = Some(name.into());
                node
            },
            {
                let mut node = treexml::Element::new("password");
                node.text = Some(password.into());
                node
            },
        ];
        self.transport.ready().await?;
        let root_node = self.transport.call(vec![req_node]).await?;
        verify_rpc_reply_contents(&root_node)
    }

    pub async fn exchange_versions(
        &mut self,
        info: &models::VersionInfo,
    ) -> Result<models::VersionInfo, Error> {
        let mut content_node = treexml::Element::new("exchange_versions");
        {
            let mut node = treexml::Element::new("major");
            node.text = info.minor.map(|v| format!("{v}"));
            content_node.children.push(node);
        }
        {
            let mut node = treexml::Element::new("minor");
            node.text = info.major.map(|v| format!("{v}"));
            content_node.children.push(node);
        }
        {
            let mut node = treexml::Element::new("release");
            node.text = info.release.map(|v| format!("{v}"));
            content_node.children.push(node);
        }
        self.get_object(vec![content_node], "server_version").await
    }

    pub async fn get_results(
        &mut self,
        active_only: bool,
    ) -> Result<Vec<models::TaskResult>, Error> {
        self.get_vec(
            vec![{
                let mut node = treexml::Element::new("get_results");
                if active_only {
                    let mut ao_node = treexml::Element::new("active_only");
                    ao_node.text = Some("1".into());
                    node.children.push(ao_node);
                }
                node
            }],
            "results",
            "result",
        )
        .await
    }

    pub async fn set_mode(
        &mut self,
        c: models::Component,
        m: models::RunMode,
        duration: f64,
    ) -> Result<(), Error> {
        self.transport.ready().await?;
        let rsp_root = self
            .transport
            .call(vec![{
                let comp_desc = match c {
                    models::Component::CPU => "run",
                    models::Component::GPU => "gpu",
                    models::Component::Network => "network",
                }
                .to_string();
                let mode_desc = match m {
                    models::RunMode::Always => "always",
                    models::RunMode::Auto => "auto",
                    models::RunMode::Never => "never",
                    models::RunMode::Restore => "restore",
                }
                .to_string();

                let mut node = treexml::Element::new(format!("set_{}_mode", &comp_desc));
                let mut dur_node = treexml::Element::new("duration");
                dur_node.text = Some(format!("{duration}"));
                node.children.push(dur_node);
                node.children.push(treexml::Element::new(mode_desc));
                node
            }])
            .await?;
        verify_rpc_reply_contents(&rsp_root)?;
        Ok(())
    }

    pub async fn get_host_info(&mut self) -> Result<models::HostInfo, Error> {
        self.get_object_by_req_tag("get_host_info", "host_info")
            .await
    }

    pub async fn set_language(&mut self, v: &str) -> Result<(), Error> {
        self.transport.ready().await?;
        verify_rpc_reply_contents(
            &self
                .transport
                .call(vec![{
                    let mut node = treexml::Element::new("set_language");
                    let mut language_node = treexml::Element::new("language");
                    language_node.text = Some(v.into());
                    node.children.push(language_node);
                    node
                }])
                .await?,
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::errors::Error;

    #[test]
    fn verify_rpc_reply_contents() {
        let mut fixture = treexml::Element::new("error");
        fixture.text = Some("Missing authenticator".into());
        let fixture = vec![fixture];
        assert_eq!(
            super::verify_rpc_reply_contents(&fixture).err().unwrap(),
            Error::Auth("Missing authenticator".to_string())
        );
    }
}
