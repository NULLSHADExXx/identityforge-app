import { invoke } from '@tauri-apps/api/core';
import { 
  Profile, 
  ProfileWithStatus, 
  Fingerprint, 
  ApiResponse, 
  CreateProfileInput, 
  UpdateProfileInput,
  LaunchProfileInput,
  ProxyConfig
} from '../types/profile';

// Profile API
export async function getProfiles(): Promise<ApiResponse<ProfileWithStatus[]>> {
  return await invoke('get_profiles');
}

export async function getProfile(profileId: string): Promise<ApiResponse<Profile>> {
  return await invoke('get_profile', { profile_id: profileId });
}

export async function createProfile(input: CreateProfileInput): Promise<ApiResponse<Profile>> {
  return await invoke('create_profile', { input });
}

export async function updateProfile(input: UpdateProfileInput): Promise<ApiResponse<Profile>> {
  return await invoke('update_profile', { input });
}

export async function deleteProfile(profileId: string): Promise<ApiResponse<void>> {
  // Using camelCase to match #[tauri::command(rename_all = "camelCase")]
  return await invoke('delete_profile', { profileId });
}

export async function regenerateFingerprint(
  profileId: string, 
  platform?: string
): Promise<ApiResponse<Profile>> {
  // Using camelCase to match #[tauri::command(rename_all = "camelCase")]
  return await invoke('regenerate_fingerprint', { profileId, platform });
}

// Bulk operations
export async function deleteAllInactiveProfiles(): Promise<ApiResponse<number>> {
  return await invoke('delete_all_inactive_profiles');
}

export async function bulkCreateProfiles(
  count: number, 
  namePrefix: string, 
  platform?: string,
  defaultUrl?: string,
  proxy?: ProxyConfig
): Promise<ApiResponse<Profile[]>> {
  return await invoke('bulk_create_profiles', { count, namePrefix, platform, defaultUrl, proxy });
}

// Launcher API
export async function launchProfile(input: LaunchProfileInput): Promise<ApiResponse<string>> {
  return await invoke('launch_profile', { input });
}

export async function closeProfileWindow(profileId: string): Promise<ApiResponse<void>> {
  // Using camelCase to match #[tauri::command(rename_all = "camelCase")]
  return await invoke('close_profile_window', { profileId });
}

export async function getActiveProfiles(): Promise<ApiResponse<string[]>> {
  return await invoke('get_active_profiles');
}

export async function navigateProfile(profileId: string, url: string): Promise<ApiResponse<void>> {
  // Using camelCase to match #[tauri::command(rename_all = "camelCase")]
  return await invoke('navigate_profile', { profileId, url });
}

// Cookie API
export async function exportCookies(profileId: string): Promise<ApiResponse<string>> {
  return await invoke('export_cookies', { profileId });
}

export async function importCookies(profileId: string, cookiesJson: string): Promise<ApiResponse<void>> {
  return await invoke('import_cookies', { profileId, cookiesJson });
}

export async function clearCookies(profileId: string): Promise<ApiResponse<void>> {
  return await invoke('clear_cookies', { profileId });
}

// Settings API
export async function getSetting(key: string): Promise<ApiResponse<string | null>> {
  return await invoke('get_setting', { key });
}

export async function setSetting(key: string, value: string): Promise<ApiResponse<void>> {
  return await invoke('set_setting', { key, value });
}

// Utility API
export async function previewFingerprint(platform?: string): Promise<ApiResponse<Fingerprint>> {
  return await invoke('preview_fingerprint', { platform });
}
