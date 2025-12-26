interface AboutModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export function AboutModal({ isOpen, onClose }: AboutModalProps) {
  if (!isOpen) return null;

  const openTelegram = () => {
    window.open('https://t.me/NULLSHADExXx', '_blank');
  };

  return (
    <div className="fixed inset-0 bg-black/70 flex items-center justify-center z-50">
      <div className="bg-gray-800 rounded-lg p-6 w-full max-w-lg max-h-[90vh] overflow-y-auto">
        <div className="flex items-center justify-between mb-6">
          <h2 className="text-xl font-bold text-white">About IdentityForge</h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Logo & Version */}
        <div className="flex items-center gap-4 mb-6">
          <div className="w-16 h-16 bg-forge-accent rounded-xl flex items-center justify-center">
            <svg className="w-10 h-10 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 11c0 3.517-1.009 6.799-2.753 9.571m-3.44-2.04l.054-.09A13.916 13.916 0 008 11a4 4 0 118 0c0 1.017-.07 2.019-.203 3m-2.118 6.844A21.88 21.88 0 0015.171 17m3.839 1.132c.645-2.266.99-4.659.99-7.132A8 8 0 008 4.07M3 15.364c.64-1.319 1-2.8 1-4.364 0-1.457.39-2.823 1.07-4" />
            </svg>
          </div>
          <div>
            <h3 className="text-lg font-bold text-white">IdentityForge</h3>
            <p className="text-sm text-gray-400">v0.1.0 • Anti-Detect Browser Manager</p>
          </div>
        </div>

        {/* How It Works */}
        <div className="bg-gray-900 rounded-lg p-4 mb-6">
          <h4 className="font-semibold text-white mb-3">How It Works</h4>
          <div className="space-y-3 text-sm text-gray-300">
            <div className="flex gap-3">
              <span className="text-forge-accent font-bold">1.</span>
              <p><strong>Create Profiles</strong> - Each profile gets a unique browser fingerprint (User-Agent, screen size, WebGL, timezone, etc.)</p>
            </div>
            <div className="flex gap-3">
              <span className="text-forge-accent font-bold">2.</span>
              <p><strong>Isolated Storage</strong> - Every profile has its own cookies, localStorage, and browsing data in separate folders</p>
            </div>
            <div className="flex gap-3">
              <span className="text-forge-accent font-bold">3.</span>
              <p><strong>Persistent Noise</strong> - Canvas/Audio noise is unique per profile but consistent across page refreshes</p>
            </div>
            <div className="flex gap-3">
              <span className="text-forge-accent font-bold">4.</span>
              <p><strong>Complete Protection</strong> - WebRTC disabled, fonts masked, media queries spoofed, timing attacks prevented</p>
            </div>
          </div>
        </div>

        {/* Features Grid */}
        <div className="mb-6">
          <h4 className="font-semibold text-white mb-3">Protection Features</h4>
          <div className="grid grid-cols-2 gap-2 text-sm">
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> Canvas Fingerprint Noise
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> WebGL Vendor Spoofing
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> WebRTC Leak Protection
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> Timezone Masking
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> Audio Fingerprint Noise
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> Font Fingerprinting
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> Media Query Spoofing
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> ClientRects Protection
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> Navigator Spoofing
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> Performance API Noise
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> Battery API Spoofing
            </div>
            <div className="bg-gray-900/50 rounded p-2 flex items-center gap-2">
              <span className="text-forge-success">✓</span> Bulk Profile Creator
            </div>
          </div>
        </div>

        {/* Key Points */}
        <div className="bg-forge-accent/10 border border-forge-accent/30 rounded-lg p-4 mb-6">
          <h4 className="font-semibold text-forge-accent mb-2">Key Points</h4>
          <ul className="text-sm text-gray-300 space-y-1">
            <li>• Noise is <strong>persistent per profile</strong> - same hash on every page load</li>
            <li>• Each profile uses <strong>separate data directory</strong> for complete isolation</li>
            <li>• Screen dimensions match <strong>window.innerWidth/outerWidth</strong></li>
            <li>• <strong>Platform-specific fonts</strong> based on spoofed OS</li>
          </ul>
        </div>

        {/* Author Credit */}
        <div className="border-t border-gray-700 pt-4">
          <div className="flex items-center justify-between">
            <div className="text-sm text-gray-400">
              Developed by
            </div>
            <button
              onClick={openTelegram}
              className="flex items-center gap-2 px-4 py-2 bg-[#0088cc] hover:bg-[#0077b5] rounded-lg transition-colors"
            >
              <svg className="w-5 h-5" viewBox="0 0 24 24" fill="currentColor">
                <path d="M11.944 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0a12 12 0 0 0-.056 0zm4.962 7.224c.1-.002.321.023.465.14a.506.506 0 0 1 .171.325c.016.093.036.306.02.472-.18 1.898-.962 6.502-1.36 8.627-.168.9-.499 1.201-.82 1.23-.696.065-1.225-.46-1.9-.902-1.056-.693-1.653-1.124-2.678-1.8-1.185-.78-.417-1.21.258-1.91.177-.184 3.247-2.977 3.307-3.23.007-.032.014-.15-.056-.212s-.174-.041-.249-.024c-.106.024-1.793 1.14-5.061 3.345-.48.33-.913.49-1.302.48-.428-.008-1.252-.241-1.865-.44-.752-.245-1.349-.374-1.297-.789.027-.216.325-.437.893-.663 3.498-1.524 5.83-2.529 6.998-3.014 3.332-1.386 4.025-1.627 4.476-1.635z"/>
              </svg>
              <span className="font-medium">@NULLSHADExXx</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
