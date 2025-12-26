use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// User agent templates for different platforms
const USER_AGENTS: &[(&str, &str)] = &[
    // Windows Chrome
    ("Win32", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
    ("Win32", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"),
    ("Win32", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36"),
    ("Win32", "Mozilla/5.0 (Windows NT 11.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
    // Windows Firefox
    ("Win32", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0"),
    ("Win32", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0"),
    // macOS Chrome
    ("MacIntel", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
    ("MacIntel", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"),
    // macOS Safari
    ("MacIntel", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.2 Safari/605.1.15"),
    ("MacIntel", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.1 Safari/605.1.15"),
    // macOS Firefox
    ("MacIntel", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:121.0) Gecko/20100101 Firefox/121.0"),
    // Linux Chrome
    ("Linux x86_64", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
    ("Linux x86_64", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36"),
    // Linux Firefox
    ("Linux x86_64", "Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0"),
    ("Linux x86_64", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0"),
];

/// Common screen resolutions
const SCREEN_RESOLUTIONS: &[(i32, i32)] = &[
    (1920, 1080),
    (2560, 1440),
    (1366, 768),
    (1536, 864),
    (1440, 900),
    (1280, 720),
    (1600, 900),
    (1680, 1050),
    (2560, 1080),
    (3440, 1440),
    (3840, 2160),
    (1280, 1024),
];

/// WebGL vendor/renderer combinations
const WEBGL_CONFIGS: &[(&str, &str)] = &[
    ("Intel Inc.", "Intel Iris OpenGL Engine"),
    ("Intel Inc.", "Intel(R) UHD Graphics 630"),
    ("Intel Inc.", "Intel(R) UHD Graphics 620"),
    ("Intel Inc.", "Intel(R) Iris(R) Xe Graphics"),
    ("Intel Inc.", "Intel(R) HD Graphics 530"),
    ("NVIDIA Corporation", "NVIDIA GeForce GTX 1080/PCIe/SSE2"),
    ("NVIDIA Corporation", "NVIDIA GeForce RTX 3060/PCIe/SSE2"),
    ("NVIDIA Corporation", "NVIDIA GeForce RTX 3070/PCIe/SSE2"),
    ("NVIDIA Corporation", "NVIDIA GeForce GTX 1660 Ti/PCIe/SSE2"),
    ("NVIDIA Corporation", "NVIDIA GeForce RTX 4070/PCIe/SSE2"),
    ("AMD", "AMD Radeon RX 580 Series"),
    ("AMD", "AMD Radeon RX 6800 XT"),
    ("AMD", "AMD Radeon Pro 5500M"),
    ("Apple Inc.", "Apple M1"),
    ("Apple Inc.", "Apple M2"),
    ("Apple Inc.", "Apple M1 Pro"),
    ("Google Inc. (NVIDIA)", "ANGLE (NVIDIA, NVIDIA GeForce GTX 1080 Direct3D11 vs_5_0 ps_5_0, D3D11)"),
    ("Google Inc. (Intel)", "ANGLE (Intel, Intel(R) UHD Graphics 630 Direct3D11 vs_5_0 ps_5_0, D3D11)"),
];

/// Timezones with their UTC offsets
const TIMEZONES: &[(&str, i32)] = &[
    ("America/New_York", 300),
    ("America/Los_Angeles", 480),
    ("America/Chicago", 360),
    ("America/Denver", 420),
    ("Europe/London", 0),
    ("Europe/Paris", -60),
    ("Europe/Berlin", -60),
    ("Asia/Tokyo", -540),
    ("Asia/Shanghai", -480),
    ("Asia/Singapore", -480),
    ("Australia/Sydney", -660),
    ("America/Toronto", 300),
    ("America/Sao_Paulo", 180),
];

/// Languages
const LANGUAGES: &[&str] = &[
    "en-US",
    "en-GB",
    "en-CA",
    "en-AU",
    "de-DE",
    "fr-FR",
    "es-ES",
    "it-IT",
    "pt-BR",
    "ja-JP",
    "zh-CN",
    "ko-KR",
];

/// Hardware concurrency options (CPU cores)
const HARDWARE_CONCURRENCY: &[i32] = &[2, 4, 6, 8, 10, 12, 16];

/// Device memory options (GB)
const DEVICE_MEMORY: &[i32] = &[2, 4, 8, 16, 32];

/// Common fonts by platform
const WINDOWS_FONTS: &[&str] = &[
    "Arial", "Arial Black", "Calibri", "Cambria", "Cambria Math", "Comic Sans MS",
    "Consolas", "Courier New", "Georgia", "Impact", "Lucida Console", "Lucida Sans Unicode",
    "Microsoft Sans Serif", "Palatino Linotype", "Segoe UI", "Tahoma", "Times New Roman",
    "Trebuchet MS", "Verdana", "Wingdings"
];

const MAC_FONTS: &[&str] = &[
    "American Typewriter", "Andale Mono", "Arial", "Arial Black", "Arial Narrow",
    "Avenir", "Avenir Next", "Baskerville", "Big Caslon", "Brush Script MT",
    "Chalkboard", "Cochin", "Comic Sans MS", "Copperplate", "Courier New",
    "Didot", "Futura", "Geneva", "Georgia", "Gill Sans", "Helvetica", "Helvetica Neue",
    "Herculanum", "Hoefler Text", "Impact", "Lucida Grande", "Marker Felt",
    "Menlo", "Monaco", "Optima", "Palatino", "Papyrus", "Phosphate", "Rockwell",
    "SF Pro", "Skia", "Times New Roman", "Trebuchet MS", "Verdana", "Zapfino"
];

const LINUX_FONTS: &[&str] = &[
    "Arial", "Cantarell", "Comic Sans MS", "Courier New", "DejaVu Sans",
    "DejaVu Sans Mono", "DejaVu Serif", "Droid Sans", "Droid Sans Mono",
    "FreeMono", "FreeSans", "FreeSerif", "Georgia", "Impact", "Liberation Mono",
    "Liberation Sans", "Liberation Serif", "Nimbus Mono L", "Nimbus Roman No9 L",
    "Nimbus Sans L", "Noto Sans", "Noto Serif", "Open Sans", "Roboto",
    "Times New Roman", "Ubuntu", "Ubuntu Mono", "Verdana"
];

/// Generated fingerprint data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fingerprint {
    pub user_agent: String,
    pub platform: String,
    pub screen_width: i32,
    pub screen_height: i32,
    pub webgl_vendor: String,
    pub webgl_renderer: String,
    pub hardware_concurrency: i32,
    pub device_memory: i32,
    pub timezone: String,
    pub language: String,
    pub default_url: String,
    // Proxy settings
    pub proxy_enabled: bool,
    pub proxy_type: String,
    pub proxy_host: String,
    pub proxy_port: i32,
    pub proxy_username: Option<String>,
    pub proxy_password: Option<String>,
}

/// Fingerprint generator with configurable options
pub struct FingerprintGenerator {
    rng: ThreadRng,
}

impl FingerprintGenerator {
    pub fn new() -> Self {
        FingerprintGenerator {
            rng: thread_rng(),
        }
    }

    /// Generate a completely random fingerprint
    pub fn generate(&mut self) -> Fingerprint {
        let (platform, user_agent) = USER_AGENTS[self.rng.gen_range(0..USER_AGENTS.len())];
        let (width, height) = SCREEN_RESOLUTIONS[self.rng.gen_range(0..SCREEN_RESOLUTIONS.len())];
        let (vendor, renderer) = WEBGL_CONFIGS[self.rng.gen_range(0..WEBGL_CONFIGS.len())];
        let hardware_concurrency = HARDWARE_CONCURRENCY[self.rng.gen_range(0..HARDWARE_CONCURRENCY.len())];
        let device_memory = DEVICE_MEMORY[self.rng.gen_range(0..DEVICE_MEMORY.len())];
        let (timezone, _) = TIMEZONES[self.rng.gen_range(0..TIMEZONES.len())];
        let language = LANGUAGES[self.rng.gen_range(0..LANGUAGES.len())];

        Fingerprint {
            user_agent: user_agent.to_string(),
            platform: platform.to_string(),
            screen_width: width,
            screen_height: height,
            webgl_vendor: vendor.to_string(),
            webgl_renderer: renderer.to_string(),
            hardware_concurrency,
            device_memory,
            timezone: timezone.to_string(),
            language: language.to_string(),
            default_url: "https://www.google.com".to_string(),
            proxy_enabled: false,
            proxy_type: "http".to_string(),
            proxy_host: String::new(),
            proxy_port: 0,
            proxy_username: None,
            proxy_password: None,
        }
    }

    /// Generate a fingerprint for a specific platform
    pub fn generate_for_platform(&mut self, target_platform: &str) -> Fingerprint {
        let platform_agents: Vec<_> = USER_AGENTS
            .iter()
            .filter(|(p, _)| p.to_lowercase().contains(&target_platform.to_lowercase()))
            .collect();

        let (platform, user_agent) = if platform_agents.is_empty() {
            USER_AGENTS[self.rng.gen_range(0..USER_AGENTS.len())]
        } else {
            *platform_agents[self.rng.gen_range(0..platform_agents.len())]
        };

        let (width, height) = SCREEN_RESOLUTIONS[self.rng.gen_range(0..SCREEN_RESOLUTIONS.len())];
        let (vendor, renderer) = WEBGL_CONFIGS[self.rng.gen_range(0..WEBGL_CONFIGS.len())];
        let hardware_concurrency = HARDWARE_CONCURRENCY[self.rng.gen_range(0..HARDWARE_CONCURRENCY.len())];
        let device_memory = DEVICE_MEMORY[self.rng.gen_range(0..DEVICE_MEMORY.len())];
        let (timezone, _) = TIMEZONES[self.rng.gen_range(0..TIMEZONES.len())];
        let language = LANGUAGES[self.rng.gen_range(0..LANGUAGES.len())];

        Fingerprint {
            user_agent: user_agent.to_string(),
            platform: platform.to_string(),
            screen_width: width,
            screen_height: height,
            webgl_vendor: vendor.to_string(),
            webgl_renderer: renderer.to_string(),
            hardware_concurrency,
            device_memory,
            timezone: timezone.to_string(),
            language: language.to_string(),
            default_url: "https://www.google.com".to_string(),
            proxy_enabled: false,
            proxy_type: "http".to_string(),
            proxy_host: String::new(),
            proxy_port: 0,
            proxy_username: None,
            proxy_password: None,
        }
    }
}

impl Default for FingerprintGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a persistent noise seed from profile ID
fn generate_persistent_seed(profile_id: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    profile_id.hash(&mut hasher);
    hasher.finish()
}

/// Get fonts list for platform
fn get_fonts_for_platform(platform: &str) -> Vec<&'static str> {
    if platform.contains("Win") {
        WINDOWS_FONTS.to_vec()
    } else if platform.contains("Mac") {
        MAC_FONTS.to_vec()
    } else {
        LINUX_FONTS.to_vec()
    }
}

/// Get timezone offset
fn get_timezone_offset(timezone: &str) -> i32 {
    TIMEZONES.iter()
        .find(|(tz, _)| *tz == timezone)
        .map(|(_, offset)| *offset)
        .unwrap_or(0)
}

/// Generate the JavaScript injection script for fingerprint spoofing
/// Now takes profile_id for persistent noise
pub fn generate_spoof_script(fingerprint: &Fingerprint, profile_id: &str) -> String {
    let persistent_seed = generate_persistent_seed(profile_id);
    let canvas_seed = (persistent_seed % 1000) as i32;
    let audio_seed = ((persistent_seed >> 10) % 1000) as i32;
    let font_seed = ((persistent_seed >> 20) % 1000) as i32;
    
    let fonts = get_fonts_for_platform(&fingerprint.platform);
    let fonts_json: Vec<String> = fonts.iter().map(|f| format!("\"{}\"", f)).collect();
    let fonts_array = fonts_json.join(", ");
    
    let tz_offset = get_timezone_offset(&fingerprint.timezone);

    format!(r#"
(function() {{
    'use strict';
    
    // ============================================
    // PERSISTENT SEEDS (unique per profile, stable across refreshes)
    // ============================================
    const CANVAS_SEED = {canvas_seed};
    const AUDIO_SEED = {audio_seed};
    const FONT_SEED = {font_seed};
    const PROFILE_ID = '{profile_id}';
    
    // ============================================
    // NAVIGATOR SPOOFING
    // ============================================
    
    Object.defineProperty(navigator, 'userAgent', {{
        get: function() {{ return '{user_agent}'; }},
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'platform', {{
        get: function() {{ return '{platform}'; }},
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'hardwareConcurrency', {{
        get: function() {{ return {hardware_concurrency}; }},
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'deviceMemory', {{
        get: function() {{ return {device_memory}; }},
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'language', {{
        get: function() {{ return '{language}'; }},
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'languages', {{
        get: function() {{ return ['{language}', '{language}'.split('-')[0]]; }},
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'appVersion', {{
        get: function() {{ return '{user_agent}'.substring(8); }},
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'vendor', {{
        get: function() {{ return 'Google Inc.'; }},
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'maxTouchPoints', {{
        get: function() {{ return 0; }},
        configurable: true
    }});
    
    // ============================================
    // SCREEN SPOOFING (with media query protection)
    // ============================================
    
    const SPOOF_WIDTH = {screen_width};
    const SPOOF_HEIGHT = {screen_height};
    
    Object.defineProperty(screen, 'width', {{
        get: function() {{ return SPOOF_WIDTH; }},
        configurable: true
    }});
    
    Object.defineProperty(screen, 'height', {{
        get: function() {{ return SPOOF_HEIGHT; }},
        configurable: true
    }});
    
    Object.defineProperty(screen, 'availWidth', {{
        get: function() {{ return SPOOF_WIDTH; }},
        configurable: true
    }});
    
    Object.defineProperty(screen, 'availHeight', {{
        get: function() {{ return SPOOF_HEIGHT - 40; }},
        configurable: true
    }});
    
    Object.defineProperty(screen, 'colorDepth', {{
        get: function() {{ return 24; }},
        configurable: true
    }});
    
    Object.defineProperty(screen, 'pixelDepth', {{
        get: function() {{ return 24; }},
        configurable: true
    }});
    
    // Window dimensions to match screen
    Object.defineProperty(window, 'innerWidth', {{
        get: function() {{ return SPOOF_WIDTH; }},
        configurable: true
    }});
    
    Object.defineProperty(window, 'innerHeight', {{
        get: function() {{ return SPOOF_HEIGHT - 140; }},
        configurable: true
    }});
    
    Object.defineProperty(window, 'outerWidth', {{
        get: function() {{ return SPOOF_WIDTH; }},
        configurable: true
    }});
    
    Object.defineProperty(window, 'outerHeight', {{
        get: function() {{ return SPOOF_HEIGHT; }},
        configurable: true
    }});
    
    // Device pixel ratio
    Object.defineProperty(window, 'devicePixelRatio', {{
        get: function() {{ return 1; }},
        configurable: true
    }});
    
    // ============================================
    // MEDIA QUERY SPOOFING
    // ============================================
    
    const originalMatchMedia = window.matchMedia;
    window.matchMedia = function(query) {{
        // Parse and modify dimension-related queries
        let modifiedQuery = query;
        
        // Replace actual dimensions with spoofed ones
        if (query.includes('width') || query.includes('height')) {{
            // Handle min-width, max-width, etc.
            modifiedQuery = query
                .replace(/\(min-width:\s*(\d+)px\)/g, function(match, px) {{
                    return '(min-width: ' + px + 'px)';
                }})
                .replace(/\(max-width:\s*(\d+)px\)/g, function(match, px) {{
                    return '(max-width: ' + px + 'px)';
                }});
        }}
        
        const result = originalMatchMedia.call(window, modifiedQuery);
        
        // Override matches for screen size queries
        if (query.includes('device-width') || query.includes('device-height')) {{
            const originalMatches = result.matches;
            Object.defineProperty(result, 'matches', {{
                get: function() {{
                    // Evaluate against spoofed dimensions
                    if (query.includes('min-device-width')) {{
                        const minWidth = parseInt(query.match(/min-device-width:\s*(\d+)/)?.[1] || '0');
                        return SPOOF_WIDTH >= minWidth;
                    }}
                    if (query.includes('max-device-width')) {{
                        const maxWidth = parseInt(query.match(/max-device-width:\s*(\d+)/)?.[1] || '99999');
                        return SPOOF_WIDTH <= maxWidth;
                    }}
                    return originalMatches;
                }}
            }});
        }}
        
        return result;
    }};
    
    // ============================================
    // WEBGL SPOOFING
    // ============================================
    
    const getParameterProxyHandler = {{
        apply: function(target, thisArg, args) {{
            const param = args[0];
            
            // UNMASKED_VENDOR_WEBGL
            if (param === 37445) {{
                return '{webgl_vendor}';
            }}
            // UNMASKED_RENDERER_WEBGL
            if (param === 37446) {{
                return '{webgl_renderer}';
            }}
            // MAX_TEXTURE_SIZE - randomize slightly
            if (param === 3379) {{
                return 16384;
            }}
            // MAX_VERTEX_ATTRIBS
            if (param === 34921) {{
                return 16;
            }}
            
            return Reflect.apply(target, thisArg, args);
        }}
    }};
    
    const originalGetParameter = WebGLRenderingContext.prototype.getParameter;
    WebGLRenderingContext.prototype.getParameter = new Proxy(originalGetParameter, getParameterProxyHandler);
    
    if (typeof WebGL2RenderingContext !== 'undefined') {{
        const originalGetParameter2 = WebGL2RenderingContext.prototype.getParameter;
        WebGL2RenderingContext.prototype.getParameter = new Proxy(originalGetParameter2, getParameterProxyHandler);
    }}
    
    // ============================================
    // CANVAS FINGERPRINT PROTECTION (PERSISTENT NOISE)
    // ============================================
    
    // Seeded random number generator for consistent noise
    function seededRandom(seed) {{
        const x = Math.sin(seed) * 10000;
        return x - Math.floor(x);
    }}
    
    const originalToDataURL = HTMLCanvasElement.prototype.toDataURL;
    HTMLCanvasElement.prototype.toDataURL = function(type, quality) {{
        const ctx = this.getContext('2d');
        if (ctx && this.width > 0 && this.height > 0) {{
            try {{
                const imageData = ctx.getImageData(0, 0, this.width, this.height);
                const data = imageData.data;
                
                // Use persistent seed for consistent noise
                for (let i = 0; i < data.length; i += 4) {{
                    const pixelIndex = i / 4;
                    const noiseSeed = CANVAS_SEED + pixelIndex;
                    
                    // Only modify specific pixels based on seed
                    if (pixelIndex % 97 === CANVAS_SEED % 97) {{
                        const noise = Math.floor(seededRandom(noiseSeed) * 3) - 1;
                        data[i] = Math.max(0, Math.min(255, data[i] + noise));
                    }}
                }}
                
                ctx.putImageData(imageData, 0, 0);
            }} catch (e) {{}}
        }}
        return originalToDataURL.apply(this, arguments);
    }};
    
    const originalToBlob = HTMLCanvasElement.prototype.toBlob;
    HTMLCanvasElement.prototype.toBlob = function(callback, type, quality) {{
        const ctx = this.getContext('2d');
        if (ctx && this.width > 0 && this.height > 0) {{
            try {{
                const imageData = ctx.getImageData(0, 0, this.width, this.height);
                const data = imageData.data;
                
                for (let i = 0; i < data.length; i += 4) {{
                    const pixelIndex = i / 4;
                    const noiseSeed = CANVAS_SEED + pixelIndex;
                    
                    if (pixelIndex % 97 === CANVAS_SEED % 97) {{
                        const noise = Math.floor(seededRandom(noiseSeed) * 3) - 1;
                        data[i] = Math.max(0, Math.min(255, data[i] + noise));
                    }}
                }}
                
                ctx.putImageData(imageData, 0, 0);
            }} catch (e) {{}}
        }}
        return originalToBlob.apply(this, arguments);
    }};
    
    // Protect getImageData as well
    const originalGetImageData = CanvasRenderingContext2D.prototype.getImageData;
    CanvasRenderingContext2D.prototype.getImageData = function(sx, sy, sw, sh) {{
        const imageData = originalGetImageData.call(this, sx, sy, sw, sh);
        const data = imageData.data;
        
        for (let i = 0; i < data.length; i += 4) {{
            const pixelIndex = i / 4;
            const noiseSeed = CANVAS_SEED + pixelIndex;
            
            if (pixelIndex % 97 === CANVAS_SEED % 97) {{
                const noise = Math.floor(seededRandom(noiseSeed) * 3) - 1;
                data[i] = Math.max(0, Math.min(255, data[i] + noise));
            }}
        }}
        
        return imageData;
    }};
    
    // ============================================
    // WEBRTC LEAK PROTECTION (COMPLETE DISABLE)
    // ============================================
    
    // Completely disable WebRTC
    if (typeof RTCPeerConnection !== 'undefined') {{
        window.RTCPeerConnection = function() {{
            throw new Error('RTCPeerConnection is disabled');
        }};
    }}
    
    if (typeof webkitRTCPeerConnection !== 'undefined') {{
        window.webkitRTCPeerConnection = function() {{
            throw new Error('webkitRTCPeerConnection is disabled');
        }};
    }}
    
    if (typeof RTCDataChannel !== 'undefined') {{
        window.RTCDataChannel = function() {{
            throw new Error('RTCDataChannel is disabled');
        }};
    }}
    
    // Remove mediaDevices.getUserMedia to prevent WebRTC enumeration
    if (navigator.mediaDevices) {{
        navigator.mediaDevices.getUserMedia = function() {{
            return Promise.reject(new Error('getUserMedia is disabled'));
        }};
        navigator.mediaDevices.enumerateDevices = function() {{
            return Promise.resolve([]);
        }};
    }}
    
    // ============================================
    // TIMEZONE SPOOFING
    // ============================================
    
    const targetTimezone = '{timezone}';
    const targetOffset = {tz_offset};
    
    const originalGetTimezoneOffset = Date.prototype.getTimezoneOffset;
    Date.prototype.getTimezoneOffset = function() {{
        return targetOffset;
    }};
    
    const originalDateTimeFormat = Intl.DateTimeFormat;
    Intl.DateTimeFormat = function(locales, options) {{
        options = options || {{}};
        if (!options.timeZone) {{
            options.timeZone = targetTimezone;
        }}
        return new originalDateTimeFormat(locales, options);
    }};
    Intl.DateTimeFormat.prototype = originalDateTimeFormat.prototype;
    Intl.DateTimeFormat.supportedLocalesOf = originalDateTimeFormat.supportedLocalesOf;
    
    // Also spoof resolvedOptions
    const originalResolvedOptions = Intl.DateTimeFormat.prototype.resolvedOptions;
    Intl.DateTimeFormat.prototype.resolvedOptions = function() {{
        const options = originalResolvedOptions.call(this);
        options.timeZone = targetTimezone;
        return options;
    }};
    
    // ============================================
    // AUDIO FINGERPRINT PROTECTION (PERSISTENT NOISE)
    // ============================================
    
    if (typeof AudioContext !== 'undefined' || typeof webkitAudioContext !== 'undefined') {{
        const AudioContextClass = window.AudioContext || window.webkitAudioContext;
        
        const originalCreateAnalyser = AudioContextClass.prototype.createAnalyser;
        AudioContextClass.prototype.createAnalyser = function() {{
            const analyser = originalCreateAnalyser.apply(this, arguments);
            
            const originalGetFloatFrequencyData = analyser.getFloatFrequencyData.bind(analyser);
            analyser.getFloatFrequencyData = function(array) {{
                originalGetFloatFrequencyData(array);
                for (let i = 0; i < array.length; i++) {{
                    if (i % 10 === AUDIO_SEED % 10) {{
                        array[i] = array[i] + seededRandom(AUDIO_SEED + i) * 0.0001;
                    }}
                }}
            }};
            
            const originalGetByteFrequencyData = analyser.getByteFrequencyData.bind(analyser);
            analyser.getByteFrequencyData = function(array) {{
                originalGetByteFrequencyData(array);
                for (let i = 0; i < array.length; i++) {{
                    if (i % 10 === AUDIO_SEED % 10) {{
                        array[i] = Math.max(0, Math.min(255, array[i] + Math.floor(seededRandom(AUDIO_SEED + i) * 2)));
                    }}
                }}
            }};
            
            return analyser;
        }};
        
        // Also protect createOscillator for audio fingerprinting
        const originalCreateOscillator = AudioContextClass.prototype.createOscillator;
        AudioContextClass.prototype.createOscillator = function() {{
            const oscillator = originalCreateOscillator.apply(this, arguments);
            const originalStart = oscillator.start.bind(oscillator);
            
            oscillator.start = function(when) {{
                // Add tiny frequency offset based on seed
                if (oscillator.frequency) {{
                    const currentFreq = oscillator.frequency.value;
                    oscillator.frequency.value = currentFreq + (seededRandom(AUDIO_SEED) * 0.001);
                }}
                return originalStart(when);
            }};
            
            return oscillator;
        }};
    }}
    
    // ============================================
    // FONT FINGERPRINT PROTECTION
    // ============================================
    
    const ALLOWED_FONTS = [{fonts_array}];
    
    // Override font checking via canvas
    const originalFillText = CanvasRenderingContext2D.prototype.fillText;
    const originalMeasureText = CanvasRenderingContext2D.prototype.measureText;
    
    CanvasRenderingContext2D.prototype.measureText = function(text) {{
        const result = originalMeasureText.call(this, text);
        
        // Add slight noise to measurements based on seed
        const noise = seededRandom(FONT_SEED + text.length) * 0.1;
        
        return {{
            width: result.width + noise,
            actualBoundingBoxLeft: result.actualBoundingBoxLeft,
            actualBoundingBoxRight: result.actualBoundingBoxRight,
            actualBoundingBoxAscent: result.actualBoundingBoxAscent,
            actualBoundingBoxDescent: result.actualBoundingBoxDescent,
            fontBoundingBoxAscent: result.fontBoundingBoxAscent,
            fontBoundingBoxDescent: result.fontBoundingBoxDescent
        }};
    }};
    
    // Override document.fonts API
    if (document.fonts && document.fonts.check) {{
        const originalCheck = document.fonts.check.bind(document.fonts);
        document.fonts.check = function(font, text) {{
            // Extract font family from the font string
            const fontFamily = font.split(' ').slice(-1)[0].replace(/['"]/g, '');
            
            // Only return true for allowed fonts
            if (ALLOWED_FONTS.some(f => fontFamily.toLowerCase().includes(f.toLowerCase()))) {{
                return originalCheck(font, text);
            }}
            return false;
        }};
    }}
    
    // ============================================
    // PLUGIN/MIME TYPE SPOOFING
    // ============================================
    
    Object.defineProperty(navigator, 'plugins', {{
        get: function() {{
            const plugins = {{
                length: 5,
                0: {{ name: 'Chrome PDF Plugin', filename: 'internal-pdf-viewer', description: 'Portable Document Format' }},
                1: {{ name: 'Chrome PDF Viewer', filename: 'mhjfbmdgcfjbbpaeojofohoefgiehjai', description: '' }},
                2: {{ name: 'Native Client', filename: 'internal-nacl-plugin', description: '' }},
                3: {{ name: 'Chromium PDF Plugin', filename: 'internal-pdf-viewer', description: 'Portable Document Format' }},
                4: {{ name: 'Chromium PDF Viewer', filename: 'mhjfbmdgcfjbbpaeojofohoefgiehjai', description: '' }},
                item: function(i) {{ return this[i]; }},
                namedItem: function(name) {{
                    for (let i = 0; i < this.length; i++) {{
                        if (this[i].name === name) return this[i];
                    }}
                    return null;
                }},
                refresh: function() {{}}
            }};
            return plugins;
        }},
        configurable: true
    }});
    
    Object.defineProperty(navigator, 'mimeTypes', {{
        get: function() {{
            return {{
                length: 2,
                0: {{ type: 'application/pdf', description: 'Portable Document Format', suffixes: 'pdf' }},
                1: {{ type: 'text/pdf', description: 'Portable Document Format', suffixes: 'pdf' }},
                item: function(i) {{ return this[i]; }},
                namedItem: function(name) {{
                    for (let i = 0; i < this.length; i++) {{
                        if (this[i].type === name) return this[i];
                    }}
                    return null;
                }}
            }};
        }},
        configurable: true
    }});
    
    // ============================================
    // BATTERY API SPOOFING
    // ============================================
    
    if (navigator.getBattery) {{
        navigator.getBattery = function() {{
            return Promise.resolve({{
                charging: true,
                chargingTime: 0,
                dischargingTime: Infinity,
                level: 1.0,
                addEventListener: function() {{}},
                removeEventListener: function() {{}}
            }});
        }};
    }}
    
    // ============================================
    // PERFORMANCE API PROTECTION
    // ============================================
    
    // Add noise to performance.now() to prevent timing attacks
    const originalPerformanceNow = performance.now.bind(performance);
    performance.now = function() {{
        return originalPerformanceNow() + seededRandom(CANVAS_SEED) * 0.1;
    }};
    
    // ============================================
    // CLIENTRECTS PROTECTION
    // ============================================
    
    const originalGetClientRects = Element.prototype.getClientRects;
    Element.prototype.getClientRects = function() {{
        const rects = originalGetClientRects.call(this);
        const noise = seededRandom(FONT_SEED) * 0.001;
        
        // Return modified DOMRectList
        const newRects = [];
        for (let i = 0; i < rects.length; i++) {{
            newRects.push({{
                x: rects[i].x + noise,
                y: rects[i].y + noise,
                width: rects[i].width + noise,
                height: rects[i].height + noise,
                top: rects[i].top + noise,
                right: rects[i].right + noise,
                bottom: rects[i].bottom + noise,
                left: rects[i].left + noise
            }});
        }}
        
        newRects.item = function(i) {{ return this[i]; }};
        newRects.length = rects.length;
        return newRects;
    }};
    
    const originalGetBoundingClientRect = Element.prototype.getBoundingClientRect;
    Element.prototype.getBoundingClientRect = function() {{
        const rect = originalGetBoundingClientRect.call(this);
        const noise = seededRandom(FONT_SEED) * 0.001;
        
        return {{
            x: rect.x + noise,
            y: rect.y + noise,
            width: rect.width + noise,
            height: rect.height + noise,
            top: rect.top + noise,
            right: rect.right + noise,
            bottom: rect.bottom + noise,
            left: rect.left + noise,
            toJSON: rect.toJSON
        }};
    }};
    
    console.log('[IdentityForge] Advanced fingerprint protection active - Profile: ' + PROFILE_ID);
}})();
"#,
        user_agent = fingerprint.user_agent.replace('\'', "\\'"),
        platform = fingerprint.platform.replace('\'', "\\'"),
        hardware_concurrency = fingerprint.hardware_concurrency,
        device_memory = fingerprint.device_memory,
        language = fingerprint.language.replace('\'', "\\'"),
        screen_width = fingerprint.screen_width,
        screen_height = fingerprint.screen_height,
        webgl_vendor = fingerprint.webgl_vendor.replace('\'', "\\'"),
        webgl_renderer = fingerprint.webgl_renderer.replace('\'', "\\'"),
        timezone = fingerprint.timezone.replace('\'', "\\'"),
        tz_offset = tz_offset,
        canvas_seed = canvas_seed,
        audio_seed = audio_seed,
        font_seed = font_seed,
        fonts_array = fonts_array,
        profile_id = profile_id.replace('\'', "\\'"),
    )
}

/// Legacy function for backward compatibility
pub fn generate_spoof_script_legacy(fingerprint: &Fingerprint) -> String {
    generate_spoof_script(fingerprint, "default")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fingerprint_generation() {
        let mut generator = FingerprintGenerator::new();
        let fp = generator.generate();
        
        assert!(!fp.user_agent.is_empty());
        assert!(!fp.platform.is_empty());
        assert!(fp.screen_width > 0);
        assert!(fp.screen_height > 0);
        assert!(fp.hardware_concurrency > 0);
        assert!(fp.device_memory > 0);
    }

    #[test]
    fn test_persistent_seed() {
        let seed1 = generate_persistent_seed("profile-123");
        let seed2 = generate_persistent_seed("profile-123");
        let seed3 = generate_persistent_seed("profile-456");
        
        assert_eq!(seed1, seed2); // Same profile = same seed
        assert_ne!(seed1, seed3); // Different profile = different seed
    }

    #[test]
    fn test_spoof_script_generation() {
        let mut generator = FingerprintGenerator::new();
        let fp = generator.generate();
        let script = generate_spoof_script(&fp, "test-profile");
        
        assert!(script.contains("navigator"));
        assert!(script.contains("screen"));
        assert!(script.contains(&fp.user_agent));
        assert!(script.contains("CANVAS_SEED"));
        assert!(script.contains("AUDIO_SEED"));
    }
}
