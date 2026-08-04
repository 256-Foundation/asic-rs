"use strict";

const native = require("./native");

class Miner {
  constructor(inner) {
    this.inner = inner;
  }

  get ip() {
    return this.inner.ip;
  }

  get model() {
    return this.inner.model;
  }

  get make() {
    return this.inner.make;
  }

  get firmware() {
    return this.inner.firmware;
  }

  get deviceInfo() {
    return this.inner.deviceInfo;
  }

  get device_info() {
    return this.deviceInfo;
  }

  get supportsSetFaultLight() {
    return this.inner.supportsSetFaultLight;
  }

  get supports_set_fault_light() {
    return this.supportsSetFaultLight;
  }

  get supportsSetPowerLimit() {
    return this.inner.supportsSetPowerLimit;
  }

  get supports_set_power_limit() {
    return this.supportsSetPowerLimit;
  }

  get supportsSetTuningPercent() {
    return this.inner.supportsSetTuningPercent;
  }

  get supports_set_tuning_percent() {
    return this.supportsSetTuningPercent;
  }

  get supportsRestart() {
    return this.inner.supportsRestart;
  }

  get supports_restart() {
    return this.supportsRestart;
  }

  get supportsPause() {
    return this.inner.supportsPause;
  }

  get supports_pause() {
    return this.supportsPause;
  }

  get supportsResume() {
    return this.inner.supportsResume;
  }

  get supports_resume() {
    return this.supportsResume;
  }

  async setAuth(username, password) {
    return this.inner.setAuth(username, password);
  }

  async setToken(token) {
    return this.inner.setToken(token);
  }

  async getData() {
    return this.inner.getData();
  }

  async get_data() {
    return this.getData();
  }

  async getHashrate() {
    return this.inner.getHashrate();
  }

  async get_hashrate() {
    return this.getHashrate();
  }

  async getExpectedHashrate() {
    return this.inner.getExpectedHashrate();
  }

  async get_expected_hashrate() {
    return this.getExpectedHashrate();
  }

  async getFans() {
    return this.inner.getFans();
  }

  async get_fans() {
    return this.getFans();
  }

  async getPsuFans() {
    return this.inner.getPsuFans();
  }

  async get_psu_fans() {
    return this.getPsuFans();
  }

  async getPools() {
    return this.inner.getPools();
  }

  async get_pools() {
    return this.getPools();
  }

  async getMessages() {
    return this.inner.getMessages();
  }

  async get_messages() {
    return this.getMessages();
  }

  async getDeviceInfo() {
    return this.inner.getDeviceInfo();
  }

  async get_device_info() {
    return this.getDeviceInfo();
  }

  getMac() {
    return this.inner.getMac();
  }

  get_mac() {
    return this.getMac();
  }

  getSerialNumber() {
    return this.inner.getSerialNumber();
  }

  get_serial_number() {
    return this.getSerialNumber();
  }

  getHostname() {
    return this.inner.getHostname();
  }

  get_hostname() {
    return this.getHostname();
  }

  getApiVersion() {
    return this.inner.getApiVersion();
  }

  get_api_version() {
    return this.getApiVersion();
  }

  getFirmwareVersion() {
    return this.inner.getFirmwareVersion();
  }

  get_firmware_version() {
    return this.getFirmwareVersion();
  }

  getWattage() {
    return this.inner.getWattage();
  }

  get_wattage() {
    return this.getWattage();
  }

  getFluidTemperature() {
    return this.inner.getFluidTemperature();
  }

  get_fluid_temperature() {
    return this.getFluidTemperature();
  }

  getOutletFluidTemperature() {
    return this.inner.getOutletFluidTemperature();
  }

  get_outlet_fluid_temperature() {
    return this.getOutletFluidTemperature();
  }

  getTuningPercent() {
    return this.inner.getTuningPercent();
  }

  get_tuning_percent() {
    return this.getTuningPercent();
  }

  getIsMining() {
    return this.inner.getIsMining();
  }

  get_is_mining() {
    return this.getIsMining();
  }

  restart() {
    return this.inner.restart();
  }

  pause() {
    return this.inner.pause();
  }

  resume() {
    return this.inner.resume();
  }

  setFaultLight(fault) {
    return this.inner.setFaultLight(fault);
  }

  set_fault_light(fault) {
    return this.setFaultLight(fault);
  }

  setPowerLimit(watts) {
    return this.inner.setPowerLimit(watts);
  }

  set_power_limit(watts) {
    return this.setPowerLimit(watts);
  }

  setTuningPercent(percent) {
    return this.inner.setTuningPercent(percent);
  }

  set_tuning_percent(percent) {
    return this.setTuningPercent(percent);
  }
}

class MinerFactory {
  constructor(inner = new native.MinerFactory()) {
    this.inner = inner;
  }

  static fromSubnet(subnet) {
    return new MinerFactory(native.MinerFactory.fromSubnet(subnet));
  }

  static from_subnet(subnet) {
    return MinerFactory.fromSubnet(subnet);
  }

  static fromRange(range) {
    return new MinerFactory(native.MinerFactory.fromRange(range));
  }

  static from_range(range) {
    return MinerFactory.fromRange(range);
  }

  static fromOctets(octet1, octet2, octet3, octet4) {
    return new MinerFactory(
      native.MinerFactory.fromOctets(
        String(octet1),
        String(octet2),
        String(octet3),
        String(octet4),
      ),
    );
  }

  static from_octets(octet1, octet2, octet3, octet4) {
    return MinerFactory.fromOctets(octet1, octet2, octet3, octet4);
  }

  withSubnet(subnet) {
    this.inner.withSubnet(subnet);
    return this;
  }

  with_subnet(subnet) {
    return this.withSubnet(subnet);
  }

  withRange(range) {
    this.inner.withRange(range);
    return this;
  }

  with_range(range) {
    return this.withRange(range);
  }

  withOctets(octet1, octet2, octet3, octet4) {
    this.inner.withOctets(String(octet1), String(octet2), String(octet3), String(octet4));
    return this;
  }

  with_octets(octet1, octet2, octet3, octet4) {
    return this.withOctets(octet1, octet2, octet3, octet4);
  }

  withConcurrentLimit(concurrent) {
    this.inner.withConcurrentLimit(concurrent);
    return this;
  }

  with_concurrent_limit(concurrent) {
    return this.withConcurrentLimit(concurrent);
  }

  async getMiner(ip) {
    const miner = await this.inner.getMiner(ip);
    return miner ? new Miner(miner) : null;
  }

  get_miner(ip) {
    return this.getMiner(ip);
  }

  async scan() {
    return (await this.inner.scan()).map((miner) => new Miner(miner));
  }
}

module.exports = {
  Miner,
  MinerFactory,
};
