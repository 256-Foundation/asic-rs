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

  get supportsSetFaultLight() {
    return this.inner.supportsSetFaultLight;
  }

  get supportsSetPowerLimit() {
    return this.inner.supportsSetPowerLimit;
  }

  get supportsSetTuningPercent() {
    return this.inner.supportsSetTuningPercent;
  }

  get supportsRestart() {
    return this.inner.supportsRestart;
  }

  get supportsPause() {
    return this.inner.supportsPause;
  }

  get supportsResume() {
    return this.inner.supportsResume;
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

  async getHashrate() {
    return this.inner.getHashrate();
  }

  async getExpectedHashrate() {
    return this.inner.getExpectedHashrate();
  }

  async getFans() {
    return this.inner.getFans();
  }

  async getPsuFans() {
    return this.inner.getPsuFans();
  }

  async getPools() {
    return this.inner.getPools();
  }

  async getMessages() {
    return this.inner.getMessages();
  }

  async getDeviceInfo() {
    return this.inner.getDeviceInfo();
  }

  getMac() {
    return this.inner.getMac();
  }

  getSerialNumber() {
    return this.inner.getSerialNumber();
  }

  getHostname() {
    return this.inner.getHostname();
  }

  getApiVersion() {
    return this.inner.getApiVersion();
  }

  getFirmwareVersion() {
    return this.inner.getFirmwareVersion();
  }

  getWattage() {
    return this.inner.getWattage();
  }

  getFluidTemperature() {
    return this.inner.getFluidTemperature();
  }

  getOutletFluidTemperature() {
    return this.inner.getOutletFluidTemperature();
  }

  getTuningPercent() {
    return this.inner.getTuningPercent();
  }

  getIsMining() {
    return this.inner.getIsMining();
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

  setPowerLimit(watts) {
    return this.inner.setPowerLimit(watts);
  }

  setTuningPercent(percent) {
    return this.inner.setTuningPercent(percent);
  }
}

class MinerFactory {
  constructor(inner = new native.MinerFactory()) {
    this.inner = inner;
  }

  static fromSubnet(subnet) {
    return new MinerFactory(native.MinerFactory.fromSubnet(subnet));
  }

  static fromRange(range) {
    return new MinerFactory(native.MinerFactory.fromRange(range));
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

  withSubnet(subnet) {
    this.inner.withSubnet(subnet);
    return this;
  }

  withRange(range) {
    this.inner.withRange(range);
    return this;
  }

  withOctets(octet1, octet2, octet3, octet4) {
    this.inner.withOctets(String(octet1), String(octet2), String(octet3), String(octet4));
    return this;
  }

  withConcurrentLimit(concurrent) {
    this.inner.withConcurrentLimit(concurrent);
    return this;
  }

  async getMiner(ip) {
    const miner = await this.inner.getMiner(ip);
    return miner ? new Miner(miner) : null;
  }

  async scan() {
    return (await this.inner.scan()).map((miner) => new Miner(miner));
  }
}

module.exports = {
  Miner,
  MinerFactory,
};
