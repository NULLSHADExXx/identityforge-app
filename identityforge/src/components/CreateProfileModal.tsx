import { useState } from 'react';
import { createProfile, previewFingerprint } from '../hooks/useApi';
import { Fingerprint, ProxyConfig } from '../types/profile';

interface CreateProfileModalProps {
  isOpen: boolean;
  onClose: () => void;
  onCreated: () => void;
}

export function CreateProfileModal({ isOpen, onClose, onCreated }: CreateProfileModalProps) {
  const [name, setName] = useState('');
  const [platform, setPlatform] = useState<string>('');
  const [defaultUrl, setDefaultUrl] = useState('https://www.google.com');
  const [loading, setLoading] = useState(false);
  const [preview, setPreview] = useState<Fingerprint | null>(null);
  
  // Proxy settings
  const [proxyEnabled, setProxyEnabled] = useState(false);
  const [proxyType, setProxyType] = useState('http');
  const [proxyHost, setProxyHost] = useState('');
  const [proxyPort, setProxyPort] = useState('');
  const [proxyUsername, setProxyUsername] = useState('');
  const [proxyPassword, setProxyPassword] = useState('');

  const handlePreview = async () => {
    setLoading(true);
    try {
      const result = await previewFingerprint(platform || undefined);
      if (result.success && result.data) {
        setPreview(result.data);
      }
    } catch (err) {
      console.error('Preview error:', err);
    }
    setLoading(false);
  };

  const handleCreate = async () => {
    if (!name.trim()) {
      alert('Please enter a profile name');
      return;
    }

    // Validate proxy if enabled
    if (proxyEnabled && (!proxyHost.trim() || !proxyPort.trim())) {
      alert('Please enter proxy host and port');
      return;
    }

    setLoading(true);
    try {
      const proxy: ProxyConfig | undefined = proxyEnabled ? {
        enabled: true,
        proxy_type: proxyType,
        host: proxyHost.trim(),
        port: parseInt(proxyPort) || 0,
        username: proxyUsername.trim() || undefined,
        password: proxyPassword.trim() || undefined,
      } : undefined;

      const result = await createProfile({
        name: name.trim(),
        platform: platform || undefined,
        default_url: defaultUrl.trim() || undefined,
        proxy,
      });
      if (result.success) {
        onCreated();
        handleClose();
      } else {
        alert('Failed to create profile: ' + result.error);
      }
    } catch (err) {
      console.error('Create error:', err);
      alert('Failed to create profile');
    }
    setLoading(false);
  };

  const handleClose = () => {
    setName('');
    setPlatform('');
    setDefaultUrl('https://www.google.com');
    setPreview(null);
    setProxyEnabled(false);
    setProxyType('http');
    setProxyHost('');
    setProxyPort('');
    setProxyUsername('');
    setProxyPassword('');
    onClose();
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-full max-w-lg max-h-[90vh] overflow-y-auto">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold text-white">New Identity</h2>
          <button
            onClick={handleClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="space-y-4">
          {/* Profile Name */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              Profile Name
            </label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="e.g., FB Ad Account 1, Gmail Farm 3"
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white placeholder-gray-500 focus:outline-none focus:border-forge-accent"
            />
          </div>

          {/* Default URL */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              Default URL
            </label>
            <input
              type="url"
              value={defaultUrl}
              onChange={(e) => setDefaultUrl(e.target.value)}
              placeholder="https://www.facebook.com"
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white placeholder-gray-500 focus:outline-none focus:border-forge-accent"
            />
            <p className="text-xs text-gray-500 mt-1">Opens this URL when launching the profile</p>
          </div>

          {/* Platform Selection */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              Platform (Optional)
            </label>
            <select
              value={platform}
              onChange={(e) => setPlatform(e.target.value)}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            >
              <option value="">Random (Mixed)</option>
              <option value="windows">Windows Only</option>
              <option value="mac">macOS Only</option>
              <option value="linux">Linux Only</option>
            </select>
          </div>

          {/* Proxy Settings Section */}
          <div className="border border-gray-700 rounded-lg p-4">
            <div className="flex items-center justify-between mb-3">
              <label className="text-sm font-medium text-gray-300">Proxy Settings</label>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={proxyEnabled}
                  onChange={(e) => setProxyEnabled(e.target.checked)}
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-gray-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-forge-accent"></div>
              </label>
            </div>

            {proxyEnabled && (
              <div className="space-y-3">
                {/* Proxy Type */}
                <div>
                  <label className="block text-xs text-gray-400 mb-1">Type</label>
                  <select
                    value={proxyType}
                    onChange={(e) => setProxyType(e.target.value)}
                    className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm focus:outline-none focus:border-forge-accent"
                  >
                    <option value="http">HTTP</option>
                    <option value="https">HTTPS</option>
                    <option value="socks5">SOCKS5</option>
                  </select>
                </div>

                {/* Host and Port */}
                <div className="grid grid-cols-3 gap-2">
                  <div className="col-span-2">
                    <label className="block text-xs text-gray-400 mb-1">Host</label>
                    <input
                      type="text"
                      value={proxyHost}
                      onChange={(e) => setProxyHost(e.target.value)}
                      placeholder="proxy.example.com"
                      className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm placeholder-gray-500 focus:outline-none focus:border-forge-accent"
                    />
                  </div>
                  <div>
                    <label className="block text-xs text-gray-400 mb-1">Port</label>
                    <input
                      type="number"
                      value={proxyPort}
                      onChange={(e) => setProxyPort(e.target.value)}
                      placeholder="8080"
                      className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm placeholder-gray-500 focus:outline-none focus:border-forge-accent"
                    />
                  </div>
                </div>

                {/* Username and Password */}
                <div className="grid grid-cols-2 gap-2">
                  <div>
                    <label className="block text-xs text-gray-400 mb-1">Username (Optional)</label>
                    <input
                      type="text"
                      value={proxyUsername}
                      onChange={(e) => setProxyUsername(e.target.value)}
                      placeholder="user"
                      className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm placeholder-gray-500 focus:outline-none focus:border-forge-accent"
                    />
                  </div>
                  <div>
                    <label className="block text-xs text-gray-400 mb-1">Password (Optional)</label>
                    <input
                      type="password"
                      value={proxyPassword}
                      onChange={(e) => setProxyPassword(e.target.value)}
                      placeholder="••••••"
                      className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm placeholder-gray-500 focus:outline-none focus:border-forge-accent"
                    />
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Preview Button */}
          <button
            onClick={handlePreview}
            disabled={loading}
            className="w-full px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded transition-colors disabled:opacity-50"
          >
            {loading ? 'Generating...' : 'Preview Fingerprint'}
          </button>

          {/* Fingerprint Preview */}
          {preview && (
            <div className="bg-gray-900 rounded p-4 text-sm">
              <h3 className="font-medium text-white mb-2">Generated Fingerprint</h3>
              <div className="space-y-1 text-gray-400">
                <div className="flex justify-between">
                  <span>Platform:</span>
                  <span className="text-gray-300">{preview.platform}</span>
                </div>
                <div className="flex justify-between">
                  <span>Screen:</span>
                  <span className="text-gray-300">{preview.screen_width}x{preview.screen_height}</span>
                </div>
                <div className="flex justify-between">
                  <span>CPU Cores:</span>
                  <span className="text-gray-300">{preview.hardware_concurrency}</span>
                </div>
                <div className="flex justify-between">
                  <span>Memory:</span>
                  <span className="text-gray-300">{preview.device_memory} GB</span>
                </div>
                <div className="flex justify-between">
                  <span>WebGL Vendor:</span>
                  <span className="text-gray-300 truncate ml-2">{preview.webgl_vendor}</span>
                </div>
                <div className="flex justify-between">
                  <span>Timezone:</span>
                  <span className="text-gray-300">{preview.timezone}</span>
                </div>
                <div className="flex justify-between">
                  <span>Language:</span>
                  <span className="text-gray-300">{preview.language}</span>
                </div>
                <div className="mt-2 pt-2 border-t border-gray-700">
                  <span className="text-gray-500">User Agent:</span>
                  <p className="text-gray-300 text-xs mt-1 break-all">{preview.user_agent}</p>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Actions */}
        <div className="flex gap-3 mt-6">
          <button
            onClick={handleClose}
            className="flex-1 px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleCreate}
            disabled={loading || !name.trim()}
            className="flex-1 px-4 py-2 bg-forge-accent hover:bg-forge-accent-hover text-white rounded font-medium transition-colors disabled:opacity-50"
          >
            {loading ? 'Creating...' : 'Create Identity'}
          </button>
        </div>
      </div>
    </div>
  );
}
