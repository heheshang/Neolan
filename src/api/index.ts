// Tauri API wrapper module for type-safe command invocation

// ==================== Types ====================

export interface PeerDto {
  ip: string;
  port: number;
  username: string | null;
  hostname: string | null;
  nickname: string | null;
  avatar: string | null;
  groups: string[];
  status: "online" | "offline" | "away";
  displayName: string;
  lastSeen: number;
}

export interface PeerStats {
  total: number;
  online: number;
  offline: number;
}

export interface ConfigDto {
  username: string;
  hostname: string;
  bindIp: string;
  udpPort: number;
  tcpPortStart: number;
  tcpPortEnd: number;
  heartbeatInterval: number;
  peerTimeout: number;
  encryptionEnabled: boolean;
  encryptionKey?: string;
  offlineMessageRetentionDays: number;
  autoAcceptFiles: boolean;
  fileSaveDir: string;
  logLevel: "trace" | "debug" | "info" | "warn" | "error";
}

export interface AppEvent {
  type: string;
  data?: any;
}

export interface MessageDto {
  id: number;
  msgId: string;
  senderIp: string;
  senderName: string;
  receiverIp: string;
  msgType: number;
  content: string;
  isEncrypted: boolean;
  isOffline: boolean;
  sentAt: number;
  receivedAt?: number;
  createdAt: number;
  delivered?: boolean;  // Message receipt acknowledgment status
}

// ==================== Peer Commands ====================

export async function getPeers(): Promise<PeerDto[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<PeerDto[]>("get_peers");
}

export async function getOnlinePeers(): Promise<PeerDto[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<PeerDto[]>("get_online_peers");
}

export async function getPeerByIp(ip: string): Promise<PeerDto | null> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<PeerDto | null>("get_peer_by_ip", { ip });
}

export async function getPeerStats(): Promise<PeerStats> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<PeerStats>("get_peer_stats");
}

// ==================== Config Commands ====================

export async function getConfig(): Promise<ConfigDto> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<ConfigDto>("get_config");
}

export async function setConfig(config: ConfigDto): Promise<void> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<void>("set_config", { config });
}

export async function resetConfig(): Promise<ConfigDto> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<ConfigDto>("reset_config");
}

export async function getConfigValue(key: string): Promise<string | null> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<string | null>("get_config_value", { key });
}

export async function setConfigValue(key: string, value: string): Promise<void> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<void>("set_config_value", { key, value });
}

// ==================== Event Commands ====================

export async function pollEvents(): Promise<AppEvent[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<AppEvent[]>("poll_events");
}

// ==================== Message Commands ====================

export async function sendMessage(peerIp: string, content: string): Promise<string> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<string>("send_message", { peerIp, content });
}

export async function sendTextMessage(peerIp: string, content: string): Promise<string> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<string>("send_text_message", { peerIp, content });
}

export async function getMessages(peerIp: string, limit?: number): Promise<MessageDto[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<MessageDto[]>("get_messages", { peerIp, limit });
}

// ==================== File Transfer Commands ====================

export interface TaskDto {
  id: string;
  direction: "upload" | "download";
  peerIp: string;
  fileName: string;
  fileSize: number;
  md5: string;
  status: "pending" | "active" | "paused" | "completed" | "failed" | "cancelled";
  transferredBytes: number;
  progress: number;
  port?: number;
  error?: string;
  createdAt: number;
  updatedAt: number;
}

export interface FileTransferRequestEvent {
  requestId: string;
  senderIp: string;
  senderName: string;
  fileName: string;
  fileSize: number;
  md5: string;
  createdAt: number;
}

export async function get_file_transfers(): Promise<TaskDto[]> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<TaskDto[]>("get_file_transfers");
}

export async function cancel_file_transfer(taskId: string): Promise<void> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<void>("cancel_file_transfer", { taskId });
}

export async function accept_file_transfer(requestId: string, tcpPort: number): Promise<string> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<string>("accept_file_transfer", { requestId, tcpPort });
}

export async function reject_file_transfer(requestId: string): Promise<void> {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke<void>("reject_file_transfer", { requestId });
}
