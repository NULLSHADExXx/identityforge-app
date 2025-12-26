import { useState } from 'react';
import { bulkCreateProfiles } from '../hooks/useApi';
import { ProxyConfig } from '../types/profile';

interface BulkCreateModalProps {
  isOpen: boolean;
  onClose: () => void;
  onCreated: () => void;
}

export function BulkCreateModal({ isOpen, onClose, onCreated }: BulkCreateModalProps) {
  const [count, setCount] = useState(10);
  const [namePrefix, setNamePrefix] = useState('Profile');
  const [platform, setPlatform] = useState<string>('');
  const [defaultUrl, setDefaultUrl] = useState('https://www.google.com');
  const [loading, setLoading] = useState(false);
  const [progress, setProgress] = useState(0);

  // Proxy settings
  const [proxyEnabled, setProxyEnabled] = useState(false);
  const [proxyType, setProxyType] = useState('http');
  const [proxyHost, setProxyHost] = useState('');
  const [proxyPort, setProxyPort] = useState('');
  const [proxyUsername, setProxyUsername] = useState('');
  const [proxyPassword, setProxyPassword] = useState('');

  if (!isOpen) return null;

  const handleCreate = async () => {
    if (count < 1 || count > 100) {
      alert('Count must be between 1 and 100');
      return;
    }
    if (!namePrefix.trim()) {
      alert('Name prefix is required');
      return;
    }
    if (proxyEnabled && (!proxyHost.trim() || !proxyPort.trim())) {
      alert('Please enter proxy host and port');
      return;
    }

    setLoading(true);
    setProgress(0);

    try {
      const proxy: ProxyConfig | undefined = proxyEnabled ? {
        enabled: true,
        proxy_type: proxyType,
        host: proxyHost.trim(),
        port: parseInt(proxyPort) || 0,
        username: proxyUsername.trim() || undefined,
        password: proxyPassword.trim() || undefined,
      } : undefined;

      const result = await bulkCreateProfiles(
        count,
        namePrefix.trim(),
        platform || undefined,
        defaultUrl.trim() || undefined,
        proxy
      );

      if (result.success) {
        setProgress(100);
        setTimeout(() => {
          onCreated();
          handleClose();
        }, 500);
      } else {
        alert('Failed to create profiles: ' + result.error);
      }
    } catch (err) {
      console.error('Bulk create error:', err);
      alert('Failed to create profiles');
    }

    setLoading(false);
  };

  const handleClose = () => {
    setCount(10);
    setNamePrefix('Profile');
    setPlatform('');
    setDefaultUrl('https://www.google.com');
    setProgress(0);
    setProxyEnabled(false);
    setProxyType('http');
    setProxyHost('');
    setProxyPort('');
    setProxyUsername('');
    setProxyPassword('');
    onClose();
  };

  return (
    <div className="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-full max-w-md max-h-[90vh] overflow-y-auto">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-xl font-bold text-white">Bulk Create Profiles</h2>
          <button
            onClick={handleClose}
            disabled={loading}
            className="text-gray-400 hover:text-white transition-colors disabled:opacity-50"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="space-y-4">
          {/* Count */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Number of Profiles
            </label>
            <input
              type="number"
              value={count}
              onChange={(e) => setCount(parseInt(e.target.value) || 1)}
              min={1}
              max={100}
              disabled={loading}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded-lg text-white focus:outline-none focus:border-forge-accent disabled:opacity-50"
            />
            <p className="text-xs text-gray-500 mt-1">Max 100 profiles at once</p>
          </div>

          {/* Name Prefix */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Name Prefix
            </label>
            <input
              type="text"
              value={namePrefix}
              onChange={(e) => setNamePrefix(e.target.value)}
              placeholder="e.g., FB Account, Gmail"
              disabled={loading}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:border-forge-accent disabled:opacity-50"
            />
            <p className="text-xs text-gray-500 mt-1">
              Profiles will be named: "{namePrefix} 1", "{namePrefix} 2", etc.
            </p>
          </div>

          {/* Default URL */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Default URL
            </label>
            <input
              type="url"
              value={defaultUrl}
              onChange={(e) => setDefaultUrl(e.target.value)}
              placeholder="https://www.facebook.com"
              disabled={loading}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:border-forge-accent disabled:opacity-50"
            />
            <p className="text-xs text-gray-500 mt-1">
              All profiles will open this URL when launched
            </p>
          </div>

          {/* Platform */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Platform (Optional)
            </label>
            <select
              value={platform}
              onChange={(e) => setPlatform(e.target.value)}
              disabled={loading}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded-lg text-white focus:outline-none focus:border-forge-accent disabled:opacity-50"
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
              <label className="text-sm font-medium text-gray-300">Proxy Settings (All Profiles)</label>
              <label className="relative inline-flex items-center cursor-pointer">
                <input
                  type="checkbox"
                  checked={proxyEnabled}
                  onChange={(e) => setProxyEnabled(e.target.checked)}
                  disabled={loading}
                  className="sr-only peer"
                />
                <div className="w-11 h-6 bg-gray-700 peer-focus:outline-none rounded-full peer peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-forge-accent peer-disabled:opacity-50"></div>
              </label>
            </div>

            {proxyEnabled && (
              <div className="space-y-3">
                <p className="text-xs text-yellow-400">⚠️ All profiles will use the same proxy. For different proxies per profile, create individually.</p>
                
                {/* Proxy Type */}
                <div>
                  <label className="block text-xs text-gray-400 mb-1">Type</label>
                  <select
                    value={proxyType}
                    onChange={(e) => setProxyType(e.target.value)}
                    disabled={loading}
                    className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm focus:outline-none focus:border-forge-accent disabled:opacity-50"
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
                      disabled={loading}
                      className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm placeholder-gray-500 focus:outline-none focus:border-forge-accent disabled:opacity-50"
                    />
                  </div>
                  <div>
                    <label className="block text-xs text-gray-400 mb-1">Port</label>
                    <input
                      type="number"
                      value={proxyPort}
                      onChange={(e) => setProxyPort(e.target.value)}
                      placeholder="8080"
                      disabled={loading}
                      className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm placeholder-gray-500 focus:outline-none focus:border-forge-accent disabled:opacity-50"
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
                      disabled={loading}
                      className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm placeholder-gray-500 focus:outline-none focus:border-forge-accent disabled:opacity-50"
                    />
                  </div>
                  <div>
                    <label className="block text-xs text-gray-400 mb-1">Password (Optional)</label>
                    <input
                      type="password"
                      value={proxyPassword}
                      onChange={(e) => setProxyPassword(e.target.value)}
                      placeholder="••••••"
                      disabled={loading}
                      className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm placeholder-gray-500 focus:outline-none focus:border-forge-accent disabled:opacity-50"
                    />
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Progress */}
          {loading && (
            <div className="mt-4">
              <div className="flex justify-between text-sm text-gray-400 mb-1">
                <span>Creating profiles...</span>
                <span>{progress}%</span>
              </div>
              <div className="w-full bg-gray-700 rounded-full h-2">
                <div
                  className="bg-forge-accent h-2 rounded-full transition-all duration-300"
                  style={{ width: `${progress}%` }}
                />
              </div>
            </div>
          )}

          {/* Preview */}
          <div className="bg-gray-900/50 rounded-lg p-3 mt-4">
            <h4 className="text-sm font-medium text-gray-300 mb-2">Preview</h4>
            <div className="text-xs text-gray-400 space-y-1">
              <p>• {count} profiles will be created</p>
              <p>• Names: {namePrefix} 1 → {namePrefix} {count}</p>
              <p>• Default URL: {defaultUrl || 'https://www.google.com'}</p>
              <p>• Platform: {platform || 'Random mix of Windows/Mac/Linux'}</p>
              {proxyEnabled && (
                <p>• Proxy: {proxyType}://{proxyHost}:{proxyPort}</p>
              )}
              <p>• Each profile gets unique fingerprint</p>
            </div>
          </div>
        </div>

        {/* Actions */}
        <div className="flex gap-3 mt-6">
          <button
            onClick={handleClose}
            disabled={loading}
            className="flex-1 px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg transition-colors disabled:opacity-50"
          >
            Cancel
          </button>
          <button
            onClick={handleCreate}
            disabled={loading || count < 1}
            className="flex-1 px-4 py-2 bg-forge-accent hover:bg-forge-accent-hover text-white rounded-lg font-medium transition-colors disabled:opacity-50 flex items-center justify-center gap-2"
          >
            {loading ? (
              <>
                <svg className="animate-spin h-4 w-4" viewBox="0 0 24 24">
                  <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4" fill="none" />
                  <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z" />
                </svg>
                Creating...
              </>
            ) : (
              <>
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                </svg>
                Create {count} Profiles
              </>
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
