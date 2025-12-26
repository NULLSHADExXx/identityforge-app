import { useState, useEffect, useCallback } from 'react';
import { ProfileCard } from './components/ProfileCard';
import { CreateProfileModal } from './components/CreateProfileModal';
import { EditProfileModal } from './components/EditProfileModal';
import { AboutModal } from './components/AboutModal';
import { BulkCreateModal } from './components/BulkCreateModal';
import { getProfiles, deleteAllInactiveProfiles } from './hooks/useApi';
import { ProfileWithStatus } from './types/profile';

function App() {
  const [profiles, setProfiles] = useState<ProfileWithStatus[]>([]);
  const [loading, setLoading] = useState(true);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [showBulkCreateModal, setShowBulkCreateModal] = useState(false);
  const [showAboutModal, setShowAboutModal] = useState(false);
  const [editingProfile, setEditingProfile] = useState<ProfileWithStatus | null>(null);

  const [searchQuery, setSearchQuery] = useState('');
  const [filterActive, setFilterActive] = useState<'all' | 'active' | 'inactive'>('all');
  const [showDeleteAllConfirm, setShowDeleteAllConfirm] = useState(false);

  const loadProfiles = useCallback(async () => {
    try {
      const result = await getProfiles();
      if (result.success && result.data) {
        setProfiles(result.data);
      }
    } catch (err) {
      console.error('Failed to load profiles:', err);
    }
    setLoading(false);
  }, []);



  useEffect(() => {
    loadProfiles();
    
    // Refresh profiles periodically to update active status
    const interval = setInterval(loadProfiles, 5000);
    return () => clearInterval(interval);
  }, [loadProfiles]);

  const filteredProfiles = profiles.filter(profile => {
    const matchesSearch = profile.name.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesFilter = 
      filterActive === 'all' ||
      (filterActive === 'active' && profile.is_active) ||
      (filterActive === 'inactive' && !profile.is_active);
    return matchesSearch && matchesFilter;
  });

  const activeCount = profiles.filter(p => p.is_active).length;

  return (
    <div className="min-h-screen bg-forge-darker text-white">
      {/* Header */}
      <header className="bg-forge-dark border-b border-gray-800 sticky top-0 z-40">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 bg-forge-accent rounded-lg flex items-center justify-center">
                <svg className="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 11c0 3.517-1.009 6.799-2.753 9.571m-3.44-2.04l.054-.09A13.916 13.916 0 008 11a4 4 0 118 0c0 1.017-.07 2.019-.203 3m-2.118 6.844A21.88 21.88 0 0015.171 17m3.839 1.132c.645-2.266.99-4.659.99-7.132A8 8 0 008 4.07M3 15.364c.64-1.319 1-2.8 1-4.364 0-1.457.39-2.823 1.07-4" />
                </svg>
              </div>
              <div>
                <h1 className="text-xl font-bold">IdentityForge</h1>
                <p className="text-xs text-gray-500">
                  v0.1.0 • Anti-Detect Browser Manager
                </p>
              </div>
            </div>
            
            <div className="flex items-center gap-3">
              <div className="text-sm text-gray-400 mr-2">
                <span className="text-forge-success font-medium">{activeCount}</span> active / 
                <span className="font-medium ml-1">{profiles.length}</span> profiles
              </div>
              
              {/* About Button */}
              <button
                onClick={() => setShowAboutModal(true)}
                className="p-2 text-gray-400 hover:text-white hover:bg-gray-700 rounded-lg transition-colors"
                title="About"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
              </button>

              {/* Bulk Create Button */}
              <button
                onClick={() => setShowBulkCreateModal(true)}
                className="px-3 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded-lg font-medium transition-colors flex items-center gap-2"
                title="Bulk Create"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
                </svg>
                Bulk
              </button>
              
              <button
                onClick={() => setShowCreateModal(true)}
                className="px-4 py-2 bg-forge-accent hover:bg-forge-accent-hover text-white rounded-lg font-medium transition-colors flex items-center gap-2"
              >
                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                </svg>
                New Identity
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Toolbar */}
      <div className="bg-forge-dark/50 border-b border-gray-800">
        <div className="container mx-auto px-4 py-3">
          <div className="flex items-center gap-4">
            {/* Search */}
            <div className="flex-1 max-w-md">
              <div className="relative">
                <svg className="w-5 h-5 absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
                </svg>
                <input
                  type="text"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  placeholder="Search profiles..."
                  className="w-full pl-10 pr-4 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:border-forge-accent"
                />
              </div>
            </div>

            {/* Delete All Inactive Button */}
            {profiles.filter(p => !p.is_active).length > 0 && (
              <div className="relative">
                {!showDeleteAllConfirm ? (
                  <button
                    onMouseDown={(e) => {
                      e.preventDefault();
                      e.stopPropagation();
                      console.log('Delete all inactive clicked');
                      setShowDeleteAllConfirm(true);
                    }}
                    className="px-3 py-1.5 bg-red-600/20 hover:bg-red-600/40 text-red-400 hover:text-red-300 rounded-lg text-sm font-medium transition-colors flex items-center gap-1.5"
                    title="Delete all inactive profiles"
                  >
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                    </svg>
                    Delete Inactive ({profiles.filter(p => !p.is_active).length})
                  </button>
                ) : (
                  <div className="flex items-center gap-2 bg-red-900/50 px-3 py-1.5 rounded-lg border border-red-600">
                    <span className="text-sm text-red-300">Delete {profiles.filter(p => !p.is_active).length} profiles?</span>
                    <button
                      onMouseDown={async (e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        console.log('Confirming delete all');
                        try {
                          const result = await deleteAllInactiveProfiles();
                          if (result.success) {
                            console.log('Deleted', result.data, 'profiles');
                            loadProfiles();
                          } else {
                            console.error('Error:', result.error);
                          }
                        } catch (err) {
                          console.error('Delete all failed:', err);
                        }
                        setShowDeleteAllConfirm(false);
                      }}
                      className="px-2 py-0.5 bg-red-600 hover:bg-red-500 text-white rounded text-sm font-medium"
                    >
                      Yes
                    </button>
                    <button
                      onMouseDown={(e) => {
                        e.preventDefault();
                        e.stopPropagation();
                        setShowDeleteAllConfirm(false);
                      }}
                      className="px-2 py-0.5 bg-gray-600 hover:bg-gray-500 text-white rounded text-sm font-medium"
                    >
                      No
                    </button>
                  </div>
                )}
              </div>
            )}

            {/* Filter */}
            <div className="flex items-center gap-2">
              <span className="text-sm text-gray-500">Filter:</span>
              <div className="flex bg-gray-800 rounded-lg p-1">
                {(['all', 'active', 'inactive'] as const).map((filter) => (
                  <button
                    key={filter}
                    onClick={() => setFilterActive(filter)}
                    className={`px-3 py-1 rounded text-sm transition-colors ${
                      filterActive === filter
                        ? 'bg-forge-accent text-white'
                        : 'text-gray-400 hover:text-white'
                    }`}
                  >
                    {filter.charAt(0).toUpperCase() + filter.slice(1)}
                  </button>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Main Content */}
      <main className="container mx-auto px-4 py-6">
        {loading ? (
          <div className="flex items-center justify-center py-20">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-forge-accent"></div>
          </div>
        ) : filteredProfiles.length === 0 ? (
          <div className="text-center py-20">
            <div className="w-20 h-20 bg-gray-800 rounded-full flex items-center justify-center mx-auto mb-4">
              <svg className="w-10 h-10 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 11c0 3.517-1.009 6.799-2.753 9.571m-3.44-2.04l.054-.09A13.916 13.916 0 008 11a4 4 0 118 0c0 1.017-.07 2.019-.203 3m-2.118 6.844A21.88 21.88 0 0015.171 17m3.839 1.132c.645-2.266.99-4.659.99-7.132A8 8 0 008 4.07M3 15.364c.64-1.319 1-2.8 1-4.364 0-1.457.39-2.823 1.07-4" />
              </svg>
            </div>
            <h2 className="text-xl font-semibold text-gray-400 mb-2">
              {profiles.length === 0 ? 'No profiles yet' : 'No matching profiles'}
            </h2>
            <p className="text-gray-500 mb-4">
              {profiles.length === 0 
                ? 'Create your first identity to get started'
                : 'Try adjusting your search or filter'}
            </p>
            {profiles.length === 0 && (
              <div className="flex gap-3 justify-center">
                <button
                  onClick={() => setShowCreateModal(true)}
                  className="px-6 py-3 bg-forge-accent hover:bg-forge-accent-hover text-white rounded-lg font-medium transition-colors"
                >
                  Create First Identity
                </button>
                <button
                  onClick={() => setShowBulkCreateModal(true)}
                  className="px-6 py-3 bg-gray-700 hover:bg-gray-600 text-white rounded-lg font-medium transition-colors"
                >
                  Bulk Create
                </button>
              </div>
            )}
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
            {filteredProfiles.map((profile) => (
              <ProfileCard
                key={profile.id}
                profile={profile}
                onUpdate={loadProfiles}
                onEdit={setEditingProfile}
              />
            ))}
          </div>
        )}
      </main>

      {/* Footer */}
      <footer className="bg-forge-dark border-t border-gray-800 py-4 mt-auto">
        <div className="container mx-auto px-4 text-center text-sm text-gray-500">
          <p>IdentityForge • Each profile uses isolated storage with unique fingerprints</p>
          <p className="mt-1 text-xs">
            Canvas noise • WebGL spoofing • WebRTC protection • Timezone masking • Font fingerprinting • Audio protection
          </p>
        </div>
      </footer>

      {/* Modals */}
      <CreateProfileModal
        isOpen={showCreateModal}
        onClose={() => setShowCreateModal(false)}
        onCreated={loadProfiles}
      />
      <EditProfileModal
        profile={editingProfile}
        onClose={() => setEditingProfile(null)}
        onUpdated={loadProfiles}
      />
      <AboutModal
        isOpen={showAboutModal}
        onClose={() => setShowAboutModal(false)}
      />
      <BulkCreateModal
        isOpen={showBulkCreateModal}
        onClose={() => setShowBulkCreateModal(false)}
        onCreated={loadProfiles}
      />
    </div>
  );
}

export default App;
