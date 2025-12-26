import { useState, useRef } from 'react';
import { ProfileWithStatus } from '../types/profile';
import { launchProfile, closeProfileWindow, deleteProfile, regenerateFingerprint, exportCookies, importCookies, clearCookies, navigateProfile } from '../hooks/useApi';

interface ProfileCardProps {
  profile: ProfileWithStatus;
  onUpdate: () => void;
  onEdit: (profile: ProfileWithStatus) => void;
}

export function ProfileCard({ profile, onUpdate, onEdit }: ProfileCardProps) {
  const [loading, setLoading] = useState(false);
  const [launchUrl, setLaunchUrl] = useState(profile.default_url || 'https://www.google.com');
  const [showUrlInput, setShowUrlInput] = useState(false);
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const [showCookieMenu, setShowCookieMenu] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleLaunch = async () => {
    setLoading(true);
    try {
      const result = await launchProfile({ 
        profile_id: profile.id, 
        start_url: launchUrl 
      });
      if (!result.success) {
        console.error('Launch failed:', result.error);
        alert('Failed to launch: ' + result.error);
      }
      onUpdate();
    } catch (err) {
      console.error('Launch error:', err);
      alert('Failed to launch browser');
    }
    setLoading(false);
    setShowUrlInput(false);
  };

  const handleClose = async () => {
    setLoading(true);
    try {
      await closeProfileWindow(profile.id);
      onUpdate();
    } catch (err) {
      console.error('Close error:', err);
    }
    setLoading(false);
  };

  const handleDeleteClick = () => {
    console.log('Delete button clicked for:', profile.name);
    setShowDeleteConfirm(true);
  };

  const handleDeleteConfirm = async () => {
    console.log('Delete confirmed for:', profile.id);
    setLoading(true);
    setShowDeleteConfirm(false);
    
    try {
      console.log('Calling deleteProfile with ID:', profile.id);
      const result = await deleteProfile(profile.id);
      console.log('Delete result:', result);
      
      if (result.success) {
        console.log('Delete successful');
        onUpdate();
      } else {
        console.error('Delete failed:', result.error);
        alert('Failed to delete profile:\n' + (result.error || 'Unknown error'));
      }
    } catch (err) {
      console.error('Delete exception:', err);
      alert('Failed to delete profile:\n' + String(err));
    }
    
    setLoading(false);
  };

  const handleRegenerate = async () => {
    if (!window.confirm('Regenerate fingerprint?\n\nThis will create new browser characteristics for this profile.')) {
      return;
    }
    
    setLoading(true);
    try {
      const result = await regenerateFingerprint(profile.id);
      if (result.success) {
        onUpdate();
      } else {
        alert('Failed to regenerate: ' + result.error);
      }
    } catch (err) {
      console.error('Regenerate error:', err);
      alert('Failed to regenerate fingerprint');
    }
    setLoading(false);
  };

  const handleTestFingerprint = async () => {
    if (profile.is_active) {
      // Navigate existing window to browserleaks
      try {
        await navigateProfile(profile.id, 'https://browserleaks.com/canvas');
      } catch (err) {
        console.error('Navigate error:', err);
      }
    } else {
      // Launch with browserleaks
      setLoading(true);
      try {
        const result = await launchProfile({ 
          profile_id: profile.id, 
          start_url: 'https://browserleaks.com/canvas'
        });
        if (!result.success) {
          alert('Failed to launch: ' + result.error);
        }
        onUpdate();
      } catch (err) {
        console.error('Launch error:', err);
      }
      setLoading(false);
    }
  };

  const handleExportCookies = async () => {
    setShowCookieMenu(false);
    try {
      const result = await exportCookies(profile.id);
      if (result.success && result.data) {
        // Download as JSON file
        const blob = new Blob([result.data], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `${profile.name}_cookies.json`;
        a.click();
        URL.revokeObjectURL(url);
      } else {
        alert('Failed to export cookies: ' + (result.error || 'No cookies found'));
      }
    } catch (err) {
      console.error('Export error:', err);
      alert('Failed to export cookies');
    }
  };

  const handleImportCookies = () => {
    setShowCookieMenu(false);
    fileInputRef.current?.click();
  };

  const handleFileSelect = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    try {
      const text = await file.text();
      const result = await importCookies(profile.id, text);
      if (result.success) {
        alert('Cookies imported successfully!');
      } else {
        alert('Failed to import cookies: ' + result.error);
      }
    } catch (err) {
      console.error('Import error:', err);
      alert('Failed to import cookies');
    }
    
    // Reset file input
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const handleClearCookies = async () => {
    setShowCookieMenu(false);
    if (!window.confirm('Clear all cookies for this profile?')) return;
    
    try {
      const result = await clearCookies(profile.id);
      if (result.success) {
        alert('Cookies cleared!');
      } else {
        alert('Failed to clear cookies: ' + result.error);
      }
    } catch (err) {
      console.error('Clear error:', err);
      alert('Failed to clear cookies');
    }
  };

  const formatDate = (timestamp: string | null) => {
    if (!timestamp) return 'Never';
    const date = new Date(parseInt(timestamp) * 1000);
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
  };

  const getPlatformIcon = () => {
    if (profile.platform.includes('Win')) return '🪟';
    if (profile.platform.includes('Mac')) return '🍎';
    if (profile.platform.includes('Linux')) return '🐧';
    return '💻';
  };

  return (
    <div className={`bg-gray-800 rounded-lg p-4 border ${profile.is_active ? 'border-forge-success' : 'border-gray-700'} hover:border-gray-600 transition-colors relative`}>
      
      {/* Hidden file input for cookie import */}
      <input
        ref={fileInputRef}
        type="file"
        accept=".json"
        onChange={handleFileSelect}
        className="hidden"
      />

      {/* Delete Confirmation Modal */}
      {showDeleteConfirm && (
        <div className="absolute inset-0 bg-gray-900/95 rounded-lg flex flex-col items-center justify-center z-10 p-4">
          <p className="text-white text-center mb-4">
            Delete <strong>"{profile.name}"</strong>?
            <br />
            <span className="text-gray-400 text-sm">All browsing data will be removed.</span>
          </p>
          <div className="flex gap-2">
            <button
              onMouseDown={() => setShowDeleteConfirm(false)}
              className="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded text-sm"
            >
              Cancel
            </button>
            <button
              onMouseDown={handleDeleteConfirm}
              className="px-4 py-2 bg-red-600 hover:bg-red-700 text-white rounded text-sm font-medium"
            >
              Delete
            </button>
          </div>
        </div>
      )}

      {/* Cookie Menu */}
      {showCookieMenu && (
        <div className="absolute right-2 top-12 bg-gray-900 border border-gray-700 rounded-lg shadow-xl z-20 py-1 min-w-[140px]">
          <button
            onMouseDown={handleExportCookies}
            className="w-full px-3 py-2 text-left text-sm text-gray-300 hover:bg-gray-700 flex items-center gap-2"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12" />
            </svg>
            Export
          </button>
          <button
            onMouseDown={handleImportCookies}
            className="w-full px-3 py-2 text-left text-sm text-gray-300 hover:bg-gray-700 flex items-center gap-2"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4" />
            </svg>
            Import
          </button>
          <button
            onMouseDown={handleClearCookies}
            className="w-full px-3 py-2 text-left text-sm text-red-400 hover:bg-gray-700 flex items-center gap-2"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
            Clear
          </button>
        </div>
      )}

      {/* Click outside to close cookie menu */}
      {showCookieMenu && (
        <div 
          className="fixed inset-0 z-10" 
          onMouseDown={() => setShowCookieMenu(false)}
        />
      )}

      {/* Header */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <span className="text-2xl">{getPlatformIcon()}</span>
          <div>
            <h3 className="font-semibold text-white">{profile.name}</h3>
            <div className="flex items-center gap-2">
              <span className={`text-xs px-2 py-0.5 rounded ${profile.is_active ? 'bg-forge-success/20 text-forge-success' : 'bg-gray-700 text-gray-400'}`}>
                {profile.is_active ? '● Active' : '○ Inactive'}
              </span>
              {profile.proxy_enabled && (
                <span className="text-xs px-2 py-0.5 rounded bg-blue-500/20 text-blue-400" title={`${profile.proxy_type.toUpperCase()}://${profile.proxy_host}:${profile.proxy_port}`}>
                  🔒 Proxy
                </span>
              )}
            </div>
          </div>
        </div>
        <div className="flex gap-1">
          {/* Test Fingerprint Button */}
          <button
            onMouseDown={handleTestFingerprint}
            disabled={loading}
            className="p-1.5 text-gray-400 hover:text-green-400 hover:bg-gray-700 rounded transition-colors disabled:opacity-50"
            title="Test Fingerprint (browserleaks.com)"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </button>

          {/* Cookie Menu Button */}
          <button
            onMouseDown={() => setShowCookieMenu(!showCookieMenu)}
            className="p-1.5 text-gray-400 hover:text-yellow-400 hover:bg-gray-700 rounded transition-colors"
            title="Cookie Management"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4" />
            </svg>
          </button>

          {/* Edit Button */}
          <button
            onMouseDown={() => onEdit(profile)}
            className="p-1.5 text-gray-400 hover:text-white hover:bg-gray-700 rounded transition-colors"
            title="Edit Profile"
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
          </button>
          
          {/* Regenerate Button */}
          <button
            onMouseDown={handleRegenerate}
            disabled={loading || profile.is_active}
            className="p-1.5 text-gray-400 hover:text-forge-warning hover:bg-gray-700 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            title={profile.is_active ? "Close browser first" : "Regenerate Fingerprint"}
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
            </svg>
          </button>
          
          {/* Delete Button */}
          <button
            onMouseDown={handleDeleteClick}
            disabled={loading || profile.is_active}
            className="p-1.5 text-gray-400 hover:text-forge-danger hover:bg-gray-700 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            title={profile.is_active ? "Close browser first to delete" : "Delete Profile"}
          >
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </button>
        </div>
      </div>

      {/* Fingerprint Details */}
      <div className="text-xs text-gray-400 space-y-1 mb-3">
        <div className="flex justify-between">
          <span>Screen:</span>
          <span className="text-gray-300">{profile.screen_width}x{profile.screen_height}</span>
        </div>
        <div className="flex justify-between">
          <span>CPU Cores:</span>
          <span className="text-gray-300">{profile.hardware_concurrency}</span>
        </div>
        <div className="flex justify-between">
          <span>Memory:</span>
          <span className="text-gray-300">{profile.device_memory} GB</span>
        </div>
        <div className="flex justify-between">
          <span>Timezone:</span>
          <span className="text-gray-300">{profile.timezone}</span>
        </div>
        <div className="flex justify-between">
          <span>Language:</span>
          <span className="text-gray-300">{profile.language}</span>
        </div>
        {profile.proxy_enabled && (
          <div className="flex justify-between">
            <span>Proxy:</span>
            <span className="text-blue-400">{profile.proxy_type}://{profile.proxy_host}:{profile.proxy_port}</span>
          </div>
        )}
        <div className="flex justify-between">
          <span>Last Used:</span>
          <span className="text-gray-300">{formatDate(profile.last_used)}</span>
        </div>
      </div>

      {/* User Agent (truncated) */}
      <div className="text-xs text-gray-500 mb-3 truncate" title={profile.user_agent}>
        UA: {profile.user_agent}
      </div>

      {/* URL Input */}
      {showUrlInput && (
        <div className="mb-3">
          <input
            type="url"
            value={launchUrl}
            onChange={(e) => setLaunchUrl(e.target.value)}
            placeholder="Enter URL..."
            className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-sm text-white placeholder-gray-500 focus:outline-none focus:border-forge-accent"
            onKeyDown={(e) => {
              if (e.key === 'Enter') {
                handleLaunch();
              }
            }}
          />
        </div>
      )}

      {/* Actions */}
      <div className="flex gap-2">
        {profile.is_active ? (
          <button
            onMouseDown={handleClose}
            disabled={loading}
            className="flex-1 px-3 py-2 bg-forge-danger hover:bg-red-600 text-white rounded text-sm font-medium transition-colors disabled:opacity-50"
          >
            {loading ? 'Closing...' : 'Close Window'}
          </button>
        ) : (
          <>
            <button
              onMouseDown={() => setShowUrlInput(!showUrlInput)}
              className="px-3 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded text-sm transition-colors"
            >
              URL
            </button>
            <button
              onMouseDown={handleLaunch}
              disabled={loading}
              className="flex-1 px-3 py-2 bg-forge-accent hover:bg-forge-accent-hover text-white rounded text-sm font-medium transition-colors disabled:opacity-50"
            >
              {loading ? 'Launching...' : 'Launch'}
            </button>
          </>
        )}
      </div>
    </div>
  );
}
