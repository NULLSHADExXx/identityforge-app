export interface ProxyConfig {
  enabled?: boolean;
  proxy_type?: string;  // "http", "https", "socks5"
  host?: string;
  port?: number;
  username?: string;
  password?: string;
}

export interface Profile {
  id: string;
  name: string;
  user_agent: string;
  screen_width: number;
  screen_height: number;
  webgl_vendor: string;
  webgl_renderer: string;
  hardware_concurrency: number;
  device_memory: number;
  platform: string;
  timezone: string;
  language: string;
  default_url: string;
  // Proxy settings
  proxy_enabled: boolean;
  proxy_type: string;
  proxy_host: string;
  proxy_port: number;
  proxy_username: string | null;
  proxy_password: string | null;
  created_at: string;
  last_used: string | null;
}

export interface ProfileWithStatus extends Profile {
  is_active: boolean;
}

export interface Fingerprint {
  user_agent: string;
  platform: string;
  screen_width: number;
  screen_height: number;
  webgl_vendor: string;
  webgl_renderer: string;
  hardware_concurrency: number;
  device_memory: number;
  timezone: string;
  language: string;
  default_url: string;
  proxy_enabled: boolean;
  proxy_type: string;
  proxy_host: string;
  proxy_port: number;
  proxy_username: string | null;
  proxy_password: string | null;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T | null;
  error: string | null;
}

export interface CreateProfileInput {
  name: string;
  platform?: string;
  default_url?: string;
  proxy?: ProxyConfig;
}

export interface UpdateProfileInput {
  id: string;
  name?: string;
  user_agent?: string;
  screen_width?: number;
  screen_height?: number;
  webgl_vendor?: string;
  webgl_renderer?: string;
  hardware_concurrency?: number;
  device_memory?: number;
  platform?: string;
  timezone?: string;
  language?: string;
  default_url?: string;
  proxy?: ProxyConfig;
}

export interface LaunchProfileInput {
  profile_id: string;
  start_url?: string;
}

export interface Cookie {
  name: string;
  value: string;
  domain: string;
  path: string;
  expires?: number;
  http_only: boolean;
  secure: boolean;
  same_site?: string;
}
