pub mod prover {
    //! Prover Configuration Constants
    //!
    //! This module contains all configuration constants for the prover system,
    //! organized by functional area for clarity and maintainability.

    // =============================================================================
    // QUEUE CONFIGURATION
    // =============================================================================
    // All queue sizes are chosen to be larger than the API page size (currently 50)
    // to provide adequate buffering while preventing excessive memory usage.

    /// Maximum number of tasks that can be queued for processing
    pub const TASK_QUEUE_SIZE: usize = 100;

    /// Maximum number of events that can be queued for UI updates
    pub const EVENT_QUEUE_SIZE: usize = 100;

    /// Maximum number of proof results that can be queued for submission
    pub const RESULT_QUEUE_SIZE: usize = 100;

    // =============================================================================
    // TASK FETCHING BEHAVIOR
    // =============================================================================

    /// Minimum queue level that triggers new task fetching
    /// When task queue drops below this threshold, fetch new tasks
    pub const LOW_WATER_MARK: usize = 1;

    /// Delay the fetch task time by this amount of seconds
    pub const FETCH_TASK_DELAY_TIME: u64 = 10;

    // =============================================================================
    // TIMING AND BACKOFF CONFIGURATION
    // =============================================================================

    /// Default backoff duration when retrying failed operations (milliseconds)
    /// Set to 2 minutes to balance responsiveness with server load
    pub const BACKOFF_DURATION: u64 = 120_000; // 2 minutes

    // =============================================================================
    // CACHE MANAGEMENT
    // =============================================================================

    /// Duration to keep task IDs in duplicate-prevention cache (milliseconds)
    /// Long enough to prevent immediate re-processing, short enough to allow
    /// eventual retry of legitimately failed tasks
    pub const CACHE_EXPIRATION: u64 = 300_000; // 5 minutes

    // =============================================================================
    // COMPUTED CONSTANTS
    // =============================================================================

    /// Maximum number of completed tasks to track (prevents memory growth)
    /// Set to 5x the task queue size to provide adequate duplicate detection
    pub const MAX_COMPLETED_TASKS: usize = TASK_QUEUE_SIZE * 5;
}

use std::sync::atomic::{AtomicU64, Ordering};
use rand::Rng;

// 默认429错误重试超时时间（秒）
const DEFAULT_RETRY_TIMEOUT: u64 = 30;

// 全局429错误重试超时时间
static RETRY_TIMEOUT: AtomicU64 = AtomicU64::new(DEFAULT_RETRY_TIMEOUT);

/// 设置全局429错误重试超时时间
pub fn set_retry_timeout(timeout_seconds: u64) {
    RETRY_TIMEOUT.store(timeout_seconds, Ordering::SeqCst);
}

/// 获取429错误重试超时时间，带±10%的随机浮动
pub fn get_retry_timeout() -> u64 {
    let base_timeout = RETRY_TIMEOUT.load(Ordering::SeqCst);

    // 确保至少有1秒的超时时间
    if base_timeout <= 1 {
        return 1;
    }

    // 计算±10%的浮动范围
    let variation_range = (base_timeout as f64 * 0.1) as u64;
    if variation_range == 0 {
        return base_timeout;
    }

    // 生成-10%到+10%之间的随机变化
    let mut rng = rand::thread_rng();
    let variation = rng.gen_range(0..=variation_range * 2) as i64 - variation_range as i64;

    // 应用变化并确保结果为正数
    let result = base_timeout as i64 + variation;
    if result < 1 {
        1
    } else {
        result as u64
    }
}
