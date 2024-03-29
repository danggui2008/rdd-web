//资源回收使用，有点像golang的defer,但这两个绝对不一样。。。
pub struct Defer<F>(Option<F>)
where
    F: FnOnce() -> ();

impl<F> Drop for Defer<F>
where
    F: FnOnce() -> (),
{
    fn drop(&mut self) {
        if let Some(f) = self.0.take() {
            f();
        }
    }
}

pub fn defer<F: FnOnce()>(f: F) -> Defer<F> {
    Defer(Some(f))
}
