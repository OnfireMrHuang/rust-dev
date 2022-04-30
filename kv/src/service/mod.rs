use crate::*;

mod command_service;

/// 对Command的处理的抽象
pub trait CommandService {
    ///处理Command, 返回Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}
