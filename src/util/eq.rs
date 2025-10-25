// Custom PartialEq and Eq implementations
// This module provides common equality trait implementations

// PartialEq implementation for log_level_t
impl PartialEq for crate::util::mcode::log_level_t {
    fn eq(&self, other: &Self) -> bool {
        use crate::util::mcode::log_level_t;
        matches!(
            (self, other),
            (log_level_t::EMERG, log_level_t::EMERG)
                | (log_level_t::ALERT, log_level_t::ALERT)
                | (log_level_t::CRIT, log_level_t::CRIT)
                | (log_level_t::ERROR, log_level_t::ERROR)
                | (log_level_t::WARN, log_level_t::WARN)
                | (log_level_t::NOTICE, log_level_t::NOTICE)
                | (log_level_t::INFO, log_level_t::INFO)
                | (log_level_t::DEBUG, log_level_t::DEBUG)
        )
    }
}

// Eq implementation for log_level_t
impl Eq for crate::util::mcode::log_level_t {}
