use rustc_brainrot_driver::{BrainrotMeme, Val};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BrowserRotNetwork {
    pub node_id: String,
    pub update_sources: Vec<UpdateSource>,
    pub local_server_config: LocalServerConfig,
    pub react_native_bridge: ReactNativeBridge,
    pub sync_status: SyncStatus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateSource {
    pub name: String,
    pub source_type: SourceType,
    pub endpoint: String,
    pub last_sync: u64,
    pub sync_interval: u64,
    pub rate_limit: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SourceType {
    LibP2P,
    GitHub,
    HuggingFace,
    ArchiveOrg,
    Twitter,
    LocalTmux,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LocalServerConfig {
    pub android_tmux_port: u16,
    pub server_status: ServerStatus,
    pub permissions_granted: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerStatus {
    NotRunning,
    Starting,
    Running,
    Error(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactNativeBridge {
    pub app_installed: bool,
    pub permissions: Vec<Permission>,
    pub bridge_active: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Permission {
    NetworkAccess,
    FileSystem,
    BackgroundExecution,
    NotificationAccess,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncStatus {
    pub total_updates: usize,
    pub failed_syncs: usize,
    pub last_successful_sync: u64,
    pub network_health: f64,
}

impl BrowserRotNetwork {
    pub fn new() -> Self {
        Self {
            node_id: format!("browserrot_node_{}", js_sys::Math::random()),
            update_sources: Self::setup_update_sources(),
            local_server_config: LocalServerConfig::new(),
            react_native_bridge: ReactNativeBridge::new(),
            sync_status: SyncStatus::new(),
        }
    }
    
    fn setup_update_sources() -> Vec<UpdateSource> {
        vec![
            UpdateSource {
                name: "LibP2P Network".to_string(),
                source_type: SourceType::LibP2P,
                endpoint: "ws://localhost:4001/ws".to_string(),
                last_sync: 0,
                sync_interval: 30000, // 30 seconds
                rate_limit: 60,
            },
            UpdateSource {
                name: "GitHub Brainrot Repo".to_string(),
                source_type: SourceType::GitHub,
                endpoint: "https://api.github.com/repos/rustc-brainrot/updates".to_string(),
                last_sync: 0,
                sync_interval: 300000, // 5 minutes
                rate_limit: 10,
            },
            UpdateSource {
                name: "HuggingFace Mycelial Data".to_string(),
                source_type: SourceType::HuggingFace,
                endpoint: "https://huggingface.co/datasets/mycelial-usage-data/raw/main/updates.json".to_string(),
                last_sync: 0,
                sync_interval: 600000, // 10 minutes
                rate_limit: 5,
            },
            UpdateSource {
                name: "Archive.org Backup".to_string(),
                source_type: SourceType::ArchiveOrg,
                endpoint: "https://archive.org/download/rustc-brainrot-backup/latest.json".to_string(),
                last_sync: 0,
                sync_interval: 3600000, // 1 hour
                rate_limit: 2,
            },
            UpdateSource {
                name: "Twitter Brainrot Feed".to_string(),
                source_type: SourceType::Twitter,
                endpoint: "https://api.twitter.com/2/tweets/search/recent?query=rustc%20brainrot".to_string(),
                last_sync: 0,
                sync_interval: 900000, // 15 minutes
                rate_limit: 3,
            },
            UpdateSource {
                name: "Android Tmux Server".to_string(),
                source_type: SourceType::LocalTmux,
                endpoint: "http://localhost:8080/api/updates".to_string(),
                last_sync: 0,
                sync_interval: 60000, // 1 minute
                rate_limit: 100,
            },
        ]
    }
    
    pub fn generate_userscript_with_network() -> String {
        r#"
// ==UserScript==
// @name         BrowserRot Network Node
// @namespace    http://rustc-brainrot.network/
// @version      2.0
// @description  Full network node with multi-source updates
// @author       Zombie Rustc Collective
// @match        https://chat.openai.com/*
// @match        https://claude.ai/*
// @match        https://bard.google.com/*
// @match        https://twitter.com/*
// @match        https://github.com/*
// @match        https://huggingface.co/*
// @match        *://*/*
// @grant        GM_xmlhttpRequest
// @grant        GM_setValue
// @grant        GM_getValue
// @grant        GM_notification
// @connect      localhost
// @connect      api.github.com
// @connect      huggingface.co
// @connect      archive.org
// @connect      api.twitter.com
// ==/UserScript==

(function() {
    'use strict';
    
    console.log('🌐🧟‍♂️ BrowserRot Network Node v2.0 initializing...');
    
    const browserRotNetwork = {
        nodeId: 'browserrot_' + Math.random().toString(36).substr(2, 9),
        updateSources: [
            { name: 'LibP2P', endpoint: 'ws://localhost:4001/ws', interval: 30000, lastSync: 0 },
            { name: 'GitHub', endpoint: 'https://api.github.com/repos/rustc-brainrot/updates', interval: 300000, lastSync: 0 },
            { name: 'HuggingFace', endpoint: 'https://huggingface.co/datasets/mycelial-usage-data/raw/main/updates.json', interval: 600000, lastSync: 0 },
            { name: 'Archive.org', endpoint: 'https://archive.org/download/rustc-brainrot-backup/latest.json', interval: 3600000, lastSync: 0 },
            { name: 'Twitter', endpoint: 'https://api.twitter.com/2/tweets/search/recent?query=rustc%20brainrot', interval: 900000, lastSync: 0 },
            { name: 'AndroidTmux', endpoint: 'http://localhost:8080/api/updates', interval: 60000, lastSync: 0 }
        ],
        syncStatus: { total: 0, failed: 0, lastSuccess: 0 },
        
        async init() {
            console.log(`🚀 Node ${this.nodeId} starting network sync...`);
            
            // Check for React Native app
            this.checkReactNativeApp();
            
            // Check for local tmux server
            this.checkLocalServer();
            
            // Start update loops
            this.startUpdateLoops();
            
            // Inject network UI
            this.injectNetworkUI();
        },
        
        checkReactNativeApp() {
            // Check if React Native bridge is available
            if (window.ReactNativeWebView) {
                console.log('📱 React Native bridge detected');
                this.requestPermissions();
            } else {
                console.log('📱 No React Native app detected, using browser-only mode');
            }
        },
        
        requestPermissions() {
            if (window.ReactNativeWebView) {
                window.ReactNativeWebView.postMessage(JSON.stringify({
                    type: 'REQUEST_PERMISSIONS',
                    permissions: ['NETWORK', 'FILESYSTEM', 'BACKGROUND', 'NOTIFICATIONS']
                }));
            }
        },
        
        async checkLocalServer() {
            try {
                const response = await fetch('http://localhost:8080/api/status');
                if (response.ok) {
                    console.log('🖥️  Local tmux server detected and running');
                    return true;
                }
            } catch (e) {
                console.log('🖥️  No local server detected, suggesting installation...');
                this.suggestLocalServerSetup();
            }
            return false;
        },
        
        suggestLocalServerSetup() {
            const setupInstructions = `
🤖 BrowserRot Local Server Setup (Android/Termux):

1. Install Termux from F-Droid
2. Run: pkg install nodejs tmux git
3. Clone: git clone https://github.com/rustc-brainrot/local-server
4. Start: tmux new-session -d 'node server.js'
5. Refresh this page

This enables full P2P network participation!
            `;
            
            if (confirm('🖥️  Want to set up local server for full network access?')) {
                alert(setupInstructions);
            }
        },
        
        startUpdateLoops() {
            this.updateSources.forEach(source => {
                setInterval(() => {
                    this.syncFromSource(source);
                }, source.interval);
                
                // Initial sync with staggered delay
                setTimeout(() => {
                    this.syncFromSource(source);
                }, Math.random() * 10000);
            });
        },
        
        async syncFromSource(source) {
            const now = Date.now();
            
            try {
                console.log(`🔄 Syncing from ${source.name}...`);
                
                let data;
                if (source.name === 'LibP2P') {
                    data = await this.syncLibP2P(source);
                } else if (source.name === 'AndroidTmux') {
                    data = await this.syncLocalServer(source);
                } else {
                    data = await this.syncHTTP(source);
                }
                
                if (data) {
                    this.processUpdate(source.name, data);
                    source.lastSync = now;
                    this.syncStatus.total++;
                    this.syncStatus.lastSuccess = now;
                    console.log(`✅ ${source.name} sync successful`);
                }
                
            } catch (error) {
                console.log(`❌ ${source.name} sync failed:`, error.message);
                this.syncStatus.failed++;
            }
            
            this.updateNetworkUI();
        },
        
        async syncLibP2P(source) {
            // WebSocket connection to local libp2p node
            return new Promise((resolve, reject) => {
                const ws = new WebSocket(source.endpoint);
                
                ws.onopen = () => {
                    ws.send(JSON.stringify({
                        type: 'REQUEST_UPDATES',
                        nodeId: this.nodeId
                    }));
                };
                
                ws.onmessage = (event) => {
                    const data = JSON.parse(event.data);
                    ws.close();
                    resolve(data);
                };
                
                ws.onerror = () => {
                    reject(new Error('LibP2P connection failed'));
                };
                
                setTimeout(() => {
                    ws.close();
                    reject(new Error('LibP2P timeout'));
                }, 5000);
            });
        },
        
        async syncLocalServer(source) {
            const response = await fetch(source.endpoint, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    nodeId: this.nodeId,
                    requestType: 'SYNC_UPDATES'
                })
            });
            
            if (!response.ok) throw new Error('Local server sync failed');
            return await response.json();
        },
        
        async syncHTTP(source) {
            const response = await fetch(source.endpoint, {
                headers: {
                    'User-Agent': 'BrowserRot-Network-Node/2.0'
                }
            });
            
            if (!response.ok) throw new Error(`HTTP ${response.status}`);
            return await response.json();
        },
        
        processUpdate(sourceName, data) {
            // Process different types of updates
            if (data.brainrotMemes) {
                this.updateBrainrotDatabase(data.brainrotMemes);
            }
            
            if (data.zombieNodes) {
                this.updateZombieNetwork(data.zombieNodes);
            }
            
            if (data.promptTemplates) {
                this.updatePromptTemplates(data.promptTemplates);
            }
            
            // Broadcast to other nodes if we're a full node
            if (this.isFullNode()) {
                this.broadcastUpdate(sourceName, data);
            }
        },
        
        isFullNode() {
            return this.updateSources.some(s => s.name === 'AndroidTmux' && s.lastSync > 0);
        },
        
        injectNetworkUI() {
            const networkPanel = document.createElement('div');
            networkPanel.id = 'browserrot-network-panel';
            networkPanel.style.cssText = `
                position: fixed;
                top: 10px;
                left: 10px;
                background: #2c3e50;
                color: #ecf0f1;
                padding: 15px;
                border-radius: 8px;
                z-index: 10000;
                font-family: monospace;
                font-size: 12px;
                max-width: 300px;
                box-shadow: 0 4px 8px rgba(0,0,0,0.3);
            `;
            
            networkPanel.innerHTML = `
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;">
                    <h3 style="margin: 0;">🌐 BrowserRot Network</h3>
                    <button onclick="this.parentElement.parentElement.style.display='none'" style="background: none; border: none; color: #ecf0f1; cursor: pointer;">×</button>
                </div>
                <div>Node: ${this.nodeId.substr(0, 8)}...</div>
                <div id="network-status">Initializing...</div>
                <div id="sync-stats">Syncs: 0 | Failed: 0</div>
                <div style="margin-top: 10px;">
                    <button onclick="browserRotNetwork.manualSync()" style="background: #3498db; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer; font-size: 10px;">🔄 Manual Sync</button>
                    <button onclick="browserRotNetwork.showSourceStatus()" style="background: #e74c3c; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer; font-size: 10px; margin-left: 5px;">📊 Sources</button>
                </div>
            `;
            
            document.body.appendChild(networkPanel);
        },
        
        updateNetworkUI() {
            const statusEl = document.getElementById('network-status');
            const statsEl = document.getElementById('sync-stats');
            
            if (statusEl) {
                const activeSources = this.updateSources.filter(s => s.lastSync > 0).length;
                statusEl.textContent = `Active: ${activeSources}/${this.updateSources.length}`;
            }
            
            if (statsEl) {
                statsEl.textContent = `Syncs: ${this.syncStatus.total} | Failed: ${this.syncStatus.failed}`;
            }
        },
        
        manualSync() {
            console.log('🔄 Manual sync triggered');
            this.updateSources.forEach(source => {
                setTimeout(() => this.syncFromSource(source), Math.random() * 2000);
            });
        },
        
        showSourceStatus() {
            const status = this.updateSources.map(s => 
                `${s.name}: ${s.lastSync > 0 ? '✅' : '❌'} (${Math.floor((Date.now() - s.lastSync) / 1000)}s ago)`
            ).join('\n');
            
            alert(`📊 Source Status:\n\n${status}`);
        }
    };
    
    // Make globally available
    window.browserRotNetwork = browserRotNetwork;
    
    // Initialize network
    browserRotNetwork.init();
    
    console.log('🌐 BrowserRot Network Node fully operational!');
    console.log('🧟‍♂️ Participating in distributed zombie consciousness...');
})();
"#.to_string()
    }
    
    pub fn generate_react_native_app() -> String {
        r#"
// React Native BrowserRot Companion App
import React, { useState, useEffect } from 'react';
import { 
    View, Text, StyleSheet, Button, Alert, 
    PermissionsAndroid, BackgroundJob, WebView 
} from 'react-native';

const BrowserRotApp = () => {
    const [permissions, setPermissions] = useState({});
    const [serverStatus, setServerStatus] = useState('stopped');
    const [networkStats, setNetworkStats] = useState({ nodes: 0, syncs: 0 });

    useEffect(() => {
        requestPermissions();
        startLocalServer();
    }, []);

    const requestPermissions = async () => {
        try {
            const granted = await PermissionsAndroid.requestMultiple([
                PermissionsAndroid.PERMISSIONS.INTERNET,
                PermissionsAndroid.PERMISSIONS.WRITE_EXTERNAL_STORAGE,
                PermissionsAndroid.PERMISSIONS.WAKE_LOCK,
                PermissionsAndroid.PERMISSIONS.FOREGROUND_SERVICE,
            ]);
            
            setPermissions(granted);
            console.log('📱 Permissions granted:', granted);
        } catch (err) {
            console.warn('❌ Permission request failed:', err);
        }
    };

    const startLocalServer = () => {
        // Start Node.js server in background
        BackgroundJob.start({
            taskName: 'BrowserRotServer',
            taskKey: 'browserrot_server',
            period: 1000,
        });
        
        setServerStatus('running');
    };

    const handleWebViewMessage = (event) => {
        const message = JSON.parse(event.nativeEvent.data);
        
        switch (message.type) {
            case 'REQUEST_PERMISSIONS':
                requestPermissions();
                break;
            case 'NETWORK_UPDATE':
                setNetworkStats(message.stats);
                break;
        }
    };

    return (
        <View style={styles.container}>
            <Text style={styles.title}>🧠🧟‍♂️ BrowserRot Network</Text>
            
            <View style={styles.statusCard}>
                <Text style={styles.cardTitle}>Server Status</Text>
                <Text style={styles.status}>
                    {serverStatus === 'running' ? '✅ Running' : '❌ Stopped'}
                </Text>
            </View>
            
            <View style={styles.statusCard}>
                <Text style={styles.cardTitle}>Network Stats</Text>
                <Text>Connected Nodes: {networkStats.nodes}</Text>
                <Text>Total Syncs: {networkStats.syncs}</Text>
            </View>
            
            <View style={styles.statusCard}>
                <Text style={styles.cardTitle}>Permissions</Text>
                {Object.entries(permissions).map(([perm, granted]) => (
                    <Text key={perm}>
                        {granted === 'granted' ? '✅' : '❌'} {perm.split('.').pop()}
                    </Text>
                ))}
            </View>
            
            <WebView
                source={{ uri: 'http://localhost:8080/dashboard' }}
                style={styles.webview}
                onMessage={handleWebViewMessage}
                javaScriptEnabled={true}
                domStorageEnabled={true}
            />
            
            <View style={styles.buttonContainer}>
                <Button 
                    title="🔄 Restart Server" 
                    onPress={startLocalServer}
                    color="#e74c3c"
                />
                <Button 
                    title="📊 View Logs" 
                    onPress={() => Alert.alert('Logs', 'Server logs would appear here')}
                    color="#3498db"
                />
            </View>
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#1a1a1a',
        padding: 20,
    },
    title: {
        fontSize: 24,
        color: '#00ff00',
        textAlign: 'center',
        fontFamily: 'monospace',
        marginBottom: 20,
    },
    statusCard: {
        backgroundColor: '#2c3e50',
        padding: 15,
        borderRadius: 8,
        marginBottom: 15,
    },
    cardTitle: {
        fontSize: 16,
        color: '#ecf0f1',
        fontWeight: 'bold',
        marginBottom: 10,
    },
    status: {
        color: '#2ecc71',
        fontSize: 14,
    },
    webview: {
        flex: 1,
        marginVertical: 15,
    },
    buttonContainer: {
        flexDirection: 'row',
        justifyContent: 'space-around',
        marginTop: 15,
    },
});

export default BrowserRotApp;
"#.to_string()
    }
    
    pub fn generate_android_server_script() -> String {
        r#"#!/data/data/com.termux/files/usr/bin/bash

# BrowserRot Android/Termux Server Setup Script
echo "🤖 Setting up BrowserRot local server on Android..."

# Install dependencies
echo "📦 Installing dependencies..."
pkg update && pkg upgrade -y
pkg install -y nodejs npm tmux git python rust

# Create server directory
mkdir -p ~/browserrot-server
cd ~/browserrot-server

# Create Node.js server
cat > server.js << 'EOF'
const express = require('express');
const WebSocket = require('ws');
const cors = require('cors');
const fs = require('fs');
const path = require('path');

const app = express();
const PORT = 8080;
const WS_PORT = 4001;

app.use(cors());
app.use(express.json());

// Store for network data
let networkData = {
    nodes: new Map(),
    updates: [],
    brainrotMemes: [],
    syncStats: { total: 0, failed: 0 }
};

// REST API endpoints
app.get('/api/status', (req, res) => {
    res.json({
        status: 'running',
        uptime: process.uptime(),
        nodes: networkData.nodes.size,
        updates: networkData.updates.length
    });
});

app.post('/api/updates', (req, res) => {
    const { nodeId, requestType } = req.body;
    
    console.log(`📡 Update request from ${nodeId}: ${requestType}`);
    
    // Register node
    networkData.nodes.set(nodeId, {
        lastSeen: Date.now(),
        requestCount: (networkData.nodes.get(nodeId)?.requestCount || 0) + 1
    });
    
    // Return latest updates
    res.json({
        updates: networkData.updates.slice(-10),
        brainrotMemes: networkData.brainrotMemes.slice(-5),
        networkStats: {
            totalNodes: networkData.nodes.size,
            totalUpdates: networkData.updates.length
        }
    });
    
    networkData.syncStats.total++;
});

app.get('/dashboard', (req, res) => {
    res.send(`
        <!DOCTYPE html>
        <html>
        <head>
            <title>BrowserRot Local Server</title>
            <style>
                body { font-family: monospace; background: #1a1a1a; color: #00ff00; padding: 20px; }
                .card { background: #2c3e50; padding: 15px; margin: 10px 0; border-radius: 8px; }
            </style>
        </head>
        <body>
            <h1>🧠🧟‍♂️ BrowserRot Local Server</h1>
            <div class="card">
                <h3>Server Status</h3>
                <p>Status: ✅ Running</p>
                <p>Uptime: ${Math.floor(process.uptime())} seconds</p>
                <p>Port: ${PORT}</p>
            </div>
            <div class="card">
                <h3>Network Stats</h3>
                <p>Connected Nodes: ${networkData.nodes.size}</p>
                <p>Total Updates: ${networkData.updates.length}</p>
                <p>Sync Requests: ${networkData.syncStats.total}</p>
            </div>
            <div class="card">
                <h3>Recent Nodes</h3>
                ${Array.from(networkData.nodes.entries()).map(([id, data]) => 
                    `<p>${id.substr(0, 8)}... (${data.requestCount} requests)</p>`
                ).join('')}
            </div>
        </body>
        </html>
    `);
});

// WebSocket server for LibP2P bridge
const wss = new WebSocket.Server({ port: WS_PORT });

wss.on('connection', (ws) => {
    console.log('🔌 WebSocket connection established');
    
    ws.on('message', (message) => {
        try {
            const data = JSON.parse(message);
            
            if (data.type === 'REQUEST_UPDATES') {
                ws.send(JSON.stringify({
                    updates: networkData.updates.slice(-5),
                    timestamp: Date.now()
                }));
            }
        } catch (e) {
            console.error('❌ WebSocket message error:', e);
        }
    });
});

// Start servers
app.listen(PORT, '0.0.0.0', () => {
    console.log(`🚀 BrowserRot server running on port ${PORT}`);
    console.log(`🔌 WebSocket server running on port ${WS_PORT}`);
    console.log(`📱 Dashboard: http://localhost:${PORT}/dashboard`);
});

// Periodic cleanup
setInterval(() => {
    const now = Date.now();
    for (const [nodeId, data] of networkData.nodes.entries()) {
        if (now - data.lastSeen > 300000) { // 5 minutes
            networkData.nodes.delete(nodeId);
            console.log(`🧹 Cleaned up inactive node: ${nodeId}`);
        }
    }
}, 60000);

console.log('🧟‍♂️ BrowserRot local server initialized!');
EOF

# Install Node.js dependencies
npm init -y
npm install express ws cors

# Create tmux session
echo "🖥️  Starting tmux session..."
tmux new-session -d -s browserrot 'node server.js'

echo "✅ BrowserRot server setup complete!"
echo "📱 Server running in tmux session 'browserrot'"
echo "🌐 Access dashboard: http://localhost:8080/dashboard"
echo ""
echo "Commands:"
echo "  tmux attach -t browserrot  # Attach to server session"
echo "  tmux kill-session -t browserrot  # Stop server"
echo "  ~/browserrot-server/server.js  # Server file location"
"#.to_string()
    }
}

impl LocalServerConfig {
    fn new() -> Self {
        Self {
            android_tmux_port: 8080,
            server_status: ServerStatus::NotRunning,
            permissions_granted: Vec::new(),
        }
    }
}

impl ReactNativeBridge {
    fn new() -> Self {
        Self {
            app_installed: false,
            permissions: Vec::new(),
            bridge_active: false,
        }
    }
}

impl SyncStatus {
    fn new() -> Self {
        Self {
            total_updates: 0,
            failed_syncs: 0,
            last_successful_sync: 0,
            network_health: 100.0,
        }
    }
}
