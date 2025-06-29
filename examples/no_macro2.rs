use simple_metrics::{MetricDef, MetricType};

fn main() {
    let data = vec![
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("cache_hits_total", "Total cache hits", MetricType::Counter).expect(""),
        MetricDef::new(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new("queue_size", "Queue size", MetricType::Gauge).expect(""),
        MetricDef::new(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram,
        )
        .expect(""),
        MetricDef::new("heap_size_bytes", "Heap size in bytes", MetricType::Gauge).expect(""),
        MetricDef::new(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge,
        )
        .expect(""),
        MetricDef::new(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter,
        )
        .expect(""),
        MetricDef::new(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram,
        )
        .expect(""),
    ];

    println!("entries: {}", data.len());
}
