export default class Repo {
  constructor(req) {
    this.req = req;
  }

  async getEquipment() {
    let q = await axios.get('/api/equipment');

    return q.data.result.filter(e => e.info !== null);
  }

  async getSensors() {
    let q = await axios.get('/api/sensor');

    return q.data.result;
  }

  async getParameterTypes() {
    let q = await axios.get('/api/parameter-type');

    return q.data.result;
  }

  async getParameters(ts_start, ts_end) {
    let q = await axios.get(`/api/parameter?ts_start=${ts_start.toISOString()}&ts_end=${ts_end.toISOString()}`);

    return q.data.result;
  }
}
