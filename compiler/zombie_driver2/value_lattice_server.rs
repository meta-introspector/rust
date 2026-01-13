use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use warp::{Filter, Reply};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JobStatus {
    id: String,
    file_path: String,
    status: String, // "queued", "processing", "completed", "failed"
    started_at: Option<SystemTime>,
    completed_at: Option<SystemTime>,
    error: Option<String>,
    progress: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QueueStats {
    queued: usize,
    processing: usize,
    completed: usize,
    failed: usize,
    memory_usage_gb: f64,
    total_files_processed: usize,
}

type JobQueue = Arc<Mutex<VecDeque<JobStatus>>>;
type JobRegistry = Arc<Mutex<HashMap<String, JobStatus>>>;

struct ValueLatticeServer {
    queue: JobQueue,
    registry: JobRegistry,
    memory_usage: Arc<Mutex<f64>>,
}

impl ValueLatticeServer {
    fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            registry: Arc::new(Mutex::new(HashMap::new())),
            memory_usage: Arc::new(Mutex::new(0.0)),
        }
    }

    fn add_file(&self, file_path: String) -> String {
        let job_id = format!("job_{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
        
        let job = JobStatus {
            id: job_id.clone(),
            file_path,
            status: "queued".to_string(),
            started_at: None,
            completed_at: None,
            error: None,
            progress: 0.0,
        };

        self.queue.lock().unwrap().push_back(job.clone());
        self.registry.lock().unwrap().insert(job_id.clone(), job);
        
        job_id
    }

    fn get_stats(&self) -> QueueStats {
        let registry = self.registry.lock().unwrap();
        let queued = registry.values().filter(|j| j.status == "queued").count();
        let processing = registry.values().filter(|j| j.status == "processing").count();
        let completed = registry.values().filter(|j| j.status == "completed").count();
        let failed = registry.values().filter(|j| j.status == "failed").count();
        let memory_usage_gb = *self.memory_usage.lock().unwrap();

        QueueStats {
            queued,
            processing,
            completed,
            failed,
            memory_usage_gb,
            total_files_processed: completed + failed,
        }
    }

    fn get_jobs(&self) -> Vec<JobStatus> {
        self.registry.lock().unwrap().values().cloned().collect()
    }

    fn start_worker(&self) {
        let queue = Arc::clone(&self.queue);
        let registry = Arc::clone(&self.registry);
        let memory_usage = Arc::clone(&self.memory_usage);

        thread::spawn(move || {
            loop {
                let job = {
                    let mut q = queue.lock().unwrap();
                    q.pop_front()
                };

                if let Some(mut job) = job {
                    // Update status to processing
                    job.status = "processing".to_string();
                    job.started_at = Some(SystemTime::now());
                    registry.lock().unwrap().insert(job.id.clone(), job.clone());

                    println!("🔄 Processing: {}", job.file_path);

                    // Simulate processing with progress updates
                    for i in 1..=10 {
                        thread::sleep(Duration::from_millis(500));
                        job.progress = i as f64 / 10.0;
                        registry.lock().unwrap().insert(job.id.clone(), job.clone());
                        
                        // Simulate memory usage
                        *memory_usage.lock().unwrap() += 0.1;
                    }

                    // Complete job
                    job.status = "completed".to_string();
                    job.completed_at = Some(SystemTime::now());
                    job.progress = 1.0;
                    registry.lock().unwrap().insert(job.id.clone(), job.clone());

                    println!("✅ Completed: {}", job.file_path);
                } else {
                    thread::sleep(Duration::from_millis(1000));
                }
            }
        });
    }
}

#[tokio::main]
async fn main() {
    println!("🚀 VALUE LATTICE CONTROL SERVER");
    println!("==============================");

    let server = Arc::new(ValueLatticeServer::new());
    server.start_worker();

    let server_filter = warp::any().map(move || Arc::clone(&server));

    // GET /status - Queue statistics
    let status = warp::path("status")
        .and(warp::get())
        .and(server_filter.clone())
        .map(|server: Arc<ValueLatticeServer>| {
            warp::reply::json(&server.get_stats())
        });

    // GET /jobs - List all jobs
    let jobs = warp::path("jobs")
        .and(warp::get())
        .and(server_filter.clone())
        .map(|server: Arc<ValueLatticeServer>| {
            warp::reply::json(&server.get_jobs())
        });

    // POST /add - Add file to queue
    let add = warp::path("add")
        .and(warp::post())
        .and(warp::body::json())
        .and(server_filter.clone())
        .map(|file_req: HashMap<String, String>, server: Arc<ValueLatticeServer>| {
            if let Some(file_path) = file_req.get("file_path") {
                let job_id = server.add_file(file_path.clone());
                warp::reply::json(&serde_json::json!({"job_id": job_id, "status": "queued"}))
            } else {
                warp::reply::json(&serde_json::json!({"error": "file_path required"}))
            }
        });

    // GET /dashboard - Simple HTML dashboard
    let dashboard = warp::path("dashboard")
        .and(warp::get())
        .map(|| {
            warp::reply::html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Value Lattice Control Dashboard</title>
    <style>
        body { font-family: monospace; margin: 20px; }
        .stats { background: #f0f0f0; padding: 10px; margin: 10px 0; }
        .job { border: 1px solid #ccc; margin: 5px 0; padding: 10px; }
        .processing { background: #fff3cd; }
        .completed { background: #d4edda; }
        .failed { background: #f8d7da; }
        button { padding: 10px; margin: 5px; }
    </style>
</head>
<body>
    <h1>🔢 Value Lattice Control Dashboard</h1>
    
    <div id="stats" class="stats">Loading stats...</div>
    
    <div>
        <input type="text" id="fileInput" placeholder="Enter file path" style="width: 400px;">
        <button onclick="addFile()">Add File</button>
        <button onclick="refresh()">Refresh</button>
    </div>
    
    <div id="jobs">Loading jobs...</div>

    <script>
        async function refresh() {
            const stats = await fetch('/status').then(r => r.json());
            document.getElementById('stats').innerHTML = `
                📊 Queued: ${stats.queued} | Processing: ${stats.processing} | 
                Completed: ${stats.completed} | Failed: ${stats.failed} | 
                Memory: ${stats.memory_usage_gb.toFixed(2)} GB
            `;
            
            const jobs = await fetch('/jobs').then(r => r.json());
            document.getElementById('jobs').innerHTML = jobs.map(job => `
                <div class="job ${job.status}">
                    <strong>${job.id}</strong> - ${job.file_path}<br>
                    Status: ${job.status} | Progress: ${(job.progress * 100).toFixed(1)}%
                </div>
            `).join('');
        }
        
        async function addFile() {
            const filePath = document.getElementById('fileInput').value;
            if (filePath) {
                await fetch('/add', {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify({file_path: filePath})
                });
                document.getElementById('fileInput').value = '';
                refresh();
            }
        }
        
        setInterval(refresh, 2000);
        refresh();
    </script>
</body>
</html>
            "#)
        });

    let routes = status
        .or(jobs)
        .or(add)
        .or(dashboard)
        .with(warp::cors().allow_any_origin());

    println!("🌐 Server running at http://localhost:3030");
    println!("📊 Dashboard: http://localhost:3030/dashboard");
    println!("🔌 API endpoints:");
    println!("   GET  /status     - Queue statistics");
    println!("   GET  /jobs       - List all jobs");
    println!("   POST /add        - Add file to queue");

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
