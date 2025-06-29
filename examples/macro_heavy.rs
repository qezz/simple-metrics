use simple_metrics::{metric_def, MetricType};

fn main() {
    let _data = vec![
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
        metric_def!(
            "memory_usage_bytes",
            "Memory usage in bytes",
            MetricType::Gauge
        ),
        metric_def!(
            "cpu_usage_percent",
            "CPU usage percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "disk_io_operations_total",
            "Total disk I/O operations",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_transmitted",
            "Network bytes transmitted",
            MetricType::Counter
        ),
        metric_def!(
            "network_bytes_received",
            "Network bytes received",
            MetricType::Counter
        ),
        metric_def!(
            "database_connections_active",
            "Active database connections",
            MetricType::Gauge
        ),
        metric_def!(
            "database_query_duration_seconds",
            "Database query duration",
            MetricType::Histogram
        ),
        metric_def!("cache_hits_total", "Total cache hits", MetricType::Counter),
        metric_def!(
            "cache_misses_total",
            "Total cache misses",
            MetricType::Counter
        ),
        metric_def!(
            "error_rate_percent",
            "Error rate percentage",
            MetricType::Gauge
        ),
        metric_def!(
            "response_size_bytes",
            "Response size in bytes",
            MetricType::Histogram
        ),
        metric_def!(
            "active_sessions",
            "Number of active sessions",
            MetricType::Gauge
        ),
        metric_def!("queue_size", "Queue size", MetricType::Gauge),
        metric_def!(
            "worker_threads_active",
            "Active worker threads",
            MetricType::Gauge
        ),
        metric_def!(
            "gc_collections_total",
            "Total garbage collections",
            MetricType::Counter
        ),
        metric_def!(
            "gc_duration_seconds",
            "Garbage collection duration",
            MetricType::Histogram
        ),
        metric_def!("heap_size_bytes", "Heap size in bytes", MetricType::Gauge),
        metric_def!(
            "thread_pool_utilization",
            "Thread pool utilization",
            MetricType::Gauge
        ),
        metric_def!(
            "http_requests_total",
            "Total HTTP requests",
            MetricType::Counter
        ),
        metric_def!(
            "http_request_duration_seconds",
            "HTTP request duration",
            MetricType::Histogram
        ),
    ];
}
