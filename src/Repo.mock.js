export default class Repo {
  async getEquipment() {
    return new Promise(resolve => {
      let q = JSON.parse('{"status":"ok","result":[{"_id":"3b995869-faec-4886-b77a-4b2a4140172b","id":"F148B34C-2EA7B840-DE4947A9","info":null},{"_id":"1952474c-dc48-4b29-8f04-2cf3b80901f9","id":"2B2E780A-2F5740E2-EB549414","info":null},{"_id":"9e394464-855b-424c-b37e-c7dbb0b97cf2","id":"00610000-33373938-17473634","info":"living room"},{"_id":"8c52b768-b6c7-40a8-929d-07e2fcbdd5e6","id":"00600000-33373938-17473634","info":"bed room"},{"_id":"808318f3-a8f3-4f33-804f-62bdf5bfc63f","id":"005A0000-33373938-17473634","info":"kitchen"},{"_id":"457d1086-7ba4-40b0-bbf4-de74579b8d2e","id":"00590000-33373938-17473634","info":"office"}]}');
      resolve(q.result.filter(e => e.info !== null));
    });
  }

  async getSensors() {
    return new Promise(resolve => {
      let q = JSON.parse('{"status":"ok","result":[{"_id":"f510d892-ecae-4035-906b-6c4a1c59eba6","id":"0847","info":null},{"_id":"63ff4f6b-5d7c-41a1-9c47-d1b12a279441","id":"BE01","info":null},{"_id":"7d4c90b3-ba09-4ac0-b55b-00d7ed82f740","id":"BE02","info":null},{"_id":"53043ad1-feaf-4ec6-a64b-84df9b2ef658","id":"BE03","info":null},{"_id":"cf91346f-2521-4a2a-9c52-d475ebdec5e7","id":"3991","info":null},{"_id":"6139c847-d482-46f6-80fb-0e4854b0c080","id":"D0B1","info":null},{"_id":"22915533-2e8c-4302-9c31-aaccea01c17a","id":"6408","info":null},{"_id":"a8969515-fd92-4b4c-b49a-5b6f8880bcaa","id":"35CD","info":null},{"_id":"39ba1ad6-f249-4739-99a9-2b33feff8da8","id":"6EE5","info":null},{"_id":"e620408e-16f3-4fac-a84b-a36c2f3e8e6e","id":"45C7","info":null},{"_id":"e6e8769f-2486-4d94-ac80-5105edad4146","id":"F6B8","info":null},{"_id":"96913f09-cc2d-4514-beeb-e07a9492c4e2","id":"514B","info":null}]}');
      resolve(q.result);
    });
  }

  async getParameterTypes() {
    return new Promise(resolve => {
      let q = JSON.parse('{"status":"ok","result":[{"_id":"e3319ea0-3792-4e1d-a577-c7c0e484d7f4","id":"temperature","unit":"°C","info":null},{"_id":"68e9eb78-e38f-4c56-9e4a-882c2c29816f","id":"pressure","unit":"mbar","info":null},{"_id":"37842863-93c5-48b5-8c9b-7639ec58f489","id":"humidity","unit":"%RH","info":null}]}');
      resolve(q.result);
    });
  }

  async getParameters(ts_start, ts_end) {
    return new Promise(resolve => {
      let q = JSON.parse('{"status":"ok","result":[{"_id":"da37b378-feb1-4dac-9c69-a95861996472","_measurement_id":"da37b378-feb1-4dac-9c69-a95861996472","_parameter_type_id":"e3319ea0-3792-4e1d-a577-c7c0e484d7f4","_sensor_id":"53043ad1-feaf-4ec6-a64b-84df9b2ef658","val":20.780001,"ts":"2019-02-22T19:00:01.000Z","_equipment_id":"457d1086-7ba4-40b0-bbf4-de74579b8d2e","index":16883,"rssi":-31},{"_id":"da37b378-feb1-4dac-9c69-a95861996472","_measurement_id":"da37b378-feb1-4dac-9c69-a95861996472","_parameter_type_id":"37842863-93c5-48b5-8c9b-7639ec58f489","_sensor_id":"53043ad1-feaf-4ec6-a64b-84df9b2ef658","val":46.05957,"ts":"2019-02-22T19:00:01.000Z","_equipment_id":"457d1086-7ba4-40b0-bbf4-de74579b8d2e","index":16883,"rssi":-31},{"_id":"da37b378-feb1-4dac-9c69-a95861996472","_measurement_id":"da37b378-feb1-4dac-9c69-a95861996472","_parameter_type_id":"68e9eb78-e38f-4c56-9e4a-882c2c29816f","_sensor_id":"53043ad1-feaf-4ec6-a64b-84df9b2ef658","val":1021.15,"ts":"2019-02-22T19:00:01.000Z","_equipment_id":"457d1086-7ba4-40b0-bbf4-de74579b8d2e","index":16883,"rssi":-31},{"_id":"0152d30c-f87a-4bdc-9048-9bbe69e7f353","_measurement_id":"0152d30c-f87a-4bdc-9048-9bbe69e7f353","_parameter_type_id":"e3319ea0-3792-4e1d-a577-c7c0e484d7f4","_sensor_id":"63ff4f6b-5d7c-41a1-9c47-d1b12a279441","val":21.940001,"ts":"2019-02-22T19:00:13.000Z","_equipment_id":"808318f3-a8f3-4f33-804f-62bdf5bfc63f","index":12456,"rssi":-70},{"_id":"0152d30c-f87a-4bdc-9048-9bbe69e7f353","_measurement_id":"0152d30c-f87a-4bdc-9048-9bbe69e7f353","_parameter_type_id":"37842863-93c5-48b5-8c9b-7639ec58f489","_sensor_id":"63ff4f6b-5d7c-41a1-9c47-d1b12a279441","val":42.861328,"ts":"2019-02-22T19:00:13.000Z","_equipment_id":"808318f3-a8f3-4f33-804f-62bdf5bfc63f","index":12456,"rssi":-70},{"_id":"0152d30c-f87a-4bdc-9048-9bbe69e7f353","_measurement_id":"0152d30c-f87a-4bdc-9048-9bbe69e7f353","_parameter_type_id":"68e9eb78-e38f-4c56-9e4a-882c2c29816f","_sensor_id":"63ff4f6b-5d7c-41a1-9c47-d1b12a279441","val":1019.19,"ts":"2019-02-22T19:00:13.000Z","_equipment_id":"808318f3-a8f3-4f33-804f-62bdf5bfc63f","index":12456,"rssi":-70},{"_id":"c247c631-2e17-4290-8d12-c60e36bbd6a3","_measurement_id":"c247c631-2e17-4290-8d12-c60e36bbd6a3","_parameter_type_id":"e3319ea0-3792-4e1d-a577-c7c0e484d7f4","_sensor_id":"7d4c90b3-ba09-4ac0-b55b-00d7ed82f740","val":20.5,"ts":"2019-02-22T19:00:19.000Z","_equipment_id":"8c52b768-b6c7-40a8-929d-07e2fcbdd5e6","index":36565,"rssi":-62},{"_id":"c247c631-2e17-4290-8d12-c60e36bbd6a3","_measurement_id":"c247c631-2e17-4290-8d12-c60e36bbd6a3","_parameter_type_id":"37842863-93c5-48b5-8c9b-7639ec58f489","_sensor_id":"7d4c90b3-ba09-4ac0-b55b-00d7ed82f740","val":48.154297,"ts":"2019-02-22T19:00:19.000Z","_equipment_id":"8c52b768-b6c7-40a8-929d-07e2fcbdd5e6","index":36565,"rssi":-62},{"_id":"c247c631-2e17-4290-8d12-c60e36bbd6a3","_measurement_id":"c247c631-2e17-4290-8d12-c60e36bbd6a3","_parameter_type_id":"68e9eb78-e38f-4c56-9e4a-882c2c29816f","_sensor_id":"7d4c90b3-ba09-4ac0-b55b-00d7ed82f740","val":1020.12,"ts":"2019-02-22T19:00:19.000Z","_equipment_id":"8c52b768-b6c7-40a8-929d-07e2fcbdd5e6","index":36565,"rssi":-62},{"_id":"ade9b5c7-4459-46ff-9cc8-5d69876aaf46","_measurement_id":"ade9b5c7-4459-46ff-9cc8-5d69876aaf46","_parameter_type_id":"e3319ea0-3792-4e1d-a577-c7c0e484d7f4","_sensor_id":"f510d892-ecae-4035-906b-6c4a1c59eba6","val":20.1577,"ts":"2019-02-22T19:00:23.000Z","_equipment_id":"9e394464-855b-424c-b37e-c7dbb0b97cf2","index":740,"rssi":-69},{"_id":"ade9b5c7-4459-46ff-9cc8-5d69876aaf46","_measurement_id":"ade9b5c7-4459-46ff-9cc8-5d69876aaf46","_parameter_type_id":"37842863-93c5-48b5-8c9b-7639ec58f489","_sensor_id":"f510d892-ecae-4035-906b-6c4a1c59eba6","val":53.166199,"ts":"2019-02-22T19:00:23.000Z","_equipment_id":"9e394464-855b-424c-b37e-c7dbb0b97cf2","index":740,"rssi":-69},{"_id":"15591271-cd52-4fdd-a8de-c3c3df23740b","_measurement_id":"15591271-cd52-4fdd-a8de-c3c3df23740b","_parameter_type_id":"e3319ea0-3792-4e1d-a577-c7c0e484d7f4","_sensor_id":"53043ad1-feaf-4ec6-a64b-84df9b2ef658","val":20.780001,"ts":"2019-02-22T19:00:57.000Z","_equipment_id":"457d1086-7ba4-40b0-bbf4-de74579b8d2e","index":16884,"rssi":-31},{"_id":"15591271-cd52-4fdd-a8de-c3c3df23740b","_measurement_id":"15591271-cd52-4fdd-a8de-c3c3df23740b","_parameter_type_id":"37842863-93c5-48b5-8c9b-7639ec58f489","_sensor_id":"53043ad1-feaf-4ec6-a64b-84df9b2ef658","val":46.123047,"ts":"2019-02-22T19:00:57.000Z","_equipment_id":"457d1086-7ba4-40b0-bbf4-de74579b8d2e","index":16884,"rssi":-31},{"_id":"15591271-cd52-4fdd-a8de-c3c3df23740b","_measurement_id":"15591271-cd52-4fdd-a8de-c3c3df23740b","_parameter_type_id":"68e9eb78-e38f-4c56-9e4a-882c2c29816f","_sensor_id":"53043ad1-feaf-4ec6-a64b-84df9b2ef658","val":1021.21,"ts":"2019-02-22T19:00:57.000Z","_equipment_id":"457d1086-7ba4-40b0-bbf4-de74579b8d2e","index":16884,"rssi":-31}]}');
      resolve(q.result);
    });
  }
}
