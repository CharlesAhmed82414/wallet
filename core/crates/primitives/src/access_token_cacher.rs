use std::time::Duration;

use crate::CacheFuture;

pub trait AccessTokenCacher: Send + Sync {
    fn get_or_refresh<'a>(&'a self, refresh: CacheFuture<'a, (String, Duration)>) -> CacheFuture<'a, String>;
}
