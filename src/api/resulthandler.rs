use std::{fmt::Debug, fmt::Display, future::Future};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResultHandler<A> {
    OkResult(A),
    ErrResult(String),
}

impl<A> ResultHandler<A> {
    pub fn pipe<B>(self, f: impl FnOnce(A) -> ResultHandler<B>) -> ResultHandler<B> {
        match self {
            ResultHandler::OkResult(ok_res_a) => f(ok_res_a),
            ResultHandler::ErrResult(err_res) => ResultHandler::ErrResult(format!("{}", err_res)),
        }
    }

    pub fn pipe_action<B>(self, f: impl FnOnce() -> ResultHandler<B>) -> ResultHandler<B> {
        match self {
            ResultHandler::OkResult(_) => f(),
            ResultHandler::ErrResult(err_res) => ResultHandler::ErrResult(format!("{}", err_res)),
        }
    }

    pub async fn pipe_action_async<B, Fut>(self, f: impl FnOnce(A) -> Fut) -> ResultHandler<B>
    where
        Fut: Future<Output = ResultHandler<B>>,
    {
        match self {
            ResultHandler::OkResult(ok_a_res) => match f(ok_a_res).await {
                ResultHandler::OkResult(ok_b_res) => ResultHandler::OkResult(ok_b_res),
                ResultHandler::ErrResult(b_res_err) => {
                    ResultHandler::ErrResult(format!("{:?>}", b_res_err))
                }
            },
            ResultHandler::ErrResult(err_res) => {
                ResultHandler::ErrResult(format!("{:?>}", err_res))
            }
        }
    }

    pub fn pipe_result_action<B, C: Debug + Display>(
        self,
        f: impl FnOnce(A) -> Result<B, C>,
    ) -> ResultHandler<B> {
        match self {
            ResultHandler::OkResult(ok_ares) => match f(ok_ares) {
                Ok(ok_bres) => ResultHandler::OkResult(ok_bres),
                Err(err_res) => ResultHandler::ErrResult(format!("{:?>}", err_res)),
            },
            ResultHandler::ErrResult(err_res) => {
                ResultHandler::ErrResult(format!("{:?>}", err_res))
            }
        }
    }

    pub async fn pipe_result_action_async<B, C: Debug + Display, Fut>(
        self,
        f: impl FnOnce(A) -> Fut,
    ) -> ResultHandler<B>
    where
        Fut: Future<Output = Result<B, C>>,
    {
        match self {
            ResultHandler::OkResult(ok_a_res) => match f(ok_a_res).await {
                Ok(ok_b_res) => ResultHandler::OkResult(ok_b_res),
                Err(b_res_error) => ResultHandler::ErrResult(format!("{:?>}", b_res_error)),
            },
            ResultHandler::ErrResult(err_res) => {
                ResultHandler::ErrResult(format!("{:?>}", err_res))
            }
        }
    }
}

pub fn to_result_handler<B, C: Debug + Display>(res: Result<B, C>) -> ResultHandler<B> {
    match res {
        Ok(ok_res) => ResultHandler::OkResult(ok_res),
        Err(str_err) => ResultHandler::ErrResult(format!("{:?>}", str_err)),
    }
}
