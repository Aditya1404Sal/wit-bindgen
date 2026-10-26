use wit_bindgen::FutureReader;

include!(env!("BINDINGS"));

struct Component;

export!(Component);

#[path = "../lists/alloc.rs"]
mod alloc;

impl crate::exports::my::test::i::Guest for Component {
    async fn many_arguments(
        a: String,
        b: String,
        c: String,
        d: String,
        e: String,
        f: String,
        g: String,
        h: String,
        i: String,
    ) {
        assert_eq!(
            [a, b, c, d, e, f, g, h, i],
            ["a", "b", "c", "d", "e", "f", "g", "h", "i"]
        );
    }

    async fn many_arguments_pending(
        x: FutureReader<()>,
        a: String,
        b: String,
        c: String,
        d: String,
        e: String,
        f: String,
        g: String,
        h: String,
    ) {
        assert_eq!(
            [a, b, c, d, e, f, g, h],
            ["a", "b", "c", "d", "e", "f", "g", "h"]
        );
        x.await
    }

    async fn many_arguments_cases(
        a: Option<u32>,
        b: Result<u32, String>,
        c: String,
        d: String,
        e: String,
        f: String,
        g: String,
        h: String,
    ) {
        assert_eq!(a, Some(42));
        assert_eq!(b, Err("error".to_string()));
        assert_eq!([c, d, e, f, g, h], ["c", "d", "e", "f", "g", "h"]);
    }

    fn allocated_bytes() -> u32 {
        alloc::get().try_into().unwrap()
    }
}
