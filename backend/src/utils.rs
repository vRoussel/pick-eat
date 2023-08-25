use std::error::Error;

pub trait RetryBehavior {
    fn is_retryable(&self) -> bool;
}

pub async fn retry_up_to_n_times<F, T, O, E>(mut func: F, max_tries: usize) -> T::Output
where
    F: FnMut() -> T,
    T: std::future::Future<Output = Result<O, E>>,
    E: Error + RetryBehavior,
{
    let mut fail_count = 0;
    loop {
        match func().await {
            Ok(t) => return Ok(t),
            Err(e) => {
                fail_count += 1;
                if fail_count >= max_tries {
                    return Err(e);
                }
            }
        }
    }
}

pub fn sentence_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
