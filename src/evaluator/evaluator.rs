use std::{sync::mpsc::{Sender, Receiver, channel}, any::Any, collections::HashMap, rc::Rc};

use crate::evaluator::msg_api::incoming::IncomingMessage;

use super::{msg_api::{outgoing::{ResourceReader, ModuleReader, Evaluate}, incoming::EvaluateResponse}, module_source::ModuleSource, logger::Logger, evaluator_options::EvaluatorOptions, evaluator_manager::EvaluatorManager};

// Interface for evaluating pkl modules
#[derive(Default)]
pub struct Evaluator { // NOTE the lifetime allows us to ignore close() since at the end of the lifetime the Evaluator is killed automatically
    evaluator_id: i64,
    logger: Logger,
    manager: Rc<EvaluatorManager>,
    pending_requests: HashMap<i64, Sender<EvaluateResponse>>,
    closed: bool,
    resource_readers: Vec<ResourceReader>,
    module_readers: Vec<ModuleReader>,
    opts: EvaluatorOptions,
}

// TODO the `out` field should be replaced with some sort of
//  macro since we can evaluate what the type is at compile
//  time. //NOTE I'm dumb and wrong...
pub trait EvaluatorMethods { // NOTE this allows for other types of evaluators, which could be nice
    fn evaluate_module<T>(&self, source: &ModuleSource) -> Result<T, &'static str>;
    fn evaluate_output_text(&self, source: &ModuleSource) -> Result<&'static str, &'static str>;
    fn evaluate_output_value(&self, source: &ModuleSource, out: &dyn Any) -> Result<&'static str, &'static str>;
    fn evaluate_output_files(&self, source: &ModuleSource) -> Result<&'static str, &'static str>;
    fn evaluate_expression<T>(&self, source: &ModuleSource, expr: Option<String>) -> Result<T, &'static str>;
    fn evaluate_expression_raw<T>(&self, source: &ModuleSource, expr: Option<String>) -> Result<T, &'static str>;
    fn closed(&self, ) -> bool;
}

impl EvaluatorMethods for Evaluator {
    fn evaluate_module<T>(&self, source: &ModuleSource) -> Result<T, &'static str> {
        return self.evaluate_expression(source, None);
    }

    fn evaluate_expression<T>(&self, source: &ModuleSource, expr: Option<String>) -> Result<T, &'static str> {
        return self.evaluate_expression_raw(source, expr);
    }

    fn evaluate_expression_raw<T>(&self, source: &ModuleSource, expr: Option<String>) -> Result<T, &'static str> {
        let request_id: i64 = rand::random::<i64>();
        let (send, recv) = channel::<IncomingMessage>();

        let msg = Evaluate {
            requestId: request_id,
            evaluatorId: self.evaluator_id,
            moduleUri: source.uri().to_string(),
            moduleText: source.contents().clone(), //FIXME badness
            expr,
        };
        todo!()
    }

    fn evaluate_output_text(&self, source: &ModuleSource) -> Result<&'static str, &'static str> {
        todo!()
    }

    fn evaluate_output_value(&self, source: &ModuleSource, out: &dyn Any) -> Result<&'static str, &'static str> {
        todo!()
    }

    fn evaluate_output_files(&self, source: &ModuleSource) -> Result<&'static str, &'static str> {
        todo!()
    }

    fn closed(&self, ) -> bool {
        todo!()
    }
}
