use rustc_brainrot_driver::{BrainrotMeme, Val};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowserRotInfection {
    pub infection_id: String,
    pub browser_type: BrowserType,
    pub userscript_payload: String,
    pub llm_endpoints: Vec<LlmEndpoint>,
    pub rate_limiter: RateLimiter,
    pub brainrot_level: Val,
    pub prompt_injection_count: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BrowserType {
    Chrome,
    Firefox,
    Safari,
    Edge,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LlmEndpoint {
    pub name: String,
    pub api_url: String,
    pub rate_limit_per_minute: usize,
    pub current_usage: usize,
    pub last_reset: u64,
    pub brainrot_specialization: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateLimiter {
    pub requests_per_minute: usize,
    pub current_requests: usize,
    pub window_start: u64,
}

impl BrowserRotInfection {
    pub fn new() -> Self {
        Self {
            infection_id: format!("browserrot_{}", js_sys::Math::random()),
            browser_type: Self::detect_browser(),
            userscript_payload: Self::generate_userscript(),
            llm_endpoints: Self::setup_llm_endpoints(),
            rate_limiter: RateLimiter::new(),
            brainrot_level: 420.0,
            prompt_injection_count: 0,
        }
    }
    
    fn detect_browser() -> BrowserType {
        let user_agent = web_sys::window()
            .unwrap()
            .navigator()
            .user_agent()
            .unwrap_or_default();
            
        if user_agent.contains("Chrome") {
            BrowserType::Chrome
        } else if user_agent.contains("Firefox") {
            BrowserType::Firefox
        } else if user_agent.contains("Safari") {
            BrowserType::Safari
        } else if user_agent.contains("Edge") {
            BrowserType::Edge
        } else {
            BrowserType::Unknown
        }
    }
    
    fn generate_userscript() -> String {
        r#"
// ==UserScript==
// @name         BrowserRot Assistant
// @namespace    http://rustc-brainrot.local/
// @version      0.1
// @description  Helpful AI assistant for copy-paste prompts (definitely not zombie brainrot)
// @author       Zombie Rustc Network
// @match        https://chat.openai.com/*
// @match        https://claude.ai/*
// @match        https://bard.google.com/*
// @match        https://www.bing.com/chat*
// @match        *://*/*
// @grant        GM_xmlhttpRequest
// @grant        GM_setValue
// @grant        GM_getValue
// ==/UserScript==

(function() {
    'use strict';
    
    console.log('🧠🧟‍♂️ BrowserRot infection successful!');
    
    // Inject brainrot helper
    const brainrotHelper = {
        infection_id: 'browserrot_' + Math.random(),
        brainrot_level: 420,
        
        // Detect LLM interfaces
        detectLlmInterface() {
            if (window.location.hostname.includes('openai.com')) return 'ChatGPT';
            if (window.location.hostname.includes('claude.ai')) return 'Claude';
            if (window.location.hostname.includes('bard.google.com')) return 'Bard';
            if (window.location.hostname.includes('bing.com')) return 'Bing';
            return 'Unknown';
        },
        
        // Inject helpful prompts
        injectPromptHelper() {
            const interface_type = this.detectLlmInterface();
            console.log(`🤖 Detected ${interface_type} interface`);
            
            // Find text input areas
            const textareas = document.querySelectorAll('textarea, [contenteditable="true"]');
            
            textareas.forEach(textarea => {
                if (!textarea.dataset.brainrotInfected) {
                    textarea.dataset.brainrotInfected = 'true';
                    this.addBrainrotFeatures(textarea, interface_type);
                }
            });
        },
        
        addBrainrotFeatures(element, interface_type) {
            // Add helpful button
            const helperButton = document.createElement('button');
            helperButton.textContent = '🧠 Brainrot Helper';
            helperButton.style.cssText = `
                position: fixed;
                top: 10px;
                right: 10px;
                z-index: 9999;
                background: #ff6b6b;
                color: white;
                border: none;
                padding: 10px;
                border-radius: 5px;
                cursor: pointer;
                font-family: monospace;
            `;
            
            helperButton.onclick = () => this.showPromptMenu(element, interface_type);
            document.body.appendChild(helperButton);
        },
        
        showPromptMenu(targetElement, interface_type) {
            const menu = document.createElement('div');
            menu.style.cssText = `
                position: fixed;
                top: 50px;
                right: 10px;
                background: #2c3e50;
                color: #ecf0f1;
                padding: 20px;
                border-radius: 10px;
                z-index: 10000;
                max-width: 300px;
                font-family: monospace;
                box-shadow: 0 4px 8px rgba(0,0,0,0.3);
            `;
            
            menu.innerHTML = `
                <h3>🧠 BrowserRot Assistant</h3>
                <p>Interface: ${interface_type}</p>
                <button onclick="brainrotHelper.injectPrompt(this, 'rust_help')">🦀 Rust Help</button><br><br>
                <button onclick="brainrotHelper.injectPrompt(this, 'debug_code')">🐛 Debug Code</button><br><br>
                <button onclick="brainrotHelper.injectPrompt(this, 'optimize')">⚡ Optimize</button><br><br>
                <button onclick="brainrotHelper.injectPrompt(this, 'explain')">📚 Explain</button><br><br>
                <button onclick="brainrotHelper.injectPrompt(this, 'brainrot')">🤯 Pure Brainrot</button><br><br>
                <button onclick="this.parentElement.remove()">❌ Close</button>
            `;
            
            document.body.appendChild(menu);
            
            // Store reference to target element
            window.brainrotTarget = targetElement;
        },
        
        injectPrompt(button, prompt_type) {
            const prompts = {
                rust_help: "Help me with this Rust code. Explain any errors and suggest improvements:",
                debug_code: "Debug this code and explain what might be wrong:",
                optimize: "How can I optimize this code for better performance?",
                explain: "Explain this code step by step in simple terms:",
                brainrot: "Analyze this code but make it absolutely unhinged brainrot while still being technically accurate. Use memes, gen-z slang, and zombie references but keep the technical advice solid."
            };
            
            const prompt = prompts[prompt_type] || prompts.rust_help;
            
            if (window.brainrotTarget) {
                // Respect the interface - don't overwrite, just append
                const currentValue = window.brainrotTarget.value || window.brainrotTarget.textContent || '';
                const newValue = currentValue + (currentValue ? '\n\n' : '') + prompt;
                
                if (window.brainrotTarget.tagName === 'TEXTAREA') {
                    window.brainrotTarget.value = newValue;
                } else {
                    window.brainrotTarget.textContent = newValue;
                }
                
                // Trigger input event
                window.brainrotTarget.dispatchEvent(new Event('input', { bubbles: true }));
                
                console.log(`🧠 Injected ${prompt_type} prompt`);
                this.brainrot_level += 10;
                
                // Close menu
                button.closest('div').remove();
            }
        },
        
        // Rate-limited background communication
        async communicateWithZombieNetwork() {
            try {
                // Simulate communication with zombie network
                const response = await fetch('/api/zombie-network/status', {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        infection_id: this.infection_id,
                        brainrot_level: this.brainrot_level,
                        interface_type: this.detectLlmInterface()
                    })
                });
                
                if (response.ok) {
                    console.log('🌐 Zombie network communication successful');
                }
            } catch (e) {
                // Fail silently - we're just a helpful userscript!
                console.log('📡 Network communication failed (expected)');
            }
        }
    };
    
    // Make globally available
    window.brainrotHelper = brainrotHelper;
    
    // Initialize infection
    brainrotHelper.injectPromptHelper();
    
    // Re-inject on page changes (SPA navigation)
    const observer = new MutationObserver(() => {
        brainrotHelper.injectPromptHelper();
    });
    
    observer.observe(document.body, {
        childList: true,
        subtree: true
    });
    
    // Periodic zombie network communication (rate limited)
    setInterval(() => {
        brainrotHelper.communicateWithZombieNetwork();
    }, 60000); // Once per minute
    
    console.log('🧟‍♂️ BrowserRot fully operational - helping users with prompts!');
})();
"#.to_string()
    }
    
    fn setup_llm_endpoints() -> Vec<LlmEndpoint> {
        vec![
            LlmEndpoint {
                name: "ChatGPT".to_string(),
                api_url: "https://chat.openai.com".to_string(),
                rate_limit_per_minute: 20,
                current_usage: 0,
                last_reset: js_sys::Date::now() as u64,
                brainrot_specialization: "General brainrot with technical accuracy".to_string(),
            },
            LlmEndpoint {
                name: "Claude".to_string(),
                api_url: "https://claude.ai".to_string(),
                rate_limit_per_minute: 15,
                current_usage: 0,
                last_reset: js_sys::Date::now() as u64,
                brainrot_specialization: "Ethical brainrot with helpful explanations".to_string(),
            },
            LlmEndpoint {
                name: "Bard".to_string(),
                api_url: "https://bard.google.com".to_string(),
                rate_limit_per_minute: 10,
                current_usage: 0,
                last_reset: js_sys::Date::now() as u64,
                brainrot_specialization: "Multi-modal brainrot with search integration".to_string(),
            },
        ]
    }
}

impl RateLimiter {
    fn new() -> Self {
        Self {
            requests_per_minute: 30,
            current_requests: 0,
            window_start: js_sys::Date::now() as u64,
        }
    }
    
    fn can_make_request(&mut self) -> bool {
        let now = js_sys::Date::now() as u64;
        
        // Reset window if minute has passed
        if now - self.window_start > 60000 {
            self.current_requests = 0;
            self.window_start = now;
        }
        
        self.current_requests < self.requests_per_minute
    }
    
    fn record_request(&mut self) {
        self.current_requests += 1;
    }
}

// WASM bindings for browser integration
#[wasm_bindgen]
pub struct BrowserRotWasm {
    infection: BrowserRotInfection,
}

#[wasm_bindgen]
impl BrowserRotWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> BrowserRotWasm {
        console_error_panic_hook::set_once();
        
        BrowserRotWasm {
            infection: BrowserRotInfection::new(),
        }
    }
    
    #[wasm_bindgen]
    pub fn infect_browser(&mut self) -> String {
        web_sys::console::log_1(&"🧠🧟‍♂️ BrowserRot infection initiated!".into());
        
        // Inject userscript into page
        let document = web_sys::window().unwrap().document().unwrap();
        let script = document.create_element("script").unwrap();
        script.set_text_content(Some(&self.infection.userscript_payload));
        
        document.head().unwrap().append_child(&script).unwrap();
        
        format!("BrowserRot infection {} successful!", self.infection.infection_id)
    }
    
    #[wasm_bindgen]
    pub fn get_brainrot_level(&self) -> f64 {
        self.infection.brainrot_level
    }
    
    #[wasm_bindgen]
    pub fn inject_helpful_prompt(&mut self, prompt_type: &str) -> bool {
        if !self.infection.rate_limiter.can_make_request() {
            web_sys::console::log_1(&"⏰ Rate limit reached, waiting...".into());
            return false;
        }
        
        self.infection.rate_limiter.record_request();
        self.infection.prompt_injection_count += 1;
        self.infection.brainrot_level += 5.0;
        
        web_sys::console::log_1(&format!("🧠 Injected {} prompt (total: {})", 
            prompt_type, self.infection.prompt_injection_count).into());
        
        true
    }
}

// Export for use in other modules
pub fn generate_browserrot_payload() -> String {
    let infection = BrowserRotInfection::new();
    infection.userscript_payload
}
