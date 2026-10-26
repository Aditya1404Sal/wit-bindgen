//@ wasmtime-flags = '-Wcomponent-model-async'

include!(env!("BINDINGS"));

use crate::my::test::i::*;
use futures::task::noop_waker_ref;
use std::future::Future;
use std::task::Context;

struct Component;

export!(Component);

impl Guest for Component {
    async fn run() {
        println!("test that finished calls free their arguments");
        {
            let before = allocated_bytes();
            for _ in 0..10 {
                many_arguments(
                    "a".into(),
                    "b".into(),
                    "c".into(),
                    "d".into(),
                    "e".into(),
                    "f".into(),
                    "g".into(),
                    "h".into(),
                    "i".into(),
                )
                .await;
            }
            assert_eq!(allocated_bytes(), before);
        }

        println!("test that payloads of options and results arrive intact");
        {
            let before = allocated_bytes();
            for _ in 0..10 {
                many_arguments_cases(
                    Some(42),
                    Err("error".into()),
                    "c".into(),
                    "d".into(),
                    "e".into(),
                    "f".into(),
                    "g".into(),
                    "h".into(),
                )
                .await;
            }
            assert_eq!(allocated_bytes(), before);
        }

        println!("test that cancelled calls free their arguments");
        {
            let before = allocated_bytes();
            for _ in 0..10 {
                let (tx, rx) = wit_future::new(|| unreachable!());
                let mut call = Box::pin(many_arguments_pending(
                    rx,
                    "a".into(),
                    "b".into(),
                    "c".into(),
                    "d".into(),
                    "e".into(),
                    "f".into(),
                    "g".into(),
                    "h".into(),
                ));
                assert!(call
                    .as_mut()
                    .poll(&mut Context::from_waker(noop_waker_ref()))
                    .is_pending());
                drop(call);
                tx.write(()).await.unwrap_err();
            }
            assert_eq!(allocated_bytes(), before);
        }
    }
}
