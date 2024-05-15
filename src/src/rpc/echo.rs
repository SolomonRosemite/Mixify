use futures_util::Future;
use std::cell::RefCell;
use tokio::sync::mpsc;

pub trait Logger {
    fn log(&self, lvl: log::Level, message: String);
}

pub struct GRPCOutputResponseSender {
    sender: mpsc::Sender<Result<crate::rpc::service::service::OutputResponse, tonic::Status>>,
}

impl GRPCOutputResponseSender {
    pub fn new(
        sender: mpsc::Sender<Result<crate::rpc::service::service::OutputResponse, tonic::Status>>,
    ) -> GRPCOutputResponseSender {
        GRPCOutputResponseSender { sender }
    }
}

impl Logger for GRPCOutputResponseSender {
    fn log(&self, lvl: log::Level, message: String) {
        log::log!(lvl, "{}", message.clone());

        let message = format_log_message_as_string(lvl, &message);
        let r = self
            .sender
            .try_send(Ok(crate::rpc::service::service::OutputResponse {
                output: message,
            }));

        if let Err(e) = r {
            log::error!("[GRPC] failed to send log to client: {}", e);
        }
    }
}

pub async fn push_context<L, F, R>(l: L, f: impl FnOnce() -> F) -> R
where
    L: Logger + 'static,
    F: Future<Output = R>,
{
    LOGGER.with(|logger| logger.borrow_mut().push(Box::new(l)));
    let r = f().await;
    LOGGER.with(|logger| logger.borrow_mut().pop());
    r
}
fn format_log_message_as_string(level: log::Level, message: &String) -> String {
    return format_log_message(
        &log::Record::builder()
            .level(level)
            .args(format_args!("{}", message))
            .build(),
    );
}

fn format_log_message(record: &log::Record) -> String {
    format!(
        "[{} {}] {}",
        chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false),
        record.level(),
        record.args()
    )
}

thread_local! {
    pub static LOGGER: RefCell<Vec<Box<dyn Logger>>> = RefCell::new(vec![]);
}

macro_rules! error {
    ($($t:tt)*) => {{
        $crate::rpc::echo::LOGGER.with(|logger| {
            match logger.borrow().last() {
                Some(logger) => logger.log(log::Level::Error, format!($($t)*)),
                None => log::log!(log::Level::Error, "{}", format!($($t)*)),
            }
        })
    }};
}

macro_rules! warning {
    ($($t:tt)*) => {{
        $crate::rpc::echo::LOGGER.with(|logger| {
            match logger.borrow().last() {
                Some(logger) => logger.log(log::Level::Warn, format!($($t)*)),
                None => log::log!(log::Level::Warn, "{}", format!($($t)*)),
            }
        })
    }};
}

macro_rules! info {
    ($($t:tt)*) => {{
        $crate::rpc::echo::LOGGER.with(|logger| {
            match logger.borrow().last() {
                Some(logger) => logger.log(log::Level::Info, format!($($t)*)),
                None => log::log!(log::Level::Info, "{}", format!($($t)*)),
            }
        })
    }};
}

macro_rules! debug {
    ($($t:tt)*) => {{
        $crate::rpc::echo::LOGGER.with(|logger| {
            match logger.borrow().last() {
                Some(logger) => logger.log(log::Level::Debug, format!($($t)*)),
                None => log::log!(log::Level::Debug, "{}", format!($($t)*)),
            }
        })
    }};
}

pub(crate) use debug;
pub(crate) use error;
pub(crate) use info;
pub(crate) use warning;
