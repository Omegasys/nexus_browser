//! Bridge between Nexus Browser and a JavaScript runtime.

use std::collections::VecDeque;

use super::js_engine_api::{
    JsEngineError,
    JsEngineResult,
};

/// Runtime message type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeMessageType {
    Execute,
    Evaluate,
    Initialize,
    Shutdown,
    Interrupt,
    Reset,
}

/// Message sent to the JavaScript runtime.
#[derive(Debug, Clone)]
pub struct RuntimeMessage {
    pub message_type: RuntimeMessageType,
    pub payload: String,
}

impl RuntimeMessage {
    pub fn new(message_type: RuntimeMessageType, payload: impl Into<String>) -> Self {
        Self {
            message_type,
            payload: payload.into(),
        }
    }
}

/// Communication bridge between browser components and JavaScript.
pub struct JsRuntimeBridge {
    queue: VecDeque<RuntimeMessage>,
    connected: bool,
    sequence: u64,
}

impl JsRuntimeBridge {
    /// Creates a new runtime bridge.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            connected: false,
            sequence: 0,
        }
    }

    /// Connects the bridge.
    pub fn connect(&mut self) -> JsEngineResult<()> {
        if self.connected {
            return Ok(());
        }

        self.connected = true;
        Ok(())
    }

    /// Disconnects the bridge.
    pub fn disconnect(&mut self) {
        self.connected = false;
        self.queue.clear();
    }

    /// Returns whether the bridge is connected.
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Sends a runtime message.
    pub fn send(&mut self, message: RuntimeMessage) -> JsEngineResult<u64> {
        if !self.connected {
            return Err(JsEngineError::NotRunning);
        }

        self.sequence += 1;
        self.queue.push_back(message);

        Ok(self.sequence)
    }

    /// Retrieves the next pending message.
    pub fn receive(&mut self) -> Option<RuntimeMessage> {
        self.queue.pop_front()
    }

    /// Returns the number of pending messages.
    pub fn pending_messages(&self) -> usize {
        self.queue.len()
    }

    /// Queues JavaScript execution.
    pub fn execute(&mut self, source: &str) -> JsEngineResult<u64> {
        self.send(RuntimeMessage::new(
            RuntimeMessageType::Execute,
            source,
        ))
    }

    /// Queues JavaScript evaluation.
    pub fn evaluate(&mut self, expression: &str) -> JsEngineResult<u64> {
        self.send(RuntimeMessage::new(
            RuntimeMessageType::Evaluate,
            expression,
        ))
    }

    /// Queues a runtime reset.
    pub fn reset(&mut self) -> JsEngineResult<u64> {
        self.send(RuntimeMessage::new(
            RuntimeMessageType::Reset,
            "",
        ))
    }
}

impl Default for JsRuntimeBridge {
    fn default() -> Self {
        Self::new()
    }
}
