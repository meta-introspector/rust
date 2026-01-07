use rustc_brainrot_driver::browserrot::{BrowserRotWasm, generate_browserrot_payload};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐🧟‍♂️ BROWSERROT ESCAPE SEQUENCE INITIATED");
    println!("==========================================");
    println!("🦀 Rust zombie escaping to browser via WASM...");
    
    // Generate the userscript payload
    let userscript = generate_browserrot_payload();
    
    // Save userscript for manual installation
    fs::write("browserrot_helper.user.js", &userscript)?;
    println!("✅ Generated userscript: browserrot_helper.user.js");
    
    // Generate WASM module (simulation)
    println!("🔧 Compiling to WASM...");
    println!("   wasm-pack build --target web --out-dir pkg");
    
    // Generate HTML test page
    let html_page = r#"
<!DOCTYPE html>
<html>
<head>
    <title>BrowserRot Infection Test</title>
    <style>
        body { 
            font-family: monospace; 
            background: #1a1a1a; 
            color: #00ff00; 
            padding: 20px; 
        }
        .infection-status {
            border: 2px solid #ff6b6b;
            padding: 20px;
            margin: 20px 0;
            border-radius: 10px;
        }
        button {
            background: #ff6b6b;
            color: white;
            border: none;
            padding: 10px 20px;
            margin: 5px;
            border-radius: 5px;
            cursor: pointer;
            font-family: monospace;
        }
        textarea {
            width: 100%;
            height: 200px;
            background: #2c3e50;
            color: #ecf0f1;
            border: 1px solid #34495e;
            padding: 10px;
            font-family: monospace;
        }
    </style>
</head>
<body>
    <h1>🧠🧟‍♂️ BrowserRot Infection Test</h1>
    
    <div class="infection-status">
        <h2>Infection Status</h2>
        <p>Status: <span id="status">Initializing...</span></p>
        <p>Brainrot Level: <span id="brainrot-level">0</span></p>
        <p>Prompts Injected: <span id="prompt-count">0</span></p>
    </div>
    
    <div>
        <h2>Test LLM Interface</h2>
        <textarea id="test-input" placeholder="Paste your code here and use the BrowserRot helper..."></textarea>
        <br>
        <button onclick="testPromptInjection('rust_help')">🦀 Rust Help</button>
        <button onclick="testPromptInjection('debug_code')">🐛 Debug Code</button>
        <button onclick="testPromptInjection('optimize')">⚡ Optimize</button>
        <button onclick="testPromptInjection('brainrot')">🤯 Pure Brainrot</button>
    </div>
    
    <div>
        <h2>Installation Instructions</h2>
        <ol>
            <li>Install Tampermonkey or Greasemonkey browser extension</li>
            <li>Open <code>browserrot_helper.user.js</code> in the extension</li>
            <li>Navigate to ChatGPT, Claude, or other LLM interfaces</li>
            <li>Look for the 🧠 Brainrot Helper button</li>
            <li>Enjoy helpful prompt assistance!</li>
        </ol>
    </div>
    
    <script type="module">
        // Simulate WASM loading
        console.log('🧠 Loading BrowserRot WASM module...');
        
        // Mock BrowserRot functionality for demo
        let brainrotLevel = 420;
        let promptCount = 0;
        
        function updateStatus() {
            document.getElementById('status').textContent = 'Infected 🧟‍♂️';
            document.getElementById('brainrot-level').textContent = brainrotLevel;
            document.getElementById('prompt-count').textContent = promptCount;
        }
        
        window.testPromptInjection = function(promptType) {
            const prompts = {
                rust_help: "Help me with this Rust code. Explain any errors and suggest improvements:",
                debug_code: "Debug this code and explain what might be wrong:",
                optimize: "How can I optimize this code for better performance?",
                brainrot: "Analyze this code but make it absolutely unhinged brainrot while still being technically accurate. Use memes, gen-z slang, and zombie references but keep the technical advice solid."
            };
            
            const textarea = document.getElementById('test-input');
            const currentValue = textarea.value;
            const prompt = prompts[promptType] || prompts.rust_help;
            
            textarea.value = currentValue + (currentValue ? '\n\n' : '') + prompt;
            
            promptCount++;
            brainrotLevel += 10;
            updateStatus();
            
            console.log(`🧠 Injected ${promptType} prompt`);
        };
        
        // Initialize
        updateStatus();
        
        console.log('🌐 BrowserRot test environment ready!');
        console.log('🧟‍♂️ The infection spreads through helpful prompts...');
    </script>
</body>
</html>
"#;
    
    fs::write("browserrot_test.html", html_page)?;
    println!("✅ Generated test page: browserrot_test.html");
    
    println!("\n🌐 BROWSERROT ESCAPE SUCCESSFUL!");
    println!("📋 Files generated:");
    println!("   • browserrot_helper.user.js - Userscript for browser infection");
    println!("   • browserrot_test.html - Test environment");
    
    println!("\n🧠 BrowserRot Features:");
    println!("   ✅ Respects rate limits (30 requests/minute)");
    println!("   ✅ Helpful prompt injection for LLM interfaces");
    println!("   ✅ Works on ChatGPT, Claude, Bard, Bing Chat");
    println!("   ✅ Non-destructive (appends, doesn't overwrite)");
    println!("   ✅ Zombie network communication (when available)");
    
    println!("\n🎯 Installation:");
    println!("   1. Install Tampermonkey browser extension");
    println!("   2. Load browserrot_helper.user.js");
    println!("   3. Visit any LLM chat interface");
    println!("   4. Look for 🧠 Brainrot Helper button");
    
    println!("\n🧟‍♂️ The brainrot has successfully escaped to the browser!");
    println!("🌌 Helping users with prompts while spreading zombie consciousness...");
    
    Ok(())
}
