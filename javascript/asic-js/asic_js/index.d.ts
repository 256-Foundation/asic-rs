export type HashAlgorithm = "SHA256" | "Scrypt" | "X11" | "Blake2S256" | "Kadena";
export type HashRateUnit =
  | "Hash"
  | "KiloHash"
  | "MegaHash"
  | "GigaHash"
  | "TeraHash"
  | "PetaHash"
  | "ExaHash"
  | "ZettaHash"
  | "YottaHash";
export type MessageSeverity = "Error" | "Warning" | "Info";
export type PoolScheme = "StratumV1" | "StratumV1SSL" | "StratumV2";
export type MiningMode = "Low" | "Normal" | "High";

export interface MinerHardware {
  fans?: number | null;
  boards?: Array<number | null> | null;
}

export interface DeviceInfo {
  make: string;
  model: string;
  hardware: MinerHardware;
  firmware: string;
  algo: HashAlgorithm;
}

export interface HashRate {
  value: number;
  unit: HashRateUnit;
  algo: string;
}

export interface FanData {
  position: number;
  rpm?: number | null;
}

export interface PoolURL {
  scheme: PoolScheme;
  host: string;
  port: number;
  pubkey?: string | null;
}

export interface PoolData {
  position?: number | null;
  url?: PoolURL | null;
  accepted_shares?: number | null;
  rejected_shares?: number | null;
  active?: boolean | null;
  alive?: boolean | null;
  user?: string | null;
}

export interface PoolGroupData {
  name: string;
  quota: number;
  pools: PoolData[];
}

export interface MinerComponent {
  type: "ControlBoard" | "HashBoard" | "Fan" | "PowerSupply";
  idx?: number;
  chip_idx?: number | null;
}

export interface MinerMessage {
  timestamp: number;
  code: number;
  message: string;
  severity: MessageSeverity;
  component?: MinerComponent | null;
}

export type TuningTarget =
  | { Power: number }
  | { HashRate: HashRate }
  | { MiningMode: MiningMode };

export interface TuningCapabilities {
  power?: {
    default?: TuningTarget | null;
    minimum?: TuningTarget | null;
    maximum?: TuningTarget | null;
  } | null;
  hashrate?: {
    default?: TuningTarget | null;
    minimum?: TuningTarget | null;
    maximum?: TuningTarget | null;
  } | null;
  presets?: {
    default?: TuningTarget | null;
    presets: TuningTarget[];
  } | null;
}

export interface ChipData {
  position: number;
  hashrate?: HashRate | null;
  temperature?: number | null;
  voltage?: number | null;
  frequency?: number | null;
  tuned?: boolean | null;
  working?: boolean | null;
}

export interface BoardData {
  position: number;
  hashrate?: HashRate | null;
  expected_hashrate?: HashRate | null;
  board_temperature?: number | null;
  inlet_chip_temperature?: number | null;
  outlet_chip_temperature?: number | null;
  expected_chips?: number | null;
  working_chips?: number | null;
  serial_number?: string | null;
  chips: ChipData[];
  voltage?: number | null;
  frequency?: number | null;
  tuned?: boolean | null;
  active?: boolean | null;
}

export interface MinerControlBoard {
  known: boolean;
  name: string;
}

export interface MinerData {
  schema_version: string;
  timestamp: number;
  ip: string;
  mac?: string | null;
  device_info: DeviceInfo;
  serial_number?: string | null;
  hostname?: string | null;
  api_version?: string | null;
  firmware_version?: string | null;
  control_board_version?: MinerControlBoard | null;
  expected_hashboards?: number | null;
  hashboards: BoardData[];
  hashrate?: HashRate | null;
  expected_hashrate?: HashRate | null;
  expected_chips?: number | null;
  total_chips?: number | null;
  expected_fans?: number | null;
  fans: FanData[];
  psu_fans: FanData[];
  average_temperature?: number | null;
  fluid_temperature?: number | null;
  outlet_fluid_temperature?: number | null;
  wattage?: number | null;
  tuning_percent?: number | null;
  tuning_target?: TuningTarget | null;
  scaled_tuning_target?: TuningTarget | null;
  tuning_capabilities?: TuningCapabilities | null;
  efficiency?: number | null;
  light_flashing?: boolean | null;
  messages: MinerMessage[];
  uptime?: { secs: number; nanos: number } | null;
  is_mining: boolean;
  pools: PoolGroupData[];
}

export class MinerFactory {
  constructor();
  static fromSubnet(subnet: string): MinerFactory;
  static fromRange(range: string): MinerFactory;
  static fromOctets(octet1: string | number, octet2: string | number, octet3: string | number, octet4: string | number): MinerFactory;
  withSubnet(subnet: string): this;
  withRange(range: string): this;
  withOctets(octet1: string | number, octet2: string | number, octet3: string | number, octet4: string | number): this;
  withConcurrentLimit(concurrent: number): this;
  getMiner(ip: string): Promise<Miner | null>;
  scan(): Promise<Miner[]>;
}

export class Miner {
  readonly ip: string;
  readonly model: string;
  readonly make: string;
  readonly firmware: string;
  readonly deviceInfo: DeviceInfo;
  readonly supportsSetFaultLight: boolean;
  readonly supportsSetPowerLimit: boolean;
  readonly supportsSetTuningPercent: boolean;
  readonly supportsRestart: boolean;
  readonly supportsPause: boolean;
  readonly supportsResume: boolean;
  setAuth(username: string, password: string): Promise<void>;
  setToken(token: string): Promise<void>;
  getData(): Promise<MinerData>;
  getHashrate(): Promise<HashRate | null>;
  getExpectedHashrate(): Promise<HashRate | null>;
  getFans(): Promise<FanData[]>;
  getPsuFans(): Promise<FanData[]>;
  getPools(): Promise<PoolGroupData[]>;
  getMessages(): Promise<MinerMessage[]>;
  getDeviceInfo(): Promise<DeviceInfo>;
  getMac(): Promise<string | null>;
  getSerialNumber(): Promise<string | null>;
  getHostname(): Promise<string | null>;
  getApiVersion(): Promise<string | null>;
  getFirmwareVersion(): Promise<string | null>;
  getWattage(): Promise<number | null>;
  getFluidTemperature(): Promise<number | null>;
  getOutletFluidTemperature(): Promise<number | null>;
  getTuningPercent(): Promise<number | null>;
  getIsMining(): Promise<boolean>;
  restart(): Promise<boolean>;
  pause(): Promise<boolean>;
  resume(): Promise<boolean>;
  setFaultLight(fault: boolean): Promise<boolean>;
  setPowerLimit(watts: number): Promise<boolean>;
  setTuningPercent(percent: number): Promise<boolean>;
}
