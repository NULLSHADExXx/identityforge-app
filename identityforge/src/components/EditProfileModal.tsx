import { useState, useEffect } from 'react';
import { updateProfile } from '../hooks/useApi';
import { ProfileWithStatus, UpdateProfileInput } from '../types/profile';

interface EditProfileModalProps {
  profile: ProfileWithStatus | null;
  onClose: () => void;
  onUpdated: () => void;
}

export function EditProfileModal({ profile, onClose, onUpdated }: EditProfileModalProps) {
  const [formData, setFormData] = useState<UpdateProfileInput>({
    id: '',
    name: '',
    user_agent: '',
    screen_width: 1920,
    screen_height: 1080,
    webgl_vendor: '',
    webgl_renderer: '',
    hardware_concurrency: 8,
    device_memory: 8,
    platform: '',
    timezone: '',
    language: '',
  });
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (profile) {
      setFormData({
        id: profile.id,
        name: profile.name,
        user_agent: profile.user_agent,
        screen_width: profile.screen_width,
        screen_height: profile.screen_height,
        webgl_vendor: profile.webgl_vendor,
        webgl_renderer: profile.webgl_renderer,
        hardware_concurrency: profile.hardware_concurrency,
        device_memory: profile.device_memory,
        platform: profile.platform,
        timezone: profile.timezone,
        language: profile.language,
      });
    }
  }, [profile]);

  const handleChange = (field: keyof UpdateProfileInput, value: string | number) => {
    setFormData(prev => ({ ...prev, [field]: value }));
  };

  const handleSubmit = async () => {
    if (!formData.name?.trim()) {
      alert('Profile name is required');
      return;
    }

    setLoading(true);
    try {
      const result = await updateProfile(formData);
      if (result.success) {
        onUpdated();
        onClose();
      } else {
        alert('Failed to update profile: ' + result.error);
      }
    } catch (err) {
      console.error('Update error:', err);
      alert('Failed to update profile');
    }
    setLoading(false);
  };

  if (!profile) return null;

  return (
    <div className="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-bold text-white">Edit Profile</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <div className="grid grid-cols-2 gap-4">
          {/* Profile Name */}
          <div className="col-span-2">
            <label className="block text-sm font-medium text-gray-300 mb-1">
              Profile Name
            </label>
            <input
              type="text"
              value={formData.name}
              onChange={(e) => handleChange('name', e.target.value)}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            />
          </div>

          {/* User Agent */}
          <div className="col-span-2">
            <label className="block text-sm font-medium text-gray-300 mb-1">
              User Agent
            </label>
            <textarea
              value={formData.user_agent}
              onChange={(e) => handleChange('user_agent', e.target.value)}
              rows={2}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white text-sm focus:outline-none focus:border-forge-accent"
            />
          </div>

          {/* Screen Resolution */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              Screen Width
            </label>
            <input
              type="number"
              value={formData.screen_width}
              onChange={(e) => handleChange('screen_width', parseInt(e.target.value))}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              Screen Height
            </label>
            <input
              type="number"
              value={formData.screen_height}
              onChange={(e) => handleChange('screen_height', parseInt(e.target.value))}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            />
          </div>

          {/* Hardware */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              CPU Cores
            </label>
            <select
              value={formData.hardware_concurrency}
              onChange={(e) => handleChange('hardware_concurrency', parseInt(e.target.value))}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            >
              {[2, 4, 6, 8, 10, 12, 16, 24, 32].map(n => (
                <option key={n} value={n}>{n}</option>
              ))}
            </select>
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              Device Memory (GB)
            </label>
            <select
              value={formData.device_memory}
              onChange={(e) => handleChange('device_memory', parseInt(e.target.value))}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            >
              {[2, 4, 8, 16, 32, 64].map(n => (
                <option key={n} value={n}>{n}</option>
              ))}
            </select>
          </div>

          {/* WebGL */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              WebGL Vendor
            </label>
            <input
              type="text"
              value={formData.webgl_vendor}
              onChange={(e) => handleChange('webgl_vendor', e.target.value)}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              WebGL Renderer
            </label>
            <input
              type="text"
              value={formData.webgl_renderer}
              onChange={(e) => handleChange('webgl_renderer', e.target.value)}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            />
          </div>

          {/* Platform */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              Platform
            </label>
            <input
              type="text"
              value={formData.platform}
              onChange={(e) => handleChange('platform', e.target.value)}
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            />
          </div>

          {/* Timezone */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              Timezone
            </label>
            <input
              type="text"
              value={formData.timezone}
              onChange={(e) => handleChange('timezone', e.target.value)}
              placeholder="e.g., America/New_York"
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            />
          </div>

          {/* Language */}
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-1">
              Language
            </label>
            <input
              type="text"
              value={formData.language}
              onChange={(e) => handleChange('language', e.target.value)}
              placeholder="e.g., en-US"
              className="w-full px-3 py-2 bg-gray-900 border border-gray-700 rounded text-white focus:outline-none focus:border-forge-accent"
            />
          </div>
        </div>

        {/* Warning for active profiles */}
        {profile.is_active && (
          <div className="mt-4 p-3 bg-forge-warning/20 border border-forge-warning/50 rounded text-forge-warning text-sm">
            ⚠️ This profile has an active browser window. Changes will apply on next launch.
          </div>
        )}

        {/* Actions */}
        <div className="flex gap-3 mt-6">
          <button
            onClick={onClose}
            className="flex-1 px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded transition-colors"
          >
            Cancel
          </button>
          <button
            onClick={handleSubmit}
            disabled={loading}
            className="flex-1 px-4 py-2 bg-forge-accent hover:bg-forge-accent-hover text-white rounded font-medium transition-colors disabled:opacity-50"
          >
            {loading ? 'Saving...' : 'Save Changes'}
          </button>
        </div>
      </div>
    </div>
  );
}
